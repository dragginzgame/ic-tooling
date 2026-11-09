# Async-only PocketIC client

Testkit 0.87 removed the blocking client. Testkit 0.86 and
earlier retain the old API. The maintained implementation remains in this
package; PocketIC server 16.1, external DTO identities, protocol bytes,
no-replay behavior and snapshot transfer budgets are unchanged.

## Pending Testkit 0.95 migration

Use `PocketIcInstance` without a type parameter. Its concrete client is
`ic_testkit::pocket_ic::PocketIc`; borrowing, explicit `close().await` and
managed-server custody retain their existing contracts. No generic client
adapter or compatibility alias remains.

Native response bodies use Host HTTP's bounded reader under PocketIC's original
absolute deadline. `ClientError::BodyAllocation` reports fallible buffer
allocation with the original request/accepted-ingress context. Include that
variant in exhaustive matches. Ordinary unbounded calls gain no application
byte ceiling, and snapshot calls keep their caller-selected HTTP bounds.
Transport errors, replay refusal and protocol bytes retain their contracts.

## Application-owned logging in 0.89

Clients no longer inspect `POCKET_IC_LOG_DIR` or `POCKET_IC_LOG_DIR_LEVELS`,
create directories/files, start a logging worker or initialize a process-global
subscriber. Configure logging at the application entrypoint before constructing
clients. Testkit retains its imported tracing events for compatibility; choosing
a subscriber, fields, clock, limits, sink and retention belongs to the caller.

Use the canonical `ic-logging::Log` for retained application logs. An application
that needs imported client events can project selected tracing metadata into its
own bounded log through an application-owned subscriber/layer. The installed
`client_installed` logging test demonstrates this projection with a registry-only
test dependency; no native adapter crate or subscriber engine is added to Testkit.
The production graph retains `tracing` events and excludes `ic-logging`,
`tracing-appender` and `tracing-subscriber`. Protocol bytes, async teardown and
accepted-ingress custody are unchanged.

## Imports and calls

### Inherent harness methods in 0.94

Remove imports and re-exports of `CanisterInstallExt`, `PocketIcSnapshotExt`,
`PocketIcDiagnosticsExt` and `pic::prelude`.
`try_create_and_install`, `wait_out_install_code_rate_limit`, `retry_install_code`,
`capture_snapshots` and `restore_snapshots` are inherent `PocketIc` methods;
ordinary method calls and their arguments remain unchanged. Fully qualified
trait calls become `PocketIc::method(...)`. Generic bounds on either retired
trait must instead use the concrete client. No compatibility traits remain.
The three diagnostic collection/writing helpers are also inherent methods.

Known sibling migrations are Canic's testing exports and installation/snapshot/diagnostics
callers, and Query's governance test imports. Their source is outside this local
change; consumers must make these import changes when adopting 0.94.

### Async client construction

Import `ic_testkit::pocket_ic::{PocketIc, PocketIcBuilder, ClientError}`.
The public `nonblocking` module is removed. All network operations are async.
Replace `builder.build_async().await` with `builder.build().await`, and
`config.build_instance_async(builder).await` with
`config.build_instance(builder).await`. Existing blocking callers add
`.await` to construction, network calls, Wire calls and lifecycle helpers.
There is no compatibility wrapper, private runtime or automatic client Drop I/O.
Read `client.instance_id` instead of `instance_id()`, and use
`SnapshotTransfer::advance(...).await` instead of `advance_async`. The obsolete
builder-thread spawn/disconnection startup errors are removed.

The caller owns a Tokio runtime with I/O and time enabled. Async tests can use
`#[tokio::test]`; synchronous test coordinators can own a runtime at their
application entrypoint. Libraries must not rebuild blocking wrappers per method. Keep the runtime alive
through every client/pool operation and explicit close. Reused HTTP connections
belong to that runtime; do not create a runtime per pooled lease. Blocking test
coordinators need a live multithread runtime so network drivers keep progressing.

`PocketIc::new(server_url, request_timeout).await` and
`PocketIc::list_instances(server_url, request_timeout).await` require explicit
endpoints and positive finite `Duration` budgets.
Builders without a URL return `ClientError::MissingServerUrl`. Provision with
`ic-testkit-server setup` or `pic::server_assets::setup`, admit installations
with `check`, and start a selected binary through `PocketIcStartupConfig`.

