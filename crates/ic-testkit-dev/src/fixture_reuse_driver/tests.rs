use super::arguments::parse_arguments;
use super::preparation::{WASM_LIMIT, git_provenance, server_program};
use super::{Error, REPORT_LIMIT, execute_command, measure, prepare, publish_report, text_command};
use ic_host_fs::read::hash_file;
use ic_host_process::tool::{ExecutionFailure, OutputStream, ToolError};

#[cfg(unix)]
use super::install_interrupt_handler;
use serde_json::{Value, json};
use std::{
    ffi::OsString,
    fs,
    io::{self, Read as _, Write as _},
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(unix)]
use std::os::unix::fs::{PermissionsExt as _, symlink};

fn snapshot()
-> Result<ic_host_process::sampling::Snapshot, ic_host_process::sampling::SamplingError> {
    ic_host_process::sampling::snapshot(64 * 1024, super::arguments::RSS_COMMAND_LIMITS)
}

fn fixture_directory() -> ic_host_fs::testing::FixtureDirectory {
    ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "testkit-fixture-driver")
        .unwrap()
}

#[test]
fn error_projection_preserves_direct_typed_sources() {
    use std::error::Error as _;

    for error in [
        Error::from(io::Error::from(io::ErrorKind::PermissionDenied)),
        Error::CargoObservation(Box::new(io::Error::from(io::ErrorKind::PermissionDenied))),
    ] {
        assert_eq!(error.to_string(), error.source().unwrap().to_string());
        assert_eq!(
            error
                .source()
                .unwrap()
                .downcast_ref::<io::Error>()
                .unwrap()
                .kind(),
            io::ErrorKind::PermissionDenied
        );
    }
    let error = Error::from(serde_json::from_str::<Value>("invalid json").unwrap_err());
    assert_eq!(error.to_string(), error.source().unwrap().to_string());
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<serde_json::Error>()
            .is_some()
    );
    assert!(Error::Interrupted.source().is_none());
    assert!(Error::InvalidArgument("argument".into()).source().is_none());
    assert!(Error::InvalidData("report").source().is_none());
    assert_eq!(Error::Interrupted.to_string(), "benchmark interrupted");
    assert_eq!(
        Error::InvalidArgument("argument".into()).to_string(),
        "invalid benchmark argument: argument"
    );
    assert_eq!(
        Error::InvalidData("report").to_string(),
        "invalid benchmark data: report"
    );
}

#[cfg(unix)]
fn init_git_fixture(root: &Path) {
    for args in [
        vec!["init", "-q"],
        vec![
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "fixture",
        ],
    ] {
        assert!(
            Command::new("git")
                .args(args)
                .current_dir(root)
                .status()
                .unwrap()
                .success()
        );
    }
}

#[test]
#[cfg(unix)]
fn git_observations_preserve_dirty_path_bytes_and_typed_failures() {
    use ic_host_process::{
        provenance::GitQuery,
        tool::{OutputLimit, OutputLimits},
    };
    use std::{error::Error as _, os::unix::ffi::OsStringExt as _};

    let scratch = fixture_directory();
    let root = scratch.path().canonicalize().unwrap();
    let limits = OutputLimits {
        stdout: OutputLimit::Terminate(64 * 1024),
        stderr: OutputLimit::Terminate(64 * 1024),
        timeout: None,
    };
    init_git_fixture(&root);
    let clean = git_provenance(&root, limits).unwrap();
    assert!(!clean.is_dirty());
    // macOS filesystems reject arbitrary invalid UTF-8 names before Git runs.
    // Every host still checks raw control/quote bytes; Linux also exercises
    // an actual invalid UTF-8 directory entry.
    for name in std::iter::once(b"untracked-\n\t\"".as_slice())
        .chain(cfg!(target_os = "linux").then_some(b"untracked-\xff".as_slice()))
    {
        let path = root.join(OsString::from_vec(name.to_vec()));
        fs::write(&path, b"fixture").unwrap();
        let dirty = git_provenance(&root, limits).unwrap();
        assert_eq!(dirty.revision(), clean.revision());
        assert!(dirty.is_dirty());
        assert_eq!(
            dirty.status_evidence().stdout,
            [b"?? ".as_slice(), name, b"\0"].concat()
        );
        assert_eq!(
            dirty.options.untracked,
            ic_host_process::provenance::UntrackedFiles::Normal
        );
        assert_eq!(
            dirty.options.ignore_submodules,
            ic_host_process::provenance::IgnoreSubmodules::None
        );
        fs::remove_file(path).unwrap();
    }

    let missing = fixture_directory();
    let error = git_provenance(&missing.path().canonicalize().unwrap(), limits).unwrap_err();
    assert!(
        error
            .source()
            .unwrap()
            .downcast_ref::<ic_host_process::provenance::GitError>()
            .is_some()
    );
    assert!(matches!(error, Error::Git(error) if error.query == GitQuery::Revision));
    let error = git_provenance(
        &root,
        OutputLimits {
            stdout: OutputLimit::Terminate(1),
            ..limits
        },
    )
    .unwrap_err();
    assert!(matches!(error, Error::Tool(_)));
}

