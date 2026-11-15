# Changelog

## [0.87.0] - Unreleased

Development notes for the redesigned, unpublished `ic-build` package. The next
minor follows published 0.86.0 because the Rust API is incompatible. Historical transplant
notes remain in Git at `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`.

### ⚠️ Breaking

- Retain the `ic-build` crates.io name for the redesigned API at 0.87.0. Existing
  callers require explicit migration; no legacy helper API is carried forward.
  Publication remains disabled during development.

### ➕ Added

- Introduce unpublished `ic-host-artifacts` and `ic-host-fs` 0.80.0 with bounded
  byte admission, held-directory custody and consuming publication states.
  Publication preserves pre-effect refusal and visible-but-unconfirmed outcomes;
  retained recovery files have a separate explicit lifecycle.

- One offline Wasm build operation owns execution, bounded Cargo decoding, exact
  selection and capture into immutable owned bytes. Failure retains process,
  protocol, diagnostic and cleanup evidence; mutable Cargo paths do not escape.

### 🧪 Testing

- Qualify the filesystem supplier against pinned Backup store, payload, capture
  and selected ledger tests in an isolated export, preserving persisted bytes.
  Add focused publication race, retention, identity and sync-failure coverage.

- Add focused byte/custody/process/protocol failure qualification and a real Wasm
  fixture in an independent qualification workspace. Development commands now
  target this workflow instead of the retired copied packages.

### 🔧 Changed

- Share byte admission and filesystem custody with Build; explicitly create its
  private target with mode 0700 instead of relying on umask-dependent defaults.

- Adopt Toko Tooling 0.44.3 in both workspace locks and the verified snapshot;
  LOC reporting now expands levels relative to the selected scope.
