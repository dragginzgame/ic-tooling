//! Explicit input and inspection budgets; there are no implicit artifact paths.

use ic_host_artifacts::artifact::Sha256Digest;
use std::path::PathBuf;

#[derive(clap::Parser)]
#[command(
    name = "ic-host-wasm-inspect",
    version,
    about = "Inspect raw Wasm without executing or transforming it"
)]
pub struct Command {
    pub path: PathBuf,
    #[arg(value_name = "MAX_BYTES")]
    pub module_bytes: usize,
    #[arg(value_name = "MAX_SECTIONS")]
    pub sections: usize,
    #[arg(value_name = "MAX_EXPORTS")]
    pub exports: u32,
    #[arg(value_name = "MAX_CUSTOM_SECTIONS")]
    pub custom_sections: usize,
    #[arg(long = "expect-sha256", value_name = "SHA256")]
    pub expected: Option<Sha256Digest>,
    #[arg(long)]
    pub details: bool,
}

#[cfg(test)]
mod tests;
