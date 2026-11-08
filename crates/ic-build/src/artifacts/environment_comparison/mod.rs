//! Keyed environment equality diagnostics, never cache or release authority.
//! Key custody, entropy, filtering, persistence and display belong to callers.
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::{collections::BTreeMap, ffi::OsString};
/// Failure to produce or compare bounded private evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum ComparisonError {
    /// Input or retained-record count exceeds the caller's bound.
    #[error("environment comparison exceeds the input limit")]
    InputLimit,
    /// Different private keys cannot establish value equality.
    #[error("environment comparisons use different keys")]
    DifferentKeys,
}
/// Private equality tags. Serialization contains no environment values; do not publish tags.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentComparison {
    key_id: [u8; 32],
    tags: BTreeMap<String, [u8; 32]>,
}
impl EnvironmentComparison {
    /// Capture only bounded ASCII key names and keyed value tags. Unknown values remain opaque.
    /// # Errors
    /// `InputLimit` when the selected input count exceeds the caller budget.
    pub fn capture(
        key: &[u8; 32],
        inputs: &[(OsString, OsString)],
        maximum_inputs: usize,
    ) -> Result<Self, ComparisonError> {
        if inputs.len() > maximum_inputs {
            return Err(ComparisonError::InputLimit);
        }
        let mut tags = BTreeMap::new();
        for (name, value) in inputs {
            if let Some(name) = name.to_str().filter(|n| safe_key(n)) {
                tags.insert(
                    name.to_owned(),
                    tag(
                        key,
                        b"environment-value",
                        &[name.as_bytes(), value.as_encoded_bytes()],
                    ),
                );
            }
        }
        Ok(Self {
            key_id: tag(key, b"environment-key-id", &[]),
            tags,
        })
    }
    /// Return changed values among common admitted keys. Added/removed keys remain caller evidence.
    /// # Errors
    /// `InputLimit` for oversized retained evidence or `DifferentKeys` for distinct private keys.
    pub fn changed_keys(
        &self,
        previous: &Self,
        maximum_inputs: usize,
        maximum_results: usize,
    ) -> Result<Vec<String>, ComparisonError> {
        if self.tags.len() > maximum_inputs || previous.tags.len() > maximum_inputs {
            return Err(ComparisonError::InputLimit);
        }
        if self.key_id != previous.key_id {
            return Err(ComparisonError::DifferentKeys);
        }
        Ok(self
            .tags
            .iter()
            .filter(|(name, value)| {
                safe_key(name) && previous.tags.get(*name).is_some_and(|old| old != *value)
            })
            .take(maximum_results)
            .map(|(name, _)| name.clone())
            .collect())
    }
}
fn safe_key(key: &str) -> bool {
    !key.is_empty()
        && key.len() <= 80
        && key.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
}
fn tag(key: &[u8; 32], domain: &[u8], fields: &[&[u8]]) -> [u8; 32] {
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC accepts a 32-byte key");
    mac.update(domain);
    for field in fields {
        mac.update(&(field.len() as u64).to_le_bytes());
        mac.update(field);
    }
    mac.finalize().into_bytes().into()
}
#[cfg(test)]
mod tests;
