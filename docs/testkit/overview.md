<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-testkit/ic-testkit-readme-header.svg" alt="IC Testkit — Internet Computer helper library" width="100%">
</p>

Memory IO metering belongs to IC Storage's `test-support` API. The unused native
heap allocator package is retired; Testkit owns simulator support.

<!-- helper-navigation:start -->
<p align="center">
  <a href="https://github.com/dragginzgame/canic"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/canic.svg" width="18" height="18" alt=""> <strong>canic</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/icydb"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/icydb.svg" width="18" height="18" alt=""> <strong>icydb</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-timers"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-timers.svg" width="18" height="18" alt=""> <strong>ic-timers</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-memory"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-memory.svg" width="18" height="18" alt=""> <strong>ic-memory</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-query"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-query.svg" width="18" height="18" alt=""> <strong>ic-query</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-backup"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-backup.svg" width="18" height="18" alt=""> <strong>ic-backup</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-blob-storage"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-blob-storage.svg" width="18" height="18" alt=""> <strong>ic-blob-storage</strong></a>
  &nbsp;&middot;&nbsp;
  <a href="https://github.com/dragginzgame/ic-testkit"><img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/icons/ic-testkit.svg" width="18" height="18" alt=""> <strong>ic-testkit</strong></a>
</p>
<!-- helper-navigation:end -->

<p align="center">
  <a href="https://crates.io/crates/ic-testkit"><img src="https://img.shields.io/crates/v/ic-testkit.svg" alt="Crates.io"></a>
  <a href="https://docs.rs/ic-testkit"><img src="https://docs.rs/ic-testkit/badge.svg" alt="Docs.rs"></a>
  <a href="https://crates.io/crates/ic-testkit"><img src="https://img.shields.io/crates/d/ic-testkit.svg" alt="Downloads"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="License"></a>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/MSRV-1.100.0-blue.svg" alt="MSRV"></a>
  <a href="README.md#toolchains-and-checks"><img src="https://img.shields.io/badge/internal%20rust-1.100.0-orange.svg" alt="Internal Rust"></a>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/edition-2024-purple.svg" alt="Rust edition"></a>
  <a href="Cargo.toml"><img src="https://img.shields.io/badge/PocketIC-16.1-green.svg" alt="PocketIC"></a>
  <a href="https://github.com/dragginzgame/ic-testkit"><img src="https://img.shields.io/badge/GitHub-dragginzgame%2Fic--testkit-black.svg" alt="Repository"></a>
</p>

`ic-testkit` helps teams check that an Internet Computer application works as a
complete system before they deploy it. It runs the application in PocketIC, a
local simulation of the Internet Computer, and adds a repeatable way to build
the application, prepare test data, run scenarios, reset state, diagnose
failures, and track performance.

In short: **PocketIC provides the simulated network; ic-testkit helps turn it
into a reliable test environment for a real application.**

## What does that mean?

