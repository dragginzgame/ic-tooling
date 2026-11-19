//! Render admitted release candidates without owning file replacement or Cargo.
mod context;
mod render;

use clap::{Parser, Subcommand};
use ic_host_artifacts::Length;
use ic_host_fs::{Access, Directory, FilePolicy};
use std::{
    io::Write,
    num::NonZeroU64,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("{0}")]
    Selection(&'static str),
    #[error("invalid TOML input")]
    Toml(#[from] toml_edit::TomlError),
    #[error("invalid manifest context")]
    Context(#[from] serde_json::Error),
    #[error("invalid TOML context")]
    ContextToml(#[from] toml::de::Error),
    #[error("input is not UTF-8")]
    Encoding(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    Fs(#[from] ic_host_fs::Error),
    #[error(transparent)]
    Bytes(#[from] ic_host_artifacts::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
#[command(version, about = "Render bounded Cargo release candidates to stdout")]
struct Arguments {
    /// Positive byte allowance for each input document.
    #[arg(long)]
    max_input_bytes: NonZeroU64,
    #[command(subcommand)]
    operation: Operation,
}
#[derive(clap::Args)]
struct Selection {
    source: PathBuf,
    previous: String,
    next: String,
}
#[derive(Subcommand)]
enum Operation {
    /// Render the exact manifest fields selected by Toko's group plan.
    GroupManifest {
        #[command(flatten)]
        selection: Selection,
        context: PathBuf,
    },
    /// Render a coordinated workspace version and bound member requirements.
    Manifest {
        #[command(flatten)]
        selection: Selection,
        context: Option<PathBuf>,
    },
    /// Render selected local lock identities and unqualified references.
    Lock {
        #[command(flatten)]
        selection: Selection,
        #[arg(required = true, num_args = 1..)]
        packages: Vec<String>,
    },
}

fn read(path: &Path, maximum: u64) -> Result<String> {
    context::check_path(path)?;
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    context::check_path(&parent.canonicalize()?)?;
    let directory = Directory::open(parent, 0)?.ok_or(Error::Selection("input parent absent"))?;
    let file = directory
        .open_file(
            path.file_name()
                .ok_or(Error::Selection("input has no filename"))?,
            Access::Read,
            FilePolicy {
                forbidden: 0,
                single_link: false,
            },
        )?
        .ok_or(Error::Selection("input absent"))?;

    Ok(String::from_utf8(
        ic_host_artifacts::read(file, Length::AtMost(maximum))?.0,
    )?)
}

fn run(arguments: &Arguments) -> Result<String> {
    let (Operation::GroupManifest { selection, .. }
    | Operation::Manifest { selection, .. }
    | Operation::Lock { selection, .. }) = &arguments.operation;
    render::versions(&selection.previous, &selection.next)?;
    let input = read(&selection.source, arguments.max_input_bytes.get())?;

    match &arguments.operation {
        Operation::GroupManifest { context, .. } => {
            let plan = serde_json::from_str(&read(context, arguments.max_input_bytes.get())?)?;

            render::manifest(&input, &selection.previous, &selection.next, &plan)
        }
        Operation::Manifest { context, .. } => {
            let dependencies = if let Some(path) = context {
                context::dependencies(
                    &selection.source,
                    &input,
                    &selection.next,
                    &read(path, arguments.max_input_bytes.get())?,
                )?
            } else {
                Vec::new()
            };

            render::manifest(
                &input,
                &selection.previous,
                &selection.next,
                &render::Plan {
                    version: render::Version::Workspace,
                    dependencies,
                },
            )
        }
        Operation::Lock { packages, .. } => {
            render::lock(&input, &selection.previous, &selection.next, packages)
        }
    }
}

fn main() -> ExitCode {
    let arguments = Arguments::parse();

    match run(&arguments)
        .and_then(|output| Ok(std::io::stdout().lock().write_all(output.as_bytes())?))
    {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!(
                "ic-cargo-metadata: {}",
                ic_host_text::escape(&error.to_string()).replace(':', "\\x3a")
            );

            ExitCode::FAILURE
        }
    }
}
