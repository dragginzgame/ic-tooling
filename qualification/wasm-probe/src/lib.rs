//! A real dependency-free Wasm target used to qualify owned build capture.

/// Stable exported value for independent inspection of the compiled artifact.
#[expect(
    unsafe_code,
    reason = "qualification fixture exposes one uniquely named Wasm ABI symbol"
)]
#[unsafe(no_mangle)]
pub static OWNED_ARTIFACT_PROBE: [u8; 17] = *b"owned-wasm-probe!";

#[cfg(feature = "refuse")]
compile_error!("intentional owned-build diagnostic oracle");