Testkit supplies reusable simulator test helpers over published upstream PocketIC.
The simulator/server is external. Application tests own topology, funding, RPC
schemas, readiness, reset coverage and assertions. Build and Host own the lower
layers described in [scope boundaries](#scope-boundaries).

The selected client and lifecycle helpers are async. Version 0.98 retired the
local client fork; 0.100.2 exposes the upstream crate through its native re-export. See the [migration guide](client-migration.md)
for the current construction, reset and cancellation contract.

### A concrete example

On the caller's Tokio runtime, build a selected instance, await its operations,
and explicitly close it while its server is available:

```rust,no_run
use ic_testkit::pic::PocketIcStartupConfig;
use ic_testkit::pocket_ic::PocketIcBuilder;
use std::{path::Path, time::Duration};

async fn example(binary: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: binary.into(),
        timeout: Duration::from_secs(30),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }.build_instance(PocketIcBuilder::new().with_max_request_time_ms(Some(60_000)).with_application_subnet()).await?;
    let canister = owned.instance().create_canister().await;
    assert!(owned.instance().canister_exists(canister).await);
    owned.close().await;
    Ok(())
}
```

## Install

Select `ic-testkit` for its upstream client re-export. Optional Testkit features select
`reply-fixtures`, `held-reply-fixtures`
and the provisioning CLI (`cli`). Declare Tokio at the application/test entrypoint
with its I/O and time drivers enabled. Testkit creates no private client runtime.
The canister-side Wasm profile remains separate and lightweight.

Provision the release-bound server explicitly with `ic-testkit-server setup`,
then admit it offline with `ic-testkit-server check`. Library callers can use
`pic::server_assets::{setup, check}`. Tests do not install servers implicitly.
See [server ownership](POCKET-IC.md#server-selection-and-provisioning).

## What ic-testkit provides

- Harness functions over the upstream async PocketIC client and its native DTOs.
- Explicit server startup/custody, installation, snapshots and bounded readiness.
- Optional typed Wire calls and raw/held reply fixtures.
- Fresh instance isolation and explicit controller-aware snapshot operations.
- Controller-aware diagnostics.
- Deterministic fixture principals.

### A typical complex-suite workflow

Build Wasm through IC Build. Admit/start a server, construct an instance and install
the declared topology. Stop/drain captured canisters before snapshot capture.
Create a fresh instance for each independent test. Tests of restoration explicitly
restore their captured state and validate readiness. Keep application assertions
and recovery policy in the consuming test suite.

Snapshots cover captured canister state. They do not automatically reset simulator
time, extra canisters, pending work, subnet state or external resources. Declare
and independently verify the domains relevant to each recipe.

## API at a glance

| Area | Surface |
| --- | --- |
| Async client | Upstream `ic_testkit::pocket_ic::nonblocking::PocketIc`, `ic_testkit::pocket_ic::PocketIcBuilder` |
| Server and instance custody | `PocketIcStartupConfig`, `PocketIcManagedServer`, `PocketIcInstance` |
| Installation | `pic::create_and_install`, `InstallSpec`, `StandaloneCanisterFixture` |
| Typed calls | Independent [`ic-testkit-wire`](wire-adapter.md): `query_wire`, `update_wire`, `WireCallSpec` |
| Snapshots | `pic::capture_snapshots`, `ControllerSnapshots`, `CachedPocketIcBaseline` |
| Reporting/readiness | `pic::collect_canister_diagnostics`, `advance_and_tick`, `tick_until` |

Harness functions take `&PocketIc`; import policy types explicitly. Pure parsing,
formatting and Wasm generation remain synchronous; simulator operations are async.

## PocketIC ownership and concurrency

A `PocketIcInstance` privately pairs a client with its optional managed server.
Use `instance()`, `instance_mut()` and `server()` to borrow them. `into_parts()`
transfers teardown ordering to the caller. `from_borrowed_server(client)` records
that the caller retains server ownership.

Await `owned.close()` to delete the instance before releasing an owned server.
Raw clients expose explicit `drop().await`. Rust Drop performs no client network
I/O; an owned server's Drop still terminates/reaps its process group and stops all
its instances. Borrowed servers must outlive their clients' explicit deletion.

Independent instances can overlap. There is no global client/runtime lock.
Keep the caller-owned runtime alive through instance operations and explicit close. Reused
HTTP connections must not outlive their runtime. Blocking coordinators require a
multithread runtime whose network drivers can continue while they wait.
## Typed Wire calls

Select the independent `ic-testkit-wire` adapter and align the consumer's Wire
minor identity with that adapter. Pass the exact target, caller, exported method, method ID, request/response
limits and borrowed request to `query_wire` or `update_wire`, then await the result.
Responses require `DeserializeOwned`; request and response budgets are independent.

Wire owns application encoding. Testkit performs one client operation and returns
typed encoding, call or decoding errors with method context. It never replays an
uncertain update. Display omits payloads. Raw calls remain available for deliberately
malformed bytes and exact original-reply evidence. Reply admission follows the
client's allocation and is not a network pre-allocation guarantee.

## External protocol calls

Encode/decode unavoidable external Candid through Platform, then await the raw
client call. The client also retains its external Candid helpers. Owned application
RPC uses Wire. Native Candid dependencies do not enter Testkit's Wasm graph.

## Startup and runtime provenance

`Spawn` accepts an already admitted executable; `Connect` borrows an explicit URL.
`from_env` prefers an explicitly configured `IC_TESTKIT_POCKET_IC_URL` over
`POCKET_IC_BIN`, and binary selection performs a bounded version probe.
Provisioning owns exact archive authentication separately.

Await `config.build_instance(builder)` for construction. The single startup deadline
covers readiness and instance creation. Timeout, port/protocol and child
failures preserve their typed causes, bounded output and secondary cleanup evidence.
Cancellation reaps an owned server. A borrowed-server creation may remain uncertain
after a lost response or cancellation and requires caller reconciliation. Upstream
construction/transport panics propagate unchanged.

The caller enables Tokio's I/O/time drivers. Readiness and network waits yield;
small file reads, process spawning and exact owned-child cleanup remain synchronous.
The deadline cannot preempt synchronous work within a future poll.

`start_managed_server()` starts one server without creating an instance; keep its
handle while using its URL. The optional server hard TTL is independent of the
server's operation-idle TTL. Caller-selected output files survive startup failure.

`run_command` and `ic-testkit-server run -- COMMAND` scope a separate process to
an owned or borrowed server and pass its URL. They retain their synchronous command
contract, child-group custody and caller-owned cancellation predicate.
See [the protocol/lifecycle boundary](POCKET-IC.md).

## Canister installation and fixtures

Await `create_and_install(pic, spec)` to create, optionally top up and install.
Upstream creation/funding/install panics propagate.
Diagnostics are separately requested; installation does not print/fetch logs.
`StandaloneCanisterFixture::install` consumes its owner; panic unwinding releases
managed-server custody. Keep a borrowed owner outside the helper for diagnostics.

`reply-fixtures` supplies synchronous raw reply module generation with explicit
method/name/reply byte ceilings. `held-reply-fixtures` adds async simulator-backed
barrier control. Construct/inspect/release held replies asynchronously; module
encoding remains pure. The caller creates, funds and installs the provider and
owns catalogs, payload interpretation and readiness.

### Install-code rate limiting

Await `retry_install_code(pic, policy, async_callable)`. Only the typed install-code
rate-limit rejection permits another attempt. The caller selects the attempt
count and simulated cooldown. Other rejections return without replay; panics propagate.
Cooldown advances selected simulated time and performs exactly two ticks.

## Snapshots and cached baselines

Capture/restore operations are async and controller-aware. Capture validates the
complete target set before effects, preserves deterministic canister order and
rolls back known captures after a returned rejection. Ordinary upstream take/load
replaces the fork's transfer engine. Cancellation or panic can leave partial
captures. The caller selects restore funding and owns timer/reset policy.

`CachedPocketIcBaseline::capture(instance, targets, metadata).await` retains instance,
snapshots and caller metadata. Stop/drain the complete set before capture/restore,
then reach readiness only after the full restore and non-snapshot reset succeed.
Automatic progression must stay disabled while those invariants matter.

Baseline pools and reset recipes are retired. Use fresh instances for independent
tests; explicit snapshot tests retain their own stop/drain, funding, reset and
readiness sequence. See the [migration](client-migration.md#baseline-pool-retirement).

## Diagnostics and time

Await `collect_canister_diagnostics(pic, request)` for independent status/log outcomes.
Senders and record/byte ceilings are explicit. Batch diagnostics validate all labels
before effects, attempt every selected canister in order and retain each failure.
Await `write_canister_diagnostics` for explicit output to a caller-owned writer;
writer failures return unchanged. Reports are secondary evidence, not replacements
for the original test failure.

`advance_and_tick` advances once and delivers exactly the selected ticks.
`tick_until` awaits a caller-owned readiness predicate before mutation and after
each permitted round. Round budgets bound simulated progression, not elapsed
network time. Zero budgets still permit the initial check; failures stop immediately.

## Building test artifacts

Depend on `ic-build` directly for Cargo execution, declared inputs and invocation-
owned captures. Persistent Wasm and generic transaction caching are retired.
Testkit owns simulator integration. The private probe
and installed Wire test demonstrate this dependency direction without a re-export.

## Installed Wire qualification

The private development package retains an installed Wire control-canister test.
The optional fixture-reuse benchmark driver and reporting workflow are retired;
[historical measurements](fixture-reuse-benchmark.md) retain their original scope.

## Deterministic principals

`fake_principal(seed)` derives stable fixture principals from numeric seeds.
It does not create identities or credentials.

## Scope boundaries

Host owns byte/stream, filesystem and process primitives. Build owns Cargo recipes,
input evidence, artifact caches and package qualification. Text owns generic
formatting/parsing. Testkit owns simulator
helpers; upstream owns the client. The unused Native package is retired. IC Storage owns
memory-IO metering; Canic owns its operational Metrics integration.

Consumers own tools/admission, schemas, targets, budgets, retries, credentials,
installation policy and paid-operation recovery. The client never owns a canister
application's lifecycle or interprets unknown effect outcomes as safe retries.
See the [extraction contract](../extraction.md).

## Toolchains and checks

Use the workspace's pinned toolchain and [focused host commands](../hosts.md).
The async-only client and helper contracts need Linux and separate native macOS
Intel/Apple Silicon evidence. Ordinary development uses focused checks; expensive
hosted/native CI is asynchronous. Source provenance, feature graphs, installed
server behavior and publication qualification remain distinct evidence.
