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
product/qualification references together. Toko Tooling is 0.39.6, Wire is 0.19
and the product Platform requirement remains 0.29 (locked to 0.29.2).

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
