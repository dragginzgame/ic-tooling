//! Real command admission, byte preservation and refusal without input mutation.
use std::{fs, os::unix::fs::symlink, process::Command};

#[test]
fn bounded_read_only_commands_and_context_identity() {
    let root = tempfile::tempdir().unwrap();
    let source = root.path().join("Cargo.toml");
    let input = "# preserve\n[workspace.package]\nversion='1.0.0'\n[workspace.dependencies]\nowned={path='member',version='^1.0.0'}\nexternal={version='1.0.0'}\nrange={package='owned',path='member',version='>=1.0, <2'}\nunselected={package='owned',path='outside',version='1.0.0'}\n";
    fs::write(&source, input).unwrap();
    fs::create_dir(root.path().join("member")).unwrap();
    fs::create_dir(root.path().join("outside")).unwrap();
    fs::write(
        root.path().join("outside/Cargo.toml"),
        "[package]\nname='owned'\nversion='1.0.0'\n",
    )
    .unwrap();
    fs::write(
        root.path().join("member/Cargo.toml"),
        "[package]\nname='owned'\nversion='1.0.0'\n",
    )
    .unwrap();
    let context = root.path().join("context.json");
    let manifest: toml::Value = toml::from_str(input).unwrap();
    let mut value = serde_json::json!({"manifest":manifest,"members":[{"name":"owned","manifest_path":root.path().join("member/Cargo.toml")}]});
    fs::write(&context, serde_json::to_vec(&value).unwrap()).unwrap();
    let invoke = |limit: &str, path: &std::path::Path| {
        Command::new(env!("CARGO_BIN_EXE_ic-cargo-metadata"))
            .args(["--max-input-bytes", limit, "manifest"])
            .arg(path)
            .args(["1.0.0", "1.0.1"])
            .arg(&context)
            .output()
            .unwrap()
    };
    let output = invoke("8192", &source);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        input
            .replacen("version='1.0.0'", "version=\"1.0.1\"", 1)
            .replace("version='^1.0.0'", "version=\"^1.0.1\"")
    );
    for limit in ["0", "1"] {
        let output = invoke(limit, &source);
        assert!(!output.status.success());
        assert!(output.stdout.is_empty());
    }
    let link = root.path().join("link");
    symlink(&source, &link).unwrap();
    let output = invoke("8192", &link);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    value["manifest"]["workspace"]["dependencies"]["owned"]["version"] = "^9.0.0".into();
    fs::write(&context, serde_json::to_vec(&value).unwrap()).unwrap();
    let output = invoke("8192", &source);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert_eq!(fs::read_to_string(&source).unwrap(), input);
    fs::write(&source, [255, 254]).unwrap();
    let output = invoke("8192", &source);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
}
