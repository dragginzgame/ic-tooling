# Text changelog

This ledger owns ic-host-text: bounded text decoding, Unicode rendering and
optional YAML. Its unchanged published baseline is 0.73.6, from the
[original Foundation release](../foundation/CHANGELOG.md#0736---2026-11-01).
Earlier history remains there once, under its original package scope; no version
reset, release, tag or upload is implied by establishing this group.

## [0.74.4] - 2026-11-10

### 🔧 Changed

- Adopt the workspace's pinned Toko Tooling 0.39.6 development selection.

## [0.74.3] - 2026-11-08

### ➕ Added

- Render decimal quantities beyond `u128` with caller-selected power-of-ten units,
  exact two-place rounding, unit promotion and terminal-safe suffixes. Invalid
  input and scales refuse without selecting a caller fallback.
  [#341 — Decimal quantity scales](https://github.com/dragginzgame/ic-tooling/issues/341).
- Render full-range `u64` ratios as percentages with exact half-up rounding to
  one decimal place. Zero denominators refuse; values above 100% remain valid.
  [#342 — Exact ratio percentages](https://github.com/dragginzgame/ic-tooling/issues/342).

## [0.74.2] - 2026-11-04

### 🧹 Cleanup

- Keep table, escaping and width unit tests in test-only modules so source
  counts distinguish tests from runtime code without changing their coverage.

## [0.74.1] - 2026-11-02

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
