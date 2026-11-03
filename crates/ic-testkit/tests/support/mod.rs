// Each integration target uses only the fixtures it needs.
#![allow(
    dead_code,
    reason = "integration targets use different subsets of the shared server/wait helpers"
)]

pub mod wait;

pub fn startup_config() -> ic_testkit::pic::PocketIcStartupConfig {
    ic_testkit::pic::PocketIcStartupConfig::from_env(std::time::Duration::from_secs(30))
        .expect("prepare POCKET_IC_BIN or configure IC_TESTKIT_POCKET_IC_URL for live tests")
}

pub fn pocket_ic() -> ic_testkit::pic::PocketIcInstance<ic_testkit::pocket_ic::PocketIc> {
    startup_config()
        .build_instance(ic_testkit::pocket_ic::PocketIcBuilder::new().with_application_subnet())
        .expect("construct explicitly configured test instance")
}

#[cfg(feature = "held-reply-fixtures")]
pub fn wat_bytes(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut escaped = String::new();
    for byte in bytes {
        write!(escaped, "\\{byte:02x}").unwrap();
    }
    escaped
}

#[cfg(feature = "held-reply-fixtures")]
pub fn install_wat(
    pic: &ic_testkit::pocket_ic::PocketIc,
    canister: ic_platform_types::principal::ExternalPrincipal,
    source: &str,
    module_bytes: usize,
) {
    let wasm = ic_host_artifacts::wasm_text::compile(
        source,
        ic_host_artifacts::wasm_text::CompileLimits {
            source_bytes: source.len(),
            module_bytes,
        },
    )
    .unwrap();
    ic_host_artifacts::wasm::validate(&wasm, module_bytes).unwrap();
    pic.install_canister(canister, wasm, vec![], None).unwrap();
}
