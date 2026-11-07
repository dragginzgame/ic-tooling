# Current handoff

## Four LOC cuts — local, 2026-11-07

Build removes the compiler-recipe wrapper/executable, cross-call prepared input
snapshots and output/lifecycle progress events. Independent batches still share
compatible input resolution within each call. Observers receive phase-aware
heartbeats; completion, cache status and bounded diagnostics remain in results.
Warm-hit and post-build source revalidation, exact publication and shared-target
lock ordering remain enforced. Configuration-admission callback machinery is
untouched; its additional removal was rejected by automatic approval review.

The private fixture benchmark driver, worker and report framework are retired,
along with Process Git provenance and RSS sampling used only by that workflow.
The installed Wire test is now `canister_wire`; its actual control canister,
archive qualification and explicit II fixture remain. Probe/Types dependencies
are dev-only; unused Dev dependencies and FS temporary-file selection are gone.

The authorized release roster is Foundation 0.75.0, Build 0.78.0, Tools 0.84.0
and Testkit 0.91.0. Tools/Testkit minor notes include their exposed Foundation
type-identity change. Text and Native remain unchanged. Package versions have
not yet been prepared; the frozen runner owns preparation and publication.
[IcyDB #630 — Adopt Build 0.78 heartbeat-only progress configuration](https://github.com/dragginzgame/icydb/issues/630)
owns removing its output toggle after supplier delivery. No sibling source changed.

Evidence is under `target/loc-cut-20261107-2/`: strict all-target/all-feature Clippy
for Build/Process/Dev; 106 cache, batch, Wasm-build and example cases; installed
PocketIC 16.1 Wire acceptance; archive failure projection; warning-denied
Build/Process Rustdoc and Build doctest; native/Wasm graphs, Candid boundary and
selected release-routing fixtures all pass on Linux. The first live test was
blocked by sandbox socket binding; its native rerun passed. Formatting and
sourced ShellCheck pass. No full release gate or new native macOS proof.

Canonical LOC falls from 30,798 to 29,438 production and 81,164 to 75,242 across
all classified Rust sources, with zero unclassified LOC. The entry patch and
lock are retained beside the logs. Every external lock selection is preserved;
only Dev's removed Rustix/Thiserror edges differ from the entry lock.
The maintainer authorized frozen deployment of this batch. Source integration,
complete Linux gates and registry delivery are now in progress. No artifacts
were retired.

## Shared Tooling 0.31.4 adoption — local, 2026-11-07

The snapshot now selects `dad6d15750db66726850710f5fb423921b652c8d` through the
canonical exporter; all 175 payloads match the committed supplier. The repaired
release-command checker substitutes both runners, isolates inherited release
settings and exercises grouped outer and captured inner dispatch.
Tooling's actual Make adapter passes both fresh-CI and deliberately contaminated
parent-batch contexts, with release effects substituted. The actual LOC caller
also passes with absent scratch parents and zero unclassified Rust lines.
Evidence is under `target/shared-tooling-0.31.4/`; the entry lock is unchanged.
The refresh also carries scratch-loss diagnostics and release-guide companions.
No commits, releases or new hosted/native macOS qualification are included.

## Platform patch adoption — local, 2026-11-07

Platform 0.23.6 is published. The ordinary lock already selected it, Runtime
0.19.5 (including macros/guard) and Types 0.18.2 when this adoption began.
Existing compatible catalog ranges and the exact management 0.11.0 pin remain.
No Rust source migration is needed for this patch adoption. Its compatible notes
are included in the subsequent Tools 0.84.0 and Testkit 0.91.0 cleanup releases.

Locked metadata, declaration boundaries, native/Wasm graph policy and strict
affected-package native/Wasm Clippy pass. Eleven Candid adapter cases, four ledger
cases, management DTO identity and four installed PocketIC 16.1 cases pass on Linux.
Evidence and the unchanged entry lock are under `target/platform-adoption-0.23.6/`.
All pre-existing external lock selections are preserved. This new graph has no
fresh native macOS or complete release qualification; earlier delivery proof below
retains its original locked inputs.

## Delivered baseline and recovery

Build 0.77.0 and Testkit 0.90.0 were qualified and published through frozen batch
`batch-Q3Zp6GO6`. Release commits are `9d87394919b812cb6de55a9abd6dd2fd026c9070`
and `c03161b9f1395c343756fda6b8721e957798c3df`, with annotated tags
`build-v0.77.0` and `testkit-v0.90.0`. Both commits/tags reached main; private
Dev/probe remain unpublished. All 34 Linux release stages passed on their
original locked inputs; newer local adoption/cleanup has no complete gate.

Captured checkout: `target/release-frozen/batch-Q3Zp6GO6/`. Durable plans, Git,
publication receipts and intent: `.git/release-state/frozen/batch-Q3Zp6GO6/`.
Registry/archive readback and outer logs remain under
`target/deployment-build-0.77.0-testkit-0.90.0/`. Deferred lock state is preserved
on `preservation/build-0.77.0-testkit-0.90.0-lock-46b8c95e`, commit
`5dba6b89122e03e06f7509f004c8a90860bc505c`; later maintainer selections remain
dirty. Reconciliation records remain under
`.git/release-state/deployment-build-0.77.0-testkit-0.90.0/`.

The native matrix at [run 42357031568](https://github.com/dragginzgame/ic-tooling/actions/runs/42357031568)
failed on all three hosts in the old release-command fixture. Shared 0.31.4
repairs that fixture locally; no new complete native macOS pass is claimed.
Tooling #338/#339 producer delivery is closed; owning consumer migrations
remain separate. Full release receipts, consumer links, archive checksums,
earlier custody refs and historical journals remain in the
[prior handoff](https://github.com/dragginzgame/ic-tooling/blob/efb49a5981d4082bfa5e780b4c4323621871d754/docs/status/current.md).
Never restore older backups over live edits or reinterpret old batch receipts.

Normal releases already qualify, commit/tag/push and publish. Select a
supplier-first roster, resume by batch ID and reconcile only after preserving
deferred changes; do not repeat publication or retire artifacts implicitly.
See [publishing](../publishing.md), [batch custody](../release-batches.md) and
[validation scope](../design/release-validation.md). Tools/Build narrowing still
needs a matched three-host comparison; changed shared inputs retain full checks.
