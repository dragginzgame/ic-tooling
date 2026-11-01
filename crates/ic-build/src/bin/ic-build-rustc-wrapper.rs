//! Explicit target-recipe wrapper; Cargo owns the compiler argument protocol.
use clap::Parser;
use std::{ffi::OsString, path::PathBuf, process::ExitCode};

#[derive(Parser)]
#[command(version, about, disable_help_flag = true)]
struct Arguments {
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, required = true)]
    rustc: Vec<OsString>,
}

fn main() -> ExitCode {
    let args = Arguments::parse();
    let run = || {
        let policy = std::env::var("IC_BUILD_RUSTC_RECIPE").map_err(std::io::Error::other)?;
        let records = PathBuf::from(
            std::env::var_os("IC_BUILD_RUSTC_RECORDS")
                .ok_or_else(|| std::io::Error::other("missing rustc recipe records directory"))?,
        );
        ic_build::cargo::rustc_recipe::run_rustc_recipe(
            &args.rustc[0],
            &args.rustc[1..],
            &policy,
            &records,
        )
    };
    match run() {
        Ok(status) => ExitCode::from(u8::try_from(status.code().unwrap_or(1)).unwrap_or(1)),
        Err(error) => {
            eprintln!("rustc recipe refused: {error}");
            ExitCode::from(2)
        }
    }
}
