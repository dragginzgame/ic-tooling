# Current handoff

## Authorized deployment — 2026-11-08

The maintainer requested deployment of the reviewed local batch. Selected releases
are Foundation 0.76.1 (patch), then Build 0.81.0 and Testkit 0.95.0 (minor).
Tools 0.88.0 and Text 0.74.3 have no product changes selected for publication.
Use the standard frozen workflow against `origin/main` and crates.io; separate
batches are required for the different increments. Managed execution remains
under `target/release-frozen/`, with durable `.git/release-state/frozen/` custody.
No sibling deployment or artifact cleanup is authorized. Later normal-checkout
edits are deferred and must survive reconciliation.

## Reviewed source

Toko Tooling 0.36.0 is selected once as an exact root workspace dependency.
All 11 real packages inherit its dev-dependency with a cargo-machete exemption.
The virtual root and real qualification packages remain; the former metadata
selection and separate `toko-tooling.lock` are removed. Cargo.lock, all members
and the authenticated 185-file snapshot agree. Producer archive SHA-256 is
`de0dd05b6437061622a76942baa25d40d2181497e7ba66d9fd809132a3082936`.
Update through Cargo and `make shared-tooling-update`; installing an existing
lock selection uses `make shared-tooling-install`.

Build has one target-mode configuration/acquisition plan, shared batch accounting,
reused Cargo fixtures and one release gate constructor. Maintenance builders now
return `Result`, reject isolated mode before effects and clear when isolated
mode is selected. Wasm batches return Host's `BatchLabelError` directly; the
single-variant wrapper and forwarding acquisition/lock helpers are removed.
Distinct cache formats, fingerprints, lock order, retention and failure timings
remain unchanged; the proposed format/timing merges were rejected.

Testkit's HTTP reader delegates to Host HTTP under the original absolute deadline,
retaining typed allocation failures and accepted-operation context. It keeps one
concrete `PocketIcInstance`; workspace callers, tests and docs are migrated.
HTTP fixtures share borrowed request views and the orphan path waiter is removed.
The cumulative provenance patch reconstructs the unchanged donor/upstream source;
licenses, algorithm choices, protocol bytes, async/no-replay and caller custody
remain unchanged. Management DTOs retain the exact 0.11.0 identity.

Wire now selects published 0.18.0, once in the resolved graph. Archive SHA-256 is
`16760e525bca7fa5c4042176551bd551a6235e39cec08564d1ecfffd20e2b130`.
Native helpers and the private Wasm probe retain Serde contracts and explicit
budgets. Direct consumer Wire selections must align for `Limits` and error types.
No workspace caller uses the removed definition-level bulk-lint API.

The normal lock also contains preserved external patch selections. Concurrent
updates selected Logging 0.24.12, Types 0.18.3 and Runtime/macros/guard 0.20.3;
review and qualify the captured lock, never replace later edits with old backups.
Earlier preserved selections include arc-swap 1.9.4, crypto-common 0.1.6 and
generic-array 0.14.9. No unrelated source changes were found in the reviewed batch.

## Evidence and consumer adoption

Focused Linux checks passed for Build artifact/cache/batch behavior and all
three affected Cargo/Wasm integration targets, native Testkit transport and
ownership, installed PocketIC 16.1 management/snapshot/gateway contracts,
management DTO identity, Clippy and doctests. Wire's nine adapter tests, private
probe budget test and installed native/Wasm typed-call test passed, alongside
native/Wasm graph policies and Wasm compilation. Tooling fixtures, source
reconstruction, dependency admission, formatting and 862 documentation links
passed. These are focused results, not the new batch's complete release gate.
Evidence remains under `target/tooling036-migration/`, `target/complexity-cleanup/`,
`target/complexity-next-two/` and `target/wire018-adoption/`.
Native macOS 15 Intel/Apple Silicon qualification remains separate/asynchronous.

Known external source adoption sites are Canic's
`crates/canic-tests/src/pic/artifacts/mod.rs` (handle the fallible maintenance
builder) and Auth's `apps/qualification/runner/tests/signatures/main.rs` (remove
the instance owner type argument). Sibling sources were not changed or qualified.

## Previous delivered baseline and retained custody

The preceding deployment published Foundation 0.76.0, Build 0.80.0, Tools 0.88.0
and Testkit 0.94.0 through completed frozen batch `batch-0sSkZugM`; Text stayed
0.74.3. Its complete Linux gates, registry checksum/VCS verification, native CI
links and earlier preservation refs are recorded in the
[previous handoff](https://github.com/dragginzgame/ic-tooling/blob/eeee3e68/docs/status/current.md).
Logs remain under `target/deployment-audit-toko034/`. Retain all frozen batches,
source pins, deferred work and recovery records. Failed older attempts must not
be resumed against already-published versions. No cleanup or retirement was run.
