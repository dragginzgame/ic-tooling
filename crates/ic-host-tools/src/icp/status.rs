//! Installation and cycle-balance projections over successful ICP CLI status JSON.

use serde::{Deserialize, Deserializer};

/// Status JSON does not establish an installed or uninstalled canister.
#[derive(Debug, thiserror::Error)]
pub enum InstallationStatusError {
    /// The complete input is not valid JSON.
    #[error("parse icp canister status JSON: {0}")]
    Json(#[source] serde_json::Error),
    /// Successful status output did not include the installation fact.
    #[error("icp canister status JSON omitted module_hash")]
    MissingModuleHash,
    /// The installation fact is neither null nor a string.
    #[error("icp canister status JSON has an invalid module_hash")]
    InvalidModuleHash,
}

/// Project the installation fact from one successful, caller-captured status.
///
/// A null `module_hash` means created but uninstalled; any string means installed.
/// Missing, malformed or failed probes never establish absence. The caller must
/// check command success first and owns input capture/byte limits and JSON schema
/// requirements beyond this single fact. Unknown fields are ignored.
///
/// # Errors
/// Returns typed JSON, missing-field or invalid-field failure without guessing.
pub fn decode_installation_status(input: &[u8]) -> Result<bool, InstallationStatusError> {
    let status: serde_json::Value =
        serde_json::from_slice(input).map_err(InstallationStatusError::Json)?;
    match status.get("module_hash") {
        Some(serde_json::Value::Null) => Ok(false),
        Some(serde_json::Value::String(_)) => Ok(true),
        Some(_) => Err(InstallationStatusError::InvalidModuleHash),
        None => Err(InstallationStatusError::MissingModuleHash),
    }
}

/// A supplied cycle balance could not be projected from one status JSON object.
#[derive(Debug, thiserror::Error)]
pub enum CycleBalanceError {
    /// Complete input exceeds the caller-selected byte allowance.
    #[error("ICP status exceeds {limit} bytes")]
    InputLimit {
        /// Maximum complete input bytes.
        limit: usize,
    },
    /// Complete input is not valid UTF-8, including ignored metadata.
    #[error("ICP output is not UTF-8: {0}")]
    Utf8(#[source] std::str::Utf8Error),
    /// Invalid JSON or status shape, including null/repeated `cycles`.
    #[error("parse ICP cycle balance JSON: {0}")]
    Json(#[source] serde_json::Error),
    /// The supplied string is not an unsigned decimal balance within `u128`.
    #[error(
        "ICP cycles must be unsigned decimal with optional three-digit underscore groups fitting u128"
    )]
    InvalidCycles,
}

#[derive(Deserialize)]
struct CycleStatus {
    #[serde(default, deserialize_with = "present_cycles")]
    cycles: Option<String>,
}

// Public status omits the balance. An explicitly supplied null must not erase a
// malformed observation into that legitimate absence.
fn present_cycles<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    String::deserialize(deserializer).map(Some)
}

/// Project the exact cycle balance from one bounded, successful status JSON.
///
/// Management status supplies a decimal string, with Candid's underscore-separated
/// groups of three digits; ungrouped ASCII decimal is also accepted. Public-only
/// status omits it and returns `None`, never zero. Empty, signed, fractional,
/// incorrectly grouped, whitespace-padded or overflowing strings refuse. Unknown fields are ignored. Missing cycles are
/// allowed; supplied null/wrong-type/repeated fields and multiple objects refuse.
/// The caller owns successful-command admission, byte allowance and funding
/// policy. This function never executes, retries or authorizes a top-up.
///
/// # Errors
/// Returns input-limit, UTF-8, JSON/shape or invalid-decimal errors. The input allowance
/// bounds parser input, not an independent parser-memory budget.
pub fn decode_canister_cycles(
    input: &[u8],
    max_bytes: usize,
) -> Result<Option<u128>, CycleBalanceError> {
    if input.len() > max_bytes {
        return Err(CycleBalanceError::InputLimit { limit: max_bytes });
    }
    // Validate the complete input: Serde may skip invalid bytes in ignored fields.
    let text = std::str::from_utf8(input).map_err(CycleBalanceError::Utf8)?;
    // Serde structs also accept positional arrays; this protocol requires an object.
    if input.iter().find(|byte| !byte.is_ascii_whitespace()) != Some(&b'{') {
        return Err(CycleBalanceError::Json(serde::de::Error::custom(
            "expected one ICP status JSON object",
        )));
    }
    let status: CycleStatus = serde_json::from_str(text).map_err(CycleBalanceError::Json)?;
    status
        .cycles
        .map(|cycles| {
            // ICP serializes Nat via Display, which groups thousands. Validate that
            // syntax before discarding separators; no Candid dependency is needed.
            let grouped = cycles.contains('_');
            if cycles.split('_').enumerate().any(|(index, group)| {
                group.is_empty()
                    || !group.bytes().all(|byte| byte.is_ascii_digit())
                    || (grouped
                        && if index == 0 {
                            group.len() > 3
                        } else {
                            group.len() != 3
                        })
            }) {
                return Err(CycleBalanceError::InvalidCycles);
            }
            cycles
                .bytes()
                .filter(|byte| *byte != b'_')
                .try_fold(0_u128, |value, byte| {
                    value
                        .checked_mul(10)
                        .and_then(|value| value.checked_add(u128::from(byte - b'0')))
                        .ok_or(CycleBalanceError::InvalidCycles)
                })
        })
        .transpose()
}

#[cfg(test)]
mod tests;
