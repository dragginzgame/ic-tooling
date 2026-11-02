# Build changelog

This ledger owns ic-build: Cargo/Wasm builds, exact inputs and artifact caches.
Its unchanged published baseline is 0.73.6, from the
[original Foundation release](../foundation/CHANGELOG.md#0736---2026-11-01).
Earlier history remains there once, under its original package scope; no version
reset, release, tag or upload is implied by establishing this group.

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
