# Current handoff — 2026-10-29

## Source and scope

All ten accepted complexity reductions are implemented locally and unstaged over
`d7aed4d0` (Release 0.69.11). The only incoming dirty path was Cargo.lock, selecting
Platform and Platform Macros 0.7.6 instead of 0.7.5. It is byte-identical to its
admission backup. No active Cargo/rustc process was visible at admission.
Package versions remain 0.69.11; compatible pending notes are 0.69.12.

No staging, commit, push, tag, release, publication, dependency update, tool
installation or sibling edit ran. Manifests and Shared snapshot are unchanged.
Shared Tooling remains `d08cbcafbaff4227e798a890232e2248bf167504` (144 files).

## Completed changes

Tool admission carries a private supported mode instead of independently optional
digest/version expectations and a no-follow flag. Its public entrypoints retain
exact byte/version checks, validation order and subsequent execution checks.
Execution Debug formatting shares its redacted output fields, retaining type
names, status representation, lengths and truncation flags without output bytes.
Crash-barrier peers share protocol marker constants; their independent waits,
acknowledgements, timeouts and cleanup behavior remain intact.

Build evidence keeps the three used input identities rather than the complete
resolved roster. The retained build still owns artifact custody, and source
freshness, mismatched-build refusal and receipt identities remain checked.
Build and transaction timings share absence-aware saturating addition, preserving
None, Some(zero) and overflow behavior.

Snapshot transfer selects each matching read/write region through one private
selector. Bounded message copying, exact canister/sender, complete-state admission,
timer/hook semantics and partial-failure cleanup are unchanged. Worker report
reading consumes its final anonymous descriptor; the earlier ownership clones
remain, along with bounded reads, worker completion and interruption handling.

Decimal rendering rounds and carries through one digit buffer. JSON response
keys reuse borrowing/escaped string decoding, preserving unknown-field handling
and duplicate-authority rejection. File commands select create/replace mode in
their own match arms and share private publication. Public APIs, feature and
dependency boundaries, cache/report formats and consumer lifecycle remain intact.
No consumer migration or new tool selection is required.

## Validation and limits

Evidence: `/tmp/ic-tooling-cleanup-06912.cqgTkd/`, selected by
`/tmp/ic-tooling-cleanup-06912-current`. Admission source/lock/handoff, commands,
attempted and final logs, final patch/source and identity records are retained.
Initial lint failures were corrected without allowances. The first CLI command
selected no tests because test-support was absent; its final command enables it.
Initial live snapshot attempts failed at listener startup in the sandbox; their
logs remain alongside the successful execution with localhost permission.

Actual Rust 1.91 Linux focused tests pass: 81 cases across tool admission,
digest-before-dispatch refusal, redacted execution evidence, acknowledged crash
barriers, build evidence/export/retention, both timing owners, decimal rendering,
response parsing, the actual file CLI, snapshot admission/cleanup and worker
report collection. Seven of these are live prepared-PocketIC 16.1.0 regressions,
including repeated coherent timer/hook/heap/stable/chunk-store restoration,
exact controller/sender behavior, refusal cleanup and recovery.

Warning-denied all-target/all-feature Clippy passes for the seven affected
packages on prepared Rust 1.99.1. Formatting, local documentation links and diff
checks pass. No full workspace/release gate ran. Native macOS 15 Intel and Apple
Silicon qualification did not run; Linux evidence is not native macOS evidence.

## Preserved obligations and evidence

[#263](https://github.com/dragginzgame/ic-tooling/issues/263) retains its consumer
acceptance scope at `/tmp/ic-tooling-263.WQ4ch3/`. This batch establishes neither
Canic adoption nor its four-case sequence qualification and changes no issue state.
The earlier race-fixture evidence and failed package attempt remain at
`/tmp/ic-tooling-preparation-race.IMHoSv/` and
`target/package-qualification-1823269-0/`.

Previous cleanup evidence remains at `/tmp/ic-tooling-cleanup-06911.Pznlfz/` and
`/tmp/ic-tooling-cleanup-06910.ZwEZDQ/`; older evidence and supplying-owner
obligations retain the scope linked through their handoffs, including
`/tmp/ic-tooling-cleanup-0698.LehyNb/`, `/tmp/ic-tooling-cleanup-0697.OHn8IL/`,
`/tmp/ic-tooling-262.zTQbiB/`, `/tmp/ic-tooling-cleanup-0696.7aCypD/` and
`/tmp/ic-tooling-deps-0696.OJreg2/`.
Migration/retirement backups remain under `/tmp/ic-tooling-migration/`. Original
Host archive refs, `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans,
failed attempts and earlier stash records are preserved. No artifact cleanup ran
beyond existing owned regression-fixture behavior.