#[cfg(unix)]
fn assert_prepared_tool_context(root: &Path, missing: bool) {
    let server = root.join("server");
    let options = parse_arguments([OsString::from("--server"), server.into_os_string()]).unwrap();
    if missing {
        assert!(matches!(
            prepare(&options), Err(Error::Tool(error)) if matches!(
                error.execution_error().map(|execution| &execution.failure),
                Some(ExecutionFailure::Io { source, .. }) if source.kind() == io::ErrorKind::NotFound
            )
        ));
    } else if std::env::var_os("IC_TESTKIT_BENCHMARK_TOOL_FAIL_BUILD").is_some() {
        let Err(Error::Build(error)) = prepare(&options) else {
            panic!("failed producer must preserve its typed cause");
        };
        assert!(matches!(
            error,
            ic_build::artifacts::WasmBuildError::CommandFailed {
                phase: ic_build::artifacts::WasmBuildPhase::CargoBuild,
                status,
                ref stdout,
                ref stderr,
            } if status.code() == Some(23)
                && stdout.contains("producer output")
                && stderr.contains("producer failure")
        ));
    } else {
        let inputs = prepare(&options).unwrap();
        assert_eq!(inputs.root, root.join("workspace"));
        assert_eq!(
            inputs.worker,
            root.join("target/cargo-reported directory/worker fixture")
        );
        let trace = fs::read_to_string(root.join("trace")).unwrap();
        let calls = trace.lines().collect::<Vec<_>>();
        assert_ne!(calls.len(), 0);
        assert_eq!(calls.len() % 3, 0);
        let records = calls.as_chunks::<3>().0;
        for (index, call) in records.iter().enumerate() {
            if call[2].contains("locate-project") {
                assert_eq!(call[1], root.to_str().unwrap());
                assert!(call[2].contains("--offline"));
                assert!(call[2].contains("--workspace"));
                continue;
            }
            assert_eq!(call[1], inputs.root.to_str().unwrap());
            if call[0].ends_with("/cargo") || call[0] == "cargo" {
                if !call[2].starts_with("--version") {
                    assert!(call[2].contains("--locked"));
                    assert!(call[2].contains("--offline"));
                }
            } else {
                assert!(call[0].ends_with("/rustc") || call[0] == "rustc");
                // The fixture Cargo invokes explicit RUSTC itself.
                assert!(
                    call[2] == "-vV"
                        || call[2] == "--version"
                            && index > 0
                            && records[index - 1][0].ends_with("cargo")
                );
            }
        }

        let target = root.join("target");
        let mutable_paths = [
                target.join("wasm32-unknown-unknown/debug/ic_testkit_perf_probe.wasm"),
                target.join("ic-testkit-incremental/wasm32-unknown-unknown/debug/ic_testkit_perf_probe.wasm"),
                target.join("ic-testkit-fixture-benchmark-wasm/wasm32-unknown-unknown/debug/ic_testkit_perf_probe.wasm"),
            ];
        for path in &mutable_paths {
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            if path.exists() {
                fs::remove_file(path).unwrap();
            }
            fs::write(path, b"replaced public Cargo output").unwrap();
        }
        let retained = &inputs.wasm.record().artifacts()[0];
        assert_eq!(
            inputs.provenance.wasm_sha256,
            hash_file(retained, WASM_LIMIT).unwrap().sha256.to_string()
        );
        for _ in 0..2 {
            let worker = Command::new("cat").arg(retained).output().unwrap();
            assert!(worker.status.success());
            assert_eq!(worker.stdout, b"\0asm\x01\0\0\0");
            for path in &mutable_paths {
                let _ = fs::remove_file(path);
            }
        }
        publish_report(&root.join("report.json"), |file| {
            Ok(serde_json::to_writer(file, &inputs.provenance)?)
        })
        .unwrap();
        let encoded: Value =
            serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
        assert_eq!(encoded["rustc"], "fixture rustc");
        assert_eq!(encoded["server"], root.join("server").to_str().unwrap());
        assert_eq!(encoded["iterations"], options.iterations.get());
        assert_eq!(encoded["workers"], options.workers.get());
    }
}

