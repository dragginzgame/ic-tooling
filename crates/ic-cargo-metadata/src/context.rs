//! Bind coordinated local requirements to the caller's current workspace projection.
use crate::{Error, Result, render::Dependency};
use serde::Deserialize;
use serde_json::Value;
use std::{collections::HashMap, path::Path};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Context {
    manifest: Value,
    members: Vec<Member>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Member {
    name: String,
    manifest_path: String,
}

pub(crate) fn check_path(path: &Path) -> Result<()> {
    if path
        .as_os_str()
        .as_encoded_bytes()
        .iter()
        .any(|b| matches!(b, 0 | b'\r' | b'\n'))
    {
        return Err(Error::Selection("path contains NUL/CR/LF"));
    }

    Ok(())
}

pub(crate) fn dependencies(
    source: &Path,
    input: &str,
    next: &str,
    context: &str,
) -> Result<Vec<Dependency>> {
    let context: Context = serde_json::from_str(context)?;
    let actual: toml::Value = toml::from_str(input)?;
    let actual = serde_json::to_value(actual)?;
    if actual.get("workspace").and_then(|w| w.get("dependencies"))
        != context
            .manifest
            .get("workspace")
            .and_then(|w| w.get("dependencies"))
    {
        return Err(Error::Selection("context requirements differ from source"));
    }
    let root = source.canonicalize()?;
    let root = root
        .parent()
        .ok_or(Error::Selection("manifest has no parent"))?;
    let mut members = HashMap::new();
    for member in &context.members {
        let path = Path::new(&member.manifest_path);
        check_path(path)?;

        if !path.is_absolute() || members.insert(&member.name, path.canonicalize()?).is_some() {
            return Err(Error::Selection("invalid or duplicate member identity"));
        }
    }
    let mut result = Vec::new();
    if let Some(dependencies) = actual
        .get("workspace")
        .and_then(|w| w.get("dependencies"))
        .and_then(Value::as_object)
    {
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
            let Some(member) = members.get(&package.to_owned()) else {
                continue;
            };
            check_path(Path::new(path))?;
            if root.join(path).join("Cargo.toml").canonicalize()? != *member {
                // A same-name dependency outside the admitted member set is unselected.
                continue;
            }
            let offset = usize::from(previous.starts_with(['~', '^', '=']));
            let version = previous[offset..].trim_start_matches([' ', '\t']);
            let parts: Vec<_> = version.split('.').collect();
            if !(1..=3).contains(&parts.len())
                || parts
                    .iter()
                    .any(|p| p.is_empty() || !p.bytes().all(|b| b.is_ascii_digit()))
            {
                // Compound ranges remain caller policy. Toko's candidate Cargo
                // admission decides whether the unchanged range permits the release.
                continue;
            }
            result.push(Dependency {
                alias: alias.clone(),
                previous: previous.to_owned(),
                next: format!(
                    "{}{}",
                    &previous[..previous.len() - version.len()],
                    next.split('.')
                        .take(parts.len())
                        .collect::<Vec<_>>()
                        .join(".")
                ),
            });
        }
    }

    Ok(result)
}