## Supported surface in the 0.88 hard cut

Benchmark marker emission, parsing, aggregation/comparison and the report example
are removed. Testkit's portable library retains deterministic principals without
a Runtime dependency. Application measurement formats and reports belong to
their consumers; the private fixture-reuse benchmark remains maintained.

The client removes sender-info query/update/submit variants, server-side
`verify_canister_signature`, `upgrade_eop_canister` and flexible HTTP response
mocking, including its mock-response DTOs. Ordinary calls and
`*_with_effective_principal` retain the same bytes and routing. There are no
replacement aliases or automatic behavior changes. Consumers needing a removed
operation must provide an explicit external-protocol adapter and qualify it.
HTTP outcall discovery still decodes both replication modes; ordinary HTTP
response mocking, stable memory, live gateways and snapshot transfer remain.

The old count/boolean `SubnetConfigSet` is removed; configure subnets through
the supported builder methods. Unused server-oriented
`ApiResponse`, `HttpGatewayDetails` and `BinaryBlob` DTOs are also removed.
The client continues decoding current server responses through its typed
request/receipt and error owners.

### Instance configuration

Testkit's PocketIC DTOs no longer implement `schemars::JsonSchema` or
`strum::IntoEnumIterator` in production. Schema/report projections belong to
consumers. Serde codecs and external management/Principal identities remain
unchanged; exhaustive rejection-code iteration is retained in Testkit's tests.

`PocketIcBuilder` creates NNS, II, system and application subnets, selects initial
time or automatic progress, gateways and caller-owned persistent state paths.
`IcpFeatures` retains registry, cycles minting, ICP token, cycles token and II
provisioning. Canic's existing feature selections remain supported. The server
owns subnet validation, feature provisioning and defaults for other settings.

Custom creation via `new_with_config` and public instance/subnet configuration
DTOs is removed. Subnet state import, benchmarking presets, special-subnet
conveniences, Bitcoin/Dogecoin node addresses, ICP flag/log overrides, mainnet
NNS IDs and disabled ingress validation are removed. Use `with_initial_time`
instead of the retired deprecated timestamp setter. Topology responses still
decode all server subnet kinds, instruction configurations and cost schedules.

Use `with_state_dir(path)` for a caller-owned persistent directory. Temporary
`PocketIcState` handles, `with_state`, `with_read_only_state`, `copy_dir` and
`drop_and_take_state` are removed. Retain the path yourself and await `drop()` or
the managed instance's `close()`; deletion never removes that caller-owned
directory. These changes neither rewrite nor remove existing instance state.

## Testkit 0.90 surface cut

All client constructors require a caller-selected `Duration`:
`PocketIcBuilder::new(request_timeout)`, `PocketIc::new(server_url, request_timeout)`
and `PocketIc::new_from_existing_instance(server_url, instance_id, request_timeout)`.
Attachment now returns `Result<PocketIc, ClientError>`; propagate or handle it.
Remove `with_max_request_time_ms` and supply the duration at construction.
Instance listing takes the same explicit budget. Zero and host-clock-overflowing
durations return `ClientError::InvalidRequestBudget` before network activity;
there is no default or unbounded branch. One deadline covers each exchange's
dispatch, response body, accepted-operation polling, pauses and best-effort prune.
Startup's total deadline and snapshot-transfer byte budgets remain separate.

Baseline recipes implement `snapshot_funding() -> SnapshotRestoreFunding` and
`reset_non_snapshot_state(...) -> Result<(), Self::Error>`. The recipe owns any
required/achieved reset schema and must reject missing or mismatched evidence
before returning success. Testkit removes its reset-policy enums,
`ResetPolicies`, `ResetRequirements`, `ResetReceipt` and `BaselinePoolContractError`.
`PreparedBaseline::Restored` is now a unit variant. Reset errors retain their
typed source in `BaselinePoolPreparationError::Recipe`; classification defaults
to `RebuildReason::ResetFailure` or a recipe-selected reason. The removed
`ResetCoverageMismatch` reason can be represented by the consumer's
`RecipeClassified` code when that distinction matters. Snapshot restore/funding
still precedes reset, readiness and final validation. Cancellation invalidation,
one recovery build, explicit close and collect-all shared shutdown remain intact.