#[cfg(unix)]
#[test]
fn explicit_server_paths_preserve_native_failures_without_path_search() {
    let cwd = std::env::current_dir().unwrap();
    let fixture = ic_host_fs::testing::FixtureDirectory::new(&cwd, "server-paths").unwrap();
    let executable = fixture.path().join("server");
    fs::write(&executable, b"#!/bin/sh\nexit 0\n").unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    let expected = executable.canonicalize().unwrap();
    for path in [executable.as_path(), executable.strip_prefix(&cwd).unwrap()] {
        assert_eq!(server_program(path, fixture.path()).unwrap(), expected);
    }
    for path in [PathBuf::new(), fixture.path().join("missing")] {
        assert!(
            matches!(server_program(&path, fixture.path()), Err(Error::Io(error)) if error.kind() == io::ErrorKind::NotFound)
        );
    }
    assert!(matches!(
        server_program(fixture.path(), fixture.path()),
        Err(Error::Resolution(super::ResolutionError::NotRegularFile))
    ));
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(matches!(
        server_program(&executable, fixture.path()),
        Err(Error::Resolution(super::ResolutionError::NotExecutable))
    ));
}

#[cfg(unix)]
fn prepare_cargo_fixture(root: &Path, target: &Path) {
    let workspace = root.join("workspace");
    fs::create_dir(&workspace).unwrap();
    fs::write(workspace.join("Cargo.toml"), "[workspace]\nmembers = ['crates/ic-testkit-dev', 'crates/ic-testkit-perf-probe']\nresolver = '3'\n").unwrap();
    for (name, target) in [
        (
            "ic-testkit-dev",
            "[[bin]]\nname = 'fixture_reuse_benchmark'\npath = 'src/lib.rs'\n",
        ),
        ("ic-testkit-perf-probe", "[lib]\ncrate-type = ['cdylib']\n"),
    ] {
        let directory = workspace.join("crates").join(name);
        fs::create_dir_all(directory.join("src")).unwrap();
        fs::write(
            directory.join("Cargo.toml"),
            format!("[package]\nname = '{name}'\nversion = '0.0.0'\nedition = '2024'\n{target}"),
        )
        .unwrap();
        fs::write(directory.join("src/lib.rs"), "").unwrap();
    }
    init_git_fixture(&workspace);
    let metadata = Command::new(env!("CARGO"))
        .current_dir(&workspace)
        .env("RUSTUP_AUTO_INSTALL", "0")
        .args(["metadata", "--offline", "--format-version", "1"])
        .output()
        .unwrap();
    assert!(
        metadata.status.success(),
        "{}",
        String::from_utf8_lossy(&metadata.stderr)
    );
    let mut catalog: Value = serde_json::from_slice(&metadata.stdout).unwrap();
    let probe = catalog["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["name"] == "ic-testkit-perf-probe")
        .unwrap();
    let artifact = json!({
        "reason": "compiler-artifact",
        "package_id": probe["id"],
        "manifest_path": probe["manifest_path"],
        "target": probe["targets"][0],
        "profile": {
            "opt_level": "0", "debuginfo": 2, "debug_assertions": true,
            "overflow_checks": true, "test": false
        },
        "features": [],
        "filenames": [target.join("ic-testkit-incremental/wasm32-unknown-unknown/debug/ic_testkit_perf_probe.wasm")],
        "executable": null,
        "fresh": false
    });
    let package = catalog["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["name"] == "ic-testkit-dev")
        .unwrap();
    let native_target = package["targets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|target| target["name"] == "fixture_reuse_benchmark")
        .unwrap();
    let mut native_artifact = artifact.clone();
    native_artifact["package_id"] = package["id"].clone();
    native_artifact["manifest_path"] = package["manifest_path"].clone();
    native_artifact["target"] = native_target.clone();
    let native_executable = target.join("cargo-reported directory/worker fixture");
    native_artifact["filenames"] = json!([native_executable]);
    native_artifact["executable"] = json!(native_executable);
    fs::write(
        root.join("native-artifact-message.json"),
        format!("{native_artifact}\n{{\"reason\":\"build-finished\",\"success\":true}}\n"),
    )
    .unwrap();
    fs::write(root.join("artifact-message.json"), format!("{artifact}\n")).unwrap();
    fs::write(root.join("full-metadata.json"), &metadata.stdout).unwrap();
    catalog["target_directory"] = json!(target);
    catalog["resolve"] = Value::Null;
    fs::write(
        root.join("metadata.json"),
        serde_json::to_vec(&catalog).unwrap(),
    )
    .unwrap();
    fs::write(root.join("probe.wasm"), b"\0asm\x01\0\0\0").unwrap();
    fs::write(
        root.join("workspace.json"),
        serde_json::to_vec(&json!({"root": workspace.join("Cargo.toml")})).unwrap(),
    )
    .unwrap();
}

#[test]
#[cfg(unix)]
fn standalone_tools_and_retained_wasm_preserve_context_and_bytes() {
    const ROOT_ENV: &str = "IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT";
    const MISSING_ENV: &str = "IC_TESTKIT_BENCHMARK_TOOL_EXPECT_MISSING";
    if let Some(root) = std::env::var_os(ROOT_ENV) {
        assert_prepared_tool_context(
            &PathBuf::from(root),
            std::env::var_os(MISSING_ENV).is_some(),
        );
        return;
    }

    let scratch = fixture_directory();
    let root = scratch.path().canonicalize().unwrap();
    let bin = root.join("bin");
    fs::create_dir(&bin).unwrap();
    let target = root.join("target");
    let wasm = target.join("wasm32-unknown-unknown/debug/ic_testkit_perf_probe.wasm");
    fs::create_dir_all(wasm.parent().unwrap()).unwrap();
    fs::write(wasm, b"\0asm\x01\0\0\0").unwrap();
    prepare_cargo_fixture(&root, &target);
    let proxy = root.join("multicall-proxy");
    fs::write(
        &proxy,
        r#"#!/bin/sh
set -eu
printf '%s\n%s\n%s\n' "$0" "$PWD" "$*" >> "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/trace"
case "${0##*/}" in
  cargo)
    if [ "$1" = --offline ]; then shift; fi
    if [ "$1" = locate-project ]; then
      cat "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/workspace.json"
      exit 0
    fi
    if [ -n "${RUSTC-}" ]; then
      [ "$RUSTC" = "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/bin/rustc" ] || exit 98
      "$RUSTC" --version >/dev/null
    fi
    case "$1" in
      --version) printf 'cargo fixture %s\n' "${IC_TESTKIT_BENCHMARK_TOOL_FAIL_BUILD-}" ;;
      metadata)
        case " $* " in
          *" --no-deps "*) cat "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/metadata.json" ;;
          *) cat "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/full-metadata.json" ;;
        esac ;;
      build)
        if [ -n "${CARGO_TARGET_DIR-}" ]; then
          if [ -n "${IC_TESTKIT_BENCHMARK_TOOL_FAIL_BUILD-}" ]; then
            printf 'producer output\n'
            printf 'producer failure\n' >&2
            exit 23
          fi
          output="$CARGO_TARGET_DIR/wasm32-unknown-unknown/debug/ic_testkit_perf_probe.wasm"
          mkdir -p "${output%/*}"
          cp "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/probe.wasm" "$output"
          cat "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/artifact-message.json"
        else
          cat "$IC_TESTKIT_BENCHMARK_TOOL_TEST_ROOT/native-artifact-message.json"
        fi ;;
      *) exit 96 ;;
    esac ;;
  rustc) printf 'fixture rustc\n' ;;
  *) exit 97 ;;
