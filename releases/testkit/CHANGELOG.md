# Testkit family changelog

This ledger owns Testkit, Testkit Native and the private development/probe packages. The published baseline at the split is 0.76.1,
from the [original joint integration release](../integration/CHANGELOG.md#0761---2026-11-01).
Joint history through 0.76.1 remains there once, with its original scope.

## [0.77.0] - 2026-11-01

### ⚠️ Breaking

- Adopt IC Platform 0.17.0 and its maintained PocketIC client. Consumers must
  use `ic_platform::pocket_ic` from Platform 0.17 for client and client-owned DTO
  identities passed to Testkit helpers. Method signatures, simulator behavior
  and the PocketIC 16.1.0 server pin remain unchanged.

### 🔧 Changed

- Select Platform's independent Runtime 0.17.1 for Wasm system calls and the
  private performance probe; preserve the executor singleton and native-only
  client boundary.

## [0.76.2] - 2026-11-01

### 🔧 Changed

- Release Host Tools and the Testkit family independently in the existing workspace.
  Select `RELEASE_GROUP=testkit` for this group; package versions and public
  contracts are unchanged. Existing integration journals retain their original
  source, policy and recovery context. ([#299](https://github.com/dragginzgame/ic-tooling/issues/299))
