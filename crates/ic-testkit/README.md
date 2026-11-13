<p align="center">
  <img src="https://raw.githubusercontent.com/dragginzgame/shared-assets/main/ic-testkit/ic-testkit-readme-header.svg" alt="IC Testkit — Internet Computer helper library" width="100%">
</p>

# ic-testkit

PocketIC-oriented test utilities for Internet Computer canister tests.

Host-side tests select Testkit for lifecycle helpers and its `pocket_ic` re-export
for the upstream client. Host Tools releases independently.

```bash
cargo add --dev ic-testkit
```

Cargo records the selected versions in the consuming manifest and lockfile.
When aligning an existing stack, select its qualified versions explicitly.

For typed application calls, use the independent
[`ic-testkit-wire`](../ic-testkit-wire/README.md) adapter. Testkit does not select
a Wire generation or own application codecs.

The minimum and pinned development compiler is Rust 1.100.0, and the selected PocketIC line is 16.
Testkit's portable library provides deterministic test principals through
`ic-platform-types`, without an IC runtime dependency. The private performance
probe selects Runtime's entrypoints directly. Native simulator operations use
`ic_testkit::pocket_ic::nonblocking::PocketIc` and
`ic_testkit::pocket_ic::PocketIcBuilder`; `pic` supplies
host-only lifecycle and fixture helpers. Platform owns the native snapshot and
held-fixture management codecs.
Standalone recipe builders return `std::io::Result<StandaloneCanisterFixture>`.

Upstream owns builder configuration, native Principal/management identities and
client behavior. The native-only re-export preserves those exact upstream types;
a separate `pocket-ic` dependency is unnecessary. Testkit maintains no client fork. See the
[breaking migration](../../docs/testkit/client-migration.md).

External interfaces that require Candid remain caller-owned: use a direct
Platform dependency for encoding/decoding, then send the bytes through the
upstream PocketIC client's raw operations.
PocketIC retains its upstream Candid dependencies; Testkit's Wasm profile remains
Candid-free. Native Testkit selects Platform's owned codec and management DTOs
for coherent snapshots; `held-reply-fixtures` uses the same adapter owner.

The [Wire adapter migration](../../docs/testkit/wire-adapter.md) describes typed
query/update calls, explicit codec budgets and the independent publication boundary.
Raw PocketIC calls remain available for any caller-selected codec or retained
frame evidence. There is no `wire` feature in Testkit.

This is the PocketIC package in the IC Tooling workspace. It provides:

- upstream rejection values and panic behavior, with no automatic replay
- canister install and retry helpers
- explicit PocketIC startup, retained server/instance ownership, async construction, and
  structured child/readiness failures
- explicit authenticated `ic-testkit-server setup`, offline `check`, and a shared
  environment startup contract with `ic-testkit-server run -- COMMAND`
  for suites spanning several test processes
- one recipe baseline pool for single- and multi-canister PocketIC reuse
- deterministic fake principals
- controller-aware, caller-labeled collect-all diagnostics
- explicit diagnostic reporting to a caller-selected writer; status/log failures
  stay in the report and writer I/O errors return unchanged

`ic-testkit` does not wrap the PocketIC simulator API or serialize independent
instances. `pic::server_assets` owns explicit server provisioning and offline
verification for Rust callers and the published CLI; tests never install implicitly.
Rust fixtures call `server_assets::check(&directory)` and pass the returned
absolute executable path to `PocketIcStartupConfig::Spawn`, without invoking the CLI.
Explicit `server_assets::setup` takes a caller-owned cancellation predicate.
Tests normally retain one
`PocketIcInstance` and use `owned.instance()` for simulator operations;
free `pic` functions provide reusable harness behavior. Import native client and
DTO identities directly from `pocket_ic`. See the [client migration](../../docs/testkit/client-migration.md).
The client and managed server remain privately paired. Use `server()` for shared
server inspection, await `close()` for explicit instance deletion, and use
`into_parts()` only when taking over teardown ordering. The client and network
helpers are async-only; the caller owns the runtime. Rust Drop performs no client
network I/O. Close and pool shutdown return `()` and await upstream best-effort
deletion; they provide no borrowed-server deletion acknowledgement.
Wrap a client on an externally owned server with
`PocketIcInstance::from_borrowed_server(client)`.
See the [0.66 migration](../../docs/changelog/0.66.md).

```bash
cargo install --locked --features cli ic-testkit
ic-testkit-server setup
ic-testkit-server check
ic-testkit-server run -- cargo test --locked
```

The `cli` feature builds the server command; library-only consumers omit it.
For Shared Tooling's lockfile-selected installer, also pass `--features cli`:

```bash
bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh --consumer "$PWD" \
  --package ic-testkit --lockfile Cargo.lock --bin ic-testkit-server \
  --profile release --features cli
```

Setup selects Testkit's reviewed PocketIC 16.1.0 archive for Linux x86-64,
macOS Intel or macOS Apple Silicon. It authenticates the archive's pinned digest,
bounds decompression and admits executable bytes/version before publication.
Setup/check print the absolute executable path; diagnostics go to stderr.
`--directory DIRECTORY` selects a physical root (default
`.tools/ic-testkit-server` in the working directory). Checks and run never
download, update or repair a server. Failed attempts and previous bundles remain
retained; a changed installation requires a fresh selected root.

