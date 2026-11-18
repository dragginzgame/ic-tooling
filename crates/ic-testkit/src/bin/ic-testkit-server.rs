//! Explicit simulator setup, offline admission and command-scoped lifetime.
use std::{
    ffi::OsString,
    fs::{self, DirBuilder, File},
    io::{self, Write},
    os::unix::fs::DirBuilderExt,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use clap::{Parser, Subcommand};
use ic_host_process::{Failure, Input, Ownership, Policy, Report, Stderr, Stream};
use ic_testkit::server::{self, Limits, PreparedServer, Server, Stopped};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
static CANCELLED: AtomicBool = AtomicBool::new(false);

#[derive(Parser)]
#[command(
    name = "ic-testkit-server",
    about = "Pinned PocketIC setup, offline admission and bounded command execution",
    version
)]
struct Cli {
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Subcommand)]
enum Operation {
    /// Explicit network preparation; retains this invocation's download evidence.
    Setup {
        directory: PathBuf,
        evidence: PathBuf,
        curl: PathBuf,
        gzip: PathBuf,
    },
    /// Offline admission only; prints the verified executable path.
    Check { directory: PathBuf },
    /// Keep a server alive for one bounded child command, then stop and reap it.
    Run {
        directory: PathBuf,
        evidence: PathBuf,
        #[arg(long, default_value_t = 300, value_parser = clap::value_parser!(u64).range(1..=3600))]
        ttl: u64,
        #[arg(last = true, required = true, num_args = 1..)]
        command: Vec<OsString>,
    },
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = ctrlc::set_handler(|| CANCELLED.store(true, Ordering::Relaxed))
        .map_err(Into::into)
        .and_then(|()| execute(cli.operation));
    match result {
        Ok(code) => ExitCode::from(code),
        Err(error) => {
            eprintln!(
                "ic-testkit-server failed: {}",
                display(error.to_string().as_bytes())
            );
            ExitCode::FAILURE
        }
    }
}

fn execute(operation: Operation) -> Result<u8> {
    match operation {
        Operation::Setup {
            directory,
            evidence,
            curl,
            gzip,
        } => {
            for path in [&directory, &evidence, &curl, &gzip] {
                absolute(path)?;
            }
            new_evidence(&evidence)?;
            if !directory.try_exists()? {
                DirBuilder::new().mode(0o700).create(&directory)?;
            }
            let archive = evidence.join("archive.gz");
            let asset = server::host_asset()?;
            let mut command = Command::new(curl);
            command
                .args([
                    "--disable",
                    "--fail",
                    "--location",
                    "--silent",
                    "--show-error",
                    "--proto",
                    "=https",
                    "--proto-redir",
                    "=https",
                    "--max-time",
                    "120",
                    "--max-filesize",
                    "67108864",
                    "--output",
                ])
                .arg(&archive)
                .arg(asset.url);
            let report = capture(&mut command, &evidence, "download", 130, 131_072, false)?;
            let code = status(&report);
            if code != 0 {
                return Ok(code);
            }
            // Only the library admits archive/executable bytes and publishes them.
            let prepared = server::prepare(&archive, &directory, &gzip)
                .and_then(|_| PreparedServer::check(&directory));
            if let Err(error) = fs::write(evidence.join("prepare.txt"), format!("{prepared:?}\n")) {
                eprintln!(
                    "preparation evidence failed: {}",
                    display(error.to_string().as_bytes())
                );
                if prepared.is_ok() {
                    return Err(error.into());
                }
            }
            if let Err(server::Error::Decompression(stopped)) = &prepared {
                if let Err(error) = retain_server(&evidence, stopped) {
                    eprintln!(
                        "decompression evidence failed: {}",
                        display(error.to_string().as_bytes())
                    );
                }
                eprintln!(
                    "decompression stderr: {}",
                    display_prefix(&stopped.output.stderr)
                );
                return Ok(status(&stopped.process));
            }
            if CANCELLED.load(Ordering::Relaxed) {
                return Err("setup cancelled; inspect retained preparation evidence".into());
            }
            println!("{}", prepared?.path().display());
            Ok(0)
        }
        Operation::Check { directory } => {
            absolute(&directory)?;
            println!("{}", PreparedServer::check(&directory)?.path().display());
            Ok(0)
        }
        Operation::Run {
            directory,
            evidence,
            ttl,
            command,
        } => {
            absolute(&directory)?;
            new_evidence(&evidence)?;
            let prepared = PreparedServer::check(&directory)?;
            if CANCELLED.load(Ordering::Relaxed) {
                return Err("startup cancelled before launch".into());
            }
            let started = Server::start(
                prepared,
                Limits {
                    startup: Duration::from_secs(30),
                    lifetime: Duration::from_secs(ttl + 60),
                    stdout_bytes: 1_048_576,
                    stderr_bytes: 1_048_576,
                },
            );
            let mut server = match started {
                Ok(server) => server,
                Err(error) => {
                    if let server::Error::Startup {
                        evidence: stopped, ..
                    } = &error
                    {
                        retain_server(&evidence, stopped)?;
                        eprintln!("startup stderr: {}", display_prefix(&stopped.output.stderr));
                    }
                    return Err(error.into());
                }
            };
            let mut child = Command::new(&command[0]);
            if CANCELLED.load(Ordering::Relaxed) {
                let stopped = server.stop()?;
                retain_server(&evidence, &stopped)?;
                return Err("startup cancelled; child command was not started".into());
            }
            fs::write(evidence.join("endpoint.txt"), server.endpoint())?;
            child
                .args(&command[1..])
                .env_remove("POCKET_IC_BIN")
                .env("IC_TESTKIT_POCKET_IC_URL", server.endpoint());
            // Stop even if opening/writing command evidence fails. Drop is fallback.
            let result = capture(&mut child, &evidence, "command", ttl, 16_777_216, true);
            let shutdown = (|| -> Result<bool> {
                let stopped = server.stop()?;
                retain_server(&evidence, &stopped)?;
                Ok(stopped.process.cleanup.is_empty()
                    && stopped.process.status.is_some()
                    && matches!(stopped.process.failure, Some(Failure::Cancelled))
                    && stopped.abandoned == 0)
            })();
            let clean = match shutdown {
                Ok(clean) => clean,
                Err(error) => {
                    eprintln!("shutdown failed: {}", display(error.to_string().as_bytes()));
                    false
                }
            };
            let code = status(&result?);
            if !clean {
                eprintln!(
                    "server did not report clean intentional shutdown; see retained evidence"
                );
            }
            Ok(if code != 0 { code } else { u8::from(!clean) })
        }
    }
}

