# Current handoff

## Five further simplifications — 2026-11-08

Implemented locally, without staging or delivery. Pending Build remains
**0.79.0** for the already documented batch API cut; its two labeled-spec domain
names now share `LabeledSpec<S>`. Pending Testkit is **0.93.1**: ordinary/snapshot
ingress calls share transport helpers with unchanged public APIs and protocol
contracts. Three HTTP fixture families share private, native test-worker custody;
responses, framing decisions, byte budgets and replay assertions stay local.

Publication now obtains release context from Tooling's actual preflight selection
and checks it against the sealed committed-release receipt. It no longer parses
Shared's private journal or fabricates a journal in its fixture. Old/missing or
changed context falls back to fresh publication lint. Group admission and fixture
iteration derive identities from the catalog; qualification mappings stay explicit.

Focused Linux/Rust 1.99.1 validation passed: Build/Testkit library/test Clippy,
55 Rust cases (23 client/snapshot, four async startup, eight time, 13 Wasm batch,
seven artifact batch), Build-only context/receipt and catalog rejection/recovery,
publication wiring/refusals, Make tooling-roster routing, ShellCheck and donor/
upstream reconstruction plus mutation guards. The adapter used the existing
cached pinned 0.73 renderer with setup/gate effects substituted. Logs remain at
`target/complexity-five-20261108/`. Package versions and dependencies are unchanged;
Cargo.lock and the separate Text draft are preserved. Native macOS and full release
qualification were not run. No Shared source, sibling callers or artifacts were retired.

## Four Build/release simplifications — 2026-11-08

Implemented locally; nothing staged, committed or deployed. Pending Build is
**0.79.0**, because removing `WasmBuildBatchConfig` and its batch argument breaks
the public API. Configure maintenance on each `WasmBuildSpec`; matching policies
run once per canonical target, while conflicting entries fail before building.
Artifact/Wasm outcome names now specialize one `CacheOutcome<Record>` owner;
records, engines and retention custody remain distinct. See the
[migration](../build/artifacts.md#pending-build-079-migration) and
[Canic #742](https://github.com/dragginzgame/canic/issues/742). No Canic source was edited.

The existing stage runner now reuses workflow/scratch matrices alongside its
evidence/retention fixtures. Current group metadata and metadata-dependent
release/publication fixtures remain fresh. Publication re-admits full workspace
lint through the existing committed-release receipt, with fresh lint fallback;
archive/license/provenance/source admission stay fresh.

Focused Linux/Rust 1.99.1 checks passed: Build all-target/all-feature Clippy,
13 batch tests, ten maintenance tests (three overlap the batch selection), four
cross-process artifact-handoff tests, cross-group stage replay/invalidation,
Build-only adapter receipt admission/refusal/recovery, publication wiring and
fallback/archive refusal, ShellCheck with source following, and local doc links.
Release fixtures substitute setup, qualification and external effects; the
adapter used the cached pinned 0.73 renderer, without claiming current native
tool-store admission. Logs: `target/complexity-four-20261108/`. Native macOS and
full release qualification were not run. Package versions are unchanged; the
entry Cargo.lock remains byte-for-byte intact. The independent Text work below
is preserved.

## Exact quantity and percentage formatting — 2026-11-08

Implemented [#341](https://github.com/dragginzgame/ic-tooling/issues/341) and
[#342](https://github.com/dragginzgame/ic-tooling/issues/342) locally in Host Text.
The pending Text version is **0.74.3**: both APIs are compatible additions to
0.74.2. Package versions remain unchanged; nothing is committed or published.

- `ic_host_text::quantity::scaled_decimal_quantity_text(value, units)` returns
  `Option<String>`. Tables use the existing descending `(u64, &str)` divisors,
  restricted to powers of ten ending in one. It reuses the existing exact decimal
  renderer, validates all input, supports values beyond `u128`, rounds half up to
  two places, trims fractional zeroes and promotes rounded units. Caller suffixes
  remain terminal-escaped. The existing integer/binary formatter is preserved.
- `ic_host_text::percent::ratio_percent_text(numerator, denominator)` returns
  `Option<String>` for `u64` inputs. It uses wide integer arithmetic, rounds exact
  halves upward to one decimal place, admits percentages above 100 and refuses
  zero denominators.

After publication, Query can replace its decimal rate selection/promotion with
the first helper using its existing cycle unit table. It retains original-input
terminal-safe fallback. Query can replace ratio arithmetic with the second
helper and retain its `"-"` fallback, checked count addition and report ownership.
No Query source was changed; its adoption waits for the published API.

Focused Linux x86-64 / Rust 1.99.1 validation passed: affected-package Clippy
with warnings denied, 44 Host Text unit tests and five public API doctests, all
with defaults disabled and the existing lock held. Regression cases include
rounded promotion, irregular decimal scale gaps, arbitrary precision, late
malformed suffixes, terminal-control suffixes, exact percentage halves and full
integer ranges. Production dependencies remain only `unicode-width`; no new
dependency, feature or policy owner was introduced. `git diff --check` passed.
Native macOS Intel/Apple Silicon acceptance and release qualification were not
run. The pre-existing Cargo.lock draft is byte-for-byte unchanged (SHA-256
`3499ed042483ec4f068bfc9aed770cebff7c616752a66feaf40586f39faf8005`).

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