Run uses that prepared selection when neither `POCKET_IC_BIN` nor
`IC_TESTKIT_POCKET_IC_URL` is selected, and exports the executable and managed
URL to the command. Explicit binary overrides admit stable 16.x servers,
independently of the Rust client's protocol-version constant. URL mode borrows an
existing server. For long pre-client work, select `run --idle-ttl SECONDS -- COMMAND`
or the `Spawn` variant's `server_idle_ttl` field. This selects operation-idle
lifetime independently of `--ttl`'s hard limit; omitted idle selection retains
PocketIC's default. Both options require owned-server mode and positive seconds.

Most users should read the
[repository README](https://github.com/dragginzgame/ic-tooling#readme) for
setup, examples, local checks, and release notes.
The [documentation index](https://github.com/dragginzgame/ic-tooling/blob/main/docs/testkit/README.md)
links current usage guidance and historical design records.

Startup failures expose `PocketIcStartupError::failure()`
and `PocketIcStartupFailure` for cause matching. Read bounded server excerpts
through `output()`, and inspect typed secondary failures through
`command_cleanup()` and `server_cleanup()`. The original cause remains primary;
raw output files remain caller-owned.

Use direct `ic-host-artifacts`, `ic-host-fs` and `ic-host-process` dependencies
for generic artifact streams, filesystem operations and process custody. Testkit
selects gzip for its reviewed PocketIC server assets; its normal dependency graph
does not select Host's Wasm-inspection feature. Consumers select the capabilities
they use rather than inheriting them through Testkit.

The published archive includes a `CHANGELOG.md` navigation file linking the
[canonical Testkit ledger](../../releases/testkit/CHANGELOG.md) and its
historical upgrade actions. Current build and artifact-cache contracts belong to IC Build;
historical Testkit artifact imports are not current entry points.

Snapshot baselines prepare metadata-upload snapshots through upstream client
management calls, preserving global timer and low-memory-hook state alongside
Wasm, heap, stable memory, globals, certified data and the chunk store. Initial
capture needs two free snapshot slots per target, transfer cycles and bounded
message buffers; it deletes the temporary source after complete preparation.
Warm reuse loads the prepared snapshot without retransferring it. Missing state
or incomplete data refuses capture and rolls back known snapshots. Cancellation
or upstream panics can leave unknown outcomes requiring caller reconciliation.
No shared server filesystem, client fork or timer-reset shim is introduced.

Custom recipes must stop/drain the whole captured set before capture and in
`prepare_restore`, then restart only after complete capture or successful
restore/reset in `drive_to_readiness`. Keep automatic progress disabled and
exclude concurrent instance operations. The standalone recipe handles its
single canister's stop/start lifecycle. Simulated time and external effects are
not rewound. Returned rejections retain known cleanup IDs; upstream panic or
cancellation can leave partial captures requiring caller reconciliation.

The repository also includes a complete
[multi-canister baseline recipe](https://github.com/dragginzgame/ic-tooling/blob/main/crates/ic-testkit/examples/multi_canister_baseline_pool.rs)
compiled by the crate's normal all-target checks. The
[transactional external-artifact example](https://github.com/dragginzgame/ic-tooling/blob/main/crates/ic-build/examples/transactional_artifact_cache.rs)
is compiled and maintained by IC Build.

Baseline recipes select `snapshot_funding()` and validate their own non-snapshot
reset evidence before returning `Ok(())` from `reset_non_snapshot_state`.
The pool restores captured snapshots with that funding, then runs reset,
readiness and final validation before exposing a lease. Consumer recipes own
reset-policy schemas and mismatched/missing evidence failures. See the
[client migration](../../docs/testkit/client-migration.md#snapshot-boundary).

Generic Cargo/Wasm builds, retained artifact caches and freshness APIs belong to
[IC Build](https://github.com/dragginzgame/ic-build). Depend on it directly and use
`ic_build::artifacts`; Testkit keeps PocketIC lifecycle helpers.

Pass `fixture.pocket_ic()` and `fixture.canister_id()` explicitly to
`ic_testkit_wire::query_wire` or `update_wire`. The standalone fixture owns
installation and lifetime; Wire owns method contracts and codec limits.
See the [canonical migration notes](../../releases/testkit/CHANGELOG.md).

### Raw reply canister fixtures

Enable `ic-testkit`'s `reply-fixtures` feature to use
`pic::fixtures::raw_reply_canister`. Supply exact raw reply bytes and explicit
aggregate method-count, UTF-8 method-name and reply-byte ceilings. Empty or
control-containing names, duplicate exports and unsupported Wasm32 extents are
rejected before encoding. Method names must be unique across query and update exports.
The generated memory has a fixed maximum and can hold replies spanning pages.
This helper does not decode Candid, install canisters, select principals or fund
instances; keep product reply catalogs and assertions in the consuming tests.

### Held provider replies

Enable `held-reply-fixtures` for
`pic::fixtures::held::{HeldReplyFixture, HeldRequest, HeldReplyError}`.
Create and fund the provider canister, then construct `HeldReplyFixture::new`
with its principal, the PocketIC instance, an update-only `RawReplyMethod` catalog,
`RawReplyLimits`, cycles per barrier call and a compiled-module byte allowance.
Install `fixture.wasm().to_vec()` into that canister yourself.

Each catalog entry is a single-use request slot with immutable original reply
bytes. Repeated method names consume slots in catalog order; other methods can
arrive in any order. `limits.methods` bounds total lifetime calls, and the name
and reply allowances sum every slot. Exhaustion rejects instead of recycling
identities. Incoming application payloads are ignored; keep protocols, caller
authority and expected results in your tests.

Submit the consumer call with PocketIC's `submit_call`. Tick with your own finite
bound until `fixture.pending(&pic)?` reports the expected slot; `request.slot()`
identifies its original catalog position. Observation does not tick. Invoke the
cancellation/revocation endpoint and assert its committed state while the original
ingress is still outstanding. Call `fixture.release(&pic, &request)?`, then await
the original ingress and assert the stale callback's application-specific result.
Different slots can be released out of order. Tokens bind the exact HTTP request,
fixture, subnet, server and instance; unknown/repeated release refuses.

This uses PocketIC's supported manual HTTP mock facility as a barrier, without a
live provider or a second application engine. Use only manual progress: never
enable `auto_progress`/`make_live`, advance simulated time, or upgrade/reinstall/restore
the fixture with held calls. Constructor, observation and release refuse automatic
progress, but cannot prevent a caller enabling it independently. Automatic progress
can make actual HTTP requests. Keep time fixed to avoid the management HTTP
60-second timeout. Timeout/rejection fails the original call; it does not return
the configured reply. Never await a held call before releasing it.

Do not clone/reconstruct a controller over live calls. A transport failure during
release is uncertain and consumes its token; abandon the isolated instance rather
than retrying. Dropping the controller performs no cleanup or simulator operation;
the caller owns instance teardown and any diagnostic retention. No state survives
a fresh fixture installation. Native Platform management types/encoding adapt the
external Candid HTTP interface; application replies remain uninterpreted. Host's
WAT compiler is selected only by this feature. Deployed Wasm graphs contain none
of these native fixture, encoder, Host or Candid dependencies.

### Cycle-accepting provider replies

Enable `held-reply-fixtures` and supply an explicit `u128` acceptance cap:

- `pic::fixtures::cycle_reply_canister(methods, limits, accept_cycles, module_bytes)`
  generates immediate replies.
- `HeldReplyFixture::new_with_cycle_acceptance(pic, canister, methods, limits,
  call_cycles, accept_cycles, module_bytes)` adds acceptance to the existing held
  controller. `call_cycles` funds its HTTP barrier independently of acceptance.

Use update-only `RawReplyMethod` slots with caller-owned raw reply bytes and
`RawReplyLimits`. Each slot is one admitted invocation, including repeated method
names; the same cap applies independently to each slot. Actual IC acceptance is
`min(attached_available, accept_cycles)`. Zero explicitly accepts nothing; a cap
above the available amount accepts only what was attached. Exhaustion rejects
before acceptance. Construction and installation accept nothing. Existing
`raw_reply_canister` and `HeldReplyFixture::new` continue accepting no cycles.

Immediate acceptance commits with the reply. For held responses, acceptance
commits in the original update before the HTTP barrier is observable. Releasing
the slot only returns its original bytes; it never accepts more cycles. A later
HTTP timeout/rejection does not undo that earlier committed acceptance. A trap
before the barrier commits rolls back acceptance and leaves the slot unused.
Unaccepted cycles refund through the actual IC
callback; malformed/application-error reply bytes do not undo acceptance either.
Keep reply decoding, accounting, authorization, duplicate-payment prevention and
recovery policy in the consumer. The held lifecycle restrictions above still apply.

Native qualification reads actual callback refunds and both canister balances.
System-subnet tests isolate exact transfers from execution fees; a separate
application-subnet case proves the same refund path with normal fees. Gross
application-subnet balance deltas also include execution/transport/HTTP costs.
These fixtures never mint cycles or fund/dispatch a live provider automatically.
No new dependency is selected, and deployed Wasm graphs remain unchanged.

## Toolchains and checks

Testkit is part of the IC Tooling workspace. Use the root
[host guide](../../docs/hosts.md) and [publication guide](../../docs/publishing.md)
for setup and group releases.

- `make testkit-check`: Testkit libraries and private driver with the prepared server.
- `make testkit-wasm-check`: deployed profiles.
- `make testkit-package-check`: all public archives at their own versions, including tests.
- `make testkit-browser-check`: browser-session support.

The [0.62 migration](../../docs/changelog/0.62.md) replaces the old independent
Testkit release flow. The [Testkit ledger](../../releases/testkit/CHANGELOG.md) covers
the published Testkit package; retired Native releases retain their own ledger.
Private qualification packages are unpublished. Host Tools releases independently.
Neither package checking nor ordinary validation
authorizes downloads, publication or artifact cleanup.