esac
"#,
    )
    .unwrap();
    fs::set_permissions(&proxy, fs::Permissions::from_mode(0o700)).unwrap();
    symlink(&proxy, bin.join("cargo")).unwrap();
    symlink(&proxy, bin.join("rustc")).unwrap();
    let server = root.join("server");
    fs::write(
        &server,
        format!(
            "#!/bin/sh\nprintf 'pocket-ic-server {}\\n'\n",
            ic_testkit::pic::POCKET_IC_SERVER_VERSION
        ),
    )
    .unwrap();
    fs::set_permissions(server, fs::Permissions::from_mode(0o700)).unwrap();
    let path = std::env::join_paths(
        std::iter::once(bin.clone())
            .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    for (cargo, rustc, missing, failed_build) in [
        (None, None, false, false),
        (
            Some(bin.join("cargo").into_os_string()),
            Some(bin.join("rustc").into_os_string()),
            false,
            false,
        ),
        (
            Some(OsString::from("bin/cargo")),
            Some(OsString::from("bin/rustc")),
            false,
            false,
        ),
        (
            Some(root.join("missing-cargo").into_os_string()),
            None,
            true,
            false,
        ),
        (None, None, false, true),
    ] {
        let mut child = Command::new(std::env::current_exe().unwrap());
        child.args(["--exact", "fixture_reuse_driver::tests::standalone_tools_and_retained_wasm_preserve_context_and_bytes", "--test-threads=1"])
                .current_dir(&root)
                .env(ROOT_ENV, &root)
                .env("PATH", &path)
                .env_remove("CARGO")
                .env_remove("RUSTC")
                .env_remove(MISSING_ENV)
                .env_remove("IC_TESTKIT_BENCHMARK_TOOL_FAIL_BUILD");
        if let Some(cargo) = cargo {
            child.env("CARGO", cargo);
        }
        if let Some(rustc) = rustc {
            child.env("RUSTC", rustc);
        }
        if missing {
            child.env(MISSING_ENV, "1");
        }
        if failed_build {
            child.env("IC_TESTKIT_BENCHMARK_TOOL_FAIL_BUILD", "1");
        }
        let output = child.output().unwrap();
        assert!(
            output.status.success(),
            "tool invocation failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        if !missing {
            fs::remove_file(root.join("trace")).unwrap();
        }
    }
}

