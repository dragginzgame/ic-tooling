use super::*;
use crate::test_support::Fixture;

fn stat(parent: &str, start: &str) -> String {
    let mut values = ["0"; 22];
    values[0] = "S";
    values[1] = parent;
    values[11] = "100";
    values[12] = "50";
    values[13] = "30";
    values[14] = "20";
    values[17] = "2";
    values[19] = start;
    values[21] = "3";
    format!("10 (worker (one)) {}", values.join(" "))
}

#[test]
fn identity_and_counters_preserve_units_and_parentheses() {
    let fixture = Fixture::new();
    fs::create_dir(fixture.path().join("10")).unwrap();
    let file = fixture.path().join("10/stat");
    fs::write(&file, stat("1", "123")).unwrap();
    let observed = basic_process(fixture.path(), 10).unwrap();
    assert_eq!(observed.name, "worker (one)");
    assert_eq!(observed.ppid, 1);
    assert_eq!(
        process_identity(fixture.path(), 10),
        Some(observed.start_ticks)
    );
    assert_eq!(observed.user_ns, 100 * 1_000_000_000 / ticks_per_second());
    assert_eq!(
        observed.waited_user_ns,
        30 * 1_000_000_000 / ticks_per_second()
    );
    assert_eq!(observed.rss_bytes, 3 * page_size());
    assert_eq!(observed.threads, 2);
    for invalid in [
        stat("bad", "123"),
        stat("1", "bad"),
        "10 (worker) S 1".into(),
    ] {
        fs::write(&file, invalid).unwrap();
        assert!(basic_process(fixture.path(), 10).is_none());
        assert!(process_identity(fixture.path(), 10).is_none());
    }
}

#[test]
fn optional_details_keep_missingness_and_do_not_read_unselected_pss() {
    let fixture = Fixture::new();
    let dir = fixture.path().join("10");
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("stat"), stat("1", "123")).unwrap();
    fs::write(dir.join("io"), "read_bytes: 21\nwrite_bytes: 34\n").unwrap();
    fs::write(dir.join("smaps_rollup"), "Pss: 12 kB\n").unwrap();
    let mut observed = basic_process(fixture.path(), 10).unwrap();
    read_details(fixture.path(), &mut observed, false);
    assert_eq!(
        (observed.read_bytes, observed.write_bytes),
        (Some(21), Some(34))
    );
    assert_eq!(observed.pss_bytes, None);
    read_details(fixture.path(), &mut observed, true);
    assert_eq!(observed.pss_bytes, Some(12 * 1024));
    fs::remove_file(dir.join("io")).unwrap();
    fs::remove_file(dir.join("smaps_rollup")).unwrap();
    read_details(fixture.path(), &mut observed, true);
    assert_eq!(
        (
            observed.read_bytes,
            observed.write_bytes,
            observed.pss_bytes
        ),
        (None, None, None)
    );
}

#[test]
fn child_lists_cover_nonleaders_and_retain_partial_evidence() {
    let fixture = Fixture::new();
    let root = fixture.path();
    assert!(children(root, 10).unavailable);
    for tid in [10, 11, 12] {
        fs::create_dir_all(root.join(format!("10/task/{tid}"))).unwrap();
    }
    fs::write(root.join("10/task/10/children"), "20 30").unwrap();
    fs::write(root.join("10/task/11/children"), "30 40 malformed").unwrap();
    let seen = children(root, 10);
    assert_eq!(seen.pids, [20, 30, 40]);
    assert!(seen.incomplete);
    assert!(!seen.unavailable);
    fs::write(root.join("10/task/11/children"), "30 40").unwrap();
    fs::write(root.join("10/task/12/children"), "").unwrap();
    let seen = children(root, 10);
    assert_eq!(seen.pids, [20, 30, 40]);
    assert!(!seen.incomplete && !seen.unavailable && !seen.truncated);
}
