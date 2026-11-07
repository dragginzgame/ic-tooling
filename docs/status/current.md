# Current handoff

## LOC hard cut — local, 2026-11-07

The approved cleanup removes unused PocketIC Candid convenience exports,
default-ID lookup, server-filesystem snapshot APIs/DTOs, no-tick/authenticated-status
helpers, blockmaker ticks, subnet deletion and seven topology shortcuts. Full
topology DTOs, used lookups, private management codecs, protocol bytes and custody
remain. Six timing records expose documented fields in place of 44 getters;
workspace callers/guides are migrated. Cache fixtures share setup and client fault
fixtures reuse bounded Host readers while retaining binary bodies and replay checks.
Testkit drops direct backoff and three unused Reqwest features. No aliases are added.

The approved #338/#339 implementation requires explicit positive finite `Duration`
budgets for client creation, attachment and listing. Zero/clock-overflowing values
refuse before requests; one deadline includes bodies, polling and pruning. Recipes
now select `snapshot_funding()` and validate their own reset before returning unit.
Generic reset schemas/receipts are removed; lifecycle, funding order, invalidation,
typed recovery and collect-all shared shutdown remain. Workspace callers and
current guides are migrated without compatibility aliases.

Authorized delivery selects Build 0.77.0 then Testkit 0.90.0 in one captured
`build,testkit` minor batch. Source integration precedes capture; the runner owns
version preparation from the current 0.76.0 / 0.89.4 manifests. Unrelated lock
updates are preserved separately from the release graph.
Canonical Rust LOC falls from 81,778 to 81,164: production 31,475 to 30,798 and
tests 47,511 to 47,586. The latest #338/#339 slice removes 196 Rust lines overall.
Earlier evidence remains under `target/{loc-cut,polling-cut,topology-cut}-20261107/`;
current evidence is `target/hard-cut-338-339/`.
Strict Testkit/Dev all-target/all-feature Clippy passes. Focused Linux checks pass:
27 client cases, reset mismatch/domain/recovery/shutdown cases, cancellation,
funding order, startup URL admission/doctest, management identity and installed
management/snapshot state on admitted PocketIC 16.1. Native/Wasm graph and donor
reconstruction/tamper guards pass. Earlier Build lint/cache/timing evidence is
unchanged. Native macOS and full release qualification were not rerun.
The new checksummed `loc-surface-cut.patch` preserves original donor fingerprints.
The fixture's first test attempt exposed binary blob bodies; corrected handling
and passing reruns retain that diagnostic. The follow-on default-library check,
native/Wasm dependency profiles and client contracts pass with narrower Reqwest
features. Cargo.lock preserves every pre-existing external selection; the only
new changes remove Testkit's backoff edge and unused MIME dependencies. The entry
backup and selection comparison are in `target/polling-cut-20261107/`; the current
lock SHA-256 is `46b8c95e13d20f536c71212430a93fe8e18d29aea2dfa2a04915431c07b770df`.
The maintainer chose consumer issues instead of concurrent sibling migrations.
Eleven owning issues are linked from
[#339 — Explicit finite request budgets](https://github.com/dragginzgame/ic-tooling/issues/339);
[Canic #738 — Reset ownership and finite budgets](https://github.com/dragginzgame/canic/issues/738)
also owns #338 adoption. Prepared sibling edits were withdrawn with concurrent
source repairs and lock changes preserved; no Cargo overrides or consumer checks
ran. Restoration evidence and original diffs remain under `target/hard-cut-338-339/`.
Both Tooling issues remain open for integration; consumer qualification stays with
their owners. Toko/Toko Miner and independent native Profiler were excluded.

## Shared 0.31, Build 0.76.0 and Testkit 0.89.4 delivered — 2026-11-06

Source integration `57520808` adopts Shared Tooling 0.31.0 revision
`74313941af2fce4399bec0e6909ca6229673c3f8`; all 175 payloads match that reviewed
GitHub commit. Local settings use `RELEASE_GROUPS`, `RELEASE_PUBLISH_TARGET`,
`RELEASE_SETUP_TARGET`, `RELEASE_SETUP_MODE` and `RELEASE_BATCH_ACTIVE`.
Retired settings refuse explicitly. New exports require version/entrypoint
annotations; schema-2 batches require a recorded publication commit.

| Group | Published version | Release commit / tag |
| --- | --- | --- |
| Build | 0.76.0 | `804aeaafbbcbef4d36fd4945c833a5045e90b6c5` / `build-v0.76.0` |
| Testkit | 0.89.4 | `387b9694f917df60e87309350427827d0a2dd6c7` / `testkit-v0.89.4` |

Both commits/tags are pushed to `dragginzgame/ic-tooling:main`; both crates are
published to crates.io. Private Dev/probe follow Testkit and remain unpublished.
Other groups remain Foundation 0.74.4, Text 0.74.2, Host Tools 0.83.0 and Native
0.80.0. Mixed increments used two sequential batches: Build minor, Testkit patch.

Build now owns one `CacheBatchEntry<T, E>` and one named Cargo fixture writer.
The public entry/signature hard cut and four-field consuming tuple are documented
in the [artifacts guide](../build/artifacts.md). Wasm retains its full failure pair.
Testkit shares empty-canister fixtures and one routing helper for 16 management
calls, with unchanged public signatures, DTO identity and fallible/no-replay behavior.
Original PocketIC donor hashes remain intact; a checksummed patch records routing.
Focused release package identities come from the group catalog; one coverage policy
owns reverse-consumer contracts. Keep the two cache engines and domain reports:
Wasm's Cargo state, source leases and target lock order differ from strict generic
staging/publication. Forcing convergence would add policy exceptions.

## Qualification and publication evidence

Both selected groups passed all 34 complete Linux release stages on Rust 1.99.1:
strict workspace lint, source/provenance guards, fresh packaged targets/docs,
native/Wasm graphs, isolated features and installed PocketIC 16.1 contracts.
The prior focused checks and LOC comparison remain under
`target/complexity-reduction-20261106/` (93 fewer Rust lines: 59 production, 34 tests).
Shared consumer routing, installed-hook interruption/recovery and publication dry
runs passed before delivery; evidence is under `target/shared-tooling-0.31.0/` and
`target/tmp/frozen-release.Jvk9sp/`. Substitute fixture gates are separate from
these two production batches' complete gates and real uploads.

| Group | Batch | Captured gate logs beneath its checkout |
| --- | --- | --- |
| Build | `batch-HfJiONwy` | `target/release-validation/build-gate.RYdCfe/logs/validation.p34aG5/` |
| Testkit | `batch-xfnG3o_V` | `target/release-validation/testkit-gate.7km6MW/logs/validation.kndc0a/` |

Captured checkouts remain at `target/release-frozen/<batch>/`; durable intents,
Git and plans are under `.git/release-state/frozen/<batch>/`. Both schema-2 intents
are complete and retain their exact publication commits. Publication admissions
are `target/publish/admission/publish.Z8UGKI/` (Build) and
`target/publish/admission/publish.id84Pq/` (Testkit), beneath their respective captures.
Registry metadata matches each prepared archive checksum and clean Git provenance:
Build `e46dc1d452989a7a9637fee92652e73b0776093c4e3f528e788921dfb26476a5`;
Testkit `cc21437fbd55ba1158f4a0dd56a372e2ebc04f43a3552bf519f990a5a235ee14`.
Outer logs/readback: `target/deployment-build-0.76.0-testkit-0.89.4/`.

The [retained native matrix](https://github.com/dragginzgame/ic-tooling/actions/runs/42277487160)
is dispatched asynchronously at `testkit-v0.89.4`, independently of later doc pushes.
It covers Linux x86-64 and macOS 15 Intel/Apple Silicon. No new native macOS pass
is claimed here. Testkit's first preflight stopped because OS package updates
changed its native tool environment during setup; no release mutation had begun.
The same saved batch resumed after updates finished and completed qualification.
Its failed setup attempt and diagnostics remain preserved.

## Ordinary checkout and retained recovery

Shared reconciliation fast-forwarded the ordinary checkout through both releases.
Unrelated lock selections were preserved first on
`preservation/build-0.76.0-testkit-0.89.4-lock-9fc6b9c2`, then after Build on
`preservation/testkit-0.89.4-lock-a8e57101`, and deliberately reapplied.
The restored lock SHA-256 is
`3d0b6a8e109e54512ac27bd2b2b6ee04e7991fb1380288bf1708dfc91ac0c58b`.
Only the owned Build/Testkit versions differ from the original saved lock;
external selections remain deferred and were not included in these releases.
Backups, narrow patches and reconciliation transcripts remain under
`.git/release-state/deployment-build-0.76.0-testkit-0.89.4/`. Never restore older
backups over later live updates implicitly. No artifacts or batches were retired.

Historical ordinary-checkout batches `batch-UqiVuh8F` and `batch-c3bPF1yB` are
complete schema-1 records. Preserve them unchanged; historical recovery requires
their original reviewed tooling. Earlier .89.3/.89.2 lock snapshots, preservation
refs and activation-stash custody remain in the [previous handoff](https://github.com/dragginzgame/ic-tooling/blob/57520808/docs/status/current.md).

Normal releases capture committed source and qualify, commit/tag/push and publish.
Do not repeat `make publish` after a successful default release. Select one
supplier-first roster per increment; resume by batch ID. Reconciliation requires
a clean checkout and fast-forwards only. Retirement requires separate authority.
See [publishing](../publishing.md) and the [batch contract](../release-batches.md).
Testkit retains caller-owned runtimes and explicit teardown; pool cancellation
and deletion recovery are in the [migration](../testkit/client-migration.md).
II attachment/cleanup remains in the [consumer guide](../../testing/browser/internet-identity/README.md).
Tools/Build narrowing still requires the matched native-host comparison in the
[qualification contract](../design/release-validation.md); unknown/shared inputs
retain the full gate. GitHub issues remain the sole follow-up tracker.
