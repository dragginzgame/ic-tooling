//! Preserve the private executable's script-facing grammar and read-only effects.
use std::{ffi::OsString, fs, os::unix::ffi::OsStringExt, process::Command};

fn command() -> Command {
    Command::new(env!("CARGO_BIN_EXE_ic-cargo-qualification"))
}

#[test]
fn help_and_usage_keep_the_existing_output_and_status() {
    let usage = "usage: ic-cargo-qualification verify-external-lock ORIGINAL CANDIDATE MAX_BYTES\n";
    let help = command().arg("--help").output().unwrap();
    assert!(help.status.success());
    assert_eq!(help.stdout, usage.as_bytes());
    assert!(help.stderr.is_empty());
    for arguments in [
        vec![],
        vec!["unknown"],
        vec!["verify-external-lock", "original", "candidate"],
        vec!["verify-external-lock", "original", "candidate", "invalid"],
        vec![
            "verify-external-lock",
            "original",
            "candidate",
            "1",
            "extra",
        ],
    ] {
        let refused = command().args(arguments).output().unwrap();
        assert_eq!(refused.status.code(), Some(2));
        assert!(refused.stdout.is_empty());
        assert_eq!(refused.stderr, usage.as_bytes());
    }
}

#[test]
fn non_utf8_paths_and_zero_limits_keep_read_only_admission() {
    let directory = tempfile::tempdir().unwrap();
    let original = directory
        .path()
        .join(OsString::from_vec(b"original-\xff".to_vec()));
    let candidate = directory.path().join("candidate");
    let bytes = b"package=[]";
    fs::write(&original, bytes).unwrap();
    fs::write(&candidate, bytes).unwrap();
    for (limit, expected) in [("10", 0), ("0", 1)] {
        let result = command()
            .arg("verify-external-lock")
            .arg(&original)
            .arg(&candidate)
            .arg(limit)
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(expected));
        assert!(result.stdout.is_empty());
        assert_eq!(fs::read(&original).unwrap(), bytes);
        assert_eq!(fs::read(&candidate).unwrap(), bytes);
    }
}
