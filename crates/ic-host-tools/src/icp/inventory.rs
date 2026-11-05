//! Bounded canister names from successful `icp canister list --json` output.

use serde::Deserialize;

/// A canister inventory could not be projected from the selected JSON output.
#[derive(Debug, thiserror::Error)]
pub enum CanisterListError {
    /// Complete input exceeds the caller-selected byte allowance.
    #[error("ICP canister list exceeds {limit} bytes")]
    InputLimit {
        /// Maximum complete input bytes.
        limit: usize,
    },
    /// Complete input is not valid UTF-8, including ignored metadata.
    #[error("ICP output is not UTF-8: {0}")]
    Utf8(#[source] std::str::Utf8Error),
    /// Invalid JSON or inventory shape, including repeated `canisters`.
    #[error("parse ICP canister list JSON: {0}")]
    Json(#[source] serde_json::Error),
}

#[derive(Deserialize)]
struct Inventory {
    canisters: Vec<String>,
}

/// Decode one bounded inventory without interpreting names or selecting targets.
///
/// Requires one JSON object with a `canisters` string array. Unknown fields are
/// ignored; empty arrays, repeated names and source order are preserved. Missing,
/// null, repeated or wrong-type `canisters` fields refuse. The input allowance
/// bounds parser input, not an independent parser-memory budget. Callers own
/// successful-command admission, sorting, name validation and fallback policy.
///
/// # Errors
/// Returns input-limit, UTF-8 or typed JSON/shape errors without inferring an inventory.
pub fn decode_canister_list(
    input: &[u8],
    max_bytes: usize,
) -> Result<Vec<String>, CanisterListError> {
    if input.len() > max_bytes {
        return Err(CanisterListError::InputLimit { limit: max_bytes });
    }
    // Validate the complete input: Serde may skip invalid bytes in ignored fields.
    let text = std::str::from_utf8(input).map_err(CanisterListError::Utf8)?;
    // Serde structs also accept positional arrays; this protocol requires an object.
    if input.iter().find(|byte| !byte.is_ascii_whitespace()) != Some(&b'{') {
        return Err(CanisterListError::Json(serde::de::Error::custom(
            "expected one ICP canister list JSON object",
        )));
    }
    let inventory: Inventory = serde_json::from_str(text).map_err(CanisterListError::Json)?;
    Ok(inventory.canisters)
}

#[cfg(test)]
mod tests;
