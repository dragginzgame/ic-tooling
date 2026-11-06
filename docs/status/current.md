# Current handoff

## Deployment selected — 2026-11-06

The maintainer authorized Git delivery and crates.io publication using Shared 0.31.
Deliver Build 0.76.0 (minor), then Testkit 0.89.4 (patch). Mixed increments require
two sequential captured batches; retain complete qualification for each selected
group. Hosted native CI remains asynchronous and is not a Linux qualification.
Save unrelated lock selections on a durable preservation branch before capture;
reconcile completed batches with Shared and deliberately reapply those selections.
Do not retire historical batches or artifacts as part of deployment.

## Complexity reduction — local, 2026-11-06

Five cleanups are implemented: named Cargo fixture writing, shared empty-canister
test setup, one `CacheBatchEntry<T, E>`, catalog-derived release package/consumer
selection, and one private routing helper for 16 management call sites.
Build's entry/signature hard cut is documented for pending 0.76.0; Testkit's
compatible cleanup is pending 0.89.4. Manifest versions remain unchanged.
The implementation is ready for the separately authorized deployment above.

Keep the two cache engines: generic transactions enforce declared staging outputs
and rename publication; Wasm caching retains Cargo state, source-lease invalidation
and shared-target lock ordering. They already share filesystem/hash primitives.
Combining them would add policy exceptions. Reports also retain distinct metrics;
only their duplicate entry storage/accessors are unified. The artifacts guide
documents the four-field consuming tuple and full Wasm failure pair.

Focused Linux checks passed: 21 batch tests, eight changed Cargo/cache fixtures,
19 client failure/no-replay cases, management DTO identity, strict Clippy across
Build/Testkit/Dev all targets/features, release-selection refusals and PocketIC
source reconstruction/tamper guards. Against admitted PocketIC 16.1, pool and
concurrency tests passed (26 cases), as did installed management/snapshot state.
No full release gate or new native macOS qualification is claimed.
Logs and canonical LOC comparison are in `target/complexity-reduction-20261106/`.
Rust code fell from 81,871 to 81,778 lines: production 31,534 to 31,475; tests
47,545 to 47,511. The new generic entry file is included in those counts.
The original donor hashes remain intact; a checksummed routing patch records the
new adaptation. No lifecycle, protocol, DTO pin or crypto change is intended.

## Shared Tooling 0.31.0 adoption — local

The requested refresh adopts GitHub revision
`74313941af2fce4399bec0e6909ca6229673c3f8` through the exporter; all 175 payloads
match that independently selected commit. Local settings now use `RELEASE_GROUPS`,
`RELEASE_PUBLISH_TARGET`, `RELEASE_SETUP_TARGET`, `RELEASE_SETUP_MODE` and
`RELEASE_BATCH_ACTIVE`; the runner/guide use release-batch naming. Retired roster
settings and empty overrides refuse before dispatch. Shared owns automatic capture,
schema-2 publication identity and checksum verification; local setup/publication
owners remain unchanged. New exports require version/entrypoint annotations.
Focused Make routing, snapshot/source comparison, pins and ShellCheck pass.
Installed hooks passed interruption/uncertain-publication recovery, two archive
dry runs, completed resume and remote refusal; product gate bodies were substituted.
Evidence: `target/shared-tooling-0.31.0/` and `target/tmp/frozen-release.Jvk9sp/`.
The first attempt stopped on a temporary tool-store lock; its diagnostics remain.
Both retained ordinary-checkout batches are complete schema-1 records. Preserve
them unchanged; any historical recovery requires their original reviewed tooling.
The adopted interface is qualified for the deployment above. At its start,
crate versions, tags, publication and unrelated Cargo.lock bytes are unchanged.
The completed .9 release below remains separate.

## Published baseline and retained recovery

Shared 0.30.9 source integration is `a36baa84`. Testkit 0.89.3 is published
from clean release commit `5d9cdbf8f0bfcfc970adf338eec5a571ee12be69`, with
`main` and annotated tag `testkit-v0.89.3` pushed to `dragginzgame/ic-tooling`.
Private Dev/probe follow Testkit and remain unpublished. Other groups are unchanged.

| Group | Published version |
| --- | --- |
| Foundation | 0.74.4 |
| Build | 0.75.0 |
| Text | 0.74.2 |
| Host Tools | 0.83.0 |
| Native | 0.80.0 |
| Testkit | 0.89.3 |

Actual batch `batch-UqiVuh8F` completed from
`target/release-frozen/batch-UqiVuh8F/`; durable Git, intent and plans remain under
`.git/release-state/frozen/batch-UqiVuh8F/`. All 34 Linux release stages passed on
Rust 1.99.1, including fresh archives, graphs and installed PocketIC contracts.
Captured gate logs: `target/release-validation/testkit-gate.6BYBIS/logs/validation.MdGIrz/`;
publication receipts: `target/publish/publish.Op8Tkp/`; registry-only contracts:
`target/published-integration.Nx50BZ/`. Registry readback matches clean Git/archive
provenance, checksum `df8ce0ea709c2b8b747274540b230a42e5bc39348e71046383189b2005d1695e`.
The native matrix was [dispatched asynchronously](https://github.com/dragginzgame/ic-tooling/actions/runs/42256140504);
no new native macOS pass is claimed here.

Shared reconciliation fast-forwarded the ordinary checkout. Transcript:
`.git/release-state/deployment-testkit-0.89.3/reconcile.log`. Deferred lock edits
were preserved on `preservation/testkit-0.89.3-lock-61aa17c8bc54` and reapplied.
The unrelated dirty lock remains at SHA-256
`9fc6b9c27993007fbe3d42594fea8be23bd3cf9ac0d86e7fafd1f82611138288`.
The deployment's `deferred-latest` identifies the saved lock; never restore older
backups implicitly. No batches, artifacts, pins or recovery records were retired.
Retirement needs separate authorization. Legacy plans retain original identities.

Normal releases capture committed source and qualify, commit/tag/push and publish.
Do not repeat `make publish` after a successful default release. Select one
supplier-first roster; recover by batch ID. Git-only delivery requires clearing
`RELEASE_PUBLISH_TARGET`. Shared owns setup, resume and reconciliation;
see the [publishing guide](../publishing.md) and [frozen contract](../release-batches.md).

## Maintained contracts and evidence owners

Testkit is async-only, with caller-owned runtimes and explicit teardown.
`CachedPocketIcBaselinePool::shutdown(&self)` waits for leases and reports all
attempted deletion failures without replay. Unattempted baselines survive
cancellation; in-flight deletions require caller reconciliation. Earlier pool
proof: `target/evidence/pool-shutdown.lGjQv6/`. See [migration](../testkit/client-migration.md).
II attachment/cleanup belongs to the [consumer guide](../../testing/browser/internet-identity/README.md);
earlier browser acceptance: `target/evidence/ii-testkit-0.89.1-001/`.
Tools/Build narrowing requires the matched native-host comparison in the
[qualification contract](../design/release-validation.md); uncertain/shared changes
retain the full gate. GitHub issues are the sole follow-up tracker.
Detailed earlier release/recovery evidence remains in the
[committed handoff](https://github.com/dragginzgame/ic-tooling/blob/2b5e39e4/docs/status/current.md),
including the .89.2 deployment snapshots and original activation-stash custody.
