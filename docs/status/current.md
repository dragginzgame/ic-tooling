# Current handoff — 2026-11-02

## Testkit 0.80 complete; Native 0.80 authorized and unblocked

The maintainer requested deployment after the Native ownership split and IC Wire
0.15 adoption. Testkit 0.80.0 is published, unyanked and independently verified:
source/tag `5cd77b223e8f35cdd84ff733e7c3e27edd3d1bc0`, `testkit-v0.80.0`.
The uploaded archive checksum and clean VCS source match registry readback.
The actual registry-only combined public caller tests pass. Do not upload or
increment Testkit again. Private Dev/probe versions are 0.80.0; other groups did
not advance. Full local Linux/native/Wasm/installed/package qualification passed.

Native was initially held for its replacement helper. During delivery, Storage
0.65.0 was published by its owner. Its checksum-verified archive exposes native
`ic_storage::testing::{MeteredMemory, MemoryIoCounts}` under `test-support`.
Archive VCS source is `84a7ecf6ed6517b989848c8e953d93fb5f561505`; helper code
matches the qualified contribution apart from rustdoc. This clears the supplier
condition: finish the authorized Native 0.80.0 release/publication through its
own group and complete gate. No other repository release is authorized here.

Native's source is allocation-only, has no dependencies and removes the former
memory module/feature. Allocation APIs are unchanged. Consumers needing IO
metering migrate to Storage 0.65's test-support surface. Six independent release
groups preserve all package names; old Testkit journals keep their original
Native membership and cannot be replayed under the new roster. Archive
qualification still covers all ten public packages. No narrower group gate is
activated; run the full standard gate for Native's own fresh release attempt.

## Checkouts, evidence and preservation

Use retained checkout `/tmp/ic-tooling-five-groups.iOEuyP/checkout` for delivery.
It has the scoped release lock; unrelated ordinary dependency refreshes stay
outside commits. The ordinary checkout was fast-forwarded to `5cd77b22` after
Testkit publication; all 456 incoming registry records are preserved exactly,
with only owned Testkit/Dev/probe versions advanced. Its only remaining dirty
file is the unrelated Cargo.lock. The task source stash was dropped only after
verification; its patch and identity remain in Testkit evidence. Preserve new work.

Testkit evidence lives under the ordinary checkout's
`.git/release-state/deployment-testkit-0.80.0/`: complete gate/dry-run/upload logs,
plans/receipts, exact registry archive, combined consumer proof and preservation
records. Temporary source/evidence inputs are `/tmp/ic-tooling-testkit-080.OPJk8Q/`.
Its `storage-final/` archive records the supplier that unblocked Native.
Create separate `.git/release-state/deployment-native-0.80.0/` evidence for the
new selected group; do not recover/replay Testkit's completed attempt.

After Native delivery, fast-forward the ordinary checkout again while preserving
its unrelated lock and any new source edits. Never apply complete old locks or
historical task stashes over the owned versions. Keep failed evidence and artifacts;
no cleanup is authorized.

## Qualification and hosted CI

The split's allocation tests, strict Native/Dev Clippy, Wasm compile, six-group
adapter/publication/Make fixtures and graph policies pass. Wire adoption's strict
affected-package native/Wasm-library Clippy, nine transport cases, one protocol
case and two public contract tests pass. Only Wire/derive advanced to 0.15;
454 other registry records were preserved during adoption. The earlier split
pruned IC Memory, Platform 0.17 and stable-structures without changing retained
registry records. The isolated release lock keeps the prior external patches.

Testkit's latest hosted CI run 41493209855 has successful classification/lint
and Linux plus both macOS 15 native jobs in progress. Browser session passes;
optimizer/runtime workflows are running. Native's later push will supersede it.
Report actual statuses asynchronously; do not wait solely for hosted matrices.
No new-source native macOS pass is claimed. Previous base `b248f5d4` passed all
three required native hosts in run 41477907607; that is separate evidence.

## Sibling contribution history

The former planned compatible Storage 0.64.1 helper contribution is absorbed by
published Storage 0.65. Do not release the old patch as another supplier version.
Historical scoped patches, source snapshots, five helper/custody tests and
qualified-source proof remain under `/tmp/ic-tooling-native-split.w9WGNC/`.
Original sibling checkouts were never edited by this Tooling task.

The earlier prepared Scheduling patch requires 0.64.1 and must be rebased for
actual Storage 0.65 and its owner's SDK selections before delivery. Its recorded
48 native cases and strict Clippy qualify that older projection, not a current
registry adoption. Consumer migration belongs to Scheduling; do not silently
ship its unrelated pending work or interpret this Tooling deploy as cross-repo
delivery authority.

Shared Tooling 0.27.27 (`651f5f7`), Rust floor/pin 1.99.1 and arithmetic-only
native Metrics remain selected. Testkit Wasm excludes Metrics. No tags, retries,
publications or deployments are implied by historical evidence directories.
