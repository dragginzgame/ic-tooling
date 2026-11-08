use super::*;
use serde_json::json;
use std::fs;
fn messages(manifest: &Path, package: &str, input: &Path) -> Vec<Message> {
    [
        json!({"reason":"compiler-artifact","package_id":package,"manifest_path":manifest,
            "target":{"name":"build-script-build","kind":["custom-build"],"crate_types":["bin"],"src_path":"/fixture/build.rs","edition":"2024","doc":false,"doctest":false,"test":false},
            "profile":{"opt_level":"0","debuginfo":0,"debug_assertions":true,"overflow_checks":true,"test":false},
            "features":[],"filenames":[],"executable":null,"fresh":false}),
        json!({"reason":"build-script-executed","package_id":package,"linked_libs":[],"linked_paths":[],"cfgs":[],"env":[["CANIC_CONFIG_SOURCE_PATH",input]],"out_dir":"/fixture/out"}),
    ].into_iter().map(|value| serde_json::from_value(value).unwrap()).collect()
}

#[test]
fn exact_producers_and_bounded_absolute_exports_are_required() {
    let root = crate::testing::fs::unique_temp_directory("producer-exports");
    let manifest = root.join("Cargo.toml");
    fs::write(&manifest, "").unwrap();
    let manifest = manifest.canonicalize().unwrap();
    let input = manifest.with_file_name("input");
    let key = BTreeSet::from(["CANIC_CONFIG_SOURCE_PATH".to_owned()]);
    let m = messages(&manifest, "path+file:///fixture#pkg@0.1.0", &input);
    let records = collect(&m, &key, "recipe", 2, 1).unwrap();
    assert_eq!(records.len(), 1);
    let r = &records[0];
    r.validate("recipe", "path+file:///fixture#pkg@0.1.0", &manifest, 1)
        .unwrap();
    assert!(
        r.validate("changed", "path+file:///fixture#pkg@0.1.0", &manifest, 1)
            .is_err()
    );
    assert!(collect(&m[1..], &key, "recipe", 2, 1).is_err());
    assert!(collect(&m, &key, "recipe", 1, 1).is_err());
    assert!(collect(&m, &key, "recipe", 2, 0).is_err());
    assert!(
        collect(
            &messages(&manifest, "pkg", Path::new("relative")),
            &key,
            "recipe",
            2,
            1
        )
        .is_err()
    );
    assert!(
        collect(&m, &BTreeSet::new(), "recipe", 2, 0).unwrap()[0]
            .inputs
            .is_empty()
    );
    fs::remove_dir_all(root).unwrap();
}
