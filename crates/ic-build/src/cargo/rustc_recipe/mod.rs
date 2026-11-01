//! Caller-selected target cfg requirements, applied after Cargo resolves flags.
//!
//! Cargo owns config/environment precedence. Its wrapper receives the resulting
//! argv, so adding a requirement cannot discard config, ordinary or encoded flags.
//! This is opt-in recipe mechanics, not a crypto recommendation or tool installer.
use ic_host_process::tool::{CompletedExecution, OutputLimit, OutputLimits, capture_command};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
};

pub(crate) const POLICY_ENV: &str = "IC_BUILD_RUSTC_RECIPE";
pub(crate) const RECORD_ENV: &str = "IC_BUILD_RUSTC_RECORDS";
pub(crate) const RECORD_DIRECTORY: &str = ".ic-build-rustc-recipes";

/// Caller-selected compiler evidence admission bounds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RustcRecipeLimits {
    /// Maximum bytes in one invocation record or compiler identity probe.
    pub record_bytes: usize,
    /// Maximum retained invocation records in a successful build.
    pub records: usize,
    /// Maximum aggregate bytes read when admitting successful evidence.
    pub total_bytes: usize,
}

/// Explicit treatment of a conflicting target cfg.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CfgConflict {
    /// Refuse a different value for any required cfg key.
    Reject,
    /// Replace that key only, recording the caller's qualification reference.
    Replace {
        /// Evidence that admits the deliberate override for this recipe.
        qualification: String,
    },
}

/// Target-scoped cfg requirements and an explicitly selected wrapper executable.
///
/// Callers admit the dependency generation before opting in; this type performs
/// no dependency/crypto selection. It preserves all unrelated arguments. Evidence
/// contains compiler arguments/paths: callers own access control and redaction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RustcRecipe {
    pub(crate) wrapper: PathBuf,
    target: String,
    cfg: BTreeMap<String, String>,
    conflict: CfgConflict,
    qualification: String,
    limits: RustcRecipeLimits,
}

impl RustcRecipe {
    /// Admit a caller-qualified target recipe and its per-record byte allowance.
    ///
    /// `wrapper` must be an absolute path to `ic-build-rustc-wrapper`. No download,
    /// installation, PATH search or recommendation is performed here.
    /// # Errors
    /// Refuses empty/invalid requirements, evidence labels or output allowances.
    pub fn new(
        wrapper: &Path,
        target: &str,
        cfg: BTreeMap<String, String>,
        qualification: &str,
        limits: RustcRecipeLimits,
    ) -> io::Result<Self> {
        if !wrapper.is_absolute()
            || target.is_empty()
            || qualification.trim().is_empty()
            || limits.record_bytes == 0
            || limits.records == 0
            || limits.total_bytes < limits.record_bytes
            || cfg.is_empty()
            || cfg.iter().any(|(key, value)| {
                key.is_empty()
                    || !key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
                    || !key.as_bytes()[0].is_ascii_alphabetic()
                    || value.contains(['\0', '\n', '\r', '"', '\\'])
            })
        {
            return Err(invalid("invalid caller-selected rustc recipe"));
        }
        Ok(Self {
            wrapper: wrapper.to_owned(),
            target: target.to_owned(),
            cfg,
            conflict: CfgConflict::Reject,
            qualification: qualification.to_owned(),
            limits,
        })
    }

    /// Explicitly admit replacing conflicting values for required keys only.
    /// # Errors
    /// Refuses an empty override qualification reference.
    pub fn with_conflict(mut self, conflict: CfgConflict) -> io::Result<Self> {
        if matches!(&conflict, CfgConflict::Replace { qualification } if qualification.trim().is_empty())
        {
            return Err(invalid("override requires qualification evidence"));
        }
        self.conflict = conflict;
        Ok(self)
    }

    pub(crate) fn policy(&self) -> String {
        json!({
            "schema": 1, "target": self.target, "cfg": self.cfg,
            "qualification": self.qualification, "record_bytes": self.limits.record_bytes,
            "records": self.limits.records, "total_bytes": self.limits.total_bytes,
            "override": match &self.conflict {
                CfgConflict::Reject => None,
                CfgConflict::Replace { qualification } => Some(qualification),
            },
        })
        .to_string()
    }

