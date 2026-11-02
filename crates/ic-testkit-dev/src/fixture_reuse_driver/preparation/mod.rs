//! Admit benchmark inputs and retain build/source provenance before scheduling runs.

use super::report::Provenance;
use super::{
    Error, INTERRUPTED, Options, SAMPLE_INTERVAL, WORKSPACE_DISCOVERY_LIMITS, check_interrupted,
    text_command,
};
use crate::fixture_reuse_protocol::Mode;
use ic_build::{
    artifacts::{WasmBuildOutcome, WasmBuildSpec, build_wasm_canisters_cached},
    cargo::{
        native::{NativeExecutableSpec, NativeTargetKind, build_native_executable},
        observation::{decode_workspace_root, workspace_root_command},
        selection::CargoSelection,
    },
};
use ic_host_fs::read::hash_file;
use ic_host_process::{
    child::OwnedChild,
    provenance::{GitObservations, IgnoreSubmodules, StatusOptions, UntrackedFiles, capture_git},
    sampling,
    tool::{
        AdmittedTool, CompletedExecution, ExecutionContext, OutputLimit, OutputLimits, VersionSpec,
        capture_command, resolve_executable,
    },
};
use std::{
    collections::BTreeMap,
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::Ordering,
};

pub(super) const WASM_LIMIT: u64 = 128 * 1024 * 1024;

pub(super) struct Inputs {
    pub(super) root: PathBuf,
    pub(super) server: PathBuf,
    pub(super) worker: PathBuf,
    pub(super) wasm: WasmBuildOutcome,
    pub(super) provenance: Provenance,
}

fn tool_program(program: OsString) -> Result<PathBuf, Error> {
    if program.is_empty() {
        return Err(
            io::Error::new(io::ErrorKind::InvalidInput, "tool name must not be empty").into(),
        );
    }
    let path = PathBuf::from(program);
    // Preserve the invocation name used by multicall proxies such as rustup.
    // Anchor explicit relative paths before commands switch to the workspace;
    // bare names retain Command's native PATH lookup.
    if path.is_relative() && path.components().count() > 1 {
        Ok(std::env::current_dir()?.join(path))
    } else {
        Ok(path)
    }
}

fn cargo_command(cargo: &Path, root: &Path, rustc: Option<&Path>) -> Command {
    let mut command = Command::new(cargo);
    command.current_dir(root);
    if let Some(rustc) = rustc {
        command.env("RUSTC", rustc);
    }
    command
}

pub(super) fn server_program(path: &Path, root: &Path) -> Result<PathBuf, Error> {
    // Preserve native path errors, then anchor exact selection against PATH lookup.
    // Host owns executable checks and the single canonicalization.
    fs::metadata(path)?;
    let path = std::env::current_dir()?.join(path);
    resolve_executable(&path, root, &[]).map_err(Error::Resolution)
}

