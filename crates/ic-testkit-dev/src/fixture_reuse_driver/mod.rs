//! Repository-local operator tooling. Owns builds, worker runs and evidence files;
//! the library and workload retain server lifecycle and measurement semantics.

mod arguments;
mod preparation;
mod report;
mod worker;

use std::{
    error::Error as StdError,
    ffi::OsString,
    fs::File,
    io::{self, Seek as _, Write as _},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};

use ic_build::artifacts::WasmBuildError;
use ic_host_artifacts::artifact::ArtifactError;
use ic_host_fs::{
    durable::{
        NamedWriteError, PublicationMode, WriteOptions, write_existing_parent_validated_with,
    },
    read::read_opened_file,
    scratch::TemporaryFile,
};
use ic_host_process::{
    provenance::GitError,
    sampling,
    signal::{SignalActionError, SignalActions},
    tool::{
        CompletedExecution, OutputLimit, OutputLimits, ResolutionError, ToolError,
        run_group_command,
    },
};
use ic_host_text::table::{self, Alignment};

use arguments::{Options, parse_arguments};
use preparation::prepare;
use report::{DriverReport, MeasuredRun, Summary, summarize};

const WORKSPACE_DISCOVERY_LIMITS: OutputLimits = OutputLimits {
    stdout: OutputLimit::Terminate(64 * 1024),
    stderr: OutputLimit::Terminate(64 * 1024),
    timeout: None,
};
const SAMPLE_INTERVAL: Duration = Duration::from_millis(10);
const REPORT_LIMIT: u64 = 16 * 1024 * 1024;
static INTERRUPTED: AtomicBool = AtomicBool::new(false);

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Arguments(#[from] clap::Error),
    #[error("invalid benchmark argument: {0}")]
    InvalidArgument(String),
    #[error("invalid benchmark data: {0}")]
    InvalidData(&'static str),
    #[error("invalid benchmark data: expected {0}")]
    ServerVersion(String),
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Statistics(#[source] crate::statistics::SampleStatisticsError),
    #[error("{0}")]
    Artifact(#[source] ArtifactError),
    #[error("{0}")]
    Resolution(#[source] ResolutionError),
    #[error("{0}")]
    Build(#[source] WasmBuildError),
    #[error("{0}")]
    CargoObservation(#[source] Box<dyn StdError>),
    #[error("{0}")]
    Sampling(#[from] sampling::SamplingError),
    #[error("{0}")]
    Signal(#[from] SignalActionError),
    #[error("{0}")]
    Tool(#[source] ToolError),
    #[error("{0}")]
    Git(#[source] GitError),
    #[error("{} exited with {status}", program.display())]
    CommandFailed {
        program: PathBuf,
        status: ExitStatus,
    },
    #[error("benchmark interrupted")]
    Interrupted,
}

#[cfg(unix)]
extern "C" fn mark_interrupted(_signal: libc::c_int) {
    // The handler performs only a lock-free flag update, never I/O or teardown.
    INTERRUPTED.store(true, Ordering::Relaxed);
}

#[cfg(unix)]
#[expect(
    unsafe_code,
    reason = "bounded native operation with the local SAFETY contract below"
)]
fn install_interrupt_handler() -> Result<SignalActions, SignalActionError> {
    INTERRUPTED.store(false, Ordering::Relaxed);
    // SAFETY: this standalone driver exclusively owns SIGINT through guard
    // drop. Nested test guards restore in reverse order in an isolated process.
    // The static callback only stores a lock-free atomic flag.
    unsafe { SignalActions::install(&[libc::SIGINT], mark_interrupted) }
}

fn check_interrupted() -> Result<(), Error> {
    if INTERRUPTED.load(Ordering::Relaxed) {
        Err(Error::Interrupted)
    } else {
        Ok(())
    }
}

fn publish_report(
    output: &Path,
    write: impl FnOnce(&mut File) -> Result<(), Error>,
) -> Result<(), Error> {
    check_interrupted()?;
    let error = match write_existing_parent_validated_with(
        output,
        WriteOptions {
            mode: PublicationMode::Replace,
            permissions: 0o600,
        },
        write,
        |()| check_interrupted(),
    ) {
        Ok(()) => return Ok(()),
        Err(error) => error,
    };
    let (source, cleanup_error) = match error {
        NamedWriteError::Producer {
            source,
            cleanup_error,
        } => (source, cleanup_error),
        NamedWriteError::BeforePublication {
            source,
            cleanup_error,
        } => (source.into(), cleanup_error),
        NamedWriteError::AfterPublication { source } => {
            // The report is complete and visible. This driver promises atomic
            // visibility, not power-loss durability; do not report a failed
            // replacement or invite a retry after publication has occurred.
            eprintln!("report published; final durability or cleanup is unconfirmed: {source}");
            return Ok(());
        }
    };
    if let Some(error) = cleanup_error {
        eprintln!("could not clean report staging: {error}");
    }
    Err(source)
}

struct CapturedRun {
    report: crate::fixture_reuse_report::WorkerReport,
    rss: report::RssMeasurements,
}

fn measure(
    command: &mut Command,
    mut snapshot: impl FnMut() -> Result<sampling::Snapshot, sampling::SamplingError>,
) -> Result<CapturedRun, Error> {
    check_interrupted()?;
    let temporary = TemporaryFile::new_in(&std::env::temp_dir(), "ic-testkit-worker-output")?;
    let mut output = temporary.as_file().try_clone()?;
    // Keep the anonymous descriptor contract: unlink before the worker starts.
    temporary.close()?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::from(output.try_clone()?));
    let mut worker = worker::Worker::spawn(command)?;
    let observed = (|| -> Result<_, Error> {
        let mut rss = report::RssMeasurements::default();
        while worker.try_wait()?.is_none() && !INTERRUPTED.load(Ordering::Relaxed) {
            let (resident, processes) = sampling::tree_rss(worker.id(), &snapshot()?)?;
            rss.sampled_peak_tree_rss_bytes = rss.sampled_peak_tree_rss_bytes.max(resident);
            rss.peak_tree_process_count = rss.peak_tree_process_count.max(processes);
            rss.rss_samples += 1;
            thread::sleep(SAMPLE_INTERVAL);
        }
        Ok(rss)
    })();
    let status = worker.finish(observed.is_err() || INTERRUPTED.load(Ordering::Relaxed))?;
    check_interrupted()?;
    let rss = observed?;
    if !status.success() {
        return Err(Error::CommandFailed {
            program: command.get_program().into(),
            status,
        });
    }
    output.rewind()?;
    let bytes = read_opened_file(
        output,
        usize::try_from(REPORT_LIMIT).expect("report limit fits supported hosts"),
    )
    .map_err(Error::Artifact)?;
    let report = serde_json::from_slice(&bytes)?;
    Ok(CapturedRun { report, rss })
}

fn execute_command(command: &mut Command) -> Result<CompletedExecution, Error> {
    check_interrupted()?;
    run_group_command(
        command.stdin(Stdio::null()).stderr(Stdio::inherit()),
        OutputLimits {
            stdout: OutputLimit::Terminate(64 * 1024),
            stderr: OutputLimit::Terminate(0),
            timeout: None,
        },
        ic_host_process::child::CleanupPolicy::KillAndWait,
        None,
        || INTERRUPTED.load(Ordering::Relaxed),
    )
    .and_then(CompletedExecution::require_success)
    .map_err(Error::Tool)
}

fn text_command(command: &mut Command) -> Result<String, Error> {
    let output = execute_command(command.stdout(Stdio::piped()))?;
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_owned())
        .map_err(|_| Error::InvalidData("tool output is not UTF-8"))
}

fn print_summary(summaries: &[(&str, Summary)]) {
    let mut rows = Vec::with_capacity(summaries.len());
    for (label, summary) in summaries {
        let restore = summary
            .restore()
            .map_or_else(|| "-".into(), |restore| format!("{:.2}", restore.mean));
        rows.push([
            (*label).to_owned(),
            format!("{:.2}", summary.work_wall_ms.p50),
            format!("{:.2}", summary.fixture_wall_ms.p50),
            format!("{:.2}", summary.iterations_per_second.p50),
            restore,
            format!("{:.2}", summary.wait().mean),
            format!(
                "{:.2}",
                summary.sampled_peak_tree_rss_bytes.p50 / 1024.0 / 1024.0
            ),
        ]);
    }
    println!(
        "{}",
        table::render(
            &[
                "Mode",
                "work ms",
                "incl. setup/drop ms",
                "tasks/s",
                "restore mean ms",
                "wait mean ms",
                "peak RSS MiB"
            ],
            &rows,
            &[
                Alignment::Left,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right
            ],
            table::Options {
                column_gap: 2,
                header_separator: false
            },
        )
    );
}

pub fn run() -> Result<(), Error> {
    let options = parse_arguments(std::env::args_os().skip(1))?;
    if !cfg!(any(target_os = "linux", target_os = "macos")) {
        return Err(Error::InvalidArgument(
            "native sampling requires Linux or macOS".into(),
        ));
    }
    #[cfg(unix)]
    let _interrupt_handler = install_interrupt_handler()?;
    let inputs = prepare(&options)?;
    let mut cases = options
        .benchmark_cases()
        .into_iter()
        .map(|case| (case, Vec::new()))
        .collect::<Vec<_>>();
    for repeat in 0..options.repeats.get() {
        let rotation = repeat % cases.len();
        let (before, after) = cases.split_at_mut(rotation);
        for (case, runs) in after.iter_mut().chain(before) {
            check_interrupted()?;
            eprintln!("Run {}/{}: {}", repeat + 1, options.repeats, case.label);
            let invocation = case.worker_arguments(
                &options,
                inputs.wasm.record().artifacts()[0].clone(),
                inputs.server.clone(),
            );
            let captured = measure(
                Command::new(&inputs.worker)
                    .args(<[OsString; 7]>::from(&invocation))
                    .current_dir(&inputs.root),
                || sampling::snapshot(64 * 1024, options.rss_limits()),
            )?;
            let measured =
                MeasuredRun::from_worker(captured.report, &invocation.workload, captured.rss)?;
            runs.push(measured);
        }
    }
    let summaries = cases
        .iter()
        .map(|(case, runs)| Ok((case.label.as_str(), summarize(runs)?)))
        .collect::<Result<Vec<_>, Error>>()?;
    check_interrupted()?;
    if let Some(output) = &options.output {
        let document = DriverReport {
            format: "ic-testkit-fixture-benchmark-v1",
            provenance: &inputs.provenance,
            runs: cases
                .iter()
                .map(|(case, runs)| (case.label.as_str(), runs.as_slice()))
                .collect(),
            summary: summaries
                .iter()
                .map(|(label, summary)| (*label, summary))
                .collect(),
        };
        publish_report(output, |file| {
            serde_json::to_writer_pretty(&mut *file, &document)?;
            file.write_all(b"\n")?;
            Ok(())
        })?;
    }
    print_summary(&summaries);
    Ok(())
}

#[cfg(test)]
mod tests;