#[test]
fn report_write_failure_preserves_existing_report_and_cleans_scratch() {
    let scratch = fixture_directory();
    let report = scratch.path().join("report.json");
    fs::write(&report, b"previous complete report").unwrap();
    let result = publish_report(&report, |file| {
        file.write_all(b"partial replacement")?;
        assert_eq!(fs::read(&report).unwrap(), b"previous complete report");
        Err(io::Error::from(io::ErrorKind::StorageFull).into())
    });
    assert!(matches!(result, Err(Error::Io(error)) if error.kind() == io::ErrorKind::StorageFull));
    assert_eq!(fs::read(&report).unwrap(), b"previous complete report");
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 1);
}

#[test]
fn report_rename_failure_preserves_destination_and_cleans_scratch() {
    let scratch = fixture_directory();
    let report = scratch.path().join("report.json");
    fs::create_dir(&report).unwrap();
    let retained = report.join("retained report.json");
    fs::write(&retained, b"caller-owned evidence").unwrap();
    assert!(matches!(
        publish_report(&report, |file| Ok(file.write_all(b"replacement")?)),
        Err(Error::Io(_))
    ));
    assert_eq!(fs::read(retained).unwrap(), b"caller-owned evidence");
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 1);
    assert!(matches!(
        publish_report(&scratch.path().join("missing/report.json"), |_| {
            panic!("a missing parent must fail before writing")
        }),
        Err(Error::Io(error)) if error.kind() == io::ErrorKind::NotFound
    ));
    let ordinary = scratch.path().join("ordinary.json");
    fs::write(&ordinary, b"previous").unwrap();
    for suffix in ["ordinary.json/", "ordinary.json/."] {
        assert!(matches!(
            publish_report(&scratch.path().join(suffix), |_| {
                panic!("a directory-required target must not be normalized")
            }),
            Err(Error::Io(error)) if error.kind() == io::ErrorKind::InvalidInput
        ));
        assert_eq!(fs::read(&ordinary).unwrap(), b"previous");
    }
}

