# Cargo metadata changelog

## [0.1.1] - 2026-11-18

### 🔧 Changed

- Refresh the package development tooling selection to Toko Tooling 0.45.8.
  Public APIs, persisted bytes and runtime behavior remain unchanged.

## [0.1.0] - 2026-11-16

### ✨ Added

- Supply the narrow `ic-cargo-metadata` command for bounded manifest and lockfile
  rendering through Toko's existing release transaction protocol. Preserve exact
  unselected bytes, source-qualified identities and explicit group ownership;
  refuse stale, duplicate and ambiguous selections before emitting a candidate.
  [Tooling #8 — Release editor successor](https://github.com/dragginzgame/ic-tooling/issues/8).
