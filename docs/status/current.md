# Current handoff

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
