use super::{ExitCode, command_exit_code, failure_exit_code};
use ic_testkit::pic::PocketIcStartupFailure;
use std::os::unix::process::ExitStatusExt as _;

#[test]
fn command_arguments_preserve_native_paths_empty_values_and_limits() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt as _, time::Duration};
    let executable = OsString::from_vec(vec![b'p', 255]);
    let child_argument = OsString::from_vec(vec![b'a', 254]);
    let mut arguments = [
        "run",
        "--ttl",
        "90",
        "--idle-ttl",
        "10",
        "--startup-timeout",
        "7",
        "--server-stdout",
        "stdout path",
        "--server-stderr",
        "stderr path",
        "--",
    ]
    .map(OsString::from)
    .to_vec();
    arguments.extend([
        executable.clone(),
        "--help".into(),
        "".into(),
        child_argument.clone(),
    ]);
    let super::Arguments::Run(arguments) = super::parse(arguments.into_iter()).unwrap() else {
        panic!("run arguments required");
    };
    assert_eq!(arguments.command[0].as_os_str(), &executable);
    assert_eq!(
        arguments.command[1..]
            .iter()
            .map(OsString::as_os_str)
            .collect::<Vec<_>>(),
        [
            std::ffi::OsStr::new("--help"),
            std::ffi::OsStr::new(""),
            child_argument.as_os_str()
        ]
    );
    assert_eq!(arguments.timeout, Duration::from_secs(7));
    assert_eq!(arguments.ttl, Some(Duration::from_secs(90)));
    assert_eq!(arguments.idle_ttl, Some(Duration::from_secs(10)));
    assert_eq!(
        arguments.server_stdout.zip(arguments.server_stderr),
        Some(("stdout path".into(), "stderr path".into()))
    );
}

#[test]
fn malformed_server_arguments_refuse_before_server_admission() {
    for arguments in [
        vec!["run", "--", ""],
        vec!["run", "command"],
        vec!["run", "--ttl", "0", "--", "command"],
        vec!["run", "--ttl", "1", "--ttl", "2", "--", "command"],
        vec!["run", "--server-stdout", "stdout", "--", "command"],
        vec!["run", "--server-stderr", "stderr", "--", "command"],
        vec!["check", "--directory", ""],
    ] {
        let error = super::parse(arguments.into_iter().map(Into::into))
            .err()
            .expect("invalid arguments");
        assert_eq!(error.exit_code(), 2);
    }
}

#[test]
fn server_help_and_defaults_are_admitted_without_provisioning() {
    for arguments in [vec!["--help"], vec!["--version"], vec!["run", "--help"]] {
        let error = super::parse(arguments.into_iter().map(Into::into))
            .err()
            .expect("display request");
        assert_eq!(error.exit_code(), 0);
    }
    let super::Arguments::Check { directory } = super::parse(
        ["check", "--directory", "-store"]
            .map(Into::into)
            .into_iter(),
    )
    .unwrap() else {
        panic!("check arguments required");
    };
    assert_eq!(directory, std::path::Path::new("-store"));
    let super::Arguments::Check { directory } =
        super::parse(["check"].map(Into::into).into_iter()).unwrap()
    else {
        panic!("check arguments required");
    };
    assert_eq!(
        directory,
        std::path::Path::new(super::server_assets::DEFAULT_DIRECTORY)
    );
    let super::Arguments::Run(arguments) =
        super::parse(["run", "--", "command"].map(Into::into).into_iter()).unwrap()
    else {
        panic!("run arguments required");
    };
    assert_eq!(arguments.timeout, std::time::Duration::from_secs(30));
    assert!(
        arguments.directory.is_none()
            && arguments.ttl.is_none()
            && arguments.idle_ttl.is_none()
            && arguments.server_stdout.is_none()
            && arguments.server_stderr.is_none()
    );
}

#[test]
fn secondary_server_cleanup_preserves_failed_command_status() {
    for (raw_status, expected) in [
        (0, 1),
        (23 << 8, 23),
        (255 << 8, 255),
        (15, 143),
        ((19 << 8) | 0x7f, 129),
    ] {
        assert_eq!(
            command_exit_code(std::process::ExitStatus::from_raw(raw_status)),
            ExitCode::from(if raw_status == 0 { 0 } else { expected })
        );
        let failure = PocketIcStartupFailure::ServerCleanup {
            command_status: std::process::ExitStatus::from_raw(raw_status),
        };
        assert_eq!(failure_exit_code(&failure), ExitCode::from(expected));
    }
    assert_eq!(
        failure_exit_code(&PocketIcStartupFailure::InvalidConfiguration {
            message: "invalid".into()
        }),
        ExitCode::FAILURE
    );
}
