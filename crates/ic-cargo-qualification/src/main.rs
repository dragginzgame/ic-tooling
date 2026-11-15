//! Narrow, read-only Cargo qualification operations outside the ordinary Build graph.
use ic_host_artifacts::Length;
use ic_host_fs::{Access, Directory, FilePolicy};
use std::{collections::BTreeMap, ffi::OsString, io, path::Path, process::ExitCode};
use thiserror::Error;

#[derive(Debug, Error)]
enum Refusal {
    #[error("usage: ic-cargo-qualification verify-external-lock ORIGINAL CANDIDATE MAX_BYTES")]
    Usage,
    #[error("{role} lock: {reason}")]
    Lock {
        role: &'static str,
        reason: &'static str,
    },
    #[error("external lock selection changed")]
    Changed,
    #[error(transparent)]
    File(#[from] ic_host_fs::Error),
    #[error(transparent)]
    Bytes(#[from] ic_host_artifacts::Error),
    #[error(transparent)]
    Io(#[from] io::Error),
}
type Rows<'a> = BTreeMap<(&'a str, &'a str, &'a str), Option<&'a str>>;

fn rows<'a>(lock: &'a toml::Value, role: &'static str) -> Result<Rows<'a>, Refusal> {
    let bad = |reason| Refusal::Lock { role, reason };
    let entries = lock
        .get("package")
        .and_then(toml::Value::as_array)
        .ok_or_else(|| bad("missing package array"))?;
    let mut rows = BTreeMap::new();
    for entry in entries {
        let table = entry
            .as_table()
            .ok_or_else(|| bad("package must be a table"))?;
        let Some(source) = table.get("source") else {
            continue;
        };
        let source = source
            .as_str()
            .ok_or_else(|| bad("source must be a string"))?;
        let name = table
            .get("name")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| bad("missing package name"))?;
        let version = table
            .get("version")
            .and_then(toml::Value::as_str)
            .ok_or_else(|| bad("missing package version"))?;
        let checksum = table
            .get("checksum")
            .map(|value| {
                value
                    .as_str()
                    .ok_or_else(|| bad("checksum must be a string"))
            })
            .transpose()?;
        if source.starts_with("registry+")
            && !checksum.is_some_and(|value| {
                value.len() == 64
                    && value
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
            })
        {
            return Err(bad("registry checksum must be lowercase SHA-256"));
        }
        if rows.insert((name, version, source), checksum).is_some() {
            return Err(bad("duplicate external identity"));
        }
    }
    Ok(rows)
}
fn parse(bytes: &[u8], role: &'static str) -> Result<toml::Value, Refusal> {
    let bad = |reason| Refusal::Lock { role, reason };
    let text = std::str::from_utf8(bytes).map_err(|_| bad("invalid UTF-8"))?;
    // Lock source URLs may contain credentials; do not echo parser source text.
    toml::from_str(text).map_err(|_| bad("invalid TOML"))
}
fn verify(original: &[u8], candidate: &[u8]) -> Result<(), Refusal> {
    let original = parse(original, "original")?;
    let candidate = parse(candidate, "candidate")?;
    let original = rows(&original, "original")?;
    let candidate = rows(&candidate, "candidate")?;
    if candidate
        .iter()
        .any(|(key, value)| original.get(key) != Some(value))
    {
        return Err(Refusal::Changed);
    }
    Ok(())
}
fn read(path: &Path, maximum: u64) -> Result<Vec<u8>, Refusal> {
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let name = path.file_name().ok_or(Refusal::Usage)?;
    let directory =
        Directory::open(parent, 0)?.ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
    let file = directory
        .open_file(
            name,
            Access::Read,
            FilePolicy {
                forbidden: 0,
                single_link: false,
            },
        )?
        .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
    Ok(ic_host_artifacts::read(file, Length::AtMost(maximum))?.0)
}
fn run(arguments: &[OsString]) -> Result<(), Refusal> {
    let [command, original, candidate, maximum] = arguments else {
        return Err(Refusal::Usage);
    };
    if command != "verify-external-lock" {
        return Err(Refusal::Usage);
    }
    let maximum = maximum
        .to_str()
        .and_then(|value| value.parse::<u64>().ok())
        .ok_or(Refusal::Usage)?;
    let original = read(Path::new(original), maximum)?;
    let candidate = read(Path::new(candidate), maximum)?;
    verify(&original, &candidate)
}
fn main() -> ExitCode {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    if arguments == [OsString::from("--help")] {
        println!("{}", Refusal::Usage);
        return ExitCode::SUCCESS;
    }
    match run(&arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            if matches!(error, Refusal::Usage) {
                ExitCode::from(2)
            } else {
                ExitCode::FAILURE
            }
        }
    }
}
#[cfg(test)]
mod tests;