#[test]
fn report_replacement_keeps_old_readers_and_publishes_complete_json() {
    let scratch = fixture_directory();
    let report = scratch.path().join("report with spaces.json");
    fs::write(&report, b"previous complete report").unwrap();
    let mut old_reader = fs::File::open(&report).unwrap();
    let retained = scratch.path().join("retained.json");
    fs::hard_link(&report, &retained).unwrap();
    let document = json!({"format": "ic-testkit-fixture-benchmark-v1", "complete": true});
    publish_report(&report, |file| {
        serde_json::to_writer_pretty(&mut *file, &document)?;
        Ok(file.write_all(b"\n")?)
    })
    .unwrap();
    let mut previous = String::new();
    old_reader.read_to_string(&mut previous).unwrap();
    assert_eq!(previous, "previous complete report");
    assert_eq!(fs::read(retained).unwrap(), previous.as_bytes());
    let published = fs::read(&report).unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&published).unwrap(),
        document
    );
    assert!(published.ends_with(b"\n"));
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 2);
    #[cfg(unix)]
    assert_eq!(
        fs::metadata(&report).unwrap().permissions().mode() & 0o777,
        0o600
    );
}

#[test]
#[cfg(unix)]
fn report_replacement_preserves_symlink_target() {
    let scratch = fixture_directory();
    let target = scratch.path().join("original.json");
    let report = scratch.path().join("linked.json");
    fs::write(&target, b"caller-owned target").unwrap();
    symlink(&target, &report).unwrap();
    publish_report(&report, |file| Ok(file.write_all(b"complete replacement")?)).unwrap();
    assert!(
        !fs::symlink_metadata(&report)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    assert_eq!(fs::read(report).unwrap(), b"complete replacement");
    assert_eq!(fs::read(target).unwrap(), b"caller-owned target");

    let parent_alias = scratch.path().join("parent-alias");
    symlink(scratch.path(), &parent_alias).unwrap();
    publish_report(&parent_alias.join("linked.json"), |file| {
        Ok(file.write_all(b"replacement through parent alias")?)
    })
    .unwrap();
    assert_eq!(
        fs::read(scratch.path().join("linked.json")).unwrap(),
        b"replacement through parent alias"
    );
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn sampling_failure_or_panic_waits_for_worker_completion() {
    for panic_in_sample in [false, true] {
        let scratch = fixture_directory();
        let finished = scratch.path().join("finished");
        let ready = scratch.path().join("ready");
        let mut command = Command::new("/bin/sh");
        command.args([
                "-c",
                "trap 'printf done > \"$1\"; exit 0' TERM; printf ready > \"$2\"; while :; do sleep 0.01; done",
                "sampling-failure-worker",
            ]).arg(&finished).arg(&ready);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            measure(&mut command, || {
                let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
                while !ready.is_file() {
                    if std::time::Instant::now() >= deadline {
                        return Err(ic_host_process::sampling::SamplingError::InvalidData(
                            "worker readiness timeout",
                        ));
                    }
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
                if panic_in_sample {
                    std::panic::panic_any("sampling callback panicked");
                }
                ic_host_process::sampling::snapshot(
                    0,
                    ic_host_process::tool::OutputLimits {
                        stdout: ic_host_process::tool::OutputLimit::Terminate(0),
                        ..super::arguments::RSS_COMMAND_LIMITS
                    },
                )
            })
        }));
        assert_eq!(fs::read(finished).unwrap(), b"done");
        if panic_in_sample {
            assert_eq!(
                *result.err().unwrap().downcast_ref::<&str>().unwrap(),
                "sampling callback panicked"
            );
            continue;
        }
        let result = result.unwrap();
        let Err(Error::Sampling(error)) = result else {
            panic!("sampling must fail with its original cause");
        };
        assert!(std::error::Error::source(&error).is_some());
        #[cfg(target_os = "linux")]
        assert!(matches!(
            error,
            ic_host_process::sampling::SamplingError::Io(_)
        ));
        #[cfg(target_os = "macos")]
        assert!(matches!(
            error,
            ic_host_process::sampling::SamplingError::Command(_)
        ));
    }
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn native_measurement_keeps_large_reports_and_rejects_failed_workers() {
    let scratch = fixture_directory();
    let input = scratch.path().join("input.json");
    fs::write(
        &input,
        serde_json::to_vec(&crate::fixture_reuse_report::worker_report_fixture(1024)).unwrap(),
    )
    .unwrap();
    let captured = measure(
        Command::new("/bin/sh")
            .args(["-c", "sleep 0.1; cat \"$1\"", "test-worker"])
            .arg(&input),
        snapshot,
    )
    .unwrap();
    assert_eq!(captured.report.samples.len(), 1024);
    assert!(captured.rss.rss_samples > 0);
    assert!(captured.rss.sampled_peak_tree_rss_bytes > 0);
    assert!(
        matches!(measure(Command::new("/bin/sh").args(["-c", "exit 23"]), snapshot), Err(Error::CommandFailed { status, .. }) if status.code() == Some(23))
    );
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn worker_report_limits_and_json_errors_preserve_caller_files() {
    let scratch = fixture_directory();
    let input = scratch.path().join("oversized.json");
    fs::File::create(&input)
        .unwrap()
        .set_len(REPORT_LIMIT + 1)
        .unwrap();
    let result = measure(
        Command::new("/bin/sh")
            .args(["-c", "cat \"$1\"", "large-worker"])
            .arg(&input),
        snapshot,
    );
    assert!(matches!(
        result,
        Err(Error::Artifact(
            ic_host_artifacts::artifact::ArtifactError::LimitExceeded { .. }
        ))
    ));
    assert_eq!(fs::metadata(input).unwrap().len(), REPORT_LIMIT + 1);
    assert!(matches!(
        measure(
            Command::new("/bin/sh").args(["-c", "printf 'not json'"]),
            snapshot
        ),
        Err(Error::Json(_))
    ));
}

#[test]
#[cfg(unix)]
fn preparation_commands_preserve_text_status_and_capture_failures() {
    assert_eq!(
        text_command(Command::new("/bin/sh").args(["-c", "printf '  ready\\n' "])).unwrap(),
        "ready"
    );
    assert!(matches!(
        text_command(Command::new("/bin/sh").args(["-c", "printf '\\377'"])),
        Err(Error::InvalidData("tool output is not UTF-8"))
    ));
    // Keep the producer alive until capture kills it: an immediately exiting
    // producer races group signalling on macOS and can retain a cleanup error.
    let Err(Error::Tool(error)) = text_command(
        Command::new("/bin/sh").args(["-c", "printf '%065537d' 0; exec /bin/sleep 30"]),
    ) else {
        panic!("expected bounded-output failure")
    };
    let ToolError::Execution(execution) = &error else {
        panic!("expected execution failure")
    };
    assert!(matches!(
        execution.failure,
        ExecutionFailure::OutputLimit {
            stream: OutputStream::Stdout
        }
    ));
    assert_eq!(execution.evidence.stdout.len(), 64 * 1024);
    assert!(execution.evidence.stdout_truncated);
    assert!(
        execution.cleanup.is_none(),
        "cleanup evidence: {execution:?}"
    );
    assert_eq!(
        execution
            .evidence
            .status
            .as_ref()
            .and_then(std::os::unix::process::ExitStatusExt::signal),
        Some(9)
    );
    execute_command(Command::new("/bin/sh").args(["-c", "sleep 0.05"])).unwrap();
    let Err(Error::Tool(error)) = execute_command(Command::new("/bin/sh").args(["-c", "exit 23"]))
    else {
        panic!("expected exit failure")
    };
    assert!(
        matches!(&error, ToolError::Execution(execution) if matches!(execution.failure, ExecutionFailure::ExitStatus))
    );
    assert_eq!(error.evidence().unwrap().status.unwrap().code(), Some(23));
    let Err(Error::Tool(ToolError::Execution(error))) =
        text_command(&mut Command::new("/missing/testkit-command"))
    else {
        panic!("expected retained spawn failure");
    };
    assert!(matches!(
        &error.failure,
        ExecutionFailure::Io {
            operation: ic_host_process::tool::ExecutionOperation::Spawn,
            source,
        } if source.kind() == io::ErrorKind::NotFound
    ));
    assert!(error.cleanup.is_none());
}

#[test]
#[cfg(unix)]
#[expect(
    unsafe_code,
    reason = "bounded native operation with the local SAFETY contract below"
)]
fn interruption_waits_for_worker_completion() {
    const CHILD_ENV: &str = "IC_TESTKIT_FIXTURE_BENCHMARK_INTERRUPT_CHILD";
    if std::env::var_os(CHILD_ENV).is_none() {
        // Keep signal-handler installation out of parallel test threads.
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "fixture_reuse_driver::tests::interruption_waits_for_worker_completion",
                "--test-threads=1",
            ])
            .env(CHILD_ENV, "1")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "interrupt worker failed: {}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        return;
    }
    let _handler = install_interrupt_handler().unwrap();
    let scratch = fixture_directory();
    let finished = scratch.path().join("finished");
    let result = measure(
            Command::new("/bin/sh")
                .args([
                    "-c",
                    "trap 'printf done > \"$2\"; exit 0' TERM; kill -INT \"$1\"; while :; do sleep 0.01; done",
                    "interrupted-worker",
                ])
                .arg(std::process::id().to_string())
                .arg(&finished),
            snapshot,
        );
    assert!(matches!(result, Err(Error::Interrupted)));
    assert_eq!(fs::read(finished).unwrap(), b"done");

    for capture_text in [true, false] {
        let _command_handler = install_interrupt_handler().unwrap();
        let pid_path = scratch.path().join("preparation.pid");
        let mut command = Command::new("/bin/sh");
        command
            .args([
                "-c",
                "echo $$ > \"$2\"; kill -INT \"$1\"; sleep 30",
                "interrupted-preparation",
            ])
            .arg(std::process::id().to_string())
            .arg(&pid_path);
        let result = if capture_text {
            text_command(&mut command).map(|_| ())
        } else {
            execute_command(&mut command).map(|_| ())
        };
        let Err(Error::Tool(error)) = result else {
            panic!("expected command cancellation")
        };
        let ToolError::Execution(execution) = &error else {
            panic!("expected execution failure")
        };
        assert!(matches!(execution.failure, ExecutionFailure::Cancelled));
        assert!(execution.cleanup.is_none());
        let pid: libc::pid_t = fs::read_to_string(&pid_path)
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        // SAFETY: signal zero only probes whether this reaped child exists.
        assert_eq!(unsafe { libc::kill(pid, 0) }, -1);
        assert_eq!(io::Error::last_os_error().raw_os_error(), Some(libc::ESRCH));
    }

    let _publication_handler = install_interrupt_handler().unwrap();
    let report = scratch.path().join("report.json");
    fs::write(&report, b"previous complete report").unwrap();
    let before = fs::read_dir(scratch.path()).unwrap().count();
    let result = publish_report(&report, |file| {
        file.write_all(b"partial replacement")?;
        // SAFETY: the scoped handler is installed on this isolated test
        // process; raise delivers SIGINT to the calling thread.
        assert_eq!(unsafe { libc::raise(libc::SIGINT) }, 0);
        Ok(())
    });
    assert!(matches!(result, Err(Error::Interrupted)));
    assert_eq!(fs::read(report).unwrap(), b"previous complete report");
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), before);
}
