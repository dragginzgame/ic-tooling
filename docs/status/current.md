# Current handoff — 2026-11-10

## Delivery

The cleanup and upstream PocketIC migration are committed and published. Latest
product release commit: `7b9749272b11f5984e83edc504c15671976ed687`.
Both completed frozen batches were reconciled into the ordinary main checkout.

| Group | Published | Release commit |
| --- | --- | --- |
| Foundation (five packages) | 0.76.3 | bea2060f |
| Build | 0.81.2 | d8c35757 |
| Text | 0.74.4 | d29889e3 |
| Host Tools | 0.89.1 | 3ca25b99 |
| Testkit | 0.98.0 | 7b974927 |

All nine versions were verified present and not yanked in the crates.io index.
Private qualification packages remain 0.96.0. The deployment preserved selected
external dependencies; release preparation changed owned versions and their
product/qualification references together. That release used Toko Tooling 0.39.6
and Wire 0.19; the product Platform requirement remains 0.29 (locked to 0.29.2).

## Local maintenance

Toko Tooling 0.39.9 is selected in both workspaces and installed through the
authenticated crates.io exporter, preserving the 113-file runtime selection.
That refresh changed only its lock entries; the pre-existing product
`rustls-pki-types` 1.16.0 selection remains intact. Tool catalogs are unchanged.
This refresh is local and unpublished; product versions remain as listed above.
The 0.39.9 checker admits native PocketIC dev-dependencies for simulator tests.
Linux snapshot/pin admission, both workspaces' actual IC dependency boundaries and
documentation links pass; logs are under `target/toko0399/`. Earlier prepared
tool, publication licence and release-refusal proofs under `target/toko0398/`
retain unchanged inputs. No full release gate or native macOS qualification ran
for these refreshes.

Wire 0.20 is now adopted locally in both workspaces. Public adapter identities
and upstream Postcard acceptance require the pending Testkit 0.99.0 minor;
product versions remain unchanged. Fourteen focused tests cover nine adapter
cases, live native replies, type identity, probe budgets and installed Wasm
requests/state/refusals. Graph policy, portable compilation, selected warning-denied
lint, formatting and snapshot/pin/doc admission pass. Logs are under
`target/wire020/`; this is Linux evidence, with no new full release gate or
native macOS qualification.

Build's local reduction batch is pending 0.82.0: shared Cargo targets are the only
compilation path, shared-target maintenance is explicit, and the full input digest
owns both cache identity and mutation validation. The separate validation digest
and receipt field are removed. New fingerprint keys preserve old cache entries;
unrelated workspace manifest/lockfile edits can invalidate exact reuse. The
[Build guide](../build/artifacts.md#pending-build-082-migration) records caller
migration. Package versions are unchanged; this work is uncommitted and unpublished.
Focused Linux cache/batch/evidence and Cargo-input transaction tests, real Cargo
Wasm/examples/artifact-handoff/process-lock tests, 103 durable-filesystem tests,
warning-denied Build Clippy/rustdoc, formatting and document links pass. Logs and
before/after Rust counts remain under `target/loc-audit/`. This is not a complete
release gate or native macOS qualification.

The same pending Build 0.82.0 batch now makes exact-cache pruning explicit too.
Acquisitions no longer schedule pruning or attach nonfatal maintenance results,
timings or phases. Callers invoke the existing prune functions and retain their
records to protect live outputs; old exact-cache schedule markers remain untouched.
Focused cache, transaction/batch and pruning tests, cross-process handoff, real
Cargo Wasm acquisition, warning-denied Clippy/rustdoc and document links pass on
Linux. Evidence is under `target/loc-audit/explicit-pruning/`. This continuation
removes another 253 Rust lines, leaving 68,768 across both workspaces; package
versions, publication and native macOS qualification remain unchanged.

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
separate and asynchronous through the manual [CI workflow](../../.github/workflows/ci.yml).
Cancelled or superseded hosted runs are not passes.

Completed batches are `batch-HIlzE9l1` (eight patch packages) and
`batch-hip1eX2W` (Testkit minor). Their source-bound plans, gate logs and upload
receipts remain under `.git/release-state/frozen/`; managed checkouts remain under
`target/release-frozen/`. Normal-checkout logs are `target/deployment-*`, including
the registry record and passing consumer tooling fixture suite.

## Custody and retained decisions

Retain all completed/failed batches, refs, plans, receipts and artifacts; no
deployment cleanup was performed. Failed batches `batch-Z5xKoNRN`,
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
