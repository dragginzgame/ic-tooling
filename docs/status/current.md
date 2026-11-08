# Current handoff

## Build, Text and Testkit cleanup deployed — 2026-11-08

The authorized cleanup is committed, pushed to main and published to crates.io.
Implementation source is `e192870a0ef0959dbf2d0280af4cb0d0c95075d6`, including
Text `441900ba`, consolidation `fe01d255` and the server-setup repair `e192870a`.

| Group | Published version | Release commit / annotated tag |
| --- | --- | --- |
| Build | 0.79.0 | `e3e514f09bc45b3ebf6c2d1cb8e2d4e8973554f4` / `build-v0.79.0` |
| Text | 0.74.3 | `439ead576bfac6b2d235d5e22de2993c47e290ee` / `text-v0.74.3` |
| Testkit | 0.93.1 | `f494ea2d08e14959ed0e067742839385d05a3f62` / `testkit-v0.93.1` |

Frozen Build batch `batch-b1n6Stqb` completed first, followed by the supplier-first
Text/Testkit patch batch `batch-XbEtnv9Z`. Both journals report complete Git and
registry delivery. Official sparse-index checksums match all three downloaded
archives; embedded clean VCS identities match their exact release commits.
Private Dev/probe follow Testkit 0.93.1 and remain unpublished. Foundation stays
0.75.0, Host Tools 0.86.0 and Native 0.80.0. The released lock retains Platform
0.25.0, Runtime/macros/guard 0.20.0, Types 0.18.2 and Wire 0.17.12. Shared remains
0.31.5 at `b2026a118e38de4af1555c527af327cb604eacdf`.

### Delivered behavior

Build removes `WasmBuildBatchConfig` and its batch argument. Each
`WasmBuildSpec` owns maintenance configuration: matching policies run once per
canonical target, and conflicting entries fail before building. Artifact/Wasm
outcome names specialize `CacheOutcome<Record>`; labeled-spec domain names
specialize `LabeledSpec<S>`. Domain engines, records and retention remain distinct.
The [migration](../build/artifacts.md#pending-build-079-migration) and
[Canic #742](https://github.com/dragginzgame/canic/issues/742) record the consumer
API change; no Canic source was edited here.

Text publishes both compatible additions from
[#341](https://github.com/dragginzgame/ic-tooling/issues/341) and
[#342](https://github.com/dragginzgame/ic-tooling/issues/342):
`quantity::scaled_decimal_quantity_text` renders arbitrary-size exact decimal
quantities with rounding/promotion and escaped suffixes;
`percent::ratio_percent_text` renders exact `u64` ratios without floating point.
Query can adopt 0.74.3 while retaining its unit tables and fallback policy.
No Query source was changed.

Testkit shares ordinary/snapshot ingress transport mechanics and private native
HTTP fixture-worker custody. Public APIs, protocol bytes, limits, caller runtime,
cancellation/no-replay behavior and fixture-specific responses remain unchanged.
Donor/upstream reconstruction and mutation guards passed. PocketIC setup now
uses the official GitHub asset API with fixed asset IDs and the unchanged digest,
size, HTTPS and executable checks. The browser-download endpoint returned HTTP
504 twice; the official API returned the identical pinned Linux archive.

Release fixtures derive group identities from the catalog. Four unchanged
workflow/scratch/evidence/retention fixtures use the existing admitted-stage cache;
Testkit actually reused all four from Text in this deployment. Metadata-dependent
fixtures remain fresh. Publication obtains context from Tooling's preflight
selection and keeps fresh lint fallback, archive and provenance checks. The real
publisher refused lint reuse on saved-context mismatch and successfully ran fresh
workspace Clippy for each upload. [#345](https://github.com/dragginzgame/ic-tooling/issues/345)
retains that efficiency defect; substituted fixtures did not expose it.

### Qualification and custody

All three complete 34-stage Linux x86-64 release gates passed on Rust 1.99.1:
workspace Clippy, formatting/docs, source/release fixtures, isolated feature and
native/Wasm graphs, installed PocketIC 16.1 contracts, archived rebuilds/tests/
doctests and installed workspace tests. Publication separately passed its source,
lint, package verification and upload checks.

Native macOS Intel and Apple Silicon qualification is **pending**, not inferred
from Linux. Four workflows were dispatched asynchronously at the exact published
source `f494ea2d08e14959ed0e067742839385d05a3f62`; all read back in progress:
[CI](https://github.com/dragginzgame/ic-tooling/actions/runs/42469752240),
[Build optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/42469755112),
[Testkit runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/42469758176)
and [Browser fixture session](https://github.com/dragginzgame/ic-tooling/actions/runs/42469761028).
Delivery did not wait for their completion. Issues requiring native evidence
remain subject to their own acceptance criteria.

The ordinary main checkout was reconciled through Shared's batch helper after
each delivery. Its unrelated Cargo.lock selections were reapplied by three-way
merge; all deferred registry package records compare exactly with the latest
saved draft, while workspace records carry the published versions. The latest
deferred selections include Platform 0.25.1, Runtime/macros/guard 0.20.1 and Wire
0.17.13; they are **not** the captured release lock. Current deferred lock SHA-256
is `607b4ce0e9f387559b9a2098a8e2fd91c367e54d4b85f38304e5b75bc5a1c95c`.

Durable preservation refs are `preservation/cleanup-release-lock-20261108`
(`7021230f`) and `preservation/cleanup-release-lock-20261108-updated`
(`7fee0a54`). Batch Git, plans and receipts remain under
`.git/release-state/frozen/`; candidate outputs and sealed proofs remain under
`target/release-frozen/`. Readbacks, locks, reconciliation logs and registry
verification remain in `target/deployment-cleanup-20261108/`; earlier focused
logs remain under `target/complexity-four-20261108/` and
`target/complexity-five-20261108/`. Superseded `batch-SeyigorP` retains both failed
preflight downloads, without tag/upload effects. No batch, tool, artifact or
sibling checkout was retired.

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
