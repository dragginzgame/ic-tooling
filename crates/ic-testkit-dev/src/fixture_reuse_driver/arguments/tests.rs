use super::parse_arguments;

#[test]
fn help_is_owned_by_clap_before_workload_admission() {
    let error = parse_arguments(["--help"].map(Into::into))
        .err()
        .expect("display help");
    assert_eq!(error.kind(), clap::error::ErrorKind::DisplayHelp);
    assert_eq!(error.exit_code(), 0);
}

#[cfg(unix)]
#[test]
fn last_selection_and_native_path_bytes_are_preserved() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};
    let server = OsString::from_vec(vec![b's', 255]);
    let output = OsString::from_vec(vec![b'o', 254]);
    let args = [
        OsString::from("--server"),
        server.clone(),
        "--output".into(),
        output.clone(),
        "--workers".into(),
        "3".into(),
        "--workers".into(),
        "2".into(),
        "--modes".into(),
        "fresh".into(),
        "--modes".into(),
        "pooled".into(),
        "--capacities".into(),
        "1".into(),
        "--capacities".into(),
        "2".into(),
    ];
    let options = parse_arguments(args).unwrap();
    assert_eq!(options.server.as_os_str(), &server);
    assert_eq!(options.output.unwrap().as_os_str(), &output);
    assert_eq!(options.workers.get(), 2);
    assert_eq!(options.modes, [crate::fixture_reuse_protocol::Mode::Pooled]);
    assert_eq!(
        options
            .capacities
            .iter()
            .map(|value| value.get())
            .collect::<Vec<_>>(),
        [2]
    );
}

#[test]
fn capacity_sweep_preserves_order_and_one_fresh_control() {
    let options = parse_arguments(
        [
            "--server",
            "/unused/pocket-ic",
            "--iterations",
            "8",
            "--workers",
            "8",
            "--modes",
            "pooled",
            "fresh",
            "--capacities",
            "1",
            "2",
            "4",
            "8",
        ]
        .map(Into::into),
    )
    .unwrap();
    let cases = options
        .benchmark_cases()
        .into_iter()
        .map(|case| (case.label, case.mode.as_str(), case.capacity.get()))
        .collect::<Vec<_>>();
    assert_eq!(
        cases,
        [
            ("pooled-1".into(), "pooled", 1),
            ("pooled-2".into(), "pooled", 2),
            ("pooled-4".into(), "pooled", 4),
            ("pooled-8".into(), "pooled", 8),
            ("fresh".into(), "fresh", 8)
        ]
    );
}

#[test]
fn default_and_fresh_only_selection() {
    let options = parse_arguments(["--server", "/unused/pocket-ic"].map(Into::into)).unwrap();
    assert_eq!(options.rss_limits(), super::RSS_COMMAND_LIMITS);
    assert_eq!(
        options
            .benchmark_cases()
            .iter()
            .map(|case| case.label.as_str())
            .collect::<Vec<_>>(),
        ["fresh", "pooled-1", "pooled-2"]
    );
    let options = parse_arguments(
        [
            "--server",
            "/unused/pocket-ic",
            "--modes",
            "fresh",
            "--capacities",
            "8",
        ]
        .map(Into::into),
    )
    .unwrap();
    assert_eq!(options.benchmark_cases().len(), 1);
    assert_eq!(options.benchmark_cases()[0].capacity.get(), 2);
}

#[test]
fn rss_limits_preserve_explicit_selection_without_a_deadline() {
    let options = parse_arguments(
        [
            "--server",
            "/unused/pocket-ic",
            "--rss-stdout-bytes",
            "1024",
            "--rss-stderr-bytes",
            "128",
        ]
        .map(Into::into),
    )
    .unwrap();
    assert_eq!(
        options.rss_limits().stdout,
        ic_host_process::tool::OutputLimit::Terminate(1024)
    );
    assert_eq!(
        options.rss_limits().stderr,
        ic_host_process::tool::OutputLimit::Terminate(128)
    );
    assert_eq!(options.rss_limits().timeout, None);
}

#[test]
fn invalid_selections_fail_before_launching_tools() {
    for selection in [
        vec!["--capacities", "0"],
        vec!["--capacities", "-1"],
        vec!["--capacities", "2", "2"],
        vec!["--modes", "pooled", "pooled"],
        vec!["--workers", "101"],
        vec!["--state-bytes", "4294967296"],
        vec!["--profile", "custom"],
        vec!["--rss-stdout-bytes", "0"],
        vec!["--rss-stderr-bytes", "18446744073709551616"],
        vec!["--modes"],
        vec!["--unknown"],
    ] {
        let args = ["--server", "/unused/pocket-ic"]
            .into_iter()
            .chain(selection)
            .map(Into::into);
        let error = parse_arguments(args).err().expect("invalid selection");
        assert_eq!(error.exit_code(), 2);
    }
    assert!(parse_arguments(["--server", "/unused", "--state-bytes", "0"].map(Into::into)).is_ok());
}
