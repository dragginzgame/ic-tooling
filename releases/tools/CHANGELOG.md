# Host Tools changelog

This ledger owns Host Tools. The published baseline at the split is 0.76.1,
from the [original joint integration release](../integration/CHANGELOG.md#0761---2026-11-01).
Joint history through 0.76.1 remains there once, with its original scope.

## [0.84.0] - Unreleased

### ⚠️ Breaking

- Adopt Foundation 0.75. The optional IC interface adapter now accepts that
  Artifacts version's Wasm facts; align direct Artifacts dependencies when
  passing inspected modules to Host Tools.

### 🔧 Changed

- Qualify locked Platform 0.23.6 and Types 0.18.2 in the Candid and ledger
  adapters, retaining the existing compatible dependency ranges.

## [0.83.0] - 2026-11-05

### ⚠️ Breaking

- Adopt Platform 0.23 and move strict certificate/hash-tree CBOR decoding there. Remove Tools'
  `certificate` feature and module; select Platform's `certification-cbor` feature
  and import `certification::cbor::strict` directly. Limits, malformed-input
  rejection and certificate/tree identities retain their contracts. Application
  verification and trust policy remain with consumers.

### 🧹 Cleanup

- Refresh compatible locked dependencies using the maintainer-selected graph.
- Remove the retired certificate test profile from CI and release validation.
  Platform owns decoder tests; Tooling retains its shared-type consumer check.

## [0.82.1] - 2026-11-05

### ➕ Added

- Decode bounded ICP canister inventory and structured cycle balances without
  scraping display text. Preserve public-status balance absence and refuse
  malformed or repeated authoritative fields; callers retain sorting, fallback
  and funding policy
  ([#335 — Share bounded ICP inventory and cycle decoding](https://github.com/dragginzgame/ic-tooling/issues/335)).

[Detailed changes](0.82.md).

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
