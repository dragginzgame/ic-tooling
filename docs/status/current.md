# Current handoff

## Published deployment — 2026-11-08

The reviewed batch is committed, pushed to `origin/main` and published to
crates.io through the standard frozen workflow. All seven official archives
match their registry checksums and clean release-commit identities.

| Group | Published version | Release commit |
| --- | --- | --- |
| Foundation, all five packages | 0.76.1 | `58a1e041` |
| Build | 0.81.0 | `9a8c3a32` |
| Testkit | 0.95.0 | `9e66ba0f` |

Tools remains 0.88.0 and Text 0.74.3. The two private Testkit packages are 0.95.0
and were not uploaded. Source integration is `3cfa53c5`; all three group tags
were pushed normally. Both frozen batches report complete:
`batch-jNmWYunP` (Foundation patch) and `batch-4F91gJ1U` (Build/Testkit minor).
The ordinary checkout was reconciled through the shared command after each batch
and contains the released versions. No second `make publish` is needed.

## Delivered changes

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

Wire selects published 0.18.0, once in the resolved graph. Archive SHA-256 is
`16760e525bca7fa5c4042176551bd551a6235e39cec08564d1ecfffd20e2b130`.
Direct consumer Wire selections must align for `Limits` and typed codec errors.
No workspace caller uses the removed definition-level bulk-lint API. The captured
lock also includes Platform 0.27.1, Runtime/macros/guard 0.20.3, Types 0.18.3,
Logging 0.24.12 and the previously preserved compatible patch selections.
No deferred ordinary-checkout edits remained at final reconciliation.

## Qualification and native CI

All three complete Linux x86-64 release gates passed: workspace Clippy,
formatting/hooks, docs, release/recovery and routing fixtures, native/Wasm graph
policies and Wasm compilation, package archives/doctests, installed PocketIC 16.1
management/snapshot/gateway contracts and the complete workspace tests. Publication
admitted qualified lint reuse while retaining fresh archive/license/provenance
checks. Independent official registry readbacks verified all seven uploads.
Final tooling admission and documentation links passed after reconciliation.

Native macOS 15 Intel and Apple Silicon evidence remains separate. Four explicit
qualification runs select published tag `testkit-v0.95.0`, commit
`9e66ba0f24d05e15a5aca307f67dc85b6ef309cd`:

- [CI](https://github.com/dragginzgame/ic-tooling/actions/runs/42526447889).
- [Build optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/42526450429).
- [Testkit runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/42526453148).
- [Browser fixtures](https://github.com/dragginzgame/ic-tooling/actions/runs/42526455647).

All four were in progress at dispatch readback. No new native macOS pass is
claimed. Explicit dispatch identities survive subsequent routine pushes; do not
wait solely for this hosted matrix during delivery.

## Consumer adoption and retained custody

Known external source adoption sites are Canic's
`crates/canic-tests/src/pic/artifacts/mod.rs` (handle the fallible maintenance
builder) and Auth's `apps/qualification/runner/tests/signatures/main.rs` (remove
the instance owner type argument). Sibling sources were not changed or qualified.
Consumers must select the new minor versions for these public API hard cuts.

Logs, registry archives/readbacks, reconciliation and dispatch evidence remain
under `target/deployment-wire018/`, with full gate output in
`target/deployment-wire018-foundation.log` and
`target/deployment-wire018-integration.log`. Managed checkouts and their builds
remain under `target/release-frozen/`; durable Git, plans, receipts and batch
intents remain under `.git/release-state/frozen/`. No cleanup or retirement ran.
The [pre-deployment handoff](https://github.com/dragginzgame/ic-tooling/blob/3cfa53c5/docs/status/current.md)
retains focused development evidence and earlier preservation references.
Preserve all unfinished older batches; never replay them against published versions.
