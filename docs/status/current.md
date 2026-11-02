# Current handoff — 2026-11-02

## Authorized Testkit 0.80 deployment

The maintainer requested deployment after adopting IC Wire 0.15. Release and
publish Testkit 0.80.0 through the normal selected-group workflow. Native's new
independent group remains at 0.79.0 with 0.80.0 pending notes: its memory API
removal waits for a published replacement helper. Live Storage remains 0.64.0;
planned 0.64.1 is not yet published. No Storage or Scheduling release is authorized.
Tools, Foundation, Build and Text do not need another publication for this batch.

Tooling source now has six groups. Testkit retains the private Dev/probe packages;
Native is allocation-only and dependency-free. Its memory module/feature and the
IC Memory/old Platform/stable-structures graph are removed from main's pending
Native source, with the registry Native 0.79 API unchanged until its own release.
Old Testkit journals preserve their original Native roster. Do not replay them
under the new policy. All ten public packages retain archive qualification.

Wire/derive 0.15 changes public Limits/Error identities. Consumers passing those
types to Testkit must adopt Wire 0.15. Core bytes/budgets and the native-only
optional transport boundary remain unchanged; no metadata bridge is enabled.
Focused native/Wasm-library Clippy, nine transport cases, one protocol case,
two public contract tests and graph policies pass before release. Native
allocation tests, six-group adapter/publication/Make fixtures, sorting and docs
also pass. Fixtures substitute release/upload effects; they are not live delivery.
The new deployment still requires its full standard release gate and dry run.

## Delivery checkout and preservation

The retained integration checkout is `/tmp/ic-tooling-five-groups.iOEuyP/checkout`.
It uses the scoped release lock, leaving ordinary-checkout unrelated dependency
refreshes outside delivery commits. New source/evidence/preservation inputs are
in `/tmp/ic-tooling-testkit-080.OPJk8Q/`; durable deployment evidence belongs at
`.git/release-state/deployment-testkit-0.80.0/` in the ordinary checkout.
Preserve new dirty work, artifacts and failed attempts. No cleanup is authorized.
After successful delivery, fast-forward the ordinary checkout while preserving
its unrelated lock selections and any new user edits. Do not apply old entire
locks or historical stashes over the new owned package versions.

Current Wire adoption evidence is `/tmp/ic-tooling-wire-015.FU6Gbv/`. Its lock
changes only Wire/derive 0.14.2 to 0.15.0, preserving 454 other registry records.
The preceding Native split pruned exactly IC Memory 0.63.1, Platform 0.17.0 and
stable-structures 0.7.2; its unrelated records were preserved. The isolated release
lock instead retains the original external patch selections. Do not publish
those unrelated ordinary lock edits as part of this deployment.

## Prepared sibling contributions

Storage's additive helper is prepared independently of its dirty B-tree/SDK work:
`/tmp/ic-tooling-native-split.w9WGNC/storage-contribution` is a clean-base
contribution with pending Storage 0.64.1 notes and unchanged actual package versions.
Native `ic_storage::testing::{MeteredMemory, MemoryIoCounts}` under `test-support`
meters Storage's owned Memory trait and delegates physical MemoryBacking identity.
Shared counters/reset, refused growth and overflow retain focused coverage.
Production APIs/formats are unchanged. Five tests and strict Clippy pass.

The isolated Scheduling migration at
`/tmp/ic-tooling-native-split.w9WGNC/scheduling` requires Storage 0.64.1, replaces
its trait bridge and drops the old Native/unused facade selections. Full graph
resolution, 48 native cases and strict Clippy pass against a projected compatible
Storage source, retaining all Wasm dependency declarations. The only projection
changes are supplier version/source path; actual Rust source identity is verified.
No live IC or new native macOS qualification was run for those contributions.

Only scoped sibling changes are in `patches/storage-0.64.1.patch` and
`patches/scheduling-metered-memory.patch` beneath that split evidence directory.
Original sibling checkouts were never edited. Do not commit their unrelated
pending work. Publish the helper at its supplying owner before adopting the real
registry lock in Scheduling or publishing Native's breaking removal.

## Prior completed releases and host evidence

Prior main is `b248f5d41b50f3a2e10cf83a3258f03783ec57ca`. Foundation 0.74.1
(`7992cfd1`), Build 0.74.0 (`5bff8453`), Text 0.74.0 (`a6005134`), Tools 0.78.0
(`3c5ce4c7`) and Testkit/Native 0.79.0 (`f64b6c71`) are published and independently
registry-readback verified. Do not increment/upload them again. Foundation
0.74.0 is Git-only. Prior plans/archives/checksums remain under
`.git/release-state/deployment-five-groups-20261102/`.

Base CI run 41477907607 completed successfully on Linux x86-64, macOS 15 Intel
and Apple Silicon, including lint/security and compiler status. Those native
macOS passes qualify `b248f5d4`, not the new uncommitted Wire/split source.
Run hosted CI asynchronously after delivery; report pending results without
waiting solely for that matrix. Full local release qualification remains required.

Shared Tooling 0.27.27 (`651f5f7`) and Rust floor/pin 1.99.1 remain unchanged.
Metrics stays native/arithmetic-only and absent from Testkit Wasm. No narrower
group validation mapping is activated; source, archive and recovery custody remain.
