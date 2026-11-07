# Current handoff

## Build 0.77.0 and Testkit 0.90.0 delivered — 2026-11-07

The approved LOC hard cut is integrated in `7b4f5985`. One supplier-first frozen
`build,testkit` minor batch, `batch-Q3Zp6GO6`, qualified and published both groups.

| Group | Published version | Release commit / annotated tag |
| --- | --- | --- |
| Build | 0.77.0 | `9d87394919b812cb6de55a9abd6dd2fd026c9070` / `build-v0.77.0` |
| Testkit | 0.90.0 | `c03161b9f1395c343756fda6b8721e957798c3df` / `testkit-v0.90.0` |

Both commits/tags reached `dragginzgame/ic-tooling:main`; both crates are published
to crates.io. Private Dev/probe follow Testkit and remain unpublished. Other groups
remain Foundation 0.74.4, Text 0.74.2, Host Tools 0.83.0 and Native 0.80.0.

Six timing records now expose documented fields instead of 44 forwarding getters;
cache fixtures share three-mode setup. Testkit removes unused Candid convenience,
default-ID, server-filesystem snapshot, no-tick/authenticated-status, blockmaker,
subnet-deletion and topology shortcut surfaces. Used topology/management DTOs,
protocol bytes, bounded snapshot transfer and caller custody remain. Direct
backoff and three unused Reqwest features are removed. A checksummed surface-cut
patch retains original donor fingerprints, licenses and reconstruction guards.

Testkit requires explicit positive finite `Duration` budgets for creation,
attachment and listing. Invalid values refuse before requests; one deadline
covers bodies, polling and pruning. Recipes select `snapshot_funding()` and
validate their own reset before returning unit. Generic reset schemas/receipts
are removed; restore/funding order, readiness, cancellation invalidation, typed
recovery and collect-all shared shutdown remain. Workspace callers and guides
are migrated without aliases. See the [0.90 migration](../testkit/client-migration.md#testkit-090-surface-cut).

[Tooling #338](https://github.com/dragginzgame/ic-tooling/issues/338) and
[#339](https://github.com/dragginzgame/ic-tooling/issues/339) are closed for producer
delivery. The maintainer chose consumer issues instead of concurrent sibling edits.
All eleven owning issues linked from #339 now record the published supplier;
[Canic #738](https://github.com/dragginzgame/canic/issues/738) also owns reset-policy
adoption. Those migrations and consumer qualification remain open. Prepared sibling
edits were withdrawn while preserving concurrent work; no Cargo overrides or
consumer checks/releases occurred. Toko/Toko Miner and independent native Profiler
remain excluded. Earlier focused evidence is under `target/hard-cut-338-339/`.

## Qualification and registry evidence

Both groups passed all 34 complete Linux release stages on Rust 1.99.1, including
strict workspace lint, provenance, fresh packaged targets/docs, native/Wasm graphs,
isolated features and installed PocketIC 16.1 contracts. Unchanged fixture stages
were reused only through admitted source-bound receipts.

Captured checkout: `target/release-frozen/batch-Q3Zp6GO6/`. Gate logs beneath it:

| Group | Complete gate logs |
| --- | --- |
| Build | `target/release-validation/build-gate.mxozID/logs/validation.etN3A9/` |
| Testkit | `target/release-validation/testkit-gate.nCqrYa/logs/validation.RFW5Fh/` |

Durable Git, plans, publication receipts and the completed schema-2 intent remain
under `.git/release-state/frozen/batch-Q3Zp6GO6/`. Outer logs and official sparse
registry/archive readback are in `target/deployment-build-0.77.0-testkit-0.90.0/`.
The downloaded archives match clean release Git identities and registry SHA-256:
Build `41883c4420ed9c70d4a0be08718141675001315f1f1964e13169b253bbb3ee01`;
Testkit `aeeb9dd0920b5a4343aed61f2cb7c0266488823dab5f7b19e0b632b54b7d8f5a`.

The frozen batch used Shared 0.31.0. Follow-up `efa094d8` adopts the committed
Shared 0.31.2 checksum-parser fix from `e11ed1c9b7377c673bca44a21cd819694c5c37a7`,
through the canonical exporter. All 175 payloads match that commit; only the
checksum helper changes. Snapshot integrity, Linux shell syntax and actual archive
checksum acceptance/rejection pass. No crate source or published identity changes.

The previous native matrix failed in macOS checksum parsing and a Linux release
command fixture. The current local gates pass that fixture; the parser fix is now
pushed. The [retained native matrix](https://github.com/dragginzgame/ic-tooling/actions/runs/42357031568)
runs asynchronously at `efa094d887582f81160bf5c5e704cda54747ed0a`, independently of
later documentation pushes. It covers Linux x86-64 and macOS 15 Intel/Apple Silicon.
Native macOS qualification remains pending; no new macOS pass is claimed here.

## Ordinary checkout and retained recovery

Shared reconciliation fast-forwarded the ordinary checkout through both releases.
Deferred lock selections were preserved on
`preservation/build-0.77.0-testkit-0.90.0-lock-46b8c95e`
(commit `5dba6b89122e03e06f7509f004c8a90860bc505c`), then deliberately reapplied by
three-way merge. Only four owned package versions change from the saved lock.
Its restored SHA-256 is
`c66fc6d830d1006e3e5804cf2f0bc8ed1c5f639ce5d68af0ece5b83cf4e5d6ad`.
External patch selections remain dirty/deferred and were not included in either
release. Durable snapshots, narrow diffs and the reconciliation transcript remain
under `.git/release-state/deployment-build-0.77.0-testkit-0.90.0/`.
Never restore older backups over later live edits. No artifacts/batches were retired.

Historical schema-1 batches `batch-UqiVuh8F` and `batch-c3bPF1yB` remain complete
under their original tooling contract. Earlier preservation refs, .89.x release
records and activation-stash custody remain in the [previous handoff](https://github.com/dragginzgame/ic-tooling/blob/7b4f5985/docs/status/current.md).
Do not rewrite old journals or reinterpret their receipts as current adapters.

Normal releases already qualify, commit/tag/push and publish; do not repeat
`make publish` after successful default delivery. Select a supplier-first roster,
resume by batch ID and reconcile only after preserving deferred changes.
Retirement requires separate authority. See [publishing](../publishing.md),
[batch custody](../release-batches.md) and [validation scope](../design/release-validation.md).
Tools/Build narrowing still needs its matched three-host comparison; unknown or
shared inputs retain the full gate. GitHub issues are the sole follow-up tracker.
