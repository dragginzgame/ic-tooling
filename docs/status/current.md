# Current handoff — 2026-11-12

## Foundation 0.76.5 delivered

Foundation 0.76.5 adds compatible APIs to Artifacts, FS and HTTP:
aggregate bounded stream reads, capability-relative permission admission, an
exclusive descriptor lock guard and optional portable strict HTTP endpoints.
The Foundation ledger describes the APIs and qualification. Default Wasm
HTTP remains empty; opting into `endpoint` selects URL parsing without transport.
Consumers still own paths, byte limits, permissions, trust and recovery.

Release commit `963df312794cf90e4756d48420a2462e963e6c69` and annotated tag
`foundation-v0.76.5` match GitHub main. All five Foundation packages are present
and non-yanked on crates.io. Frozen `batch-eTIfLgJR` completed and was reconciled
into ordinary main. Source review commits are `715af316` and `bb4df402`.
The complete 32-stage Linux gate passed, including warning-denied Clippy/docs,
partial-read/permission/duplicate-lock tests, independent endpoint native/Wasm
checks, installed PocketIC and extracted-package qualification. Fresh native
macOS qualification remains asynchronous and is not claimed here.

Retain `.git/release-state/frozen/batch-eTIfLgJR/`, its managed checkout under
`target/release-frozen/`, and `target/deploy-foundation765/` logs and registry
observations. Gate evidence is `target/release-validation/foundation-gate.XaK96U/`
within that checkout. No sibling release or cleanup was performed.

Frozen `batch-lc826t2Z` stopped before preparation at a stale Make fixture's
isolated-feature expectation. Clippy and formatting passed; no release/tag/upload
was created. Its inputs and evidence remain retained. The fixture's corrected
two-command expectation passed focused and full-gate qualification in the new batch.

Build 0.83.0, Text 0.74.5, Tools 0.91.0 and Testkit 0.101.0 remain published.
Their pending notes describe contributor-tooling adoption, not a need to republish.
Private qualification packages remain 0.96.0. Do not release unchanged groups.

## Tooling and selected inputs

Both maintained workspaces select Toko Tooling 0.39.15 and the authenticated
115-file snapshot, retaining every entrypoint. `make deps-update` now invokes
`cargo update -v` sequentially in both workspaces; review/help/reporting commands
are retired. Snapshot/pin admission, local command routing and document links
passed on Linux during adoption. Cargo effects were substituted for the update
caller proof. Tool pins, baseline and product release gates did not change.
Saved adoption inputs and evidence remain under `target/toko03913/`,
`target/toko03914/` and `target/toko03915/`.

The release graph retains Platform 0.31.2, Types 0.19.4 and Wire 0.21.0;
qualification Runtime/macros/guard remain 0.21.1. Supplier freshness observations
and both all-feature metadata graphs are under `target/deploy-foundation765/`.
Optional patch/native-tool adoptions have dispositions in the Foundation ledger.

Unrelated ordinary lock updates were saved outside the release graph at
`.git/release-state/preserve/deploy-20261112-foundation765/`, including both full
locks, their patch and the prior handoff. The durable branch
`preserve/deploy-20261112-foundation765-dependencies` retains the latest deferred
choices. Both locks are restored after reconciliation, preserving every external
package block byte-for-byte while retaining Foundation 0.76.5 and the HTTP edge.
Both all-feature locked metadata checks pass. These deferred selections remain
ordinary-checkout changes, outside the released graph. No artifacts were retired.

## Testkit snapshot correction and open issue

[#263 — Cached snapshots and timer state](https://github.com/dragginzgame/ic-tooling/issues/263)
is corrected in published Testkit 0.101.0, release `08a193d8`, source `a54410f1`.
Frozen `batch-Aps5IIGp` completed and was reconciled. Its complete 32-stage Linux
gate passed. Retain `.git/release-state/frozen/batch-Aps5IIGp/`, its managed
checkout and `target/deploy263/`. The registry checksum and delivery verification
are recorded in those receipts and the Testkit ledger.

Native capture prepares metadata-upload snapshots through upstream PocketIC and
Platform-owned DTOs/codec, preserving timer/hook, globals, heap/stable, Wasm,
certified data and chunks. It requires two slots per target and transfer cycles;
stop/drain/start, exact senders, funding and partial-failure recovery remain
caller-owned. No client fork, second transport, timer reset or time rewind.
Installed regressions cover repeated mutated-baseline reuse and callback rearming.
Keep #263 open for Canic's exact two-case acceptance against the corrected release.
No Canic edits, dependency overrides or new tests are authorized by this delivery.
[Timers #87 — Snapshot regression qualification](https://github.com/dragginzgame/ic-timers/issues/87)
tracks its own coverage; the timer ownership guard remains intact.

[Push CI on 43ebe178](https://github.com/dragginzgame/ic-tooling/actions/runs/43180163629)
now passes native Linux, native macOS 15 Intel and Apple Silicon, lint/security
and MSRV. The failed Apple Silicon PocketIC HTTP 403 download cleared on retry;
the same URL independently returned the reviewed archive checksum. Retained
diagnostics remain under `target/ci43180163629/`. This qualifies the previous
source; do not use it to qualify the new Foundation changes.

## Next independent fixes

1. Preserve exact paths in [Build tool probes](../../crates/ic-build/src/tools/probe/mod.rs)
   and check raw directory names before archive normalization.
2. Propagate filesystem errors from [Cargo configuration selection](../../crates/ic-build/src/cargo/environment/mod.rs).
3. Reject empty/malformed successful [process-state observations](../../crates/ic-host-process/src/testing/mod.rs).

These findings are not part of this release. Canic's separate Build migration is
[Canic #762 — Explicit maintenance and pruning](https://github.com/dragginzgame/canic/issues/762).

## Retained evidence and contracts

The [previous committed handoff](https://github.com/dragginzgame/ic-tooling/blob/43ebe17817da21f9a0c376d8e87d7638a76582f6/docs/status/current.md)
records earlier batches, exact publication identities, preservation refs and
recovery paths. Retain them all; failed attempts must not resume under newer
published versions or today's adapters. Existing `preserve/*` refs and
`.git/release-state/preserve/` hold deferred lock/reconciliation evidence.
The retired client archive remains `/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`;
original source is `0c17900090f0c2d2ebf3182598b49e651223845e`.

Keep two-pass generated-root discovery, Darwin `File::sync_all` durability and
discovery error downcast identity. Retain the snapshot cleanup callback's explicit
future type; attempted `AsyncFnMut` alternatives failed its Send lifetime contract.
Native macOS 15 Intel and Apple Silicon evidence is separate from Linux. Hosted
qualification after delivery is asynchronous; do not wait or infer passes.
