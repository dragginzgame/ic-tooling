use clap::Parser;
use ic_build::cargo::{
    lock_projection::WorkspaceVersionProjection,
    release_metadata::{
        DependencyVersionProjection, render_workspace_lock, render_workspace_manifest,
    },
};
use serde::Deserialize;
use serde_json::Value;
use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    num::NonZeroUsize,
    path::{Component, Path, PathBuf},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(
    name = "ic-build-release-metadata",
    about = "Render read-only Cargo release candidates",
    version
)]
struct Arguments {
    /// Positive input byte allowance per document
    #[arg(long, value_name = "max-input-bytes")]
    max_input_bytes: NonZeroUsize,
    #[command(subcommand)]
    operation: Operation,
}

#[derive(clap::Args)]
struct Selection {
    #[arg(value_name = "source")]
    source: PathBuf,
    #[arg(value_name = "previous")]
    previous: String,
    #[arg(value_name = "next")]
    next: String,
}

#[derive(clap::Subcommand)]
enum Operation {
    /// Render a workspace manifest candidate
    Manifest {
        #[command(flatten)]
        selection: Selection,
        #[arg(value_name = "workspace-input-json")]
        context: Option<PathBuf>,
    },
    /// Render a lockfile candidate for the selected local packages
    Lock {
        #[command(flatten)]
        selection: Selection,
        #[arg(value_name = "package", required = true, num_args = 1..)]
        packages: Vec<String>,
    },
}

pub fn run() -> Result<()> {
    let arguments = Arguments::parse();
    let limit = arguments.max_input_bytes.get();
    let selection = match &arguments.operation {
        Operation::Manifest { selection, .. } | Operation::Lock { selection, .. } => selection,
    };
    let Selection {
        source,
        previous,
        next,
    } = selection;
    check_path(source)?;
    let input = read(source, limit)?;
    let output = match &arguments.operation {
        Operation::Manifest { context, .. } => {
            let dependencies = if let Some(context) = context {
                let context = serde_json::from_str(&read(context, limit)?)?;
                dependency_versions(source, &input, next, &context)?
            } else {
                Vec::new()
            };
            let projections: Vec<_> = dependencies
                .iter()
                .map(|(alias, previous, next)| DependencyVersionProjection {
                    alias,
                    previous,
                    next,
                })
                .collect();
            render_workspace_manifest(&input, previous, next, &projections)?
        }
        Operation::Lock { packages, .. } => {
            let projections = packages
                .iter()
                .map(|name| WorkspaceVersionProjection {
                    package: name,
                    previous,
                    next,
                })
                .collect::<Vec<_>>();
            render_workspace_lock(&input, &projections)?
        }
    };
    std::io::stdout().lock().write_all(output.as_bytes())?;
    Ok(())
}

fn check_path(path: &Path) -> Result<()> {
    if path
        .as_os_str()
        .as_encoded_bytes()
        .iter()
        .any(|byte| matches!(byte, 0 | b'\r' | b'\n'))
    {
        return Err("policy path contains NUL/CR/LF".into());
    }
    Ok(())
}
fn read(path: &Path, limit: usize) -> Result<String> {
    check_path(path)?;
    let absolute = if path.is_absolute() {
        path.to_owned()
    } else {
        std::env::current_dir()?.join(path)
    };
    check_path(&absolute)?;
    check_path(
        &absolute
            .parent()
            .ok_or("input has no parent")?
            .canonicalize()?,
    )?;
    // Parent namespace custody remains caller-owned. Host admits the final opened
    // descriptor, refuses final symlinks/special files and enforces the byte limit.
    Ok(String::from_utf8(ic_host_fs::read::read_file_no_follow(
        path, limit,
    )?)?)
}
fn identity(path: &Path) -> Result<PathBuf> {
    check_path(path)?;
    if !path.is_absolute() {
        return Err("saved identity must be absolute".into());
    }
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::Normal(_) => result.push(component.as_os_str()),
            Component::CurDir => {}
            Component::ParentDir => {
                if !result.pop() {
                    return Err("identity escapes filesystem root".into());
                }
            }
            Component::Prefix(_) => return Err("unsupported path prefix".into()),
        }
    }
    Ok(result)
}
fn requirement(previous: &str, next: &str) -> Option<String> {
    let offset = usize::from(previous.starts_with(['~', '^', '=']));
    let version = previous[offset..].trim_start_matches([' ', '\t']);
    let prefix = &previous[..previous.len() - version.len()];
    let parts: Vec<_> = version.split('.').collect();
    if !(1..=3).contains(&parts.len())
        || parts
            .iter()
            .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return None;
    }
    Some(format!(
        "{prefix}{}",
        next.split('.')
            .take(parts.len())
            .collect::<Vec<_>>()
            .join(".")
    ))
}
// Live contexts bind existing filesystem identities; saved contexts bind the
// admitted source projection without requiring its original checkout to exist.
enum ContextRoot {
    Live(PathBuf),
    Saved(PathBuf),
}

