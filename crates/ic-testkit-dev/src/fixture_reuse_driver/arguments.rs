//! Validates the operator's workload selection before launching any tools.

use ic_host_process::tool::{OutputLimit, OutputLimits};
use std::{ffi::OsString, num::NonZeroUsize, path::PathBuf};

use crate::fixture_reuse_protocol::{Mode, WorkerArguments, Workload};
use clap::{CommandFactory, Parser, error::ErrorKind};

pub(super) const RSS_COMMAND_LIMITS: OutputLimits = OutputLimits {
    stdout: OutputLimit::Terminate(16 * 1024 * 1024),
    stderr: OutputLimit::Terminate(64 * 1024),
    timeout: None,
};

#[derive(Parser)]
#[command(
    name = "fixture_reuse_benchmark_driver",
    about = "Fixture reuse benchmark for Linux/macOS",
    after_help = "Builds use the selected lockfile and prepared offline caches; no server download.",
    version,
    no_binary_name = true,
    args_override_self = true
)]
pub(super) struct Options {
    #[arg(
        long,
        value_name = "server",
        help = "Exact PocketIC server binary",
        allow_hyphen_values = true
    )]
    pub(super) server: PathBuf,
    #[arg(long, value_name = "iterations", default_value = "100")]
    pub(super) iterations: NonZeroUsize,
    #[arg(long, value_name = "workers", default_value = "2")]
    pub(super) workers: NonZeroUsize,
    #[arg(long, value_name = "repeats", default_value = "3")]
    pub(super) repeats: NonZeroUsize,
    #[arg(long, value_name = "modes", action = clap::ArgAction::Set, num_args = 1.., default_values = ["fresh", "pooled"])]
    pub(super) modes: Vec<Mode>,
    #[arg(long, value_name = "capacities", action = clap::ArgAction::Set, num_args = 1.., default_values = ["1", "2"])]
    pub(super) capacities: Vec<NonZeroUsize>,
    #[arg(long, value_name = "state-bytes", default_value = "1048576")]
    pub(super) state_bytes: u32,
    #[arg(long, value_name = "profile", default_value = "release", value_parser = ["release", "dev"])]
    pub(super) profile: String,
    #[arg(long, value_name = "output", allow_hyphen_values = true)]
    pub(super) output: Option<PathBuf>,
    #[arg(long, value_name = "rss-stdout-bytes", default_value = RSS_COMMAND_LIMITS.stdout.retained_bytes().to_string())]
    rss_stdout_bytes: NonZeroUsize,
    #[arg(long, value_name = "rss-stderr-bytes", default_value = RSS_COMMAND_LIMITS.stderr.retained_bytes().to_string())]
    rss_stderr_bytes: NonZeroUsize,
}

pub(super) struct Case {
    pub(super) label: String,
    pub(super) mode: Mode,
    pub(super) capacity: NonZeroUsize,
}

impl Case {
    pub(super) const fn worker_arguments(
        &self,
        options: &Options,
        wasm: PathBuf,
        server: PathBuf,
    ) -> WorkerArguments {
        WorkerArguments {
            workload: Workload {
                mode: self.mode,
                capacity: self.capacity,
                iterations: options.iterations,
                workers: options.workers,
                state_bytes: options.state_bytes,
            },
            wasm,
            server,
        }
    }
}

impl Options {
    pub(super) const fn rss_limits(&self) -> OutputLimits {
        OutputLimits {
            stdout: OutputLimit::Terminate(self.rss_stdout_bytes.get()),
            stderr: OutputLimit::Terminate(self.rss_stderr_bytes.get()),
            timeout: RSS_COMMAND_LIMITS.timeout,
        }
    }

    pub(super) fn benchmark_cases(&self) -> Vec<Case> {
        let mut cases = Vec::new();
        for &mode in &self.modes {
            match mode {
                Mode::Fresh => cases.push(Case {
                    label: "fresh".into(),
                    mode,
                    capacity: self.workers,
                }),
                Mode::Pooled => cases.extend(self.capacities.iter().map(|&capacity| Case {
                    label: format!("pooled-{capacity}"),
                    mode,
                    capacity,
                })),
            }
        }
        cases
    }
}

pub(super) fn parse_arguments(
    args: impl IntoIterator<Item = OsString>,
) -> Result<Options, clap::Error> {
    let options = Options::try_parse_from(args)?;
    let mut command = Options::command();
    // Clap owns parsing and individual value constraints. These relationships
    // compare independently typed values and select a unique workload matrix.
    Workload::admit_concurrency(options.workers, options.iterations)
        .map_err(|message| command.error(ErrorKind::ValueValidation, message))?;
    if options
        .modes
        .iter()
        .enumerate()
        .any(|(index, value)| options.modes[..index].contains(value))
        || options
            .capacities
            .iter()
            .enumerate()
            .any(|(index, value)| options.capacities[..index].contains(value))
    {
        return Err(command.error(
            ErrorKind::ValueValidation,
            "each mode and capacity must be selected only once",
        ));
    }
    Ok(options)
}

#[cfg(test)]
mod tests;
