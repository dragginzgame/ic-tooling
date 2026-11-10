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

#[cfg(unix)]
#[test]
fn cross_root_aliases_use_the_complete_exclusion_set_before_hashing() {
    use crate::artifacts::digest::{LabeledPathDigestCache, digest_labeled_paths_composable};
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "cross-root-exclusions")
            .unwrap();
    let first = fixture.path().join("a");
    let second = fixture.path().join("b");
    let nested = second.join("nested");
    let target = nested.join("target");
    fs::create_dir_all(&first).unwrap();
    fs::create_dir_all(&target).unwrap();
    fs::write(nested.join("Cargo.toml"), "[workspace]\n").unwrap();
    fs::write(target.join("generated"), b"first build").unwrap();
    fs::write(nested.join("source.rs"), b"source").unwrap();
    std::os::unix::fs::symlink(&target, first.join("early-alias")).unwrap();
    let roots = [first.as_path(), second.as_path()];
    let observe = || {
        let mut excluded = Vec::new();
        exclude_nested_workspace_targets(
            roots.into_iter(),
            &[],
            &mut excluded,
            &mut WorkspaceDeclarations::new(),
        )
        .unwrap();
        assert_eq!(excluded.as_slice(), std::slice::from_ref(&target));
        digest_labeled_paths_composable(
            "cross-root/1",
            [
                (Path::new("a"), first.as_path()),
                (Path::new("b"), second.as_path()),
            ],
            &excluded,
            &mut LabeledPathDigestCache::default(),
        )
        .unwrap()
    };
    let original = observe();
    fs::write(target.join("generated"), b"later build").unwrap();
    assert_eq!(observe(), original);
    fs::write(nested.join("source.rs"), b"changed source").unwrap();
    assert_ne!(observe(), original);
}
