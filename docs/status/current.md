# Current handoff

## Platform/Wire adoption deployed — 2026-11-07

Adoption source `877900d5` selects Platform 0.24.0, Wire 0.17.8 and Shared
Tooling 0.31.5. Frozen batch `batch-__7k7VD8` completed both minor releases:

| Group | Published version | Release commit / annotated tag |
| --- | --- | --- |
| Host Tools | 0.85.0 | `07ef09c8723ab79fc9e764fa77d7f1ccfc0482b6` / `tools-v0.85.0` |
| Testkit | 0.92.0 | `ceb99e1c682a92026a80d743bfccada350319c08` / `testkit-v0.92.0` |

Both commits/tags reached `dragginzgame/ic-tooling:main`; both crates are
published to crates.io. Official sparse-index checksums and downloaded clean
Git provenance match their respective release commits. Private Dev/probe follow
Testkit 0.92.0 and remain unpublished. Foundation stays 0.75.0, Build 0.78.0,
Text 0.74.2 and Native 0.80.0.

Tools' bounded Candid adapter exposes Platform 0.24's `DecodeLimits`; Testkit's
native client exposes its `CborError`. Consumers naming those types must align
direct Platform dependencies. The removed unconfigured decoder/key-derivation
APIs had no callers here. Management DTO identity remains exactly 0.11.0;
Runtime 0.19.5 and Types 0.18.2 retain their identities. Protocol bytes, quotas,
server custody and no-replay behavior remain unchanged. Wire's compatible
minimum stays 0.17.0; 0.17.8 is the qualified released lock selection.

Shared's exact revision is `b2026a118e38de4af1555c527af327cb604eacdf`. The
canonical exporter verified all 175 selected payloads, unchanged from 0.31.4;
only the snapshot's display version and source revision changed.

## Qualification and publication evidence

Both groups passed the complete 34-stage Linux gate on Rust 1.99.1, with zero
failed stages: workspace Clippy, native/Wasm graphs, isolated features, installed
PocketIC 16.1, packaged rebuilds/tests/doctests and workspace installed tests.
The two unchanged fixture stages reused admitted receipts in the second group;
group-specific adapter checks ran fresh. The subsequent native matrix on source
`283bd149` passed Linux and macOS 15 Apple Silicon. macOS 15 Intel failed a
deadline fixture: its 50 ms budget could expire after acceptance but before the
expected polling request reached the scripted server. See the
[hosted run](https://github.com/dragginzgame/ic-tooling/actions/runs/42372609530).
The dependent MSRV summary failed with that native job; it was not a separate
compiler failure. Intel qualification remains outstanding.

The follow-up fixture repair observes the poll before advancing Tokio's test
clock. The snapshot cancellation case similarly waits for its accepted ingress
request before dropping the future. Production deadlines and client behavior
are unchanged. All 22 scripted failure-handling tests, focused Testkit Clippy,
Rustfmt and normal/build dependency checks passed locally. Tokio's test clock
feature remains dev-only, and the Wasm graph remains free of native dependencies.
Evidence is under `target/intel-deadline-fixture-20261107/`; fresh native CI runs
asynchronously after delivery. Testkit 0.92.1 has pending test notes only; no
additional release or publication was performed for this repair.

Captured checkout: `target/release-frozen/batch-__7k7VD8/`. Sealed proofs and
logs are beneath its `target/release-validation/`:

| Group | Gate / log directory |
| --- | --- |
| Tools | `tools-gate.nE3hb9/logs/validation.e4aawE/` |
| Testkit | `testkit-gate.x9l2qM/logs/validation.F12NBU/` |

Durable Git, plans, publication receipts and the completed schema-2 intent remain
under `.git/release-state/frozen/batch-__7k7VD8/`. Outer logs, registry readback,
reconciliation and lock comparisons remain in
`target/deployment-platform024-wire0178-20261107/`. The released external package
records match the deployment entry lock exactly. Earlier focused adoption checks
remain under `target/upstream-adoption-20261107/`.

## Ordinary checkout and deferred lock selections

The ordinary checkout was reconciled through `ceb99e1c` with Shared's helper.
During frozen qualification another updater selected Wire 0.17.9, crypto-common
0.1.6 and generic-array 0.14.9 in its lockfile. Those changes never entered the
active release. Their exact original lock is durably preserved on local branch
`preservation/tools-0.85-testkit-0.92-lock-11ad7694`, commit
`6e6aa8ed159eba5492caca2385e6fe74e126f475` (parent `877900d5`).

After checking no active ordinary-target build and proving the lock still matched
that preservation commit, reconciliation fast-forwarded the release metadata.
A clean three-way merge reapplied the deferred external selections while keeping
Tools 0.85.0 and Testkit/Dev/probe 0.92.0. Structured comparisons prove that the
restored external records equal the deferred lock and its owned records equal
the delivered lock. Cargo.lock remains dirty for the next candidate; Wire
0.17.9 has no qualification or publication claim in this deployment.

Before the CI fixture follow-up, another updater advanced the live deferred Wire
selection to 0.17.10, retaining crypto-common 0.1.6 and generic-array 0.14.9.
That newer lock is byte-preserved against
`target/intel-deadline-fixture-20261107/entry-Cargo.lock` and excluded from the
fixture commit. It is newer than the preservation branch above; it has no release
qualification claim. Never restore an older lock backup over later user edits.
No artifacts were retired.

## Earlier delivery and recovery

The prior cleanup batch `batch-vTGm2p2z` delivered Foundation 0.75.0, Build
0.78.0, Tools 0.84.0 and Testkit 0.91.0. Its four LOC cuts, consumer migration
links, sealed proofs, first preflight-only failure `batch-KNkEBs_1`, earlier
preservation refs and historical custody remain in the
[prior handoff](https://github.com/dragginzgame/ic-tooling/blob/877900d5d45776af3c2632e75bc899e89d59ae6c/docs/status/current.md).
Do not resume failed old attempts as new deployments or reinterpret old receipts.

Normal release commands already qualify, commit/tag/push and publish; do not
repeat publication. Resume by saved batch ID, preserve deferred edits before
reconciling and request explicit retirement scope. See [publishing](../publishing.md),
[batch custody](../release-batches.md) and [validation scope](../design/release-validation.md).