    pub(crate) fn admit(
        &self,
        target: &str,
        environment: &BTreeMap<OsString, OsString>,
    ) -> io::Result<()> {
        if target != self.target || !target.starts_with("wasm32-") {
            return Err(invalid(
                "rustc recipe must match the explicitly selected Wasm target",
            ));
        }
        for name in [
            "RUSTC_WRAPPER",
            "RUSTC_WORKSPACE_WRAPPER",
            "CARGO_BUILD_RUSTC_WRAPPER",
            "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER",
            "CARGO_BUILD_BUILD_DIR",
            POLICY_ENV,
            RECORD_ENV,
        ] {
            if environment
                .get(OsStr::new(name))
                .is_some_and(|v| !v.is_empty())
            {
                return Err(invalid(
                    "rustc recipe refuses an existing compiler wrapper or reserved recipe environment",
                ));
            }
        }
        Ok(())
    }

    pub(crate) fn admit_configuration(configuration: &toml::Value) -> io::Result<()> {
        if ["rustc-wrapper", "rustc-workspace-wrapper", "build-dir"]
            .iter()
            .any(|name| {
                configuration
                    .get("build")
                    .and_then(|build| build.get(name))
                    .is_some()
            })
        {
            return Err(invalid(
                "qualified rustc recipe refuses configured wrappers or a shared compiler build directory",
            ));
        }
        if [
            "RUSTC",
            "RUSTC_WRAPPER",
            "RUSTC_WORKSPACE_WRAPPER",
            "CARGO_BUILD_RUSTC",
            "CARGO_BUILD_RUSTC_WRAPPER",
            "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER",
            "CARGO_TARGET_DIR",
            "CARGO_BUILD_TARGET_DIR",
            "CARGO_BUILD_BUILD_DIR",
            POLICY_ENV,
            RECORD_ENV,
        ]
        .iter()
        .any(|name| {
            configuration
                .get("env")
                .and_then(|env| env.get(name))
                .is_some()
        }) {
            return Err(invalid(
                "qualified rustc recipe refuses configured environment overrides of compiler custody",
            ));
        }
        Ok(())
    }

