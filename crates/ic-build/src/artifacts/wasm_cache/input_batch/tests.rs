use super::*;
use crate::artifacts::test_support::fake_wasm_build_spec;
use std::{fs, thread};
#[cfg(unix)]
#[test]
fn prepared_reader_counts_are_concurrent_saturating_and_only_for_matches() {
    let (root, spec) = fake_wasm_build_spec("reader-counter");
    let state = super::WasmBuildInputSnapshotState::prepare(std::slice::from_ref(&spec)).unwrap();
    let unknown = spec
        .clone()
        .with_environment(crate::testing::cargo::environment_with([(
            "OTHER_RECIPE",
            "yes",
        )]));
    assert!(state.admit(&[&unknown]).is_err());
    assert_eq!(state.admit(&[&spec, &unknown]).unwrap_err(), 1);
    assert_eq!(state.reader_reuses(), 0);
    thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| {
                for _ in 0..32 {
                    assert!(state.admit(&[&spec]).is_ok());
                }
            });
        }
    });
    assert_eq!(state.reader_reuses(), 256);
    state.reader_reuses.store(usize::MAX - 1, Ordering::Relaxed);
    thread::scope(|scope| {
        for _ in 0..8 {
            scope.spawn(|| assert!(state.admit(&[&spec]).is_ok()));
        }
    });
    assert_eq!(state.reader_reuses(), usize::MAX);
    assert!(state.admit(&[&unknown]).is_err());
    assert_eq!(state.reader_reuses(), usize::MAX);
    fs::remove_dir_all(root).unwrap();
}
