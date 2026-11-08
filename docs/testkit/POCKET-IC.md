<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-testkit/ic-testkit-readme-header.svg" alt="IC Testkit — Internet Computer helper library" width="100%">
</p>

# PocketIC upstream boundary

Testkit maintains the PocketIC 16.1 client in `ic_testkit::pocket_ic`.
The simulator remains external infrastructure. Historical decisions remain in the
[design records](README.md#historical-design-records) and
[qualification receipts](qualification-history.md#historical-qualification-receipts).
Those records do not define current APIs or imply that a proposed upstream change
has shipped.

## Ownership

PocketIC owns the simulator and server protocol. Testkit owns the maintained
native client and client-owned DTO/error identities in `ic_testkit::pocket_ic`.
The client requires an explicit endpoint. The `pic` module supplies lifecycle
and testing helpers. Both ship in one Testkit package and release group.
Platform continues to supply general Candid and management adapters.
See [the client migration](client-migration.md) for imports and provenance.

Testkit owns explicit server provisioning and fixture lifecycle policy. It adds
bounded startup, server/instance custody, contextual calls, installation policy,
controller-aware snapshots, reusable baseline contracts, diagnostics and benchmark
markers. Application topology, reset coverage, funding amounts, byte/work budgets,
readiness predicates and performance thresholds remain caller-owned.

The [repository ownership map](overview.md#scope-boundaries) identifies the other
owners: IC Build supplies generic build/cache and package qualification mechanics;
Host supplies streams, filesystem and process primitives. Testkit owns its
benchmark averages without depending on Canic capabilities such as IC Metrics.

## Server selection and provisioning

Server acquisition and startup require explicit selection:

- `server_assets::setup` and `ic-testkit-server setup` explicitly provision one
  release-bound, reviewed archive. Testkit owns the version/digests and bundle
  layout; Host supplies no-follow reads, gzip hashing/copying, durable publication
  and executable admission.
- `server_assets::check` and the CLI's `check` admit an existing bundle offline.
  They do not repair it or download a missing binary. Failed setup attempts and
  retained installations remain evidence.
- `PocketIcStartupConfig::Spawn` receives an already selected binary. Its caller
  owns byte admission and namespace custody. `from_env` gives an explicitly set
  `IC_TESTKIT_POCKET_IC_URL` precedence over `POCKET_IC_BIN`; binary mode performs
  a bounded version probe, not archive authentication.
- `PocketIcStartupConfig::Connect` borrows an existing server without claiming
  process ownership. It does not infer the remote server's executable identity.

`POCKET_IC_SERVER_VERSION` selects the provisioning release. The separate
`supports_pocket_ic_server` predicate admits stable 16.x probe output, matching
the client protocol range. The upstream `LATEST_SERVER_VERSION` is available as
an upstream fact; it does not silently choose Testkit's provisioning artifact.

An upstream resolver could replace part of this integration if its published
contract preserves explicit download policy, byte authentication, offline admission
and usable failure evidence. Testkit's release pins and caller-owned trust policy
would still need an explicit owner.

## Bounded startup and process lifetime

Testkit's async `build_instance` uses the explicit configuration, preserve a typed
`PocketIcStartupFailure`, and return `PocketIcInstance` with the concrete client
instance and its server lifetime. Startup cancellation/failure cleans up the
owned child; borrowed-server failure leaves ownership with its caller.

`start_managed_server` exposes the same launcher without building an instance.
Its `PocketIcManagedServer` can support several independent instances through an
explicit URL. On Unix, Host's lifecycle engine keeps custody of the exact process
group through descendant termination and leader reaping. Caller-owned output files
remain available after failure; bounded excerpts and secondary cleanup errors are
available through the structured startup error.

The port path must be absent before spawning PocketIC. The launcher owns a private
directory and waits for a newline-terminated port while also checking child exit.
The server's default operation-idle lifetime remains distinct from an optional
hard lifetime; neither creates a default deadline for arbitrary caller commands.

`PocketIcStartupConfig::run_command` scopes a command to an owned or borrowed
server and supplies `IC_TESTKIT_POCKET_IC_URL`. The CLI's `run -- COMMAND` delegates
to that implementation. Cancellation or owned-server exit terminates the owned
command group and preserves failure evidence. External server processes remain
caller-owned. No process-global server registry or instance lock is involved.

Upstream typed, deadline-aware construction could retire local orchestration when
it supplies equivalent cancellation, custody and error contracts. The existing
synthetic and real instance-lifetime tests qualify those obligations separately
from ordinary simulator behavior.

### Bounded instance teardown

The construction deadline ends when startup returns. All client network operations
use the caller's Tokio runtime. Await `owned.close().await` to delete an instance
while retaining its managed server, or await `client.drop().await` before releasing
a separately held server. Rust Drop performs no client network I/O. Dropping a
managed server still stops its instances and terminates/reaps its process group.
Borrowed servers and uncertain deletion outcomes remain caller-owned.

The controlled loopback test observes DELETE, confirms that explicit close is
waiting, then acknowledges it and verifies completion. Its parent bounds its own
waits and owns child termination/reaping; no PocketIC binary is required:

```bash
cargo test -p ic-testkit --locked --test pocket_ic_teardown \
  instance_close_waits_for_the_deletion_response -- --exact --nocapture
```

### Temporary-directory ownership

Keep temporary directories available for the lifetime of any server using them.
The managed launcher retains its startup directory until its child has been
terminated and reaped. Borrowed or upstream background servers remain their
caller's responsibility; validation does not remove their files or discover and
terminate unrelated processes. CI retains the caller's temporary-directory
environment instead of adding a scratch lifetime shorter than those servers.
Earlier server-directory failure observations remain in the historical host
receipts; they are not fresh qualification of every selected server release.

## Calls, installation and diagnostics

PocketIC's upstream Candid helpers remain upstream APIs. Testkit offers optional
method-bound Wire calls for owned application endpoints and raw operations for
caller-owned external protocol adapters. It does not add a Candid codec layer.

The maintained async client returns typed transport, body, server, codec and
canister failures. Accepted operation/ingress identities survive failures; an
uncertain update is never replayed automatically. Wire delegates to that same
client and preserves the typed cause. Unrelated Rust panics propagate. The former
panic-string transport adapter and blocking client are absent.

`PocketIc::try_create_and_install` preserves create, funding and installation failures. Retry
policy uses `ErrorCode::CanisterInstallCodeRateLimited` and caller-selected
attempt/cooldown values. Diagnostics are a separate explicit operation:
`PocketIc::collect_canister_diagnostics` retains status, controller-aware logs and bounded
rendering. Installation does not silently fetch or print diagnostic logs.

Upstream error enrichment could reduce these adapters if it preserves context,
original rejects and bounded decoding. Merely adding another typed-call spelling
would not remove their current responsibilities.

## Snapshot baselines and reuse

PocketIC owns canister snapshots. Testkit captures/restores a declared snapshot
set and retains original rejection/operation causes. A baseline pool owns the
recipe and canonical restoration; recipes own non-snapshot reset work, readiness
and invariant checks. Reuse is admitted only when achieved reset coverage matches
requirements. Canister snapshots do not automatically restore PocketIC time,
extra canisters, pending work or external resources.

Each pool bounds its own leases and resource use. Independent PocketIC instances
are not serialized through a host-wide guard. Failed reuse disposes of the stale
slot and may rebuild once under the recipe's failure policy; caller panics still
unwind. These are test-fixture guarantees, separate from IC Build's persisted
artifact caches and application deployment recovery.

## Benchmarking and provenance

Testkit owns the `ICTK` marker format, span pairing and counter interpretation.
Canister-side `performance::counters` reads the system API through `ic0`; the private probe
uses CDK entrypoint macros to qualify actual marker emission and trap handling.
Testkit owns checked benchmark averages. The native report example demonstrates
consumer-owned output schemas, files and workload policy.

Callers that need runtime provenance should retain the exact selected path,
admission evidence and observed version. A borrowed URL does not establish those
facts. The installed Wire test selects its server explicitly and consumes
IC Build's retained Wasm rather than reconstructing mutable build paths.

## Qualification and future retirement

Use the [current host matrix](hosts.md#current-focused-qualification) for
focused commands and declared native support. Source review and Linux execution
do not establish native macOS qualification.

Retire a local workaround only after checking the selected upstream implementation,
its actual consumers and success/failure/cancellation behavior. Keep supported
fixtures and caller policy unless an equivalent published interface gives them a
clearer owner. Historical proposals and similarly named functions alone are not
proof that the local obligation has expired.