pub(super) fn prepare(options: &Options) -> Result<Inputs, Error> {
    let cargo = tool_program(std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")))?;
    let rustc = std::env::var_os("RUSTC").map(tool_program).transpose()?;
    let mut workspace = workspace_root_command(
        env!("CARGO_MANIFEST_DIR"),
        cargo_command(&cargo, &std::env::current_dir()?, rustc.as_deref()),
    )?;
    let output = capture_command(&mut workspace, WORKSPACE_DISCOVERY_LIMITS)
        .and_then(CompletedExecution::require_success)
        .and_then(CompletedExecution::require_complete)
        .map_err(Error::Tool)?;
    let root = decode_workspace_root(&output.stdout)?;
    if !root
        .join("crates/ic-testkit-perf-probe/Cargo.toml")
        .is_file()
    {
        return Err(Error::InvalidArgument(
            "benchmark requires the repository's perf_probe checkout".into(),
        ));
    }
    let server = server_program(&options.server, &root)?;
    let server_version = text_command(Command::new(&server).arg("--version"))?;
    if !ic_testkit::pic::supports_pocket_ic_server(server_version.as_bytes()) {
        return Err(Error::ServerVersion(
            ic_testkit::pic::pocket_ic_server_requirement(),
        ));
    }
    check_interrupted()?;
    let native = build_native_executable(
        &NativeExecutableSpec {
            package: "ic-testkit-dev",
            target_name: "fixture_reuse_benchmark",
            target_kind: NativeTargetKind::Binary,
            selection: CargoSelection {
                profile: Some(options.profile.clone()),
                locked: true,
                offline: true,
                ..CargoSelection::default()
            },
            max_message_bytes: 16 * 1024 * 1024,
        },
        || cargo_command(&cargo, &root, rustc.as_deref()),
        io::stderr().lock(),
        || {
            if INTERRUPTED.load(Ordering::Relaxed) {
                Err(io::Error::from(io::ErrorKind::Interrupted))
            } else {
                Ok(())
            }
        },
        |command| OwnedChild::spawn(command, ic_host_process::child::CleanupPolicy::KillAndWait),
        || INTERRUPTED.load(Ordering::Relaxed),
    )
    .map_err(|error| Error::CargoObservation(error))?;
    let target = &native.target_directory;
    let mut environment: BTreeMap<_, _> = std::env::vars_os().collect();
    if let Some(rustc) = &rustc {
        environment.insert("RUSTC".into(), rustc.as_os_str().to_owned());
    }
    let spec = WasmBuildSpec::new(
        &root,
        &target.join("ic-testkit-fixture-benchmark-wasm"),
        &["ic-testkit-perf-probe"],
        environment,
        CONFIGURATION_LIMITS,
    )
    .with_cargo_program(cargo)
    .with_cargo_selection(CargoSelection {
        locked: true,
        offline: true,
        ..CargoSelection::default()
    })
    .with_shared_incremental_target(target.join("ic-testkit-incremental"));
    check_interrupted()?;
    let wasm = build_wasm_canisters_cached(&spec, |command| {
        OwnedChild::spawn(command, ic_host_process::child::CleanupPolicy::KillAndWait)
    });
    check_interrupted()?;
    let wasm = wasm.map_err(Error::Build)?;
    let worker = native.executable;
    let provenance = provenance(
        &root,
        options,
        &server,
        &server_version,
        &wasm.record().artifacts()[0],
        wasm.record().facts(),
    )?;
    Ok(Inputs {
        root,
        server,
        worker,
        wasm,
        provenance,
    })
}

// Git is selected from the operator's installed PATH, then admitted and
// rechecked by Host. The observed identity is not a trusted installation pin.
pub(super) fn git_provenance(root: &Path, limits: OutputLimits) -> Result<GitObservations, Error> {
    check_interrupted()?;
    let environment = std::env::vars_os().collect::<Vec<_>>();
    let search = std::env::var_os("PATH")
        .map(|path| std::env::split_paths(&path).collect::<Vec<_>>())
        .unwrap_or_default();
    let executable =
        resolve_executable(Path::new("git"), root, &search).map_err(Error::Resolution)?;
    let context = ExecutionContext {
        current_dir: root,
        environment: &environment,
    };
    let git = AdmittedTool::observe_version(
        &VersionSpec {
            executable: &executable,
            executable_bytes: 128 * 1024 * 1024,
            version_arguments: &["--version".into()],
        },
        &context,
        limits,
    )
    .map_err(Error::Tool)?;
    let observed = capture_git(
        &git,
        &context,
        StatusOptions {
            untracked: UntrackedFiles::Normal,
            ignore_submodules: IgnoreSubmodules::None,
        },
        limits,
    )
    .map_err(Error::Git)?;
    check_interrupted()?;
    Ok(observed)
}

#[expect(
    unsafe_code,
    reason = "bounded native operation with the local SAFETY contract below"
)]
fn provenance(
    root: &Path,
    options: &Options,
    server: &Path,
    server_version: &str,
    wasm: &Path,
    facts: &ic_build::artifacts::CargoBuildFacts,
) -> Result<Provenance, Error> {
    // Capture the selected build and source facts before running cases.
    let git = git_provenance(
        root,
        OutputLimits {
            stdout: OutputLimit::Terminate(16 * 1024 * 1024),
            stderr: OutputLimit::Terminate(64 * 1024),
            timeout: None,
        },
    )?;
    // SAFETY: sysconf reads the host's online CPU count with no pointer arguments.
    #[cfg(unix)]
    let cpus = unsafe { libc::sysconf(libc::_SC_NPROCESSORS_ONLN) };
    #[cfg(not(unix))]
    let cpus = 0;
    let cpus = u64::try_from(cpus)
        .ok()
        .filter(|count| *count > 0)
        .ok_or(Error::InvalidData("logical CPU count unavailable"))?;
    let rustc = std::str::from_utf8(&facts.rustc_identity)
        .map_err(|_| Error::InvalidData("captured compiler identity is not UTF-8"))?
        .lines()
        .next()
        .filter(|value| !value.is_empty())
        .ok_or(Error::InvalidData("captured compiler identity is empty"))?;
    Ok(Provenance {
        server: server
            .to_str()
            .ok_or(Error::InvalidData("server path is not UTF-8"))?
            .to_owned(),
        server_version: server_version.to_owned(),
        platform: text_command(Command::new("uname").arg("-srm"))?,
        logical_cpus: cpus,
        rustc: rustc.to_owned(),
        revision: git.revision().to_owned(),
        working_tree_dirty: git.is_dirty(),
        host_profile: options.profile.clone(),
        wasm_profile: facts
            .selection
            .profile
            .as_deref()
            .unwrap_or("dev")
            .to_owned(),
        wasm_sha256: hash_file(wasm, WASM_LIMIT)
            .map_err(Error::Artifact)?
            .sha256
            .to_string(),
        rss_sample_interval_ms: SAMPLE_INTERVAL.as_millis(),
        rss_sampler: sampling::source(),
        rss_command_stdout_bytes: options.rss_limits().stdout.retained_bytes(),
        rss_command_stderr_bytes: options.rss_limits().stderr.retained_bytes(),
        iterations: options.iterations.get(),
        workers: options.workers.get(),
        repeats: options.repeats.get(),
        state_bytes_per_canister: options.state_bytes,
        modes: options.modes.iter().map(|mode| mode.as_str()).collect(),
        pool_capacities: if options.modes.contains(&Mode::Pooled) {
            options
                .capacities
                .iter()
                .map(|capacity| capacity.get())
                .collect()
        } else {
            vec![]
        },
    })
}

// This caller admits small configuration trees; libraries do not select this policy.
const CONFIGURATION_LIMITS: ic_build::cargo::environment::ConfigurationLimits =
    ic_build::cargo::environment::ConfigurationLimits {
        file_bytes: 1024 * 1024,
        total_bytes: 16 * 1024 * 1024,
        files: 128,
        depth: 32,
    };