The client exports `PocketIc` once and no longer exports `call_candid`,
`call_candid_as`, `query_candid`, `query_candid_as`, `update_candid`,
`update_candid_as`, `with_candid` or `get_default_effective_canister_id`.
Management calls keep their private Candid boundary. For explicit external
protocols, compose the selected codec with the raw client's `query_call`,
`update_call` or effective-principal variant. Owned RPC keeps the existing
`pic::query_wire`/`update_wire` adapters. There are no replacement aliases.

`await_call_no_ticks`, authenticated `ingress_status_as` and its
`IngressStatusResult` wrapper are removed. Use `ingress_status` for anonymous
status reads with caller-owned polling cadence and wall-clock deadlines;
use the existing `await_call` when simulator ticking is intended. Anonymous
status keeps its optional result, canister rejection, forbidden-response error
and accepted-ingress custody. Testkit no longer selects a backoff policy for
no-tick polling.

`tick_with_configs`, `TickConfigs`, `SubnetBlockmakers`, `RawTickConfigs`,
`RawSubnetBlockmakers`, `RawNodeId` and `delete_subnet` are removed. Ordinary
`tick` still sends `{"blockmakers":null}` and preserves accepted-operation
polling and no-replay failures. Topology no longer offers `get_cloud_engines`,
`get_verified_app_subnets`, `get_benchmarking_app_subnets`, `get_bitcoin`,
`get_fiduciary`, `get_sns` or `get_system_subnets`. Keep using `get_app_subnets`,
`get_nns`, `get_ii` and canister-range `get_subnet`; the full `subnet_configs`
map and its server DTOs remain available for explicit inspection.

Server-filesystem `canister_snapshot_download`/`canister_snapshot_upload` and
their `RawCanisterSnapshotDownload`, `RawCanisterSnapshotUpload` and
`RawCanisterSnapshotId` DTOs are removed. Use the retained bounded
`pocket_ic::snapshot::SnapshotTransfer` and explicit metadata/data operations;
filesystem layout, pending snapshot custody and uncertain-effect recovery remain
caller-owned. The existing `pic` snapshot adapter continues to use this owner.

`BaselinePoolTimings` now has documented public fields. Replace `timings.build()`
with `timings.build`, and likewise for `wait`, `restore`, `reset`, `readiness`,
`validation`, `stale_teardown` and `total`. `BaselinePoolError::timings()` and
the pool outcome's `timings` field keep their existing roles. Canic and other
consumers using explicit timing getters must make this source migration.

## IC packet codecs

