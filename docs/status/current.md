# Current handoff

## Tools 0.86.0 and Testkit 0.93.0 deployed — 2026-11-07

The user selected both minor releases. Frozen batch `batch-GpvCzBKt` completed
Git delivery and crates.io publication from implementation source `c86b9a27`:

| Group | Published version | Release commit / annotated tag |
| --- | --- | --- |
| Host Tools | 0.86.0 | `b5543ed3c4021d5a12435322e736d1e76f8a177a` / `tools-v0.86.0` |
| Testkit | 0.93.0 | `91bc66f4285f85d8584a21b27abbb783f4162901` / `testkit-v0.93.0` |

Both commits/tags reached `dragginzgame/ic-tooling:main`. Downloaded registry
archives match official sparse-index checksums and their exact clean release
commits. Private Dev/probe follow Testkit 0.93.0 and remain unpublished.
Foundation stays 0.75.0, Build 0.78.0, Text 0.74.2 and Native 0.80.0.

The released lock selects Platform 0.25.0, Runtime/macros/guard 0.20.0, Types
0.18.2 and Wire 0.17.12. Management DTOs remain exactly 0.11.0. Shared Tooling
remains 0.31.5, revision `b2026a118e38de4af1555c527af327cb604eacdf`.

## Consumer migration and ownership

For [#340](https://github.com/dragginzgame/ic-tooling/issues/340), import all five
Candid decoding allowances from `ic_host_tools::candid::decode::DecodeLimits`.
Tools converts its configuration privately to Platform. Match Testkit failures
with `ClientError::Cbor { kind, source }` and its `CborErrorKind`, replacing the
former supplier-error tuple variant. Original typed causes remain intact.
Ordinary configuration and error matching no longer require Platform identity
alignment; optional supplier downcasts retain their explicit coupling.

The [identity inventory](../extraction.md#public-supplier-identities) records
intentional shared boundaries. The optional ICRC ledger adapter still uses
Platform's `Value`. Tools remains the sole passive `LedgerTip` owner; Query's
verified-result migration stays in
[Query #118](https://github.com/dragginzgame/ic-query/issues/118).
No sibling source was edited. Codecs, protocol bytes, quotas, server custody,
deadlines and no-replay behavior retain their owners and contracts.

## Qualification

Both complete 34-stage Linux gates passed on Rust 1.99.1 with zero failures:
workspace Clippy, native/Wasm graphs, isolated features, installed PocketIC 16.1,
packaged rebuilds/tests/doctests and workspace installed tests. Testkit's gate
reused the two admitted unchanged evidence/retention fixtures. All other selected
stages ran through their normal qualification. Final ordinary-tree formatting
and locked metadata also passed after reconciliation.

The native matrix was explicitly dispatched at published commit `91bc66f4`:
[CI run 42392802957](https://github.com/dragginzgame/ic-tooling/actions/runs/42392802957).
It runs asynchronously and is not cancelled by subsequent routine pushes.
Linux, macOS 15 Intel and Apple Silicon hosted results are pending; no current
native macOS pass is claimed. #340 remains open for required native acceptance.
Do not poll solely to hold delivery open; revisit on an authorized maintenance pass.

The latest PocketIC adaptation also binds the earlier timing-fixture handshakes,
repairing their missing provenance update. The reconstruction and mutation guards
passed without changing donor/upstream fingerprints. Focused #340 evidence (49
cases, consumer contracts and typed causal chains) remains under
`target/public-boundaries-340/`.

## Frozen delivery evidence and custody

Captured checkout: `target/release-frozen/batch-GpvCzBKt/`. Sealed qualification
records and logs are beneath its `target/release-validation/`:

| Group | Gate / log directory |
| --- | --- |
| Tools | `tools-gate.9pGM30/logs/validation.DDNGL8/` |
| Testkit | `testkit-gate.4LUu75/logs/validation.AxRNv1/` |

Durable Git, plans, publication receipts and the completed schema-2 intent remain
under `.git/release-state/frozen/batch-GpvCzBKt/`. Outer logs, registry/VCS
readback, CI identity and reconciliation comparisons remain under
`target/deployment-tools086-testkit093-20261107/`.

## Ordinary checkout and deferred selections

Shared's reconcile helper fast-forwarded ordinary main through `91bc66f4`.
A clean three-way merge reapplied the user's crypto-common 0.1.6 and generic-array
0.14.9 selections, which were excluded from the release. Cargo.lock remains
dirty. Structured comparisons prove its external records equal the deferred
entry and its owned records equal the delivered Tools/Testkit versions.

The exact pre-release lock is durably preserved on local branch
`preservation/tools-0.86-testkit-0.93-lock-fa737230`, commit
`12e7a4ae515b607ede137a4603a940dba3fddd87` (parent `fa737230`).
Never restore an older backup over newer user edits. No artifacts were retired.

Earlier completed batches `batch-__7k7VD8` and `batch-vTGm2p2z`, old preservation
refs and prior recovery custody remain in the
[preceding handoff](https://github.com/dragginzgame/ic-tooling/blob/c86b9a27685b7bf81ad4580e951d3e312735b762/docs/status/current.md).
Do not resume failed historical attempts as new releases or reinterpret receipts.
Normal releases already publish; do not run a second `make publish`. Resume a
specific batch by ID, preserve deferred edits before reconciliation and obtain
explicit scope before retirement. See [publishing](../publishing.md),
[batch custody](../release-batches.md) and [validation](../design/release-validation.md).
