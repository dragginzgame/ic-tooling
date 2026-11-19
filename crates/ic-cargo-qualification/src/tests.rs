use super::*;
use std::{ffi::OsString, fs};

fn run(arguments: &[OsString]) -> Result<(), Refusal> {
    let parsed = Arguments::try_parse_from(
        std::iter::once(OsString::from("ic-cargo-qualification")).chain(arguments.iter().cloned()),
    )
    .map_err(|_| Refusal::Usage)?;
    super::run(&parsed)
}

fn registry(name: &str, version: &str, checksum: &str) -> String {
    format!(
        "[[package]]\nname='{name}'\nversion='{version}'\nsource='registry+example'\nchecksum='{checksum}'\n"
    )
}

#[test]
fn exact_external_subset_preserves_source_and_optional_checksum_identity() {
    let a = registry("one", "1.0.0", &"a".repeat(64));
    let b = registry("unused", "2.0.0", &"b".repeat(64));
    let git = "[[package]]\nname='git'\nversion='1'\nsource='git+example#exact'\n";
    let original = format!("{a}{b}{git}");
    assert!(
        verify(
            original.as_bytes(),
            format!("{a}{git}[[package]]\nlocal=true\n").as_bytes()
        )
        .is_ok()
    );
    assert!(verify(original.as_bytes(), b"package=[]").is_ok());
    for changed in [
        a.replace("one", "other"),
        a.replace("1.0.0", "1.0.1"),
        a.replace("registry+example", "registry+elsewhere"),
        a.replace(&"a".repeat(64), &"c".repeat(64)),
        format!("{git}checksum='new'\n"),
    ] {
        assert!(matches!(
            verify(original.as_bytes(), changed.as_bytes()),
            Err(Refusal::Changed)
        ));
    }
}

#[test]
fn malformed_and_duplicate_rows_refuse_in_either_role() {
    let valid = registry("one", "1.0.0", &"a".repeat(64));
    for (invalid, expected) in [
        ("".to_owned(), "missing package array"),
        ("package=[false]".into(), "package must be a table"),
        ("package=[{source=7}]".into(), "source must be a string"),
        (format!("{valid}{valid}"), "duplicate external identity"),
        (
            valid.replace(&"a".repeat(64), &"A".repeat(64)),
            "registry checksum must be lowercase SHA-256",
        ),
        (
            valid.replace("checksum=", "other="),
            "registry checksum must be lowercase SHA-256",
        ),
        (
            "[[package]]\nsource='git+secret'\nversion='1'".into(),
            "missing package name",
        ),
    ] {
        assert!(matches!(
            verify(valid.as_bytes(), invalid.as_bytes()),
            Err(Refusal::Lock { role: "candidate", reason }) if reason == expected
        ));
        assert!(matches!(
            verify(invalid.as_bytes(), valid.as_bytes()),
            Err(Refusal::Lock { role: "original", reason }) if reason == expected
        ));
    }
    assert!(matches!(
        verify(b"package=[]", &[255]),
        Err(Refusal::Lock {
            role: "candidate",
            reason: "invalid UTF-8"
        })
    ));
    let error = verify(b"package=[]", b"token='SECRET").unwrap_err();
    assert!(matches!(
        error,
        Refusal::Lock {
            role: "candidate",
            reason: "invalid TOML"
        }
    ));
    assert!(!format!("{error:?}").contains("SECRET"));
}

#[test]
fn file_admission_bounds_and_refusals_never_change_inputs() {
    use std::os::unix::fs::symlink;
    let scratch = tempfile::tempdir().unwrap();
    let original = scratch.path().join("original");
    let candidate = scratch.path().join("candidate");
    fs::write(&original, b"package=[]").unwrap();
    fs::write(&candidate, b"package=[]").unwrap();
    let mut args = vec![
        "verify-external-lock".into(),
        original.clone().into_os_string(),
        candidate.clone().into_os_string(),
        "10".into(),
    ];
    run(&args).unwrap();
    args[3] = "9".into();
    assert!(matches!(run(&args), Err(Refusal::Bytes(_))));
    args[3] = "10".into();
    fs::remove_file(&candidate).unwrap();
    symlink(&original, &candidate).unwrap();
    assert!(matches!(run(&args), Err(Refusal::File(_))));
    assert_eq!(fs::read(&original).unwrap(), b"package=[]");
    fs::remove_file(&candidate).unwrap();
    assert!(
        matches!(run(&args), Err(Refusal::Io(error)) if error.kind() == io::ErrorKind::NotFound)
    );
    assert_eq!(fs::read(&original).unwrap(), b"package=[]");
    assert!(!candidate.exists());
    assert!(matches!(run(&args[..3]), Err(Refusal::Usage)));
    args[3] = "invalid".into();
    assert!(matches!(run(&args), Err(Refusal::Usage)));
}
