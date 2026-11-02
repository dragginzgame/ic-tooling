# Native allocation measurement changelog

This ledger owns `ic-testkit-native`, independently of Testkit. Its published
baseline is 0.79.0; [history through that release](../testkit/CHANGELOG.md#0790---2026-11-02)
remains in the original Testkit ledger with its original package scope.

## [0.80.0] - 2026-11-02

### ⚠️ Breaking

- Remove the `stable-memory` feature and `memory::{MeteredMemory, MemoryIoCounts}`.
  Use IC Storage 0.65's `test-support` helpers at `ic_storage::testing` for backing IO
  metering. The generic allocation API is unchanged and has no dependencies.

### 🔧 Changed

- Release Native independently with `RELEASE_GROUP=native`. PocketIC/Testkit
  changes no longer advance allocation tooling. Existing versions, tags and
  saved Testkit release journals keep their original ownership.
