//! Repository-local worker invocation; callers retain workload and report admission.

use std::{ffi::OsString, num::NonZeroUsize, path::PathBuf};

#[derive(
    Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize, clap::ValueEnum,
)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Fresh,
    Pooled,
}

impl Mode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Pooled => "pooled",
        }
    }
}

/// Typed workload selection, independent of executable and artifact paths.
#[derive(clap::Args)]
pub struct Workload {
    #[arg(value_name = "mode")]
    pub mode: Mode,
    #[arg(value_name = "capacity")]
    pub capacity: NonZeroUsize,
    #[arg(value_name = "iterations")]
    pub iterations: NonZeroUsize,
    #[arg(value_name = "workers")]
    pub workers: NonZeroUsize,
    #[arg(value_name = "state_bytes")]
    pub state_bytes: u32,
}

impl Workload {
    pub(super) const fn admit_concurrency(
        workers: NonZeroUsize,
        iterations: NonZeroUsize,
    ) -> Result<(), &'static str> {
        if workers.get() > iterations.get() {
            Err("workers must not exceed iterations")
        } else {
            Ok(())
        }
    }
}

#[derive(clap::Parser)]
#[command(
    name = "fixture_reuse_benchmark",
    about = "Run one fixture-reuse benchmark workload",
    version,
    no_binary_name = true
)]
pub struct WorkerArguments {
    #[command(flatten)]
    pub workload: Workload,
    #[arg(value_name = "wasm", allow_hyphen_values = true)]
    pub wasm: PathBuf,
    #[arg(value_name = "server", allow_hyphen_values = true)]
    pub server: PathBuf,
}

impl TryFrom<Vec<OsString>> for WorkerArguments {
    type Error = clap::Error;

    fn try_from(args: Vec<OsString>) -> Result<Self, Self::Error> {
        use clap::{CommandFactory, Parser, error::ErrorKind};
        let result = Self::try_parse_from(args)?;
        Workload::admit_concurrency(result.workload.workers, result.workload.iterations)
            .map_err(|message| Self::command().error(ErrorKind::ValueValidation, message))?;
        Ok(result)
    }
}

impl From<&WorkerArguments> for [OsString; 7] {
    fn from(args: &WorkerArguments) -> Self {
        [
            args.workload.mode.as_str().into(),
            args.workload.capacity.to_string().into(),
            args.workload.iterations.to_string().into(),
            args.workload.workers.to_string().into(),
            args.workload.state_bytes.to_string().into(),
            args.wasm.as_os_str().to_owned(),
            args.server.as_os_str().to_owned(),
        ]
    }
}

#[cfg(test)]
mod tests;
