//! Strict file-dependency reads; callers select records and output policy.

use crate::artifacts::cache_fs::canonicalize_allow_missing;
use std::{
    collections::BTreeSet,
    io,
    path::{Path, PathBuf},
};

/// Parse Cargo/rustc Make depfiles without interpreting unsupported Make syntax.
///
/// Supports escaped space/tab/backslash/hash/colon, `$$`, line continuations,
/// multiple rules and empty phony rules. Full-line comments are ignored; their
/// environment facts must be represented in caller identity. Relative paths
/// remain relative until a record read binds the caller's explicit working directory.
///
/// # Errors
/// Returns `InvalidData` for malformed or unsupported syntax, or an empty input set.
pub fn parse_cargo_depfile(text: &str) -> io::Result<Vec<PathBuf>> {
    let mut parser = DepfileParser::default();
    let mut chars = text.chars();
    while let Some(character) = chars.next() {
        match character {
            '\\' => match chars.next() {
                Some('\n') => parser.flush_token(),
                Some('\r') if chars.next() == Some('\n') => {
                    parser.flush_token();
                }
                Some(escaped @ (' ' | '\t' | '\\' | '#' | ':')) => parser.token.push(escaped),
                _ => return Err(invalid("unsupported depfile escape")),
            },
            '$' => {
                if chars.next() != Some('$') {
                    return Err(invalid("unsupported depfile expansion"));
                }
                parser.token.push('$');
            }
            ':' => {
                if parser.separated {
                    return Err(invalid("unescaped colon in depfile dependency"));
                }
                parser.flush_token();
                if !parser.has_target {
                    return Err(invalid("depfile target is empty"));
                }
                parser.separated = true;
            }
            '#' if !parser.separated && !parser.has_target && parser.token.is_empty() => {
                for comment in chars.by_ref() {
                    if comment == '\n' {
                        break;
                    }
                }
            }
            '\n' => {
                parser.finish_rule()?;
            }
            ' ' | '\t' | '\r' => {
                parser.flush_token();
            }
            '#' | ';' | '|' | '*' | '?' | '[' | ']' | '=' | '%' => {
                return Err(invalid("unsupported Make depfile syntax"));
            }
            control if control.is_control() => return Err(invalid("control byte in depfile")),
            ordinary => parser.token.push(ordinary),
        }
    }
    parser.finish_rule()?;
    if parser.paths.is_empty() {
        return Err(invalid("depfile has no source dependencies"));
    }
    Ok(parser.paths.into_iter().collect())
}

/// Read source inputs from one explicitly selected Cargo/rustc depfile.
///
/// The caller owns target selection, so unrelated records in a shared output
/// directory are not inspected. Uses bounded, regular-file/no-follow
/// reads, strict parsing, alias resolution and generated-root exclusions.
/// Missing records return `NotFound`; the caller decides
/// whether absence is an admissible cold-build inventory.
///
/// # Errors
/// Returns non-absolute selections, record overflow, unsupported syntax or I/O.
pub fn read_cargo_depfile_inputs(
    current_dir: &Path,
    record: &Path,
    excluded_roots: &[PathBuf],
    maximum_record_bytes: usize,
) -> io::Result<Vec<PathBuf>> {
    if !current_dir.is_absolute()
        || !record.is_absolute()
        || excluded_roots.iter().any(|path| !path.is_absolute())
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "depfile selections must be absolute",
        ));
    }
    let excluded = excluded_roots
        .iter()
        .map(|path| canonicalize_allow_missing(path))
        .collect::<io::Result<Vec<_>>>()?;
    let mut inputs = BTreeSet::new();
    read_record_inputs(
        current_dir,
        record,
        &excluded,
        maximum_record_bytes,
        &mut inputs,
    )?;
    Ok(inputs.into_iter().collect())
}

fn read_record_inputs(
    current_dir: &Path,
    record: &Path,
    excluded: &[PathBuf],
    maximum_record_bytes: usize,
    inputs: &mut BTreeSet<PathBuf>,
) -> io::Result<()> {
    let bytes = ic_host_fs::read::read_file_no_follow(record, maximum_record_bytes)
        .map_err(io::Error::from)?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    for path in parse_cargo_depfile(text)? {
        let path = if path.is_absolute() {
            path
        } else {
            current_dir.join(path)
        };
        let path = match path.canonicalize() {
            Ok(path) => path,
            Err(error) if error.kind() == io::ErrorKind::NotFound => path,
            Err(error) => return Err(error),
        };
        if !path
            .components()
            .any(|component| component == std::path::Component::ParentDir)
            && excluded.iter().any(|root| path.starts_with(root))
        {
            continue;
        }
        inputs.insert(path);
    }
    Ok(())
}

#[derive(Default)]
struct DepfileParser {
    token: String,
    has_target: bool,
    separated: bool,
    paths: BTreeSet<PathBuf>,
}

impl DepfileParser {
    fn flush_token(&mut self) {
        if !self.token.is_empty() {
            if self.separated {
                self.paths
                    .insert(PathBuf::from(std::mem::take(&mut self.token)));
            } else {
                self.has_target = true;
                self.token.clear();
            }
        }
    }

    fn finish_rule(&mut self) -> io::Result<()> {
        self.flush_token();
        if self.has_target && !self.separated {
            return Err(invalid("depfile rule has no target separator"));
        }
        self.has_target = false;
        self.separated = false;
        Ok(())
    }
}

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
