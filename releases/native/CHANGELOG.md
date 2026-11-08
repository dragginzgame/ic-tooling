# Native allocation measurement history

This ledger is frozen: `ic-testkit-native` and its release group are retired from
active maintenance because no current workload consumes the remaining allocation
API. Nothing moved into PocketIC Testkit. Published versions, tags and original
receipts remain available; the final published version is 0.80.0. Its original
independent baseline is 0.79.0; [history through that release](../testkit/CHANGELOG.md#0790---2026-11-02)
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