Platform owns read-state envelope encoding and certificate/subnet-metrics decoding.
Testkit retains the HTTP exchange, status admission and certificate tree lookup.
Match `ClientError::Cbor { kind, source }` with Testkit's `CborErrorKind`:
`InputTooLarge`, `OutputTooLarge`, `Decode` or `Encode`. Replace the former
tuple-variant match and Platform error-kind imports once. The original typed
codec failure and its nested cause remain in `source`; ordinary recovery uses
the Testkit classification without a supplier downcast or direct Platform
dependency. Choosing to downcast to a supplier error still couples that caller
to its version. Limits, protocol bytes, request budgets and no-replay custody
are unchanged. Testkit declares no direct CBOR, certificate or transport DTO
dependency. See the [identity inventory](../extraction.md#public-supplier-identities)
for deliberate management, Principal and subnet-metrics interoperability.

`get_subnet_metrics` retains its optional result: a missing tree path or a present
CBOR null leaf returns `None`; malformed data is an error. Decoding does not
authenticate certificates or add retries, replay or application verification.

## Explicit teardown and cancellation

Await `owned.close().await` on a `PocketIcInstance` to delete its
instance while retaining the managed server. For a raw client, await
`client.drop().await` while its server remains available. Rust Drop performs
no client network calls. Dropping an owned server still terminates/reaps its
process group and stops all its instances. Borrowed servers stay caller-owned.

Dropping a network future does not prove that its operation was rejected.
Preserve accepted operation/ingress identities and reconcile uncertain effects;
do not replay updates automatically. Cancelling managed construction reaps its
owned server. A borrowed-server creation may need caller reconciliation after
cancellation or a lost response. Explicit close is also fallible and must not
be retried implicitly. Snapshot capture rolls back known work on returned failures;
cancelling its future cannot run async rollback. Retain isolated server custody or
reconcile temporary snapshots explicitly on a borrowed server.

## Helpers and baseline pools

Await installation, snapshot capture/restore, diagnostics, simulated-time helpers,
held-reply inspection and Wire calls. Pure Wasm fixture generation and benchmark
analysis remain synchronous. Retry/readiness callbacks are async callables.

Implement asynchronous `PocketIcBaselineRecipe` lifecycle methods, including
reset and validation. Recipe futures are Send and metadata is Send + Sync.
`StandaloneCanisterRecipe` builders return a Send future. A closure that owns
shared inputs can clone them before returning `async move { ... }`.

Await `pool.acquire().await`. Tokio owns FIFO capacity admission and queued
cancellation. Cancelled preparation retains its slot value for teardown and forces
a rebuild with `RebuildReason::PreparationCancelled`; it never exposes partially
reset state. Fatal failures, one permitted recovery build and both recovery causes
remain distinct. Dropping a lease returns its slot; it does not delete the baseline.
Await `pool.close().await` after all leases end, before stopping a borrowed
server; it returns every instance deletion failure. Do not discard those failures.
For process-wide or `static` pools, await `pool.shutdown().await` through a shared
reference. Shutdown is terminal: queued/new acquisitions return
`BaselinePoolError::Closed`, while already admitted work and held leases must
finish before deletion. Release every lease before awaiting shutdown in the same
task. Inspect all returned deletion errors before stopping a borrowed server.
Concurrent shutdowns serialize and each call reports its own deletion attempts;
completed attempts are never retried. Cancellation keeps admission closed and
retains unattempted baselines. Reconcile the unknown outcome of an in-flight
deletion explicitly before shutting down the external server.

Wire requests, local baseline metadata and diagnostic writers may be non-Send;
use their futures on the calling runtime. The pool's separate Send requirements
permit concurrent recipe preparation.

## Dependency and provenance boundaries

The client is native-only. Wasm measurement retains Runtime/Types without HTTP,
server, Tokio, tempfile or Candid. Platform owns binary Candid and management
DTOs, with canonical Types Principal and explicit work/allocation budgets.
There is no registry management dependency. Native certification/transport
supplier identities retain private byte boundaries; IC transport still selects
upstream Candid transitively. No PocketIC feature returns to Platform. Earlier Platform client imports must migrate directly
to Testkit; no compatibility exports are supplied.

[Provenance](../../crates/ic-testkit/pocket-ic-provenance/README.md) retains the
exact donor and upstream reconstruction. The maintenance patch records this cut;
original donor/upstream hashes are unchanged. Use the focused commands in
[host qualification](../hosts.md). Linux evidence is separate from native macOS
Intel and Apple Silicon qualification.

## Owned management and Principal hard cut

The pending adoption replaces the registry management DTO and external Principal
identities with Platform's owned types. Use `ic_platform_types::Principal` (also
exported by Platform); do not select `principal-external` or import
`pocket_ic::CanisterId`. Pass canonical principals directly to client, fixture and
snapshot methods. Management types stay with `ic_platform::management_canister_types`.

`CallLimits` now selects `candid_bytes`, `header_bytes`, `type_entries`, `work` and
`allocation`, alongside the outer `http_bytes`. Work and cumulative logical
allocation charges belong to the owned codec; they cannot reuse numeric upstream
decoding/skipping quotas as equivalent policy. `ClientError::CandidEncode` and
`CandidDecode` retain the owned codec's typed refusals. Snapshot ingress receipts
and uncertain-outcome custody remain available; no call is replayed automatically.

No stable-memory format or simulator persistence migration is required. Required
external Candid bytes are preserved. Native IC transport still brings its upstream
Candid/Principal identities; Wasm Testkit remains Types-only. The complete published
Platform/Types/Runtime/Wire cohort must pass the downstream graphs before release;
local path-selected evidence alone does not qualify registry identities.
