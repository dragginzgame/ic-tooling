use super::{Mode, WorkerArguments};
use std::ffi::OsString;

#[test]
fn worker_invocation_admits_both_modes_and_zero_state() {
    for mode in [Mode::Fresh, Mode::Pooled] {
        let args = [
            mode.as_str(),
            "2",
            "4",
            "2",
            "0",
            "-probe wasm",
            "-server path",
        ]
        .map(OsString::from);
        let parsed = WorkerArguments::try_from(args.to_vec()).unwrap();
        assert_eq!(parsed.workload.mode, mode);
        assert_eq!(parsed.workload.capacity.get(), 2);
        assert_eq!(parsed.workload.iterations.get(), 4);
        assert_eq!(parsed.workload.workers.get(), 2);
        assert_eq!(parsed.workload.state_bytes, 0);
        assert_eq!(<[OsString; 7]>::from(&parsed), args);
    }
}

#[test]
fn invalid_worker_invocations_refuse_before_work_starts() {
    for args in [
        vec!["fresh", "2", "4", "2", "0", "wasm"],
        vec!["fresh", "2", "4", "2", "0", "wasm", "server", "extra"],
        vec!["unknown", "2", "4", "2", "0", "wasm", "server"],
        vec!["fresh", "0", "4", "2", "0", "wasm", "server"],
        vec!["fresh", "2", "0", "2", "0", "wasm", "server"],
        vec!["fresh", "2", "4", "0", "0", "wasm", "server"],
        vec!["fresh", "2", "4", "5", "0", "wasm", "server"],
        vec!["fresh", "2", "4", "2", "4294967296", "wasm", "server"],
    ] {
        let error =
            WorkerArguments::try_from(args.into_iter().map(OsString::from).collect::<Vec<_>>())
                .err()
                .unwrap();
        assert_eq!(error.exit_code(), 2);
    }
}

#[cfg(unix)]
#[test]
fn worker_paths_preserve_native_non_utf8_bytes() {
    use std::os::unix::ffi::OsStringExt as _;
    let mut args = ["fresh", "1", "1", "1", "0", "wasm", "server"].map(OsString::from);
    args[5] = OsString::from_vec(vec![b'w', 255]);
    args[6] = OsString::from_vec(vec![b's', 254]);
    assert_eq!(
        <[OsString; 7]>::from(&WorkerArguments::try_from(args.to_vec()).unwrap()),
        args
    );
}
