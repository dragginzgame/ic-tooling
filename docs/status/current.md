# Current handoff — 2026-11-10

## Delivery

The cache reductions, Wire adoption and tooling refresh are committed and
published. Latest product release commit:
`6f4128ae2ecc7c4fe25814cd7c1ccc48c8cc17ec`. Both frozen batches were reconciled
into the ordinary main checkout.

| Group | Published | Release commit |
| --- | --- | --- |
| Foundation (five packages) | 0.76.4 | a1b50ffc |
| Build | 0.82.0 | ea58d4b9 |
| Text | 0.74.5 | f0352237 |
| Host Tools | 0.89.2 | fe337366 |
| Testkit | 0.99.0 | 6f4128ae |

All nine versions were verified present and not yanked in the crates.io index.
Private qualification packages remain 0.96.0. The deployment preserved selected
external dependencies; release preparation changed owned versions and their
product/qualification references together. The release uses Toko Tooling 0.39.9
and Wire 0.20; the product Platform requirement remains 0.29 (locked to 0.29.2).

## Delivered reductions and adoption

Toko Tooling 0.39.9 is selected in both workspaces and installed through the
authenticated crates.io exporter, preserving the 113-file runtime selection.
That refresh changed only its lock entries; the pre-existing product
`rustls-pki-types` 1.16.0 selection remains intact. Tool catalogs are unchanged.
The 0.39.9 checker admits native PocketIC dev-dependencies for simulator tests.
Focused adoption evidence remains under `target/toko0399/` and `target/toko0398/`;
the complete release gates now qualify the delivered batch on Linux.

Wire 0.20 is adopted in both workspaces and published through Testkit 0.99.0.
The minor release includes public adapter identities and upstream Postcard
acceptance changes. Fourteen focused tests cover nine adapter
cases, live native replies, type identity, probe budgets and installed Wasm
requests/state/refusals. Graph policy, portable compilation, selected warning-denied
lint, formatting and snapshot/pin/doc admission pass. Focused logs remain under
`target/wire020/`; native macOS qualification is separate below.

Build 0.82.0 makes shared Cargo targets the only compilation path and makes
shared-target maintenance explicit. The full input digest owns both cache
identity and mutation validation. The separate validation digest
and receipt field are removed. New fingerprint keys preserve old cache entries;
unrelated workspace manifest/lockfile edits can invalidate exact reuse. The
[Build guide](../build/artifacts.md#build-082-migration) records caller migration.
Focused Linux cache/batch/evidence and Cargo-input transaction tests, real Cargo
Wasm/examples/artifact-handoff/process-lock tests, 103 durable-filesystem tests,
warning-denied Build Clippy/rustdoc, formatting and document links pass. Logs and
before/after Rust counts remain under `target/loc-audit/`.

Build 0.82.0 also makes exact-cache pruning explicit.
Acquisitions no longer schedule pruning or attach nonfatal maintenance results,
timings or phases. Callers invoke the existing prune functions and retain their
records to protect live outputs; old exact-cache schedule markers remain untouched.
Focused cache, transaction/batch and pruning tests, cross-process handoff, real
Cargo Wasm acquisition, warning-denied Clippy/rustdoc and document links pass on
Linux. Evidence is under `target/loc-audit/explicit-pruning/`. This continuation
removed another 253 Rust lines. The two reduction passes removed 1,631 Rust lines
overall, leaving 68,768 across both workspaces: 25,609 library/application,
42,170 tests, 500 examples, 328 tooling and 161 benchmarks. These are source
counts, not measured executable or Wasm savings.

## Current contracts

Testkit 0.98.0 replaces the maintained fork with published `pocket-ic` 16.1.
Import its async client, builder and native DTOs directly. Harness helpers are
free `pic` functions; native IDs use upstream identities, while portable helpers
retain Platform Types Principal. There is no compatibility facade. Upstream
panics, rejection types and best-effort deletion replace fork transport errors;
close/shutdown return `()`. Ordinary snapshots replace bounded transfer, with
timer/hook reset owned by callers. See the [migration](../testkit/client-migration.md).
Sibling caller migration is separate from this repository's deployment.

Default/Wire Testkit excludes Platform; only held fixtures select its adapters.
Client dependencies remain native and the Wasm graph remains Types-only.
The delivered cleanup also shares Linux stat parsing, streams typed package
inventories, preserves probe/cleanup failures, simplifies response decoding and
uses semaphore-owned pool closure. Group ledgers own the release descriptions.

All eleven Rust packages inherit the canonical root `LICENSE.md`; the two
private Node manifests also resolve that root file. Source/archive publication
checks admit the canonical bytes. Release fixtures now declare the checker and
root-file inputs, fingerprint them for reuse and omit retired reference trees.
The 113-file Toko snapshot retains runtime inputs and canonical root files;
shared reference documentation is linked at its recorded upstream revision.

## Qualification

All five complete Linux release gates passed, including warning-denied lint in
both workspaces, installed PocketIC, extracted-package tests, native/Wasm graphs,
Wasm compilation, documentation and workspace tests. Cargo also verified each
published package. Native macOS 15 Intel and Apple Silicon qualification remains
separate and asynchronous. The manual [release-tag qualification run](https://github.com/dragginzgame/ic-tooling/actions/runs/42895053349)
is in progress at this handoff; no native macOS pass is claimed.
Cancelled or superseded hosted runs are not passes.

Completed batches are `batch-4hupb2d0` (Foundation, Text and Tools patches) and
`batch-UEWDKToB` (Build and Testkit minors). Their source-bound plans, gate logs and upload
receipts remain under `.git/release-state/frozen/`; managed checkouts remain under
`target/release-frozen/`. Normal-checkout logs are `target/deploy-patches.log` and
`target/deploy-minors.log`; independent registry observations are under
`target/deploy-registry-20261110/`. Reconciliation logs are
`target/deploy-patch-reconcile.log` and `target/deploy-minor-reconcile.log`.

## Custody and retained decisions

Retain all completed/failed batches, refs, plans, receipts and artifacts; no
deployment cleanup was performed. Earlier completed batches `batch-HIlzE9l1`
and `batch-hip1eX2W` remain retained and reconciled, with their
`target/deployment-*` evidence. Failed batches `batch-Z5xKoNRN`,
`batch-N0QLIhim` and `batch-3v7LxlbU` produced no release/upload and were superseded
by the completed patch batch. Do not resume them against published versions.
The older failed `batch-st9pUW60` also produced no release/upload. Original
deferred lock selections remain under
`.git/release-state/deferred-platform029-20261109` and `preserve/*` refs.

The reviewed retired client archive remains at
`/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`; original committed
source remains at `0c17900090f0c2d2ebf3182598b49e651223845e`. The extraction
contract retains source provenance. The [earlier handoff archive](archive-2026-11-10.md)
remains unchanged; its historical pending versions are not release instructions.

Retain the two-pass generated-root discovery before hashing/cache admission.
Do not replace Darwin `File::sync_all` with plain `fsync`: Rust selects
`F_FULLFSYNC` there. Discovery's handwritten parser error also preserves the
cause's downcast identity, which the rejected derive changed. Earlier focused
Linux observations do not establish native macOS qualification.
