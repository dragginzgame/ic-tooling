//! TOML spans own syntax; only admitted string literals are replaced.
#[cfg(test)]
mod tests;

use crate::{Error, Result};
use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};
use toml_edit::{Document, Item, Value};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Plan {
    pub version: Version,
    pub dependencies: Vec<Dependency>,
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Version {
    Workspace,
    Package,
    Unchanged,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Dependency {
    pub alias: String,
    pub previous: String,
    pub next: String,
}
type Edit = (Range<usize>, String);

pub(crate) fn versions(previous: &str, next: &str) -> Result<()> {
    for input in [previous, next] {
        let version = semver::Version::parse(input)
            .map_err(|_| Error::Selection("expected stable semantic version"))?;

        if !version.pre.is_empty() || !version.build.is_empty() {
            return Err(Error::Selection("expected stable semantic version"));
        }
    }
    if previous == next {
        return Err(Error::Selection("version selection changes nothing"));
    }

    // Ordering remains Toko's policy; this renderer can also render recovery inputs.
    Ok(())
}

fn edit(value: &Value, previous: &str, next: &str) -> Result<Edit> {
    if value.as_str() != Some(previous) {
        return Err(Error::Selection("stale selected literal"));
    }

    Ok((
        value
            .span()
            .ok_or(Error::Selection("literal has no source span"))?,
        Value::from(next).to_string(),
    ))
}

fn apply(input: &str, mut edits: Vec<Edit>) -> Result<String> {
    edits.sort_by_key(|(span, _)| std::cmp::Reverse(span.start));
    let mut output = input.to_owned();
    let mut boundary = input.len();
    for (span, replacement) in edits {
        if span.end > boundary || input.get(span.clone()).is_none() {
            return Err(Error::Selection("overlapping source spans"));
        }
        boundary = span.start;
        output.replace_range(span, &replacement);
    }
    Document::parse(output.as_str())?;

    Ok(output)
}

pub(crate) fn manifest(input: &str, previous: &str, next: &str, plan: &Plan) -> Result<String> {
    versions(previous, next)?;
    let doc = Document::parse(input)?;
    let root = doc.as_item();
    let table = match plan.version {
        Version::Workspace => Some(root.get("workspace").and_then(|w| w.get("package"))),
        Version::Package => Some(root.get("package")),
        Version::Unchanged => None,
    };
    let mut edits = Vec::new();
    if let Some(table) = table {
        let value = table
            .and_then(|t| t.get("version"))
            .and_then(Item::as_value)
            .ok_or(Error::Selection("missing selected manifest version"))?;
        edits.push(edit(value, previous, next)?);
    }
    let mut aliases = HashSet::new();
    for dependency in &plan.dependencies {
        if dependency.alias.is_empty()
            || !aliases.insert(&dependency.alias)
            || semver::VersionReq::parse(&dependency.previous).is_err()
            || semver::VersionReq::parse(&dependency.next).is_err()
        {
            return Err(Error::Selection(
                "invalid or duplicate dependency selection",
            ));
        }
        let value = root
            .get("workspace")
            .and_then(|w| w.get("dependencies"))
            .and_then(|d| d.get(&dependency.alias))
            .and_then(|d| d.get("version"))
            .and_then(Item::as_value)
            .ok_or(Error::Selection("missing selected dependency requirement"))?;
        edits.push(edit(value, &dependency.previous, &dependency.next)?);
    }

    apply(input, edits)
}

pub(crate) fn lock(input: &str, previous: &str, next: &str, selected: &[String]) -> Result<String> {
    versions(previous, next)?;
    let names: HashSet<_> = selected.iter().map(String::as_str).collect();
    if names.is_empty()
        || names.len() != selected.len()
        || names.iter().any(|n| {
            n.is_empty()
                || !n
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        })
    {
        return Err(Error::Selection("invalid or duplicate package selection"));
    }
    let doc = Document::parse(input)?;
    if !matches!(
        doc.as_item().get("version").and_then(Item::as_integer),
        Some(3 | 4)
    ) {
        return Err(Error::Selection("expected lock format 3 or 4"));
    }
    let packages = doc
        .as_item()
        .get("package")
        .and_then(Item::as_array_of_tables)
        .ok_or(Error::Selection("missing lock packages"))?;
    let mut identities = HashSet::new();
    let mut counts = HashMap::<&str, usize>::new();
    let mut local = HashSet::new();
    let mut edits = Vec::new();
    for package in packages {
        let name = package
            .get("name")
            .and_then(Item::as_str)
            .ok_or(Error::Selection("invalid package name"))?;
        let version = package
            .get("version")
            .and_then(Item::as_value)
            .ok_or(Error::Selection("missing package version"))?;
        let version_text = version
            .as_str()
            .ok_or(Error::Selection("invalid package version"))?;
        let source = package
            .get("source")
            .map(|s| s.as_str().ok_or(Error::Selection("invalid source")))
            .transpose()?;
        if !identities.insert((name, version_text, source)) {
            return Err(Error::Selection("duplicate lock identity"));
        }
        *counts.entry(name).or_default() += 1;

        if source.is_none() && names.contains(name) {
            if !local.insert(name) {
                return Err(Error::Selection("ambiguous local package"));
            }
            edits.push(edit(version, previous, next)?);
        }
    }
    if local != names {
        return Err(Error::Selection("missing selected local package"));
    }
    for package in packages {
        if let Some(dependencies) = package.get("dependencies") {
            let dependencies = dependencies
                .as_array()
                .ok_or(Error::Selection("invalid dependency array"))?;

            for dependency in dependencies {
                let reference = dependency
                    .as_str()
                    .ok_or(Error::Selection("invalid dependency reference"))?;
                let words: Vec<_> = reference.split(' ').collect();
                if !names.contains(words[0]) {
                    continue;
                }
                if words.len() == 1 && counts[words[0]] != 1 {
                    return Err(Error::Selection("ambiguous bare dependency reference"));
                }

                if words.len() == 2 && words[1] == previous {
                    if identities.iter().any(|(name, version, source)| {
                        *name == words[0] && *version == next && source.is_some()
                    }) {
                        return Err(Error::Selection(
                            "replacement would make dependency reference ambiguous",
                        ));
                    }
                    if identities
                        .iter()
                        .filter(|(name, version, _)| *name == words[0] && *version == previous)
                        .count()
                        != 1
                    {
                        return Err(Error::Selection("ambiguous versioned dependency reference"));
                    }
                    edits.push(edit(
                        dependency,
                        reference,
                        &format!("{} {next}", words[0]),
                    )?);
                }
            }
        }
    }

    apply(input, edits)
}
