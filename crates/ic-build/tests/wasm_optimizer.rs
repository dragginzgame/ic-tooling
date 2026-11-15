//! Native optimizer qualification over fresh caller-owned output artifacts.

use std::{fs, path::PathBuf, process::Command};

#[test]
#[ignore = "requires explicitly admitted wasm-opt, Node and a retained evidence directory"]
fn optimizer_preserves_wasm_execution() {
    let evidence =
        PathBuf::from(std::env::var_os("IC_BUILD_WASM_EVIDENCE").expect("evidence root"));
    let optimizer =
        PathBuf::from(std::env::var_os("IC_BUILD_WASM_OPT").expect("admitted optimizer"));
    let node = PathBuf::from(std::env::var_os("IC_BUILD_WASM_NODE").expect("prepared Node"));
    let input = evidence.join("input.wasm");
    for flag in ["O3", "Os", "Oz"] {
        let argument = format!("-{flag}");
        let output = evidence.join(format!("{flag}.wasm"));
        let log = fs::File::create(evidence.join(format!("{flag}.log"))).expect("optimizer log");
        let status = Command::new(&optimizer)
            .arg(&input)
            .arg(&argument)
            .arg("-o")
            .arg(&output)
            .stdout(log.try_clone().expect("clone log"))
            .stderr(log)
            .status()
            .expect("run admitted optimizer");
        assert!(status.success(), "optimizer {flag} failed: {status}");
    }
    let log = fs::File::create(evidence.join("execution.log")).expect("execution log");
    let status = Command::new(node)
        .arg(evidence.join("check.cjs"))
        .args(["input.wasm", "O3.wasm", "Os.wasm", "Oz.wasm"].map(|name| evidence.join(name)))
        .stdout(log.try_clone().expect("clone execution log"))
        .stderr(log)
        .status()
        .expect("execute Wasm oracle");
    assert!(status.success(), "Wasm execution failed: {status}");
}