fn retain_server(evidence: &Path, stopped: &Stopped) -> io::Result<()> {
    fs::write(evidence.join("server.stdout"), &stopped.output.stdout)?;
    fs::write(evidence.join("server.stderr"), &stopped.output.stderr)?;
    fs::write(evidence.join("server.txt"), format!("{stopped:?}\n"))
}

fn absolute(path: &Path) -> Result<()> {
    if !path.is_absolute()
        || path
            .as_os_str()
            .as_encoded_bytes()
            .iter()
            .any(|b| matches!(b, b'\r' | b'\n'))
    {
        return Err("select absolute paths without CR/LF".into());
    }
    Ok(())
}

fn new_evidence(path: &Path) -> Result<()> {
    absolute(path)?;
    // Exclusive new directory: no stale diagnostics or pre-existing output links.
    DirBuilder::new().mode(0o700).create(path)?;
    eprintln!(
        "PocketIC evidence: {}",
        display(path.as_os_str().as_encoded_bytes())
    );
    Ok(())
}

fn capture(
    command: &mut Command,
    evidence: &Path,
    name: &str,
    seconds: u64,
    limit: usize,
    passthrough: bool,
) -> Result<Report> {
    let mut stdout = File::create(evidence.join(format!("{name}.stdout")))?;
    let mut stderr = File::create(evidence.join(format!("{name}.stderr")))?;
    let mut diagnostic = Vec::new();
    let report = ic_host_process::run(
        command,
        Policy {
            ownership: Ownership::Group,
            input: if passthrough {
                Input::Inherit
            } else {
                Input::Null
            },
            stdout_bytes: limit,
            stderr: Stderr::Capture(limit),
            elapsed: Duration::from_secs(seconds),
        },
        &CANCELLED,
        |stream, bytes| -> io::Result<()> {
            match stream {
                Stream::Stdout => {
                    stdout.write_all(bytes)?;
                    // Explicit child command IO is byte-preserving passthrough;
                    // server/download diagnostics are escaped separately.
                    if passthrough {
                        io::stdout().write_all(bytes)?;
                        io::stdout().flush()?;
                    }
                    Ok(())
                }
                Stream::Stderr => {
                    diagnostic
                        .extend_from_slice(&bytes[..bytes.len().min(4096 - diagnostic.len())]);
                    stderr.write_all(bytes)?;
                    if passthrough {
                        io::stderr().write_all(bytes)?;
                        io::stderr().flush()?;
                    }
                    Ok(())
                }
            }
        },
    );
    if let Err(error) = fs::write(
        evidence.join(format!("{name}.txt")),
        format!("{report:?}\n"),
    ) {
        // Diagnostic retention must not replace an already observed command failure.
        if status(&report) == 0 {
            return Err(error.into());
        }
        eprintln!(
            "report retention failed: {}",
            display(error.to_string().as_bytes())
        );
    }
    if status(&report) != 0 {
        eprintln!(
            "{name} failed (exit {}, {}): {}",
            status(&report),
            display(format!("{report:?}").as_bytes()),
            display(&diagnostic)
        );
    }
    Ok(report)
}

fn status(report: &Report) -> u8 {
    let code = report
        .status
        .and_then(|s| s.code())
        .and_then(|c| u8::try_from(c).ok())
        .unwrap_or(1);
    if code != 0 {
        code
    } else {
        u8::from(report.failure.is_some() || !report.cleanup.is_empty())
    }
}

fn display(bytes: &[u8]) -> String {
    // Text owns terminal escaping; CI workflow-command punctuation is local policy.
    ic_host_text::escape(&String::from_utf8_lossy(bytes)).replace(':', "\\x3a")
}

fn display_prefix(bytes: &[u8]) -> String {
    display(&bytes[..bytes.len().min(4096)])
}
