use std::{env, fs, io, path::PathBuf, sync::atomic::AtomicBool, time::Duration};

use crate::{BuildRequest, Failure, Limits, build};

pub(crate) fn limits() -> Limits {
    Limits {
        stdout_bytes: 4 * 1024 * 1024,
        stderr_bytes: 1024 * 1024,
        record_bytes: 1024 * 1024,
        candidates: 8,
        artifact_bytes: 4 * 1024 * 1024,
        elapsed: Duration::from_secs(60),
    }
}

fn request(scratch: PathBuf) -> BuildRequest {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    let home = PathBuf::from(env::var_os("HOME").unwrap());
    let toolchain = PathBuf::from(env!("CARGO")).parent().unwrap().to_owned();
    // Explicit overrides allow native macOS qualification with its prepared tools.
    let cargo =
        env::var_os("IC_BUILD_TEST_CARGO").map_or_else(|| toolchain.join("cargo"), PathBuf::from);
    let rustc =
        env::var_os("IC_BUILD_TEST_RUSTC").map_or_else(|| toolchain.join("rustc"), PathBuf::from);
    let manifest = root.join("qualification/owned-wasm-probe/Cargo.toml");
    let package_id = format!("path+file://{}#0.0.0", manifest.parent().unwrap().display());

    let mut environment = vec![
        ("HOME".into(), home.into_os_string()),
        ("PATH".into(), env::var_os("PATH").unwrap()),
    ];
    if let Some(mode) = env::var_os("CARGO_NET_OFFLINE") {
        environment.push(("CARGO_NET_OFFLINE".into(), mode));
    }

    BuildRequest {
        manifest,
        package: "owned-wasm-probe".into(),
        package_id,
        library: "owned_wasm_probe".into(),
        cargo,
        rustc,
        scratch_parent: scratch,
        profile: "dev".into(),
        features: Vec::new(),
        default_features: false,
        environment,
        limits: limits(),
    }
}

#[test]
fn real_wasm_capture_survives_another_build_and_removes_scratch() {
    let scratch_root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/owned-build-tests");
    fs::create_dir_all(&scratch_root).unwrap();
    let scratch = tempfile::tempdir_in(scratch_root).unwrap();
    let request = request(scratch.path().to_owned());
    let first = build(&request, &AtomicBool::new(false), &mut io::sink()).unwrap();
    assert_eq!(&first.bytes()[..8], b"\0asm\x01\0\0\0");
    assert!(
        first
            .bytes()
            .windows(17)
            .any(|bytes| bytes == b"owned-wasm-probe!")
    );
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 0);
    let original = first.bytes().to_vec();
    let second = build(&request, &AtomicBool::new(false), &mut io::sink()).unwrap();
    assert_eq!(first.bytes(), original);
    assert_eq!(&second.bytes()[..4], b"\0asm");
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 0);
}

#[test]
fn cargo_child_uses_the_requested_network_mode_and_keeps_the_lock() {
    let scratch = tempfile::tempdir().unwrap();
    let mut request = request(scratch.path().to_owned());
    request.cargo =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/cargo-network-mode.sh");
    for mode in [None, Some("true"), Some("false")] {
        request
            .environment
            .retain(|(key, _)| key != "CARGO_NET_OFFLINE");
        if let Some(value) = mode {
            request
                .environment
                .push(("CARGO_NET_OFFLINE".into(), value.into()));
        }
        let failure = build(&request, &AtomicBool::new(false), &mut io::sink()).unwrap_err();
        assert_eq!(failure.status.unwrap().code(), Some(7));
        let observed = std::str::from_utf8(&failure.diagnostics).unwrap();
        let mut lines = observed.lines();
        assert_eq!(lines.next(), Some(mode.unwrap_or("unset")));
        let args: Vec<_> = lines.collect();
        assert!(args.contains(&"--locked"));
        assert!(!args.contains(&"--offline"));
        assert!(!args.contains(&"--frozen"));
        assert!(!args.iter().any(|arg| arg.starts_with("net.offline")));
    }
}

#[test]
fn natural_tool_failure_keeps_protocol_cause_and_cleans_target() {
    let scratch = tempfile::tempdir().unwrap();
    let executable = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests/failed-cargo.sh");
    let mut request = request(scratch.path().to_owned());
    request.cargo = executable;
    let failure = build(&request, &AtomicBool::new(false), &mut io::sink()).unwrap_err();
    assert!(
        matches!(failure.primary, Failure::ToolFailed),
        "{failure:?}"
    );
    assert!(matches!(failure.protocol, Some(Failure::Json(_))));
    assert_eq!(failure.status.unwrap().code(), Some(7));
    assert_eq!(failure.diagnostics, b"diagnostic");
    assert!(failure.cleanup.is_empty());
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 0);
}

#[test]
fn invalid_request_and_precancellation_have_no_build_effects() {
    let scratch = tempfile::tempdir().unwrap();
    let mut request = request(scratch.path().to_owned());
    let failure = build(&request, &AtomicBool::new(true), &mut io::sink()).unwrap_err();
    assert!(matches!(failure.primary, Failure::Cancelled));
    request
        .environment
        .push(("RUSTC".into(), "override".into()));
    let failure = build(&request, &AtomicBool::new(false), &mut io::sink()).unwrap_err();
    assert!(matches!(failure.primary, Failure::InvalidRequest(_)));
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 0);
}

#[test]
fn capture_limit_failure_removes_real_cargo_output() {
    let scratch_root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/owned-build-tests");
    fs::create_dir_all(&scratch_root).unwrap();
    let scratch = tempfile::tempdir_in(scratch_root).unwrap();
    let mut request = request(scratch.path().to_owned());
    request.limits.artifact_bytes = 1;
    let failure = build(&request, &AtomicBool::new(false), &mut io::sink()).unwrap_err();
    assert!(matches!(failure.primary, Failure::ArtifactLimit));
    assert!(failure.status.unwrap().success());
    assert!(failure.cleanup.is_empty());
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 0);
}

#[test]
fn real_compiler_refusal_preserves_rendered_diagnostics() {
    let scratch_root =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/owned-build-tests");
    fs::create_dir_all(&scratch_root).unwrap();
    let scratch = tempfile::tempdir_in(scratch_root).unwrap();
    let mut request = request(scratch.path().to_owned());
    request.features.push("refuse".into());
    let mut streamed = Vec::new();
    let failure = build(&request, &AtomicBool::new(false), &mut streamed).unwrap_err();
    assert!(
        matches!(failure.primary, Failure::ToolFailed),
        "{failure:?}"
    );
    assert_eq!(streamed, failure.diagnostics);
    assert!(
        String::from_utf8(streamed)
            .unwrap()
            .contains("intentional owned-build diagnostic oracle")
    );
    assert!(failure.cleanup.is_empty());
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 0);
}
