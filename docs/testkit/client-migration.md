# Async-only PocketIC client

Testkit 0.87 removed the blocking client. Testkit 0.86 and
earlier retain the old API. The maintained implementation remains in this
package; PocketIC server 16.1, external DTO identities, protocol bytes,
no-replay behavior and snapshot transfer budgets are unchanged.

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

`PocketIc::new(server_url).await` and
`PocketIc::list_instances(server_url).await` require explicit endpoints.
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

## IC packet codecs

Platform 0.23 owns read-state envelope encoding and certificate/subnet-metrics decoding.
Testkit retains the HTTP exchange, status admission and certificate tree lookup.
`ClientError::Cbor` now carries `ic_platform::certification::cbor::CborError`;
code naming the former backend error type must adopt that owner. Testkit no
longer declares the CBOR, certificate or transport DTO crates directly.

`get_subnet_metrics` retains its optional result: a missing tree path or a present
CBOR null leaf returns `None`; malformed data is an error. Decoding does not
authenticate certificates or add retries, replay or application verification.

## Explicit teardown and cancellation

Await `owned.close().await` on a `PocketIcInstance<PocketIc>` to delete its
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

Wire requests, local baseline metadata and diagnostic writers may be non-Send;
use their futures on the calling runtime. The pool's separate Send requirements
permit concurrent recipe preparation.

## Dependency and provenance boundaries

The client is native-only. Wasm measurement retains Runtime/Types without HTTP,
server, Tokio, tempfile or Candid. Principal, management SDK 0.11, certification and
transport identities stay unchanged. Platform supplies Candid/management adapters
without a PocketIC feature. Earlier Platform client imports must migrate directly
to Testkit; no compatibility exports are supplied.

[Provenance](../../crates/ic-testkit/pocket-ic-provenance/README.md) retains the
exact donor and upstream reconstruction. The maintenance patch records this cut;
original donor/upstream hashes are unchanged. Use the focused commands in
[host qualification](../hosts.md). Linux evidence is separate from native macOS
Intel and Apple Silicon qualification.
