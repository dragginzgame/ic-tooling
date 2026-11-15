use super::*;
use std::{fs, path::PathBuf};

fn tool(name: &str) -> PathBuf {
    let root = PathBuf::from(std::env::var_os("CANIC_SIGNER_EVIDENCE").unwrap());
    let tool = root.join(name);
    assert!(tool.is_file());
    tool
}

#[test]
fn signer_admission_and_principal_refusals_do_not_retry() {
    let bad = tool("bad-version");
    assert!(signing_principal("valid", &bad).is_err());
    assert_eq!(
        fs::read(bad.with_extension("calls")).unwrap(),
        b"--version\0\n"
    );
    for identity in [
        "valid",
        "invalid",
        "utf8",
        "nonzero",
        "stdout-over",
        "stderr-over",
        "--literal;$(false)",
    ] {
        let path = tool(if identity.starts_with("--") {
            "literal"
        } else {
            identity
        });
        let result = signing_principal(identity, &path);
        if identity == "valid" || identity.starts_with("--") {
            assert_eq!(result.unwrap(), "2vxsx-fae");
        } else {
            assert!(result.is_err(), "{identity}");
        }
        assert_eq!(
            fs::read(path.with_extension("calls")).unwrap(),
            format!("--version\0\nidentity\0principal\0--identity\0{identity}\0\n").as_bytes()
        );
    }
}
