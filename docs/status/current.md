# Current handoff

## Cleanup deployed — 2026-11-07

Shared Tooling 0.31.4 and the four approved LOC cuts are integrated in `5d5c63df`.
The empty-script-root preflight correction is `cf872e74`. One supplier-first
frozen batch, `batch-vTGm2p2z`, qualified, tagged, pushed and published:

| Group | Version | Release commit / annotated tag |
| --- | --- | --- |
| Foundation (five public packages) | 0.75.0 | `81de54ec4b176a21418f996f229353c20a48f69a` / `foundation-v0.75.0` |
| Build | 0.78.0 | `5bdfc9f697cd778f4b9639b4e36c2c0f92defd46` / `build-v0.78.0` |
| Host Tools | 0.84.0 | `fbd155bf17628a81d7bf1302e1474de485bfec3c` / `tools-v0.84.0` |
| Testkit | 0.91.0 | `5a2fe419cb0a0cea1198937b4de4bbd3e423d5eb` / `testkit-v0.91.0` |

All eight archives are published to crates.io. Their official sparse-index
checksums and downloaded clean Git provenance match the respective release
commits. Private Dev/probe follow Testkit 0.91.0 and remain unpublished.
Text stays 0.74.2; Native stays 0.80.0. No unrelated group was released.

Build removes compiler-recipe wrappers, cross-call prepared snapshots and rich
output/lifecycle progress events. Independent batches retain shared resolution
within each call, warm-hit/post-build source checks, exact publication and target
lock ordering. Observers receive phase-aware heartbeats; completion and bounded
diagnostics remain in results. Configuration-admission callback machinery stays
intact; automatic approval review rejected its additional removal.

The optional fixture benchmark driver, worker and reporting framework are gone,
with their unused Process Git-provenance/RSS helpers. Installed Wire acceptance,
the private control canister, archive qualification and the II fixture remain.
Probe/Types dependencies are dev-only; normal private binaries also compile
without test-feature unification. Tools/Testkit adopt Foundation 0.75's exposed
type identities and therefore receive minor releases.

[IcyDB #630 — Adopt Build 0.78 heartbeat-only progress configuration](https://github.com/dragginzgame/icydb/issues/630)
owns its output-toggle removal and alignment of direct Host/Tools dependencies.
No sibling source or lockfile was edited. See the [Build migration](../build/artifacts.md#build-078-migration).

## Qualification and custody

Each group passed the complete 34-stage Linux gate on Rust 1.99.1, including
workspace Clippy, native/Wasm graphs, isolated features, installed PocketIC 16.1,
clean archive rebuilding/tests/doctests and workspace installed tests. The two
unchanged fixture stages reused admitted source-bound receipts after Foundation;
group-specific adapter checks ran fresh. Native macOS qualification remains
separate and asynchronous; no final macOS pass is claimed here.

The captured checkout is `target/release-frozen/batch-vTGm2p2z/`. Its sealed
proofs and all gate logs remain under `target/release-validation/` there:

| Group | Gate directory |
| --- | --- |
| Foundation | `foundation-gate.EswMHl` |
| Build | `build-gate.zBfOcO` |
| Tools | `tools-gate.JbbF9P` |
| Testkit | `testkit-gate.o5RGpe` |

Durable Git, plans, publication receipts and completed schema-2 intent remain
under `.git/release-state/frozen/batch-vTGm2p2z/`. Outer logs, reconciliation,
registry readback and entry/current external-lock comparisons are under
`target/deployment-loc-cut-20261107/`. All external package records are identical
to the deployment entry lock; only owned release versions changed. Shared's
reconcile helper fast-forwarded the ordinary checkout through `5a2fe419`.
No deferred source edits or artifact retirement were needed.

The first attempt, `batch-KNkEBs_1`, stopped in preflight before preparation,
tags or uploads because retiring the compiler fixture emptied the scripts Rust
root. The local caller now admits that empty root while still checking new Rust
fixtures and symlinks; its focused regression passes. That failed attempt is
retained and must not be resumed as a new deployment.

## Adopted inputs and earlier evidence

Shared's exact revision is `dad6d15750db66726850710f5fb423921b652c8d`; all 175
snapshot payloads match. Actual caller qualification, including fresh/contaminated
release contexts and LOC scratch behavior, remains in `target/shared-tooling-0.31.4/`.
The released lock preserves Platform 0.23.6, Runtime 0.19.5, Types 0.18.2,
Wire 0.17.6 and the maintainer's other selected patches. Management DTO identity
remains exactly 0.11.0. Earlier focused adoption evidence is under
`target/platform-adoption-0.23.6/`.

Focused cleanup evidence is under `target/loc-cut-20261107-2/`: production LOC
fell from 30,798 to 29,438 and all classified Rust from 81,164 to 75,242, with
zero unclassified LOC. Those counts exclude later metadata-only release changes.
Earlier batch `batch-Q3Zp6GO6`, preservation refs and historical custody remain
in the [prior handoff](https://github.com/dragginzgame/ic-tooling/blob/cf872e745e48c58d43e5a7cd25378e49bbdd184e/docs/status/current.md).
Never restore old backups over newer edits or reinterpret old batch receipts.

Normal releases already qualify, commit/tag/push and publish; do not repeat
publication. Resume by the saved batch ID, preserve deferred edits before
reconciling and request explicit retirement scope. See [publishing](../publishing.md),
[batch custody](../release-batches.md) and [validation scope](../design/release-validation.md).
