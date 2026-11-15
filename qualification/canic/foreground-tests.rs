//! Export-only consumer tests; no production Tooling or sibling test module.
use super::*;

fn fixture(name: &str, mode: &str) -> (Cli, PathBuf) {
    let root = PathBuf::from(std::env::var_os("CANIC_FOREGROUND_EVIDENCE").unwrap()).join(name);
    std::fs::create_dir(&root).unwrap();
    let tool = std::env::var("CANIC_FOREGROUND_TOOL").unwrap();
    let cli = Cli::try_parse_from([
        "canic-profile",
        "--canister",
        "2vxsx-fae",
        "--network",
        mode,
        "--identity",
        root.to_str().unwrap(),
        "--icp-bin",
        &tool,
        "--root-key",
        "fixture root",
        "info",
    ])
    .unwrap();
    (cli, root)
}

fn once(root: &std::path::Path) {
    assert_eq!(
        std::fs::read(root.join("calls")).unwrap(),
        b"call\n",
        "no implicit retry"
    );
}

#[test]
fn exact_arguments_and_reply_bytes_stay_consumer_owned() {
    let (cli, root) = fixture("argument custody", "valid");
    let result = invoke(
        &cli,
        "--target $(not executed)",
        "method with space",
        &[0, 255],
        true,
    )
    .unwrap();
    assert_eq!(result, [0, 255, 16]);
    once(&root);
    let arguments = std::fs::read(root.join("arguments")).unwrap();
    let actual: Vec<_> = arguments
        .split(|&b| b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| std::str::from_utf8(s).unwrap())
        .collect();
    assert_eq!(
        actual,
        [
            "canister",
            "call",
            "--args-format",
            "hex",
            "--output",
            "hex",
            "--network",
            "valid",
            "--identity",
            root.to_str().unwrap(),
            "--root-key",
            "fixture root",
            "--query",
            "--",
            "--target $(not executed)",
            "method with space",
            "00ff"
        ]
    );
    let (cli, root) = fixture("binary arguments", "valid");
    let input = root.join("submitted bytes.bin");
    std::fs::write(&input, [0, 255]).unwrap();
    assert_eq!(
        invoke_arguments(&cli, "target", "method", None, Some(&input), false).unwrap(),
        [0, 255, 16]
    );
    once(&root);
    let arguments = std::fs::read(root.join("arguments")).unwrap();
    let actual: Vec<_> = arguments
        .split(|&b| b == 0)
        .filter(|s| !s.is_empty())
        .map(|s| std::str::from_utf8(s).unwrap())
        .collect();
    assert_eq!(&actual[2..4], ["--args-format", "bin"]);
    assert_eq!(
        &actual[actual.len() - 5..],
        [
            "--args-file",
            input.to_str().unwrap(),
            "--",
            "target",
            "method"
        ]
    );
}

#[test]
fn exact_limit_passes_and_overflow_is_not_decoded_or_retried() {
    let (cli, root) = fixture("exact", "exact");
    assert_eq!(
        invoke(&cli, "target", "method", &[], false).unwrap(),
        vec![0; 8192]
    );
    once(&root);
    let (cli, root) = fixture("overflow", "overflow");
    let error = invoke(&cli, "target", "method", &[], false).unwrap_err();
    let evidence = error.downcast_ref::<InvocationFailure>().unwrap();
    assert!(matches!(
        evidence.report.failure,
        Some(process::Failure::Limit(process::Stream::Stdout))
    ));
    assert!(evidence.stdout.len() <= 16384);
    assert!(evidence.report.status.is_some());
    assert!(evidence.report.cleanup.is_empty());
    once(&root);
}

#[test]
fn nonzero_status_and_invalid_hex_never_become_replies() {
    let (cli, root) = fixture("nonzero", "nonzero");
    let error = invoke(&cli, "target", "method", &[], false).unwrap_err();
    let evidence = error.downcast_ref::<InvocationFailure>().unwrap();
    assert_eq!(evidence.report.status.unwrap().code(), Some(7));
    assert_eq!(evidence.stdout, b"00");
    assert!(evidence.report.failure.is_none());
    assert!(evidence.report.cleanup.is_empty());
    once(&root);
    for mode in ["utf8", "odd", "nonhex"] {
        let (cli, root) = fixture(mode, mode);
        assert!(invoke(&cli, "target", "method", &[], false).is_err());
        once(&root);
    }
}

#[test]
#[ignore = "requires controlling PTY with fixture-answer on stdin"]
fn inherited_terminal_input_and_stderr_remain_interactive() {
    let (cli, root) = fixture("terminal", "terminal");
    assert_eq!(
        invoke(&cli, "target", "method", &[], false).unwrap(),
        [0, 255]
    );
    once(&root);
}
