# Current handoff — 2026-10-29

## Source and accepted scope

The package-qualification race-fixture fix is local and unstaged over admission
HEAD `55b1bdf1` (commit “0.69.10”). That commit contains all ten preceding cleanup
changes; the previous handoff's unstaged description is stale. Package versions
remain 0.69.9 and the pending changelog is 0.69.10. The working tree and staged
tree were clean at admission. Cargo.lock, manifests, Shared snapshot and refs
are unchanged. No unrelated active Cargo/rustc process was visible at admission.
No staging, commit, push, tag, release, publication or tool download ran.

## Failure and correction

The user's release attempt stopped in testkit-package-check, while executing
IC Build unit tests from the extracted coordinated archives. Original evidence
remains at `target/package-qualification-1823269-0/`; its test-targets.json records
276 passing tests and one failing preparation-input-race test. The package-cache
lock messages are not the failing assertion. The original release validation
log and extracted sources/targets are preserved without editing or cleanup.

The fixture observes content-lock creation after initial input capture, but
preparation may still be performing its verification hash. Its in-place fs::write
could truncate the input while that hash ran. Production correctly refused the
mutation, yet sometimes reported the transient input digest instead of the final
complete replacement digest expected by the test. Diagnostic execution reproduced
the assertion twice in 24 parallel attempts.

The replacement is now fully written before starting the worker and published
with an atomic rename at the existing synchronization point. Readers observe
complete old or new bytes. Exact before/after digest checks, one-attempt refusal,
retained output verification and explicit recovery remain intact. Assertion
failures now include expected and observed digests. Production code is unchanged.
The pending 0.69.10 notes include this qualification-fixture correction.

## Validation and evidence

Evidence is under `/tmp/ic-tooling-preparation-race.IMHoSv/`, selected by
`/tmp/ic-tooling-preparation-race-current`. Admission source/lock, previous
handoff, original failure logs, diagnostic/fixed runs and repeat-driver source
are retained there. After correction, the focused case passes on prepared
Rust 1.99.1 and actual Rust 1.91.0. A bounded run of 128 attempts with eight
parallel test processes passes without failure on Rust 1.99.1. The diagnostic
and fixed repeat logs retain every executed attempt.

Warning-denied IC Build library/test Clippy with all features, Rust formatting,
local documentation links and diff checks pass. No full workspace/release gate
was rerun; the original failed qualification remains failed evidence. Native
macOS 15 Intel and Apple Silicon qualification did not run. Linux execution
is not native macOS qualification. The user owns committing the fix and retrying
the standard release flow; no release plan or lock was cleared or modified.

## Preserved earlier work and obligations

The ten preceding cleanups are recorded in the admission source and
`/tmp/ic-tooling-cleanup-06910.ZwEZDQ/`. Their 248 focused Linux tests and actual
fresh/pooled driver smoke retain their original source, dependency and tool scope.
No blanket reuse or new full qualification is claimed after dependency changes.
The snapshot correction and consumer obligations for
[#263](https://github.com/dragginzgame/ic-tooling/issues/263) remain recorded at
`/tmp/ic-tooling-263.WQ4ch3/` and in the previous handoff. This fixture repair
establishes no publication, Canic adoption or four-case consumer qualification.
Sibling checkouts and their diagnostic work remain untouched.

Earlier evidence remains at `/tmp/ic-tooling-cleanup-0698.LehyNb/`,
`/tmp/ic-tooling-cleanup-0697.OHn8IL/`, `/tmp/ic-tooling-262.zTQbiB/`,
`/tmp/ic-tooling-cleanup-0696.7aCypD/` and `/tmp/ic-tooling-deps-0696.OJreg2/`.
Supplying-owner and optimizer qualification obligations retain their recorded
scope. Shared Tooling remains revision
`d08cbcafbaff4227e798a890232e2248bf167504` (144 files).
Migration/retirement backups remain under `/tmp/ic-tooling-migration/`. Original
Host archive refs, `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, prior release
plans, failed attempts and earlier backup/stash records remain preserved through
the previous handoffs. No external issue-state inspection or mutation ran.
