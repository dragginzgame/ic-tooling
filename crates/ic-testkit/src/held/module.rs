//! Single-purpose update fixtures; the HTTP response opens a fixed reply callback.

use std::{collections::BTreeMap, fmt::Write as _};

use ic_platform_protocol::http_request::{EncodeError, encode_http_get_args};
use ic_platform_types::Principal;

use super::Catalog;

///
/// CompileError
/// Refusal while generating an admitted catalog's module.
///

#[derive(Debug, thiserror::Error)]
pub enum CompileError {
    /// The generated artifact exceeds the selected byte cap.
    #[error("fixture exceeds compiled-module limit {limit}")]
    ModuleLimit {
        /// Caller-selected maximum.
        limit: usize,
    },
    /// Encoded data or callback indices exceed nonnegative Wasm32 addressing.
    #[error("fixture exceeds Wasm32 representation")]
    Representation,
    /// Platform refused the bounded HTTP request encoding.
    #[error("barrier encoding failed: {0:?}")]
    Encoding(EncodeError),
    /// The Wasm compiler refused the generated source.
    #[error("fixture compilation failed: {0}")]
    Compilation(#[from] wat::Error),
}

impl Catalog {
    /// Generate a held-update module for a caller-created target.
    ///
    /// Repeated methods consume slots in catalog order. A successful HTTP barrier
    /// callback returns the slot's immutable bytes, ignoring the HTTP response.
    /// Rejection and exhaustion reject the original call. Attached cycles are not
    /// accepted. No server, installation, funding or request authority is created.
    ///
    /// The caller supplies cycles per barrier and the compiled artifact byte cap.
    /// This bounds returned bytes, not compiler peak memory. Request control is a
    /// separate boundary: use [`super::Fixture`] for an exclusive controller.
    /// The target must not be reinstalled while calls are live.
    ///
    /// # Errors
    /// Returns module-budget, representation, Platform encoding or compilation
    /// failures without returning partial Wasm.
    pub fn compile(
        &self,
        target: Principal,
        call_cycles: u128,
        module_bytes: usize,
    ) -> Result<Vec<u8>, CompileError> {
        self.compile_prefix(
            &format!("https://ic-testkit.invalid/held/{target}/"),
            call_cycles,
            module_bytes,
        )
    }

    pub(super) fn compile_prefix(
        &self,
        prefix: &str,
        call_cycles: u128,
        module_bytes: usize,
    ) -> Result<Vec<u8>, CompileError> {
        let mut source = String::from(
            r#"(module
(import "ic0" "msg_reply_data_append" (func $append (param i32 i32)))
(import "ic0" "msg_reply" (func $reply))
(import "ic0" "msg_reject" (func $reject (param i32 i32)))
(import "ic0" "call_new" (func $new (param i32 i32 i32 i32 i32 i32 i32 i32)))
(import "ic0" "call_data_append" (func $data (param i32 i32)))
(import "ic0" "call_cycles_add128" (func $cycles (param i64 i64)))
(import "ic0" "call_perform" (func $perform (result i32)))
(func $failed (param i32) (call $reject (i32.const 12) (i32.const 14)))
"#,
        );
        let mut layout = Layout {
            end: 0,
            limit: module_bytes,
        };
        layout.append(&mut source, b"http_requestfixture failed")?;
        let mut methods: BTreeMap<&str, Vec<usize>> = BTreeMap::new();
        for (index, slot) in self.slots.iter().enumerate() {
            // Callback zero is failure; each slot gets one independent callback.
            let callback = index
                .checked_add(1)
                .filter(|n| *n <= i32::MAX as usize)
                .ok_or(CompileError::Representation)?;
            let reply = layout.append(&mut source, &slot.bytes)?;
            let encoded = encode_http_get_args(
                &format!("{prefix}{index}"),
                Some(&(index as u64).to_le_bytes()),
                256,
                true,
                module_bytes,
            )
            .map_err(CompileError::Encoding)?;
            let request = layout.append(&mut source, &encoded)?;
            // Writing into String is infallible apart from process allocation failure.
            let _ = writeln!(
                source,
                "(global $used{index} (mut i32) (i32.const 0))\n\
                 (func $reply{index} (param i32)\n\
                   (call $append (i32.const {reply}) (i32.const {})) (call $reply))\n\
                 (func $start{index}\n\
                   (global.set $used{index} (i32.const 1))\n\
                   (call $new (i32.const 0) (i32.const 0) (i32.const 0) (i32.const 12)\n\
                     (i32.const {callback}) (i32.const 0) (i32.const 0) (i32.const 0))\n\
                   (call $data (i32.const {request}) (i32.const {}))\n\
                   (call $cycles (i64.const {}) (i64.const {}))\n\
                   (if (call $perform) (then (call $failed (i32.const 0)))))",
                slot.bytes.len(),
                encoded.len(),
                (call_cycles >> 64) as u64 as i64,
                call_cycles as u64 as i64
            );
            methods.entry(&slot.method).or_default().push(index);
        }
        let pages = layout.end.div_ceil(65536).max(1);
        let _ = write!(
            source,
            "(memory {pages} {pages})\n(table {} funcref)\n(elem (i32.const 0) $failed",
            self.len() + 1
        );
        for index in 0..self.len() {
            let _ = write!(source, " $reply{index}");
        }
        source.push_str(")\n");
        for (method, indices) in methods {
            source.push_str("(func (export \"");
            hex(&mut source, b"canister_update ");
            hex(&mut source, method.as_bytes());
            source.push_str("\")\n");
            for index in indices {
                let _ = writeln!(
                    source,
                    "(if (i32.eqz (global.get $used{index}))\n\
                       (then (call $start{index}) (return)))"
                );
            }
            source.push_str("(call $failed (i32.const 0)))\n");
        }
        source.push(')');
        let wasm = wat::parse_str(&source)?;
        if wasm.len() > module_bytes {
            return Err(CompileError::ModuleLimit {
                limit: module_bytes,
            });
        }

        Ok(wasm)
    }
}

///
/// Layout
/// Assigns bounded offsets to data segments in the generated Wasm module.
///

struct Layout {
    end: usize,
    limit: usize,
}

impl Layout {
    fn append(&mut self, source: &mut String, bytes: &[u8]) -> Result<usize, CompileError> {
        let start = self.end;
        let end = start
            .checked_add(bytes.len())
            .filter(|end| *end <= i32::MAX as usize)
            .ok_or(CompileError::Representation)?;
        // Active data is embedded verbatim, so it is a lower bound on module size.
        if end > self.limit {
            return Err(CompileError::ModuleLimit { limit: self.limit });
        }
        let _ = write!(source, "(data (i32.const {start}) \"");
        hex(source, bytes);
        source.push_str("\")\n");
        self.end = end;

        Ok(start)
    }
}

fn hex(out: &mut String, bytes: &[u8]) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";

    for &byte in bytes {
        out.push('\\');
        out.push(char::from(DIGITS[usize::from(byte >> 4)]));
        out.push(char::from(DIGITS[usize::from(byte & 15)]));
    }
}
