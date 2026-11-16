use super::*;

#[test]
fn manifest_replaces_only_selected_literals() {
    let input = "# version = '1.2.3'\r\n[package]\r\nname='tool'\r\nversion = '1.2.3' # λ\r\n[workspace.dependencies]\r\nowned = { path='x', version='^1.2.3', optional=true }\r\nexternal = { version='1.2.3' }\r\n";
    let plan = Plan {
        version: Version::Package,
        dependencies: vec![Dependency {
            alias: "owned".into(),
            previous: "^1.2.3".into(),
            next: "^1.2.4".into(),
        }],
    };
    let output = manifest(input, "1.2.3", "1.2.4", &plan).unwrap();
    assert_eq!(
        output,
        input
            .replacen("version = '1.2.3' #", "version = \"1.2.4\" #", 1)
            .replacen("version='^1.2.3'", "version=\"^1.2.4\"", 1)
    );
    assert!(matches!(
        manifest(input, "1.2.2", "1.2.4", &plan),
        Err(Error::Selection(_))
    ));
    let duplicate = Plan {
        version: Version::Unchanged,
        dependencies: vec![
            Dependency {
                alias: "owned".into(),
                previous: "^1.2.3".into(),
                next: "^1.2.4".into(),
            },
            Dependency {
                alias: "owned".into(),
                previous: "^1.2.3".into(),
                next: "^1.2.4".into(),
            },
        ],
    };
    assert!(matches!(
        manifest(input, "1.2.3", "1.2.4", &duplicate),
        Err(Error::Selection(_))
    ));
    assert!(matches!(
        manifest("[broken", "1.2.3", "1.2.4", &plan),
        Err(Error::Toml(_))
    ));
}

#[test]
fn lock_preserves_registry_bytes_and_qualified_references() {
    let input = "version=4\n[[package]]\nname='owned'\nversion='1.2.3' # local\n[[package]]\nname='owned'\nversion='9.0.0'\nsource='registry+https://example.invalid'\nchecksum='opaque'\n[[package]]\nname='consumer'\nversion='0.0.0'\ndependencies=['owned 1.2.3', 'owned 9.0.0 (registry+https://example.invalid)']\n";
    let output = lock(input, "1.2.3", "1.2.4", &["owned".into()]).unwrap();
    assert_eq!(
        output,
        input
            .replace("version='1.2.3'", "version=\"1.2.4\"")
            .replace("'owned 1.2.3'", "\"owned 1.2.4\"")
    );
    for invalid in [
        input.replace("9.0.0", "1.2.3"),
        input.replace("9.0.0", "1.2.4"),
        input.replace("'owned 1.2.3'", "'owned'"),
        format!("{input}\n[[package]]\nname='owned'\nversion='1.2.3'\n"),
    ] {
        assert!(matches!(
            lock(&invalid, "1.2.3", "1.2.4", &["owned".into()]),
            Err(Error::Selection(_))
        ));
    }
    assert!(lock(input, "1.2.3", "1.2.4", &["missing".into()]).is_err());
    assert!(lock(input, "1.2.3", "1.2.4", &["owned".into(), "owned".into()]).is_err());
}

#[test]
fn no_op_manifest_and_bare_lock_edges_preserve_every_other_byte() {
    let input = "# untouched\n[package]\nversion='7.0.0'\n";
    assert_eq!(
        manifest(
            input,
            "1.0.0",
            "2.0.0",
            &Plan {
                version: Version::Unchanged,
                dependencies: vec![]
            }
        )
        .unwrap(),
        input
    );
    let lockfile = "version=3\n[[package]]\nname='one'\nversion='1.0.0'\ndependencies=['two']\n[[package]]\nname='two'\nversion='1.0.0'\n";
    assert_eq!(
        lock(lockfile, "1.0.0", "2.0.0", &["one".into(), "two".into()]).unwrap(),
        lockfile.replace("'1.0.0'", "\"2.0.0\"")
    );
    for version in ["01.0.0", "1.0", "1.0.0-alpha", "1.0.0+build"] {
        assert!(versions(version, "2.0.0").is_err());
    }
}
