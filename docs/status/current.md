# Current handoff — 2026-10-28

## Current fix and source

The local correction for [#262 — Fix macOS source-inventory mode compilation](https://github.com/dragginzgame/ic-tooling/issues/262)
is unstaged over clean HEAD `835cea7a5ced5317c3d2db9398c2699f2e980e78`
(the maintainer's `0.69.6` preparation commit). The package version remains
0.69.5, and the pending 0.69.6 notes now include this compatible repair.
No staging, commit, push, tag, release or publication ran. No unrelated active
Cargo/rustc process was visible at admission. Sibling checkouts, consumer
compatibility pins, package manifests, Cargo.lock and the real index are intact.

The issue and comment were read through GitHub. Exact native Intel and Apple
Silicon logs, plus the optimizer Intel log, confirm E0308 in released 0.69.5:
Darwin's Rustix mode bits are u16 while the inventory required u32. The same
failure affects consumers preparing IC Build's release editor; their temporary
0.69.4 selections remain consumer-owned until a corrected release is qualified.

## Implementation and contracts

The private source record now stores Rustix Mode directly. The descriptor's mode
is masked with typed user/group/other permission flags, and its numeric bits are
projected through the existing serde_json value representation only at encoding.
This removes the platform-width mismatch without a cast or Linux same-type
conversion warning. Permission masking remains exactly 0o777; SHA-256 framing,
held-descriptor custody, JSON field ordering and the encoded evidence schema
remain unchanged. No public API or artifact format migration is required.

The existing inventory regression now exercises special permission bits and the
ordinary permission range before comparing complete JSON bytes with the former
projection. It retains byte/mode change, symlink and resource-overflow checks.
Strict Rust 1.91 Clippy exposed one pre-existing unfulfilled result_large_err
expectation in the batch runner. Its deliberate inline-error exception is now
a narrow allow with a compiler-variance reason, preserving the runtime layout
while allowing both supported lint paths to pass without command-line overrides.

## Validation and limits

Evidence is under `/tmp/ic-tooling-262.zTQbiB/`, selected by
`/tmp/ic-tooling-262-current`. It retains the admission HEAD/status, lockfile and
previous handoff, exact hosted failure logs, validation logs and final patch/source
capture. The failed first MSRV lint attempt remains alongside its passing retry.
No tools or compiler targets were installed or downloaded.

Actual Rust 1.91 Linux package/source-inventory checks pass: 13 tests, including
complete JSON byte equality with the permission mask. Warning-denied IC Build
all-target/all-feature Clippy passes on both Rust 1.91 and the development
compiler. The artifact-transform integration target wasm_optimizer compiles on
Rust 1.91. Formatting, local documentation links and diff checks are recorded
beside those logs. No full workspace/release gate ran.

The pinned local optimizer selection is unavailable, so real optimizer execution
was not run; the read-only admission failure is retained. Neither Darwin target's
standard library is installed locally, and no fresh native macOS 15 Intel or
Apple Silicon qualification ran. Historical failing macOS logs identify the
cause; they are not qualification of this patch. Required native CI and the
corrected published Build artifact remain part of the maintainer's delivery flow.
Keep #262 open until its qualification/delivery requirements are satisfied.

## Preserved consolidation and dependency work

The preceding seven-improvement cleanup and Platform 0.7.2 / Wire 0.7.19 adoption
are already included in HEAD and remain unchanged by this correction. Their
current pending 0.69.6 notes remain intact. The admission handoff is preserved as
`previous-handoff.md` in this issue's evidence directory. It records the 239-test
cleanup evidence under `/tmp/ic-tooling-cleanup-0696.7aCypD/` and focused adoption
evidence under `/tmp/ic-tooling-deps-0696.OJreg2/`, including two live Linux Wire
checks against prepared PocketIC 16.1. That evidence keeps its original scope;
this fix does not assert a new workspace or native macOS qualification.

Shared Tooling remains the exact 144-file snapshot at
`d08cbcafbaff4227e798a890232e2248bf167504`. Supplying [Platform #50](https://github.com/dragginzgame/ic-platform/issues/50)
and historical [Tooling #240](https://github.com/dragginzgame/ic-tooling/issues/240)
retain their existing evidence and obligations; no advisory exemption or blanket
crypto qualification is supplied. Earlier records remain linked in the preserved
handoff. Migration/retirement archives remain under `/tmp/ic-tooling-migration/`.
Original Host refs, stale-lock evidence at
`/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans, failed attempts
and earlier backup/stash evidence remain preserved.
