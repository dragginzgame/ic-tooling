// Each integration target uses only the fixtures it needs.
#![allow(
    dead_code,
    reason = "independent integration targets use different subsets; default concurrency uses every helper, so a shared expectation cannot cover all target/feature selections"
)]

pub fn empty_install_spec() -> ic_testkit::pic::InstallSpec {
    ic_testkit::pic::InstallSpec {
        wasm: b"\0asm\x01\0\0\0".to_vec(),
        init_bytes: vec![],
        cycles: 0,
        install_sender: None,
    }
}

pub async fn install_empty_canister(
    pic: &pocket_ic::nonblocking::PocketIc,
) -> pocket_ic::CanisterId {
    ic_testkit::pic::create_and_install(pic, empty_install_spec()).await
}

pub fn startup_config() -> ic_testkit::pic::PocketIcStartupConfig {
    ic_testkit::pic::PocketIcStartupConfig::from_env(std::time::Duration::from_secs(30))
        .expect("prepare POCKET_IC_BIN or configure IC_TESTKIT_POCKET_IC_URL for live tests")
}

pub async fn pocket_ic() -> ic_testkit::pic::PocketIcInstance {
    startup_config()
        .build_instance(
            pocket_ic::PocketIcBuilder::new()
                .with_max_request_time_ms(Some(300_000))
                .with_application_subnet(),
        )
        .await
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
pub async fn install_wat(
    pic: &pocket_ic::nonblocking::PocketIc,
    canister: pocket_ic::CanisterId,
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
    pic.install_canister(canister, wasm, vec![], None).await;
}

/// Catch an async caller panic on its own runtime, retaining borrowed test inputs.
pub fn catch_async_panic<T: Send>(
    operation: impl AsyncFnOnce() -> T + Send,
) -> std::thread::Result<T> {
    let runtime = tokio::runtime::Handle::current();
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    runtime.block_on(operation())
                }))
            })
            .join()
            .expect("panic catcher itself should remain live")
    })
}

/// Each blocking test coordinator delegates async simulator work to a caller-owned runtime.
pub fn spawn_async<T: Send + 'static>(
    operation: impl AsyncFnOnce() -> T + Send + 'static,
) -> std::thread::JoinHandle<T> {
    let runtime = tokio::runtime::Handle::current();
    std::thread::spawn(move || runtime.block_on(operation()))
}

#[cfg(feature = "held-reply-fixtures")]
pub async fn await_held_slot(
    pic: &pocket_ic::nonblocking::PocketIc,
    fixture: &ic_testkit::pic::fixtures::held::HeldReplyFixture,
    slot: usize,
) {
    for _ in 0..20 {
        pic.tick().await;
        if fixture
            .pending(pic)
            .await
            .unwrap()
            .iter()
            .any(|request| request.slot() == slot)
        {
            return;
        }
    }
    panic!("slot {slot} must reach the real HTTP barrier");
}
