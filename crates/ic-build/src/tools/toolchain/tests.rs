use super::*;
use crate::testing::fs::unique_temp_directory;
#[test]
fn inventory_retains_exact_host_target_and_library_inputs() {
    let root = unique_temp_directory("toolchain-inputs");
    fs::create_dir_all(root.join("bin")).unwrap();
    for n in ["rustc", "cargo"] {
        fs::write(root.join("bin").join(n), b"tool").unwrap();
    }
    for t in ["native", "wasm32-unknown-unknown"] {
        fs::create_dir_all(root.join("lib/rustlib").join(t)).unwrap();
    }
    fs::write(root.join("lib/compiler.so"), b"library").unwrap();
    let p = inventory(&root, "native", "wasm32-unknown-unknown", 8).unwrap();
    assert_eq!(p.len(), 5);
    assert!(p.contains(&root.join("lib/compiler.so")));
    assert!(inventory(&root, "native", "missing", 8).is_err());
    assert!(inventory(&root, "native", "native", 0).is_err());
    assert!(valid_target("../foreign").is_err());
    fs::remove_dir_all(root).unwrap();
}