impl ContextRoot {
    fn owns(&self, path: &str, member: &Path) -> Result<bool> {
        match self {
            Self::Live(root) => Ok(root.join(path).join("Cargo.toml").canonicalize()? == member),
            Self::Saved(root) => {
                if identity(&root.join(path).join("Cargo.toml"))? != member {
                    return Err("cannot establish saved dependency ownership".into());
                }
                Ok(true)
            }
        }
    }
}

#[derive(Deserialize)]
struct SourceContext {
    // Absence selects live custody; an explicit null is never a live context.
    #[serde(default, deserialize_with = "source_root")]
    source_root: Option<String>,
    members: Vec<ContextMember>,
    #[serde(default)]
    manifest: Value,
}

#[derive(Deserialize)]
struct ContextMember {
    name: String,
    manifest_path: String,
}

fn source_root<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error> {
    String::deserialize(deserializer).map(Some)
}

struct WorkspaceContext<'a> {
    root: ContextRoot,
    members: HashMap<&'a str, PathBuf>,
    dependencies: Option<&'a serde_json::Map<String, Value>>,
}

impl<'a> WorkspaceContext<'a> {
    fn admit(source: &Path, input: &str, context: &'a SourceContext) -> Result<Self> {
        let root = if let Some(root) = &context.source_root {
            ContextRoot::Saved(identity(Path::new(root))?)
        } else {
            ContextRoot::Live(
                source
                    .canonicalize()?
                    .parent()
                    .ok_or("manifest has no parent")?
                    .to_owned(),
            )
        };
        let mut members = HashMap::new();
        for member in &context.members {
            let name = member.name.as_str();
            let path = identity(Path::new(&member.manifest_path))?;
            if members.insert(name, path).is_some() {
                return Err("duplicate member identity".into());
            }
        }
        let manifest: toml::Value = toml::from_str(input)?;
        let actual = manifest
            .get("workspace")
            .and_then(|value| value.get("dependencies"));
        let expected = context
            .manifest
            .get("workspace")
            .and_then(|value| value.get("dependencies"));
        if serde_json::to_value(actual)? != expected.cloned().unwrap_or(Value::Null) {
            return Err("saved dependency declarations differ from source".into());
        }
        Ok(Self {
            root,
            members,
            dependencies: expected.and_then(Value::as_object),
        })
    }
}

fn dependency_versions(
    source: &Path,
    input: &str,
    next: &str,
    context: &SourceContext,
) -> Result<Vec<(String, String, String)>> {
    let context = WorkspaceContext::admit(source, input, context)?;
    let mut selected = Vec::new();
    if let Some(dependencies) = context.dependencies {
        for (alias, dependency) in dependencies {
            let (Some(path), Some(previous)) = (
                dependency.get("path").and_then(Value::as_str),
                dependency.get("version").and_then(Value::as_str),
            ) else {
                continue;
            };
            let package = dependency
                .get("package")
                .and_then(Value::as_str)
                .unwrap_or(alias);
            let Some(member) = context.members.get(package) else {
                continue;
            };
            if context.root.owns(path, member)?
                && let Some(candidate) = requirement(previous, next)
            {
                selected.push((alias.clone(), previous.to_owned(), candidate));
            }
        }
    }
    Ok(selected)
}
