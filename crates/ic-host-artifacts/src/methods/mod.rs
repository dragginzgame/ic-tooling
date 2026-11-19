//! Inventory public IC method exports in a complete, bounded core Wasm module.

#[cfg(test)]
mod tests;

use std::{collections::BTreeMap, io::Read};

use thiserror::Error;
use wasmparser::{ExternalKind, Parser, Payload, Validator};

use crate::{Identity, Length, read};

/// Execution kind encoded by an IC method export.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// A replicated update.
    Update,
    /// A query.
    Query,
    /// A composite query.
    CompositeQuery,
}

/// Refusal to produce a complete inventory.
#[derive(Debug, Error)]
pub enum Error {
    /// Reading or bounding the original artifact failed.
    #[error(transparent)]
    Admission(#[from] crate::Error),
    /// The artifact lacks a core Wasm module header.
    #[error("expected a core WebAssembly module")]
    Format,
    /// The module failed structural or instruction validation.
    #[error(transparent)]
    Wasm(#[from] wasmparser::BinaryReaderError),
    /// A public IC entry point has parameters or results.
    #[error("invalid IC entry point signature for {0:?}")]
    Signature(String),
    /// Two public exports identify the same method.
    #[error("ambiguous IC method {0:?}")]
    Ambiguous(String),
}

/// Read a raw Wasm artifact and inventory its public methods by exact name.
/// Returns the original bytes' identity with the method map. Other function
/// exports (including lifecycle hooks) and all non-function exports are omitted.
///
/// # Errors
/// Refuses incomplete admission, invalid core Wasm, incompatible public entry
/// point signatures and duplicate method identities. This inspects exports
/// without executing the module or checking all IC installation requirements.
pub fn inventory(
    reader: impl Read,
    length: Length,
) -> Result<(BTreeMap<String, Kind>, Identity), Error> {
    let (bytes, identity) = read(reader, length)?;
    if !Parser::is_core_wasm(&bytes) {
        return Err(Error::Format);
    }
    let types = Validator::new().validate_all(&bytes)?;
    let mut methods = BTreeMap::new();
    for payload in Parser::new(0).parse_all(&bytes) {
        let Payload::ExportSection(exports) = payload? else {
            continue;
        };
        for export in exports {
            let export = export?;
            if export.kind != ExternalKind::Func {
                continue;
            }
            let Some((prefix, name)) = export.name.split_once(' ') else {
                continue;
            };
            let kind = match prefix {
                "canister_update" => Kind::Update,
                "canister_query" => Kind::Query,
                "canister_composite_query" => Kind::CompositeQuery,
                _ => continue,
            };
            let signature = types[types.as_ref().core_function_at(export.index)].unwrap_func();
            if !signature.params().is_empty() || !signature.results().is_empty() {
                return Err(Error::Signature(name.to_owned()));
            }
            if methods.insert(name.to_owned(), kind).is_some() {
                return Err(Error::Ambiguous(name.to_owned()));
            }
        }
    }

    Ok((methods, identity))
}
