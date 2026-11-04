use super::SamplingError;
use super::{Process, Snapshot, command_snapshot, parse_ps, parse_stat, snapshot, tree_rss};
use crate::tool::{ExecutionFailure, OutputLimit, OutputLimits, OutputStream};
use std::process::Command;

const COMMAND_LIMITS: OutputLimits = OutputLimits {
    stdout: OutputLimit::Terminate(1024 * 1024),
    stderr: OutputLimit::Terminate(4096),
    timeout: None,
};

#[test]
fn command_capture_admits_complete_rows_without_a_deadline() {
    let observed = command_snapshot(
        Command::new("/bin/sh").args(["-c", "printf '10 1 4\\n20 10 8\\n'"]),
        COMMAND_LIMITS,
    )
    .unwrap();
    assert_eq!(tree_rss(10, &observed).unwrap(), (12 * 1024, 2));
    assert!(matches!(
        command_snapshot(
            Command::new("/bin/sh").args(["-c", "printf '\\377'"]),
            COMMAND_LIMITS
        ),
        Err(SamplingError::InvalidData(_))
    ));
}

#[test]
fn failed_table_commands_preserve_evidence_without_printing_it() {
    use std::error::Error as _;
    let error = command_snapshot(
        Command::new("/bin/sh").args([
            "-c",
            "printf '10 1 4\\n'; printf 'private diagnostic' >&2; exit 23",
        ]),
        COMMAND_LIMITS,
    )
    .unwrap_err();
    assert!(!error.to_string().contains("private diagnostic"));
    let SamplingError::Command(source) = &error else {
        panic!("table command failure retains the capture error");
    };
    let capture = source.execution_error().unwrap();
    assert!(matches!(capture.failure, ExecutionFailure::ExitStatus));
    assert_eq!(capture.evidence.status.unwrap().code(), Some(23));
    assert_eq!(capture.evidence.stdout, b"10 1 4\n");
    assert_eq!(capture.evidence.stderr, b"private diagnostic");
    assert!(error.source().unwrap().is::<crate::tool::ToolError>());
}

#[test]
fn overflowing_or_truncated_tables_never_yield_partial_snapshots() {
    for stdout in [OutputLimit::Terminate(7), OutputLimit::Truncate(7)] {
        let error = command_snapshot(
            Command::new("/bin/sh").args(["-c", "printf '10 1 4\\n20 10 8\\n'"]),
            OutputLimits {
                stdout,
                ..COMMAND_LIMITS
            },
        )
        .unwrap_err();
        let SamplingError::Command(source) = error else {
            panic!("partial tables must fail capture admission");
        };
        let capture = source.execution_error().unwrap();
        assert!(matches!(
            capture.failure,
            ExecutionFailure::OutputLimit {
                stream: OutputStream::Stdout
            }
        ));
        assert!(capture.evidence.stdout.len() <= 7);
        assert!(capture.evidence.stdout_truncated);
    }
}

#[test]
fn tree_counts_runner_and_descendants_but_not_parent_or_sibling() {
    let processes: Snapshot = [
        (1, 0, 10000),
        (10, 1, 100),
        (11, 1, 20000),
        (20, 10, 200),
        (21, 20, 300),
        (22, 10, 400),
    ]
    .into_iter()
    .map(|(pid, parent, resident_bytes)| {
        (
            pid,
            Process {
                parent,
                resident_bytes,
            },
        )
    })
    .collect();
    assert_eq!(tree_rss(10, &processes).unwrap(), (1000, 4));
    assert_eq!(tree_rss(999, &processes).unwrap(), (0, 0));
    let cycles = [
        (
            1,
            Process {
                parent: 2,
                resident_bytes: 1,
            },
        ),
        (
            2,
            Process {
                parent: 1,
                resident_bytes: 2,
            },
        ),
    ]
    .into();
    assert_eq!(tree_rss(1, &cycles).unwrap(), (3, 2));
}

#[test]
fn kernel_stat_with_parentheses_in_comm_preserves_parent_and_rss() {
    let fields = ["S", "42"]
        .into_iter()
        .chain(["0"; 19])
        .chain(["7"])
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(
        parse_stat(&format!("123 (name with ) parentheses) {fields}"), 4096).unwrap(),
        Process {
            parent: 42,
            resident_bytes: 7 * 4096
        }
    );
    assert!(matches!(
        parse_stat("123 (name) S bad", 4096),
        Err(SamplingError::InvalidData(_))
    ));
}

#[test]
#[cfg(target_os = "linux")]
fn linux_stat_records_must_be_complete_within_the_byte_allowance() {
    use ic_host_artifacts::artifact::ArtifactError;
    use std::{fs, io};

    let fixture = crate::test_support::Fixture::new();
    fs::create_dir(fixture.path().join("123")).unwrap();
    let fields = ["S", "42"]
        .into_iter()
        .chain(["0"; 19])
        .chain(["12345"])
        .collect::<Vec<_>>()
        .join(" ");
    let record = format!("123 (name with ) parentheses) {fields}");
    fs::write(fixture.path().join("123/stat"), &record).unwrap();
    let exact = u64::try_from(record.len()).unwrap();

    // Cutting within RSS must not turn 12345 pages into a valid 123-page
    // observation. Zero and one-byte-short bounds must also detect overflow.
    for allowance in [exact - 2, 0, exact - 1] {
        let error = super::process_snapshot(fixture.path(), 4096, allowance)
            .expect_err("oversized stat records cannot yield partial RSS");
        let SamplingError::Io(source) = error else {
            panic!("record allowance refusal retains its typed stream cause");
        };
        assert_eq!(source.kind(), io::ErrorKind::Other);
        assert!(matches!(
            source.get_ref().unwrap().downcast_ref::<ArtifactError>(),
            Some(ArtifactError::LimitExceeded { limit }) if *limit == allowance
        ));
    }
    for allowance in [exact, exact + 1] {
        let observed = super::process_snapshot(fixture.path(), 4096, allowance).unwrap();
        assert_eq!(observed[&123].parent, 42);
        assert_eq!(observed[&123].resident_bytes, 12345 * 4096);
    }

    fs::create_dir(fixture.path().join("124")).unwrap();
    let observed = super::process_snapshot(fixture.path(), 4096, exact).unwrap();
    assert_eq!(observed.len(), 1); // A vanished stat entry still skips.

    fs::write(fixture.path().join("124/stat"), [0xff]).unwrap();
    assert!(matches!(
        super::process_snapshot(fixture.path(), 4096, exact),
        Err(SamplingError::Io(source)) if source.kind() == io::ErrorKind::InvalidData
    ));
}

#[test]
fn darwin_ps_preserves_parents_and_kib_units_and_rejects_bad_rows() {
    let processes = parse_ps(" 10 1 4\n 20 10 8\n 21 20 0\n").unwrap();
    assert_eq!(tree_rss(10, &processes).unwrap(), (12 * 1024, 3));
    for malformed in [
        "10 1",
        "10 1 -1",
        "10 1 18446744073709551615",
        "10 1 1 extra",
        "10 1 1\n10 1 2",
    ] {
        assert!(matches!(
            parse_ps(malformed),
            Err(SamplingError::InvalidData(_))
        ));
    }
}

#[test]
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn native_snapshot_includes_this_process_once() {
    let processes = snapshot(64 * 1024, COMMAND_LIMITS).unwrap();
    let own = processes
        .get(&std::process::id())
        .expect("native sampler observes test process");
    assert!(own.parent > 0);
    assert!(own.resident_bytes > 0);
    assert!(tree_rss(std::process::id(), &processes).unwrap().0 >= own.resident_bytes);
}
