# Build changelog

This ledger owns ic-build: Cargo/Wasm builds, exact inputs and artifact caches.
Its unchanged published baseline is 0.73.6, from the
[original Foundation release](../foundation/CHANGELOG.md#0736---2026-11-01).
Earlier history remains there once, under its original package scope; no version
reset, release, tag or upload is implied by establishing this group.

## [0.74.2]

### 🛠️ Fixes

- Qualify dependency-free package archives while still rejecting empty package
  graphs, malformed locks and changed external identities/checksums.
- Let explicit CI qualification dispatches finish independently of newer routine
  pushes, preserving normal newest-revision cancellation and all native checks.
  Accept a fully qualified main-branch dispatch as a release-plan baseline, with
  the same source, host, step and recovery requirements. This prevents #314's
  matched-input comparison from being repeatedly cancelled during delivery.

### 🧹 Cleanup

- Share standalone/batch input hashing and artifact verification decisions while
  preserving fingerprints, mutation observation points, lock order and failure
  attribution. Carry each configured shared target through its maintenance
  operation while retaining fresh filesystem admission.
- Bind release verification to exhaustive group plans and retained stage evidence.
  Native/Text-only releases can narrow from a complete three-host baseline;
  shared changes and the four wider groups retain the full gate.
  [#314](https://github.com/dragginzgame/ic-tooling/issues/314).
- Keep release-stage inventory and three-host baseline admission with one owner
  each, retaining independent Make dispatch checks and saved-evidence revalidation.
- Compare full and focused qualification on the same native CI inputs, retaining
  stage timing and installed-archive evidence. Narrow release admission requires
  that proof on Linux and both macOS hosts.
- Trim unused Shared producer regression fixtures from the consumer snapshot;
  retain the same adopted revision and local consumer qualification suites.

## [0.74.1] - 2026-11-02

### 🧹 Cleanup

- Keep Wasm progress and shared-target maintenance with their private owners,
  and use one input-identity comparison across cache acquisition, transactions
  and retained evidence. Preserve mutation checks, lock order, failure attribution
  and public receipts.
- Share successful batch outcome counting while retaining separate artifact and
  Wasm reports. Build fixtures now declare their protocol output and script hooks
  directly, avoiding edits tied to the generated shell script's layout.
- Derive Wasm stamp admission length from its encoder so cache reads and writes
  follow the same format.

## [0.74.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 and pin contributor/release compilation to that version.
  Older compiler support is retired.
  [#313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313).

### 🧹 Cleanup

- Own one independent release group, ledger and tag namespace. Unchanged
  Foundation, Text and integration packages need no coordinated bump.

- Carry admitted shared-target paths and maintenance policy through Wasm cache
  acquisition without reinterpreting raw settings. Preserve input revalidation,
  lock order, retained artifacts and maintenance scheduling.
