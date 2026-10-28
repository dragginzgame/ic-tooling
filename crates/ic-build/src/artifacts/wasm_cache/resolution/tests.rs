use super::*;

#[test]
fn workspace_observations_retry_failures_and_end_with_metadata_context() {
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "workspace-observations")
            .unwrap();
    let root = fixture.path();
    let metadata: schema::Metadata = serde_json::from_value(serde_json::json!({
        "packages": [], "workspace_members": [], "workspace_default_members": [],
        "resolve": {"nodes": [], "root": null}, "workspace_root": root,
        "target_directory": root.join("target"), "metadata": null, "version": 1,
    }))
    .unwrap();
    let parsed = ParsedCargoMetadata::parse(&metadata).unwrap();
    assert!(matches!(parsed.workspace(), Err(WasmBuildError::Io { .. })));
    fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
    fs::write(root.join("Cargo.lock"), "invalid TOML").unwrap();
    assert!(parsed.workspace().is_err());
    fs::write(root.join("Cargo.lock"), "version = 4\n").unwrap();
    assert!(parsed.workspace().unwrap().locked_packages.is_empty());

    // Successful facts belong to this one discovery context. A later
    // acquisition/guard creates a new context and must observe changed bytes.
    fs::write(root.join("Cargo.toml"), "invalid TOML").unwrap();
    assert!(matches!(
        ParsedCargoMetadata::parse(&metadata).unwrap().workspace(),
        Err(WasmBuildError::InvalidCargoConfiguration { .. })
    ));
    fs::write(root.join("Cargo.toml"), "[workspace]\n").unwrap();
    fs::write(
        root.join("Cargo.lock"),
        "[[package]]\nname = 'external'\nversion = '1.0.0'\nsource = 'registry+example'\n",
    )
    .unwrap();
    assert_eq!(
        ParsedCargoMetadata::parse(&metadata)
            .unwrap()
            .workspace()
            .unwrap()
            .locked_packages
            .len(),
        1
    );
}
