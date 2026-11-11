# Upstream PocketIC migration — Testkit 0.98 and later

Testkit no longer maintains a PocketIC client. The native-only
`ic_testkit::pocket_ic` re-export now exposes its selected upstream `pocket-ic`
16.1 dependency. Import the asynchronous client from
`ic_testkit::pocket_ic::nonblocking::PocketIc` and its builder from
`ic_testkit::pocket_ic::PocketIcBuilder`; no separate client dependency is needed.
This additive export is pending publication after 0.100.1. Releases 0.98 through
0.100.1 require a direct `pocket-ic` dependency. The re-export has upstream type
identity and API behavior, not the retired fork's API.
The server remains an explicitly admitted external PocketIC 16.1 binary.

## Construction and server custody

```rust,no_run
use std::time::Duration;
use ic_testkit::pocket_ic::PocketIcBuilder;
use ic_testkit::pic::PocketIcStartupConfig;
# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let config = PocketIcStartupConfig::from_env(Duration::from_secs(30))?;
let owned = config.build_instance(
    PocketIcBuilder::new()
        .with_max_request_time_ms(Some(30_000))
        .with_application_subnet(),
).await?;
let pic = owned.instance();
let canister = pic.create_canister().await;
assert!(pic.canister_exists(canister).await);
owned.close().await;
# Ok(())
# }
```

`PocketIcStartupConfig` retains explicit server selection, child-process custody,
startup deadlines and captured output. It always passes its selected URL to the
upstream builder. Raw upstream builders also expose their own provisioning and
configuration; use Testkit's startup helpers when Testkit should own the server.
Upstream's request-time setting is its polling policy, not Testkit's retired
absolute HTTP/body deadline. The caller owns its runtime and any outer timeout.

Native ids, management DTOs and metrics now have upstream PocketIC identities.
`pocket_ic::CanisterId` is the upstream Principal type. Convert explicitly at
application boundaries with `CanisterId::from_slice(owned_principal.as_slice())`
and the inverse Platform Types construction. Portable `fake_principal` continues
to return `ic_platform_types::Principal`; Wasm selects no PocketIC or Candid.

## Failure and cleanup changes

Upstream transport, parsing and some management convenience operations panic.
Other calls return upstream `RejectResponse`, which is a structured value but
not a `std::error::Error`. Testkit does not catch or translate transport panics,
recreate typed client errors, or expose the removed accepted-operation contexts.
There is no Testkit response ceiling or snapshot-transfer budget on upstream calls.

`PocketIcInstance::close`, baseline close and pool shutdown now return `()`.
They await upstream best-effort deletion while retaining their managed server.
Upstream logs deletion failures instead of returning them; completion does not
acknowledge deletion on a borrowed server. Borrowed-server recovery belongs to
its caller. Pool shutdown still closes admission, wakes queued callers, waits
for existing leases and does not repeat completed cleanup attempts. A panic or
cancelled preparation still forces a fresh build before another lease is exposed.
The standalone recipe treats returned management rejections as fatal; the removed
typed dead-transport classifier and automatic same-acquisition recovery are gone.

Upstream owns its tracing/environment behavior and dependency feature set.
Testkit no longer promises that raw client construction ignores logging variables
or avoids installing an upstream subscriber. Do not rely on the removed logging
or bounded-client contracts.

## Harness functions

Helpers are free functions taking `&PocketIc`, rather than inherent client methods:

| Former call | New call |
| --- | --- |
| `pic.try_create_and_install(spec).await?` | `pic::create_and_install(&pic, spec).await` |
| `StandaloneCanisterFixture::try_install(owner, spec).await?` | `StandaloneCanisterFixture::install(owner, spec).await` |
| `pic.wait_out_install_code_rate_limit(delay).await?` | `pic::wait_out_install_code_rate_limit(&pic, delay).await` |
| `pic.retry_install_code(policy, operation).await` | `pic::retry_install_code(&pic, policy, operation).await` |
| `pic.capture_snapshots(targets).await` | `pic::capture_snapshots(&pic, targets).await` |
| `pic.restore_snapshots(&snapshots, funding).await` | `pic::restore_snapshots(&pic, &snapshots, funding).await` |
| Client diagnostic methods | Corresponding `pic` functions with the client as first argument |