    pub(crate) fn verify_records(&self, directory: &Path) -> io::Result<()> {
        let records =
            read_rustc_recipe_records(directory, self.limits.records, self.limits.total_bytes)?;
        let policy: Value = serde_json::from_str(&self.policy()).map_err(invalid)?;
        if records.is_empty()
            || records.iter().any(|record| record["policy"] != policy)
            || !records.iter().any(|record| {
                let args: Vec<OsString> = record["arguments"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .filter_map(Value::as_str)
                    .map(OsString::from)
                    .collect();
                option(&args, "--target") == Some(self.target.as_str())
            })
        {
            return Err(invalid(
                "missing or mismatched effective target compiler evidence",
            ));
        }
        Ok(())
    }
}

/// Read retained invocation records within caller-selected aggregate bounds.
///
/// Returned JSON records contain the actual rustc argv, compiler identity and
/// qualification. A cache hit reads the original records, not new execution.
/// # Errors
/// Refuses non-files, incomplete/invalid records or exhausted output allowances.
pub fn read_rustc_recipe_records(
    directory: &Path,
    maximum_records: usize,
    maximum_bytes: usize,
) -> io::Result<Vec<Value>> {
    let mut paths = fs::read_dir(directory)?
        .take(maximum_records.saturating_add(1))
        .map(|entry| entry.map(|e| e.path()))
        .collect::<io::Result<Vec<_>>>()?;
    if paths.len() > maximum_records {
        return Err(invalid("rustc recipe record count exceeds allowance"));
    }
    paths.sort();
    let mut remaining = maximum_bytes;
    paths
        .into_iter()
        .map(|path| {
            let data = ic_host_fs::read::read_file(&path, remaining).map_err(io::Error::other)?;
            if path.file_name().and_then(OsStr::to_str)
                != Some(format!("{:x}.json", Sha256::digest(&data)).as_str())
            {
                return Err(invalid("rustc recipe record digest mismatch"));
            }
            remaining = remaining
                .checked_sub(data.len())
                .ok_or_else(|| invalid("rustc recipe bytes exceed allowance"))?;
            let value: Value = serde_json::from_slice(&data).map_err(invalid)?;
            if value.get("schema") != Some(&json!(1))
                || !value["arguments"]
                    .as_array()
                    .is_some_and(|args| args.iter().all(Value::is_string))
            {
                return Err(invalid("invalid rustc recipe record"));
            }
            Ok(value)
        })
        .collect()
}

/// Execute Cargo's rustc-wrapper protocol with explicit policy and evidence paths.
///
/// The CLI supplies these from its child-only environment. Native compilations
/// and Cargo probes keep their arguments. Compiler stdout/stderr/stdin are inherited.
/// # Errors
/// Refuses contradictory cfgs, malformed policy, unrecordable arguments or I/O.
pub fn run_rustc_recipe(
    compiler: &OsStr,
    arguments: &[OsString],
    policy: &str,
    records: &Path,
) -> io::Result<ExitStatus> {
    let policy: Value = serde_json::from_str(policy).map_err(invalid)?;
    if policy["schema"] != 1 {
        return Err(invalid("unsupported rustc recipe schema"));
    }
    let target = policy["target"]
        .as_str()
        .ok_or_else(|| invalid("missing recipe target"))?;
    let mut effective = arguments.to_vec();
    if option(arguments, "--target").is_some_and(|v| v == target) {
        for (key, value) in policy["cfg"]
            .as_object()
            .ok_or_else(|| invalid("missing recipe cfg"))?
        {
            compose_cfg(
                &mut effective,
                key,
                value
                    .as_str()
                    .ok_or_else(|| invalid("invalid recipe cfg value"))?,
                policy["override"].is_string(),
            )?;
        }
    }
    // Only actual crate compilations have an output directory. Avoid recording
    // Cargo's rustc capability probes as if they had produced artifacts.
    if option(arguments, "--out-dir").is_some() {
        let text_args = effective
            .iter()
            .map(|v| {
                v.to_str()
                    .ok_or_else(|| invalid("rustc recipe evidence requires Unicode arguments"))
            })
            .collect::<io::Result<Vec<_>>>()?;
        let limit = policy["record_bytes"]
            .as_u64()
            .and_then(|n| usize::try_from(n).ok())
            .ok_or_else(|| invalid("invalid rustc record allowance"))?;
        let mut probe = Command::new(compiler);
        probe.arg("-vV");
        let identity = capture_command(
            &mut probe,
            OutputLimits {
                stdout: OutputLimit::Terminate(limit),
                stderr: OutputLimit::Terminate(limit),
                timeout: None,
            },
        )
        .and_then(CompletedExecution::require_success)
        .and_then(CompletedExecution::require_complete)
        .map_err(io::Error::other)?;
        let record = serde_json::to_vec(&json!({
            "schema": 1, "compiler": compiler.to_str().ok_or_else(|| invalid("compiler path must be Unicode for evidence"))?,
            "compiler_identity": String::from_utf8(identity.stdout).map_err(invalid)?,
            "compiler_identity_stderr": String::from_utf8(identity.stderr).map_err(invalid)?,
            "arguments": text_args, "policy": policy,
        })).map_err(invalid)?;
        if record.len() > limit {
            return Err(invalid("rustc recipe record exceeds allowance"));
        }
        fs::create_dir_all(records)?;
        let name = format!("{:x}.json", Sha256::digest(&record));
        // Each Cargo invocation owns this fresh directory. Concurrent crates use
        // content names; readers run only after the successful Cargo command.
        match fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(records.join(name))
        {
            Ok(mut file) => file.write_all(&record)?,
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    }
    Command::new(compiler).args(&effective).status()
}

fn option<'a>(arguments: &'a [OsString], name: &str) -> Option<&'a str> {
    arguments.iter().enumerate().find_map(|(index, arg)| {
        if arg == name {
            arguments.get(index + 1)?.to_str()
        } else {
            arg.to_str()?.strip_prefix(name)?.strip_prefix('=')
        }
    })
}

fn compose_cfg(
    arguments: &mut Vec<OsString>,
    key: &str,
    value: &str,
    replace: bool,
) -> io::Result<()> {
    let required = format!("{key}=\"{value}\"");
    let mut found = false;
    let mut index = 0;
    while index < arguments.len() {
        let (cfg, count) = if arguments[index] == "--cfg" {
            (arguments.get(index + 1).and_then(|a| a.to_str()), 2)
        } else {
            (
                arguments[index]
                    .to_str()
                    .and_then(|a| a.strip_prefix("--cfg=")),
                1,
            )
        };
        if let Some(cfg) = cfg {
            let selected_key = cfg.split_once('=').map_or(cfg, |(key, _)| key).trim();
            if selected_key == key {
                if cfg
                    .split_once('=')
                    .is_some_and(|(_, selected)| selected.trim() == format!("\"{value}\""))
                {
                    found = true;
                } else if replace {
                    arguments.drain(index..index + count);
                    continue;
                } else {
                    return Err(invalid(format!("conflicting target cfg for {key}")));
                }
            }
        }
        index += 1;
    }
    if !found {
        arguments.push(format!("--cfg={required}").into());
    }
    Ok(())
}

fn invalid(error: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, error)
}
