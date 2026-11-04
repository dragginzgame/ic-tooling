# Host Tools changelog

This ledger owns Host Tools. The published baseline at the split is 0.76.1,
from the [original joint integration release](../integration/CHANGELOG.md#0761---2026-11-01).
Joint history through 0.76.1 remains there once, with its original scope.

## [0.82.0] - 2026-11-04

### ⚠️ Breaking

- Adopt Platform 0.22. Certificate/hash-tree decoding now returns upstream
  `ic-certification` identities shared directly with native Agent. Align consumer
  Platform dependencies and remove former certificate conversion adapters;
  strict decoding and feature boundaries remain unchanged.

## [0.81.0] - 2026-11-03

### ⚠️ Breaking

- Adopt Platform 0.21 for external IC adapters. Align direct consumer Platform
  dependencies when exchanging Platform-owned certificate, hash-tree or Candid
  decoder-limit types with Host Tools. Types 0.18, Runtime 0.19 and the existing
  feature and protocol boundaries remain unchanged.

## [0.80.0] - 2026-11-03

### ⚠️ Breaking

- Adopt Platform 0.20 for external IC adapters. Align direct consumer Platform
  dependencies when exchanging its exposed certificate or adapter types.
  Canonical Types remains 0.18; feature and protocol boundaries are unchanged.

## [0.79.0] - 2026-11-03

### ⚠️ Breaking

- Adopt Platform 0.19 and canonical Types 0.18. Align direct consumer
  dependencies when exchanging owned registry or Platform types. Feature
  boundaries and external protocol identities remain unchanged.

## [0.78.1] - 2026-11-02

### 🧹 Cleanup

- Select canonical Types directly for registry principals. Registry-only native
  and Wasm consumers no longer need the Platform facade; public Principal
  identity and the existing protocol and feature boundaries are unchanged.

## [0.78.0] - 2026-11-02

### ⚠️ Breaking

- Adopt Platform 0.18 and canonical Types 0.17. Align consumer dependencies when
  exchanging public principal or certificate identities with Host Tools.

- Require Rust 1.99.1 and pin contributor/release compilation to that version.
  Native qualification now owns the former compiler-lane feature and Wasm
  obligations; the stable MSRV status reuses its results. Older compiler support
  is retired; package release groups remain independent.
  [#313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313).

## [0.77.0] - 2026-11-01

### ⚠️ Breaking

- Adopt IC Platform 0.17.0 and its canonical Types 0.16 identities. Consumers
  exchanging Platform-owned principals or certificates with Host Tools must
  align their Platform dependency; existing encoding and feature boundaries remain.

## [0.76.2] - 2026-11-01

### 🔧 Changed

- Release Host Tools and the Testkit family independently in the existing workspace.
  Select `RELEASE_GROUP=tools` for this group; package versions and public
  contracts are unchanged. Existing integration journals retain their original
  source, policy and recovery context. ([#299](https://github.com/dragginzgame/ic-tooling/issues/299))
