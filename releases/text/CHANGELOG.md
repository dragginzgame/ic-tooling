# Text changelog

This ledger owns ic-host-text: bounded text decoding, Unicode rendering and
optional YAML. Its unchanged published baseline is 0.73.6, from the
[original Foundation release](../foundation/CHANGELOG.md#0736---2026-11-01).
Earlier history remains there once, under its original package scope; no version
reset, release, tag or upload is implied by establishing this group.

## [0.74.1]

### ➕ Added

- Render exact scaled integer quantities with caller-selected unit divisors and
  suffixes. Round half up to two places, trim fractional zeroes and promote units
  at rounded boundaries without floating-point arithmetic. Reject invalid scales
  explicitly and preserve the full `u128` value range.

## [0.74.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 and pin contributor/release compilation to that version.
  Older compiler support is retired.
  [#313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313).

### 🧹 Cleanup

- Own one independent release group, ledger and tag namespace. Unchanged
  Foundation, Build and integration packages need no coordinated bump.
