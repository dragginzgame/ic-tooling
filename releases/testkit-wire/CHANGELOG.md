# Testkit Wire changelog

This independent group owns `ic-testkit-wire`. Typed calls previously shipped
inside [Testkit](../testkit/CHANGELOG.md); that history remains there.

## [0.1.1] - Unreleased

### 🧪 Testing

- Make the registry-only caller proof use Testkit's selected client re-export
  and explicitly select its ledger adapter features. This qualification-only
  maintenance requires Git delivery, with no product version increment or upload.

## [0.1.0] - 2026-11-13

### ➕ Added

- Own native typed Wire calls in an independent adapter, preserving explicit
  method/caller identity, all codec limits, typed causes and one-shot transport.
  Wire 0.22 adoption can advance this package without releasing simulator
  lifecycle machinery. See [#357](https://github.com/dragginzgame/ic-tooling/issues/357).

### 🧪 Testing

- Qualify the independent adapter's release metadata and recovery while preserving
  other groups. Wrong-group receipt probes now select from the actual catalog.