Creation, funding and upstream `install_canister` return values directly or panic;
`CanisterInstallError`, `StandaloneCanisterInstallError` and the unused
`InstallSpec::label` retire. A standalone
installation panic unwinds its managed-server owner. Keep ownership outside a
borrowed helper when the caller needs to inspect an instance after catching a panic.

Wire calls retain IC Wire's limits and typed encoding/decoding failures. Their
call rejection field now contains upstream `RejectResponse`; transport panics
propagate. Response decoding never retries an update.

## Wire identity alignment — Testkit 0.99 and 0.100

Testkit 0.99.0 selects IC Wire 0.20. Callers using the `wire` feature must select
Wire 0.20 too: `WireCallSpec` uses its `Limits`, and encoding/decoding failures
retain its `Error` identity. No budget conversions or codec bridges are needed.
This resolves
[#350 — Testkit Wire types block Blob's Wire 0.20 adoption](https://github.com/dragginzgame/ic-tooling/issues/350).

Testkit 0.100.0 adopts Wire 0.21. Select that Wire minor when upgrading
its typed call helpers; limits and codec errors have the new supplier identity.
Wire 0.21's breaking TypeScript naming change requires no regeneration here:
Testkit's native/probe profiles do not select its TypeScript generator.
Runtime bytes, budgets and decoding acceptance remain unchanged from Wire 0.20.

Decoder acceptance follows upstream Postcard, including redundant varint groups
within width bounds and primitive normalization. Canonical encoder bytes and
caller-selected budgets are unchanged; retain received frames for exact retries,
signatures and hashes rather than relying on byte-identical re-encoding.
Update response refusals do not imply the update had no effect and never cause
automatic resubmission. The independent upstream PocketIC migration above also
applies when upgrading from Testkit 0.97 or earlier.

## Snapshot boundary

Testkit 0.100.0 also corrects standalone recipe lifecycle failures.
Builder failures remain the original `io::Error`; snapshot failures retain
`ControllerSnapshotError`. Stop/start rejections instead retain
`StandaloneCanisterLifecycleError::{Stop, Start}` inside `io::Error::get_ref()`.
Downcast that cause to inspect the exact canister and original PocketIC rejection.
A stop refusal before capture is no longer labelled as a snapshot failure.
Transport panics and explicit teardown ownership are unchanged.

The baseline uses upstream `take_canister_snapshot` and `load_canister_snapshot`.
It retains deterministic ordering, duplicate-target admission, exact captured
senders, caller-selected funding and rollback of known captures after a returned
rejection. It no longer downloads/copies snapshot data or exposes metadata/chunk
transfer APIs. Do not assume the retired transfer's global-timer and low-memory
hook restoration: use fresh instances or explicit application-owned reset for
those domains. Simulated time, pending work, extra canisters and external effects
remain outside baseline rollback. Cancellation or upstream panic can leave partial
captures on a borrowed server; reconcile them explicitly.

## Qualification and consumers

The installed dependency proof covers module installation/status, certified subnet
metrics and repeated stable-memory snapshot restoration. Harness tests retain
startup/cancellation/server custody, panic propagation, pool closure, exact senders,
ordinary snapshot refusal/rollback, diagnostics, Wire and held-reply behavior.
Original imported sources, notices and reconstruction guards retire together;
upstream retains its own licensing. Package metadata selects the repository's
root licence, which Cargo includes in the archive.

Canic, Query and other consumers importing the retired module or helper methods
must migrate removed fork/helper APIs. With the new re-export they can select
Testkit alone for the harness and client; existing direct upstream imports remain
valid when they resolve to the same supplier version. Sibling source
and releases are separate work; this migration changes only Tooling and its private
qualification workspace. Product release and native macOS gates remain required.
