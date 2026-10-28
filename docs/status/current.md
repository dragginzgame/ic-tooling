# Current handoff — 2026-10-28

## Accepted cleanup and source

The accepted ten-candidate complexity review is complete: seven simplifications
are retained, and three were rejected after source review or contract tests.
Changes are local and unstaged over clean admission HEAD
`cba072ee8fe288eb92bb4a4ddf9753b8abc4b580` (Release 0.69.6). Pending compatible
notes are 0.69.7; package versions remain 0.69.6. No staging, commit, push, tag,
release, publication or tool download ran. No unrelated Cargo/rustc process was
visible at admission. Cargo.lock, package manifests, Shared snapshot and the real
index remain unchanged. Sibling checkouts and existing artifacts are preserved.

## Retained changes and contracts

Wasm acquisition records its authoritative failure phase from specification
onward, removing error-type inference. Cargo configuration traversal groups its
observations, active include paths and remaining limits. Native Cargo message
handling groups selection, completion and diagnostic state, keeping the strict
native protocol distinct from Wasm's stream handling.

Package qualification groups its retained input observations and final
revalidation. Verification consumes the baseline, preserving progressive release
of original manifest/archive entries. Observation and rejection order, final
source evidence, archive hashes and admitted manifest/lock checks remain intact.
A regression mutates each of the six retained file classes independently.

PocketIC supervision shares its liveness-to-failure conversion across readiness,
instance construction and command supervision. Callers retain their own cleanup
and cancellation. Benchmark report handling shares pre-publication cleanup
reporting, preserving successful publication with a durability warning. Worker
callers use status/identity operations instead of accessing the raw child.
Cooperative worker termination and descendant custody are unchanged.

The error-derive candidate changed generic boxed source identity in two enums;
the existing baseline-pool regression and new artifact-batch regression caught
it. Both production implementations were restored. The new regression retains
source downcast and pointer-identity coverage. Batch maintenance stays lazy and
ordered: eager normalization would change filesystem failure observation or
require more cloning/representation. Retained-write dispatch keeps its single
public signature per operation instead of duplicating platform implementations.
No public API, feature, persisted format or consumer policy migration is needed.

## Validation and limits

Evidence: `/tmp/ic-tooling-cleanup-0697.OHn8IL/`, selected by
`/tmp/ic-tooling-cleanup-0697-current`. It retains the admission HEAD archive,
incoming lockfile, previous handoff, commands/check logs and final patch/source
capture. Failed experiments and initial lint attempts remain beside corrections.
No full workspace or release gate ran; no performance measurement is claimed.

Actual Rust 1.91 Linux focused checks pass: 177 tests across configuration,
native Cargo, package revalidation, Wasm acquisition/batches, artifact batches,
baseline errors, benchmark workers/reports and startup lifecycle. The startup
selection leaves its six explicitly live cases ignored. Two selected live
sync/async ownership tests additionally pass against the existing PocketIC
16.1.0 Linux binary after offline CLI admission. Total: 179 passing tests.
The MSRV release-editor/package-source regression from #262 remains included
in the package selection; this batch does not assert issue closure or delivery.

Warning-denied all-target/all-feature Clippy for the three changed packages
passes with prepared development Rust 1.99.1. Rust 1.91 Clippy remains blocked
by 14 diagnostics in unchanged tests: two lease drop-scope findings and twelve
exact floating-point comparisons. Those files match admission HEAD. New local
Clippy findings were corrected; the failed logs retain the precise distinction.
The installed 1.91 toolchain has no Rustfmt; prepared development Rustfmt passes.
Documentation links and diff checks pass. Neither native macOS 15 Intel nor Apple
Silicon qualification ran for this patch; Linux execution is not macOS evidence.

## Preserved earlier evidence

The released 0.69.6 handoff is frozen at the admission commit and copied to
`previous-handoff.md` in this batch's evidence. #262 evidence remains under
`/tmp/ic-tooling-262.zTQbiB/`, including hosted Darwin failures, the portable mode
repair and the unavailable pinned-optimizer admission. This cleanup adds no
native macOS or real optimizer qualification and makes no GitHub issue updates.

Earlier cleanup evidence remains at `/tmp/ic-tooling-cleanup-0696.7aCypD/`;
Platform/Wire adoption evidence remains at `/tmp/ic-tooling-deps-0696.OJreg2/`.
Their claims retain their original source/dependency scope. Shared Tooling stays
at exact revision `d08cbcafbaff4227e798a890232e2248bf167504` (144 files).
Supplying-owner dependency and crypto qualification obligations retain the
evidence recorded in the preserved handoff; no advisory exemption or blanket
crypto qualification is supplied.

Migration and retirement backups remain under `/tmp/ic-tooling-migration/`.
Original Host archive refs, stale-lock evidence at
`/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans, failed attempts,
and earlier backup/stash records remain preserved through the prior handoffs.
