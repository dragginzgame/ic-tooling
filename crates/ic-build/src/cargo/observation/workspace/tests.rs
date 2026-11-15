use super::*;
use ic_host_process::tool::{
    CompletedExecution, ExecutionFailure, OutputLimit, OutputLimits, ToolError, capture_command,
};
use std::fs;

fn execute_workspace(
    directory: &Path,
    limits: OutputLimits,
    command: Command,
) -> io::Result<PathBuf> {
    let output = capture_command(&mut workspace_root_command(directory, command)?, limits)
        .and_then(CompletedExecution::require_success)
        .and_then(CompletedExecution::require_complete)
        .map_err(|error| {
            let kind = match error.execution_error().map(|execution| &execution.failure) {
                Some(ExecutionFailure::Io { source, .. }) => source.kind(),
                Some(ExecutionFailure::ExitStatus) => io::ErrorKind::InvalidInput,
                _ => io::ErrorKind::Other,
            };
            io::Error::new(kind, error)
        })?;
    decode_workspace_root(&output.stdout)
}

#[test]
fn discovery_refuses_oversized_complete_json_and_preserves_invocation_errors() {
    let root = crate::testing::fs::unique_temp_directory("discovery-capture");
    let limits = OutputLimits {
        stdout: OutputLimit::Terminate(64 * 1024),
        stderr: OutputLimit::Terminate(64 * 1024),
        timeout: None,
    };
    let mut response = br#"{"root":"/tmp/Cargo.toml"}"#.to_vec();
    response.resize(64 * 1024, b' ');
    fs::write(root.join("response"), &response).unwrap();
    let mut command = Command::new("/bin/sh");
    command.current_dir(&root).env("SELECTED_RECIPE", "workspace discovery").args([
            "-c",
            "test \"$SELECTED_RECIPE\" = 'workspace discovery' && test \"$RUSTUP_AUTO_INSTALL\" = 0 && cat response",
        ]);
    assert_eq!(
        execute_workspace(&root, limits, command).unwrap(),
        Path::new("/tmp")
    );
    response.push(b' ');
    fs::write(root.join("response"), &response).unwrap();
    let mut command = Command::new("/bin/sh");
    command.current_dir(&root).args(["-c", "cat response"]);
    let error = execute_workspace(&root, limits, command).unwrap_err();
    assert!(
        error
            .get_ref()
            .unwrap()
            .downcast_ref::<ToolError>()
            .is_some()
    );
    let mut failed = Command::new("/bin/sh");
    failed.args(["-c", "printf refused >&2; exit 23"]);
    let error = execute_workspace(&root, limits, failed).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    let host = error
        .get_ref()
        .unwrap()
        .downcast_ref::<ToolError>()
        .unwrap();
    let evidence = host.evidence().unwrap();
    assert_eq!(evidence.stderr, b"refused");
    assert_eq!(evidence.status.unwrap().code(), Some(23));
    let missing = Command::new(root.join("absent"));
    let error = execute_workspace(&root, limits, missing).unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
    assert!(
        error
            .get_ref()
            .unwrap()
            .downcast_ref::<ToolError>()
            .is_some()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn workspace_response_validation_rejects_unusable_roots_without_filesystem_inference() {
    for bytes in [
        b"not json".as_slice(),
        b"{}",
        br#"{"root":null}"#,
        br#"{"root":"relative/Cargo.toml"}"#,
        br#"{"root":"/tmp/NotCargo.toml"}"#,
    ] {
        assert_eq!(
            decode_workspace_root(bytes).unwrap_err().kind(),
            io::ErrorKind::InvalidData
        );
    }
    assert_eq!(
        decode_workspace_root(br#"{"root":"/not-required-to-exist/Cargo.toml"}"#).unwrap(),
        Path::new("/not-required-to-exist")
    );
}
