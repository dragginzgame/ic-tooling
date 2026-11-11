# Current handoff — 2026-11-11

## Delivery

Platform 0.31 adoption is published. Latest product release commit:
`f760451d02d95664bbca888ea90d5c78e856058a`. Frozen Tools batch
`batch-OsoImRVe` and Testkit batch `batch-LRmZoenK` completed and were reconciled
into ordinary main.

| Group | Published | Release commit |
| --- | --- | --- |
| Foundation (five packages) | 0.76.4 | a1b50ffc |
| Build | 0.83.0 | 662516e5 |
| Text | 0.74.5 | f0352237 |
| Host Tools | 0.91.0 | 1b60f3a0 |
| Testkit | 0.100.2 | b9415868 |

Both new versions were independently verified present and not yanked, and
GitHub's annotated tags match the local identities. Private qualification packages
remain 0.96.0. Each group passed its complete 32-stage Linux gate, including
warning-denied lint, archive qualification, installed PocketIC, Wasm/probe and
workspace tests. Native macOS 15 Intel and Apple Silicon results are separate;
new hosted qualification is asynchronous and not claimed as passing here.

Both batches captured Toko Tooling 0.39.10, Platform 0.31.0 and Types 0.19.3.
Runtime/macros/guard remain 0.21.1; release Wire selections remain 0.21.0.
Unrelated ordinary lock selections were preserved outside both releases and
reapplied after reconciliation. Durable copies are recorded by
`preserve/deploy-20261111-platform31-dependencies`,
`preserve/deploy-platform31-before-tools-reconcile` and
`preserve/deploy-platform31-before-testkit-reconcile`; original files and patches
remain under `.git/release-state/preserve/deploy-20261111-platform31-tools91-testkit1001/`.
No batches or artifacts were retired. Candidate Git/journals remain under
`.git/release-state/frozen/`, with managed checkouts under `target/release-frozen/`.
Logs are `target/deploy-platform31-tools91.log` and
`target/deploy-platform31-testkit1001.log`, with corresponding `-reconcile.log`
files. Independent registry observations remain in `target/deploy-platform31-registry/`.

The release-commit CI and Testkit runtime runs were in progress when checked;
Browser fixture session passed. The final handoff delivery requests an explicit
asynchronous native qualification run so a documentation-only push cannot
supersede the required three-host proof. No hosted result is inferred from Linux.

## Published Testkit client re-export

The maintainer approved native-only `ic_testkit::pocket_ic` as a direct re-export
of Testkit's existing upstream dependency. Callers can select Testkit alone for
the harness and client; no fork APIs, wrappers, dependencies or feature gates
are added. The earlier no-re-export instruction is superseded for this path.
The library, README, migration guidance and extraction contract are aligned.
Testkit 0.100.2 is published and independently verified present/not yanked.
Commit `b9415868f0a30d2013f3abf3d2cdbc588b224594` and tag `testkit-v0.100.2`
match GitHub. Frozen `batch-t7v74QxS` completed and was reconciled; all 32 local
stages and publication archive verification passed. Gate evidence remains at
`target/release-frozen/batch-t7v74QxS/target/release-validation/testkit-gate.81L3DM/`.
Normal log: `target/deploy-testkit-reexport.log`. The saved unrelated lock draft
is reapplied after reconciliation; originals/patch remain in
`.git/release-state/preserve/testkit-reexport-20261111/`. No other group released.
Focused Linux library Clippy, the public consumer doctest, native/Wasm dependency
closures, Rust formatting and document links pass. The native graph has one
PocketIC and no Platform/runtime; all-feature Wasm excludes PocketIC/Candid.
The complete release gate also passed installed simulator and workspace tests.
Hosted native qualification remains asynchronous; no fresh macOS pass is claimed.

## Current contracts

Testkit uses upstream PocketIC 16.1, preserving its native DTO identities,
panics, polling and best-effort deletion. Harness helpers remain free `pic`
functions; portable helpers retain Platform Types Principal. Fork-specific
recovery/transfer APIs stay retired. See [the migration](../testkit/client-migration.md).
Default/Wire Testkit excludes Platform; only held fixtures select its adapters.
Client dependencies remain native and the Wasm graph remains Types-only.
Wire 0.21 public identities require aligned consumer selection. Toko 0.39.10
remains installed; no supplier or sibling deployment is included in this work.

## Next three fixes

1. Preserve exact paths in [tool probes](../../crates/ic-build/src/tools/probe/mod.rs)
   and check raw directory names before archive-path normalization.
2. Propagate filesystem errors from [Cargo configuration selection](../../crates/ic-build/src/cargo/environment/mod.rs).
3. Reject empty or malformed successful [process-state observations](../../crates/ic-host-process/src/testing/mod.rs).

These three audit findings are not implemented. Canic's existing Build caller
migration remains in [Canic #762 — Migrate IC Build callers to explicit maintenance and pruning](https://github.com/dragginzgame/canic/issues/762).

## Historical evidence

Superseded release and focused-test narratives are preserved in the
[previous handoff](https://github.com/dragginzgame/ic-tooling/blob/2f448a59cf290859a65855a72339f97dd442d1a4/docs/status/current.md).
Its old pending descriptions are historical, not new release instructions.
Retain batches `batch-OHBHvLXU`, `batch-4hupb2d0` and `batch-UEWDKToB`,
their frozen journals/candidates, `preserve/deploy-20261111-dependencies` and
`preserve/deploy-20261111-followup`, and the retention locations in that record.
Retain the snapshot cleanup callback's explicit future type: the attempted
`AsyncFnMut` replacements failed the standalone recipe's Send lifetime contract.
The prior native release-tag proof at `6f4128ae` does not qualify these changes.

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
