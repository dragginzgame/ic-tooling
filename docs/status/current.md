# Current handoff — 2026-11-02

## Issue delivery

The reviewed batch contains the accepted Build/benchmark consolidation and
[#314](https://github.com/dragginzgame/ic-tooling/issues/314)'s group release plans.
Build shares standalone/batch input discovery and hashing, verification decisions
at the existing three boundaries, and configured shared-target resolution.
The private benchmark shares typed workload/concurrency admission without changing
its CLI, report bytes, measurement boundaries or cooperative cleanup.

Release validation uses the existing Shared runner, one policy stage inventory
and one baseline admission predicate. Foundation, Build, Tools, Testkit and
uncertain/shared changes retain all 35 original obligations. Native/Text can
narrow only from a complete admitted three-host baseline with unchanged shared
inputs and reviewed consumers. Version-3 receipts bind selection, context and
executed/excluded evidence; historical version-2 recovery uses its original source.
The exact archive driver can select public packages, retaining its full default
roster. Dependency-free archives qualify while empty graphs and altered external
selections still refuse. See the [mapping](../design/release-validation.md).

Native CI keeps the complete gate and then runs both focused profiles on the same
source, lock, compiler and host. Each host retains full-gate elapsed time, focused
commands/logs/timings and installed-archive evidence without compilation caches.
Baseline admission requires those actual checks and uploads on Linux x86-64,
macOS 15 Intel and Apple Silicon. Older green CI without them selects the full
gate. No measured release-time saving is claimed; CI remains asynchronous.

[#313](https://github.com/dragginzgame/ic-tooling/issues/313)'s published Rust 1.99.1
adoption is verified at `fc66f67b241af549034ac993c1254b46ab378934`:
[CI 41521867036](https://github.com/dragginzgame/ic-tooling/actions/runs/41521867036)
and optimizer/runtime/browser workflows passed. Downloaded qualification records
from all three native hosts contain compiler 1.99.1, that source and completion.
The stale current-support 1.91 sentence is corrected; the pinned published editor
retains its separate historical floor.

## Validation and evidence

Unchanged product source retains focused Linux qualification: Build/Dev Clippy,
204 Build artifact tests, 26 benchmark tests, 15 archive tests and three archive
driver tests. Real Native/Text lint, graphs, tests, docs and extracted archives
passed. The six-group adapter exercised real metadata, rollback, receipts and
commit custody with qualification effects substituted. Plan failure/recovery,
Make routing, lifecycle retention, snapshot integrity and document checks passed.
The actual workflow bodies also pass required-failure propagation fixtures.
New native macOS execution of this batch belongs to its hosted CI run; the older
native records above establish #313, not the new #314 comparison.

Evidence and incoming dirty snapshots remain under:
- `/tmp/ic-tooling-local-consolidation.oIR89O/` — original four refactors.
- `/tmp/ic-tooling-314.6kSCkb/` — group plans, adapter and installed archive proofs.
- `/tmp/ic-tooling-audit-cleanup.fFoBMf/` — policy consolidation and snapshot trim.
- `/tmp/ic-tooling-issue-delivery.ABbnjm/` — issue delivery, native records and checks.

Shared remains 0.27.27 at `651f5f7acb2e3055abd51ba5f835a2112801666a` with
159 managed files. The supported exporter retired three unused producer fixtures;
local consumer qualification remains. No product artifact cleanup occurred.

## Published state

| Group | Version | Release source |
|---|---|---|
| Foundation | 0.74.2 | `aa62709bff680ac7c8a050c88cff769c818eebf2` |
| Build | 0.74.1 | `aeff79aa1b6d3e6466b7f990b679db9379215466` |
| Tools | 0.78.1 | `17b79dc82fa10da41afa455d9c3c9f2540a41bbe` |
| Testkit | 0.80.2 | `fc66f67b241af549034ac993c1254b46ab378934` |

Text 0.74.0 and Native 0.80.0 remain unchanged. Pending notes select compatible
Build 0.74.2 and Testkit 0.80.3; package versions, release tags and registry uploads
are unchanged by this source delivery. The unrelated Cargo.lock selections remain
local and byte-preserved, outside the delivery commits. Earlier deployment proof
remains under `.git/release-state/deployment-cleanup-20261102/` and
`/tmp/ic-tooling-cleanup-deploy.VLiO5k/`.

## Supplying-owner follow-ups

[Platform #134](https://github.com/dragginzgame/ic-platform/issues/134) owns typed
client failures; [#136](https://github.com/dragginzgame/ic-platform/issues/136) owns
bounded snapshot transfer. Client 0.18.0 remains the newest published version in
the latest registry readback; Testkit keeps its adapters until replacements ship.
No sibling source/release or DFINITY/PocketIC issue was changed. The private
checked-average exception remains in AGENTS.md, coordinated through
[Shared #347](https://github.com/dragginzgame/shared-tooling/issues/347) and
[Metrics #127](https://github.com/dragginzgame/ic-observability/issues/127).
