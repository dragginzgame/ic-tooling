//! Offline preparation after an explicit caller-owned download of the pinned URL.
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() == 1 && args[0] == "--asset" {
        let asset = ic_testkit::server::host_asset()?;
        println!("{}\nsha256:{}", asset.url, asset.archive_sha256);
        return Ok(());
    }
    if args.len() != 3 {
        return Err(
            "usage: prepare_server --asset | ARCHIVE EXISTING_DIRECTORY ABSOLUTE_GZIP".into(),
        );
    }
    let prepared = ic_testkit::server::prepare(
        &PathBuf::from(&args[0]),
        &PathBuf::from(&args[1]),
        &PathBuf::from(&args[2]),
    )?;
    println!("{}", prepared.path().display());
    Ok(())
}
