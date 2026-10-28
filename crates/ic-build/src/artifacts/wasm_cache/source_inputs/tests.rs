use super::*;

#[test]
fn reused_declarations_keep_each_entries_roots_sources_and_exclusions() {
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "workspace-declarations")
            .unwrap();
    let root = fixture.path();
    let nested = root.join("nested");
    let target = nested.join("target");
    let source = target.join("input.rs");
    fs::create_dir_all(&target).unwrap();
    fs::write(nested.join("Cargo.toml"), "[workspace]\n").unwrap();
    fs::write(&source, "pub fn source() {}\n").unwrap();
    let mut declarations = WorkspaceDeclarations::new();
    let discover =
        |roots: &[PathBuf], sources: &[PathBuf], declarations: &mut WorkspaceDeclarations| {
            let mut exclusions = Vec::new();
            exclude_nested_workspace_targets(
                roots.iter().map(PathBuf::as_path),
                sources,
                &mut exclusions,
                declarations,
            )
            .unwrap();
            exclusions
        };
    let roots = [root.to_owned()];
    assert_eq!(
        discover(&roots, &[], &mut declarations).as_slice(),
        std::slice::from_ref(&target)
    );
    assert_eq!(
        discover(&roots, &[source], &mut declarations),
        [] as [PathBuf; 0]
    );
    assert_eq!(
        discover(&[root.to_owned(), target], &[], &mut declarations),
        [] as [PathBuf; 0]
    );
    fs::write(nested.join("Cargo.toml"), "[package]\nname = 'ordinary'\n").unwrap();
    assert_eq!(
        discover(&roots, &[], &mut WorkspaceDeclarations::new()),
        [] as [PathBuf; 0]
    );
}
