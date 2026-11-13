# IC Testkit Wire

Native typed Wire query/update calls over Testkit's selected upstream PocketIC
client. This package owns Wire/client composition independently of simulator
provisioning, lifecycle and custody in `ic-testkit`.

Use `ic_testkit_wire::{WireCallSpec, query_wire, update_wire}` and select a direct
`ic-wire = "0.22"` dependency when supplying limits or matching codec errors.
The initial 0.1 release supports Wire 0.22 and the selected PocketIC 16.1 client.

```rust,no_run
use ic_testkit::pocket_ic::{CanisterId, nonblocking::PocketIc};
use ic_testkit_wire::{WireCallError, WireCallSpec, query_wire};

async fn call(pic: &PocketIc, target: CanisterId, spec: WireCallSpec)
    -> Result<u64, WireCallError>
{
    query_wire(pic, target, CanisterId::anonymous(), spec, &()).await
}
```

Every call carries the exact method name/identity and explicit request/response
byte, allocation, node and depth limits. Wire alone encodes and admits frames.
Request rejection occurs before transport. A query/update executes once, on the
caller's runtime, with the supplied caller. Codec errors retain their Wire type;
canister rejections retain the original upstream value and operation context.
Transport panics propagate. Reply bytes are not logged or retained by errors.
A decode failure after an update does not prove that the update had no effect.
Use the raw PocketIC methods when original frames are required as evidence.

Borrowed serializable requests and owned Deserialize-only responses are supported.
Successful decoding can normalize collections or accept nonminimal integers;
re-encoding need not preserve original byte identity. This adapter adds no retry,
endpoint registry, second codec, client, runtime or installation policy.

A Wire compatibility change belongs to the independent `testkit-wire` release
group. Testkit has no Wire dependency or feature, even with all features selected.
See the [migration and release boundary](../../docs/testkit/wire-adapter.md).
