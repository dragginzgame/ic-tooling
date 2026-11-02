# IC Testkit Native

Std-only native allocation measurement. This package has no PocketIC,
Host or canister dependencies and exposes no APIs on wasm32.

Install `MeasuredAllocator::new(System)` as the test binary's global allocator,
or wrap another allocator. `measure(iterations, operation)` returns allocation
and reallocation counts, net bytes, peak positive net bytes and elapsed time for
this thread only. Keep workload construction and output formatting outside the
measurement. Other threads are deliberately excluded; this is not a process RSS
or IC instruction measurement.

Allocations freed during the scope may predate it: net bytes are signed and the
peak is relative to the scope's starting point. Nested measurement refuses before
running its operation. Unwinding disables measurement; the original panic remains.

This package has its own `native` release group, independent of PocketIC/Testkit.

Memory IO metering belongs to IC Storage's `test-support` surface. Migrate
`ic_testkit_native::memory::{MeteredMemory, MemoryIoCounts}` imports to
`ic_storage::testing::{MeteredMemory, MemoryIoCounts}` with Storage 0.65 and its `test-support` feature. The `stable-memory` feature is removed.
