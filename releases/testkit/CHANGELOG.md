# Testkit family changelog

This ledger owns Testkit and the private development/probe packages. Native's
[separate ledger](../native/CHANGELOG.md) owns subsequent releases; its history
through 0.79.0 retains its original scope below. The published baseline at the split is 0.76.1,
from the [original joint integration release](../integration/CHANGELOG.md#0761---2026-11-01).
Joint history through 0.76.1 remains there once, with its original scope.

## [0.93.1] - 2026-11-08

### 🩹 Fixed

- Download the unchanged PocketIC 16.1 server through GitHub's official asset
  API, avoiding the browser download route's observed gateway failures. Archive
  digests, HTTPS/size/deadline limits and executable/version admission remain.

### 🧹 Cleanup

- Share submit/await ingress mechanics between ordinary and bounded snapshot
  calls. Public signatures, routing bytes, limits, rejection results and accepted
  ingress evidence remain unchanged; uncertain operations are never replayed.
- Consolidate native HTTP test-worker custody across client-failure, startup and
  simulated-time fixtures. Each scenario retains its own responses, framing
  admission, request budgets, cancellation controls and replay assertions.
  Original client/donor provenance remains reconstructible.

## [0.93.0] - 2026-11-07

### ⚠️ Breaking

- Match `ClientError::Cbor { kind, source }` using Testkit's `CborErrorKind`
  instead of the former Platform-error tuple variant. Input/output admission
  and decoding/encoding remain distinct, and the original typed causal chain
  is preserved. Unrelated Platform facade updates no longer change ordinary
  error matching ([#340](https://github.com/dragginzgame/ic-tooling/issues/340)).
  Platform 0.25 still owns the codecs; management DTOs remain exactly 0.11.0.
  Server custody, deadlines and protocol behavior retain their existing contracts.

### 🔧 Changed

- Adopt Wire 0.17.12 in native calls and the Wasm control canister, retaining
  the compatible Wire minimum and existing codecs and payload bytes.
- Adopt Runtime 0.20 and its matching entrypoint macros and singleton guard for
  the private Wasm control canister. Its existing direct Runtime imports remain
  supported; the published Testkit API, native dependency graph and Wire
  protocol are unchanged.

### 🧪 Testing

- Observe accepted polling and ingress-wait requests before exercising deadline
  expiry or cancellation. Use a controlled test clock and held replies instead
  of racing multi-request fixtures against millisecond wall-clock budgets.
  Preserve receipt, no-replay and cancellation assertions; production budgets
  and native client behavior remain unchanged.

## [0.92.0] - 2026-11-07

### ⚠️ Breaking

- Adopt Platform 0.24's `CborError` identity in native client failures. Align
  direct Platform dependencies when inspecting those errors. Management DTOs
  remain exactly 0.11.0; server custody, deadlines and protocol behavior retain
  their existing contracts.

### 🔧 Changed

- Qualify Wire 0.17.8 for native calls and the Wasm control canister without
  raising the compatible Wire minimum or changing owned payload bytes.

## [0.91.0] - 2026-11-07

### ⚠️ Breaking

- Adopt Foundation 0.75's public execution and cleanup error identities. Align
  direct Host dependencies when inspecting Testkit's startup and teardown errors;
  server custody and protocol behavior retain their existing contracts.

### 🔧 Changed

- Qualify locked Platform 0.23.6, Runtime 0.19.5 and Types 0.18.2 across the
  native client and Wasm probe. Preserve PocketIC 16.1's exact management DTO
  identity and existing compatible dependency ranges.

### 🧹 Cleanup

- Retire the private fixture benchmark driver, worker and report framework.
  Keep the installed Wire acceptance test and private control canister; archive
  qualification and the explicit Internet Identity fixture remain supported.
  Probe/Types dependencies are test-only, and unused native development
  dependencies and file-capture features are removed.

## [0.90.0] - 2026-11-07

### ⚠️ Breaking

- Require a caller-selected positive finite `Duration` for PocketIC construction,
  attachment and instance listing. Remove default/unbounded request budgets;
  invalid values fail before network effects, and polling/body reads share one
  deadline. See [#339 — Require explicit finite PocketIC request budgets](https://github.com/dragginzgame/ic-tooling/issues/339).
- Move reset-policy schemas and evidence matching into consumer recipes. Recipes
  select snapshot funding and validate their reset before returning success;
  the pool retains restore ordering, cancellation invalidation and recovery.
  See [#338 — Move baseline reset-policy schemas into consumer recipes](https://github.com/dragginzgame/ic-tooling/issues/338)
  and the [migration](../../docs/testkit/client-migration.md#testkit-090-surface-cut).
- Remove unused anonymous/authenticated Candid convenience exports and default
  effective-canister-ID lookup. Keep the management codec helpers private;
  callers use raw query/update methods with their explicit external codec, or
  the existing Wire adapters for owned RPC.
- Remove server-filesystem snapshot download/upload methods and their endpoint
  DTOs. Retain the bounded snapshot transfer API and caller-owned recovery.
- Remove `await_call_no_ticks`, authenticated `ingress_status_as` and
  `IngressStatusResult`. Use anonymous `ingress_status` with caller-owned polling
  cadence and deadlines, or the existing `await_call` when ticking is intended.
- Remove custom blockmaker ticks, subnet deletion and unused topology shortcuts.
  Ordinary `tick` keeps its protocol bytes; application/NNS/II discovery and
  canister-range lookups remain supported, with complete topology DTOs retained.
- Read `BaselinePoolTimings` through its documented public fields; remove the
  forwarding getters without compatibility aliases.

### 🧹 Cleanup

- Reuse Host Process's bounded socket acceptance and HTTP request reader in
  client failure tests. Keep stalled/truncated responses and no-replay assertions.
- Preserve original PocketIC donor hashes through a checksummed surface-cut layer
  and its tamper-refusal check.
- Remove Testkit's direct backoff dependency and unused Reqwest blocking,
  multipart and stream features. Preserve anonymous status bytes, rejection/error
  behavior and accepted-ingress custody without retries or replay.

## [0.89.4] - 2026-11-06

### 🧹 Cleanup

- Centralize canister-scoped management-call routing without changing public
  signatures, protocol DTOs, sender selection or fallible/no-replay behavior.
  Preserve the independent donor reconstruction with a checksummed routing patch.
- Share empty-canister setup across pool and concurrency tests while keeping
  lifecycle assertions and fault injection in their owning tests.

## [0.89.3] - 2026-11-06

### 🧪 Testing

- Deliver Testkit through Shared Tooling 0.30.9's captured setup and recovery
  hooks, preserving deferred checkout edits and using guarded reconciliation
  after publication. Public Rust contracts and dependency selections are unchanged.

## [0.89.2] - 2026-11-06

### ➕ Added

- Await `CachedPocketIcBaselinePool::shutdown(&self)` for shared or process-wide
  pools. Shutdown rejects queued/new acquisitions, waits for leases, attempts
  every cached deletion and returns every failure without replaying requests.
  Keep the runtime and borrowed server alive through completion.

### 🧪 Testing

- Use Shared's frozen source for maintainer release and publication, deferring
  concurrent normal-checkout dependency edits. Normal release commands include
  publication; use the retained batch ID for recovery.

## [0.89.1] - 2026-11-06

### 🧪 Testing

- Include the PocketIC management server URL and instance ID in the local II
  fixture connection record so consumers can install application canisters in
  the same instance. Document borrowed attachment and fixture-owned teardown.

## [0.89.0] - 2026-11-06

### ⚠️ Breaking

- PocketIC clients no longer read `POCKET_IC_LOG_DIR` or
  `POCKET_IC_LOG_DIR_LEVELS`, create log files, retain a logging worker or install
  a global subscriber. Applications own their `ic-logging` sink and any projection
  of imported tracing events. See the
  [logging migration](../../docs/testkit/client-migration.md#application-owned-logging-in-089)
  and [#333 — Return client logging setup to the application](https://github.com/dragginzgame/ic-tooling/issues/333).

### 🧪 Testing

- Supply a pinned local Internet Identity browser fixture with explicit provider
  identities and trust roots. Real AuthClient sign-in, virtual-passkey use and
  signed-ingress acceptance retain certificate/signature rejection controls.
  Consumers reuse the repository fixture at an immutable Git revision; no new
  published API or authentication policy is added. See the
  [fixture guide](../../testing/browser/internet-identity/README.md) and
  [#336 — Real local Internet Identity acceptance](https://github.com/dragginzgame/ic-tooling/issues/336).

## [0.88.0] - 2026-11-06

### ⚠️ Breaking

- Remove benchmark log markers, parsing, aggregation, comparison and the report
  example. Consumers own measurement/report formats; Testkit no longer needs
  Runtime on Wasm. The private fixture-reuse workload and installed Wire/trap
  qualification remain supported.
- Remove unused PocketIC sender-info call variants, server-side signature
  verification, Motoko EOP upgrade convenience and flexible HTTP response
  mocking. Ordinary and explicitly routed calls retain their protocol bytes,
  typed failures and accepted-ingress custody. See the
  [client migration](../../docs/testkit/client-migration.md).
- Drop the unused count/boolean subnet configuration and orphaned server-side
  response/blob DTOs. Configure instances through the supported builder methods;
  creation DTOs and subnet specifications are now internal.
- Narrow the PocketIC builder to the configurations used by maintained consumers.
  Remove custom subnet/state cloning, compiler-limit presets, Bitcoin/Dogecoin
  nodes, configuration overrides and unused system-canister features. Persistent
  state directories, NNS/II/system/application subnets, selected ICP features,
  initial time/progress and gateways remain supported. The builder carries one
  typed creation request; the server retains validation and provisioning.
- Remove temporary state handles and state-taking deletion. Callers retain their
  persistent path and await explicit deletion; existing state directories are
  preserved. Testkit drops its direct `slog` and `tempfile` dependencies.
- Remove unused JSON-schema and runtime enum-iteration implementations from
  Testkit's PocketIC DTOs. Serde wire bytes and external DTO identities remain
  unchanged. Schema generation leaves the dependency graph; enum iteration
  remains a dev-dependency for exhaustive protocol-catalog tests.

### 🔧 Dependencies

- Include the incoming compatible Platform 0.23.2 and Wire 0.17.3 lock selections
  in release qualification, retaining the exact management DTO identity.

## [0.87.0] - 2026-11-05

### ⚠️ Breaking

- Adopt Platform 0.23's typed read-state and certificate codecs for subnet metrics.
  `ClientError::Cbor` now carries Platform's `CborError`. PocketIC transport,
  status handling, tree lookup, absent metrics and upstream DTO identities stay
  unchanged; decoding does not authenticate a certificate or authorize replay.
  Certificate, transport DTO and CBOR dependencies now enter through Platform;
  direct declarations are removed and the local dependency policy prevents their
  reintroduction.
- Keep one async PocketIC client at `ic_testkit::pocket_ic::PocketIc`. Remove
  the blocking facade, its private runtimes/worker channels and the public
  `nonblocking` path. Await `builder.build()`, `config.build_instance(builder)`,
  network operations and Testkit lifecycle/Wire helpers. Async retry/readiness
  callbacks and Send pool recipe futures replace their synchronous contracts.
  The caller owns the runtime; server 16.1 and external protocol identities stay
  unchanged. See the [migration guide](../../docs/testkit/client-migration.md).
- Replace client destructor I/O with explicit async deletion. Await instance or
  baseline `close()`, or raw `PocketIc::drop()`, while retaining the server.
  Close pools before releasing borrowed servers and retain their deletion failures.
  Owned server Drop still stops/reaps its process group; uncertain remote effects
  require caller reconciliation without automatic replay.

### 🧹 Cleanup

- Refresh compatible locked dependencies, including Runtime 0.19.2 and Candid
  0.10.40, while retaining the exact management DTO pin and native/Wasm boundaries.
- Check integration helper dead-code expectations, use typed install-error
  assertions and derive the example recipe's mechanical error forwarding.
  Declare the private tooling and benchmark package roles for source reports.
- Isolate the private driver's negative Git fixture from ancestor repository
  discovery when caller-selected scratch is beneath the checkout.
- Delegate FIFO pool capacity and queued cancellation to Tokio. Cancelled slot
  preparation forces a rebuild before reuse, preserving its value for teardown.
  Retain fatal/recovery distinctions, snapshot rollback and typed failure evidence.
- Narrow Tokio's production features and migrate workspace callers, benchmark
  release measurement and native contract tests. Preserve donor/upstream source
  reconstruction with a separately checked maintenance patch.

## [0.86.0] - 2026-11-04

### ⚠️ Breaking

- Adopt IC Wire 0.17 for typed calls and probe fixtures. Consumers exchanging
  Wire limits or errors with Testkit should select the same minor line. Core
  codec builds no longer pull in Wire's optional schema derive; frame bytes,
  budgets and simulator behavior remain unchanged.

### 🧪 Testing

- Reuse a dedicated archive compiler cache across qualification attempts. Cargo
  rebuilds every extracted workspace package, including changed same-version
  payloads, while retaining compiled registry dependencies. Each attempt still
  packages, extracts, admits and tests its exact sources; evidence stays separate
  from the owned compiler cache and normal workspace artifacts.

## [0.85.1] - 2026-11-04

### 🧹 Maintenance

- Adopt Platform 0.22 for native Candid and management adapters. The maintained
  PocketIC client, its public DTO/error identities, canonical Types 0.18 and
  native/Wasm feature boundaries remain unchanged.

## [0.85.0] - 2026-11-04

### ⚠️ Breaking

- Require an explicit server URL in the maintained client. `new` and
  `list_instances` take an endpoint; builders without one return
  `ClientError::MissingServerUrl`. Remove `with_server_binary`, `StartServerParams`
  and the implicit downloader/launcher. Provision and start through `pic` instead;
  managed instance builders supply their server URL. Remove direct flate2 and
  semver dependencies. See the client migration guide.

### 🧹 Cleanup

- Move private unit tests and helpers into test-only modules across Testkit,
  its CLI, the development drivers and the probe. Preserve test names, Cargo
  target roots and production visibility.
- Simplify 71 blocking client forwarders without changing their request/error
  semantics. Preserve donor/upstream reconstruction through a separately checked
  maintenance patch.

## [0.84.1] - 2026-11-03

### 🧹 Maintenance

- Adopt Platform 0.21's external-protocol adapters after its PocketIC client
  retirement. Testkit retains the maintained client, public error/DTO identities,
  management SDK 0.11, Types 0.18, Runtime 0.19 and server 16.1.0. Consumer imports
  and native/Wasm feature boundaries are unchanged.

## [0.84.0] - 2026-11-03

### ⚠️ Breaking

- Own the maintained PocketIC 16.1 client in `ic_testkit::pocket_ic`, so client,
  DTO/error types and lifecycle helpers share one Testkit version. Replace
  `ic_platform_pocket_ic` imports and remove the separate client dependency.
  Preserve external IC identities, typed failures, uncertain-operation custody
  and the native-only dependency graph
  ([#324](https://github.com/dragginzgame/ic-tooling/issues/324)).
  See the [migration guide](../../docs/testkit/client-migration.md).

### 🧹 Cleanup

- Delegate snapshot copying to the maintained client's bounded transfer engine.
  Keep Testkit's exact sender, funding and rollback policy, recording each upload
  identity before copying data. Preserve the existing management reply and chunk
  limits while bounding the outer HTTP envelope to 16 MiB.

## [0.83.0] - 2026-11-03

### ⚠️ Breaking

- Adopt Platform and its independent PocketIC client 0.20. Align direct client
  dependencies when exchanging Testkit's exposed client, DTO and error types;
  exhaustive client-error matches must handle `ResponseTooLarge`. Runtime 0.19,
  Types 0.18, Wire 0.16 and the PocketIC server 16.1.0 remain selected.
  Existing snapshot helpers retain their current transfer behavior and limits.

## [0.82.0] - 2026-11-03

### ⚠️ Breaking

- Adopt IC Wire 0.16 for native typed calls and the private Wasm probe. Align
  direct consumer dependencies to Wire 0.16 when supplying `WireCallSpec` limits
  or matching Wire error causes; these are exposed dependency identities.
  Keep the existing feature boundaries and private control method IDs/budgets.
  Tooling does not select Wire's optional Platform schemas or send Account DTOs;
  consumers using Account must follow Wire/Types' own byte and identity migration.

## [0.81.0] - 2026-11-03

### ⚠️ Breaking

- Adopt Platform, PocketIC and Runtime 0.19 with canonical Types 0.18.
  Client operations now return upstream `ClientError` directly, preserving
  request context and accepted operation/ingress identities. Remove Testkit's
  `PocketIcOperationError`, `PocketIcCallFailure`, `try_pocket_ic_operation` and
  `try_pocket_ic_call` panic-string adapters. Startup uses `BuilderFailed`.
- Make simulated advancement, install cooldowns and retries fallible. Stop on
  client/cooldown failure; retry only explicit install-rate-limit rejections.
  Diagnostics retain typed status/log failures and writer I/O errors; unrelated
  collection/formatting/writer panics propagate. Align direct consumer dependencies
  to the selected family. The caller-selected PocketIC server remains 16.1.0.

### 🧹 Cleanup

- Share typed benchmark workload settings and concurrency admission between the
  driver and worker. Report admission uses only the requested workload, retaining
  the existing command syntax, strict report schema and measurement boundaries.
- Allow the private archive qualification driver to select explicit public
  packages for group plans. Its default retains the complete public roster;
  unknown, private and duplicate selections refuse before creating artifacts.

## [0.80.2] - 2026-11-02

### 🧹 Cleanup

- Select `ic-platform-pocket-ic` directly for native simulator operations, so
  client adoption can follow its independent release group. Keep Platform for
  Candid and management adapters without its `pocket-ic` feature. Existing client
  and DTO identities and the PocketIC 16.1.0 server pin are unchanged.
- Keep server custody and final admission together during blocking and async
  instance construction, preserving cancellation cleanup and the original error
  evidence. Separate benchmark input/provenance preparation from run scheduling.
- Reuse Wasm fixture preparation without merging the distinct held-reply and
  cycle-refund programs. Generate the published integration consumer's manifest
  and identity checks from one selection, retaining registry-only admission and
  the facade's existing version exception.
- Match held-reply releases against raw pending requests without allocating all
  pending tokens. Preserve exact request/fixture identity, admission and transport
  error precedence, and consumption before uncertain dispatch.

## [0.80.1] - 2026-11-02

### 🧹 Cleanup

- Remove the Metrics dependency from Testkit's native benchmark analysis. Keep
  checked scaled averages private to that analysis, preserving exact floor
  rounding, zero-denominator precedence, representable wide intermediates and
  overflow errors. Enforce Metrics-free native/Wasm dependency graphs.
  Ownership follow-up: [Shared #347](https://github.com/dragginzgame/shared-tooling/issues/347)
  and [Metrics #127](https://github.com/dragginzgame/ic-observability/issues/127).

## [0.80.0] - 2026-11-02

### ⚠️ Breaking

- Adopt IC Wire 0.15 for native transport and the private Wasm performance probe.
  Consumers exchanging `WireCallSpec` limits or matching typed codec errors must
  select Wire 0.15 too. Preserve the core frames, caller-selected budgets and
  optional native transport boundary; no metadata bridge is enabled.

### 🔧 Changed

- Release Native allocation measurement independently with `RELEASE_GROUP=native`.
  Testkit releases no longer bump Native; the private interoperability fixture
  no longer depends on Storage-specific memory metering.

### 🩹 Fixed

- Accept Cargo's version-string and table dependency declarations in the
  registry-only integration verifier. Preserve exact versions and readback evidence.

## [0.79.0] - 2026-11-02

### ⚠️ Breaking

- Adopt canonical Types 0.17 and Runtime 0.18 directly, with Platform/PocketIC
  client 0.18 for native adapters. Align consumers passing client-owned DTOs or
  principals. The installed PocketIC server remains 16.1.0.
- Native's optional stable-memory adapter uses IC Memory 0.63's public backing
  interface, retaining the upstream Memory trait identity. Its native-only
  dependency closure includes IC Memory's record codec and stable-only Platform
  0.17 backend; default Native and Wasm remain dependency-free.

- Require Rust 1.99.1 and pin contributor/release compilation to that version.
  Native qualification now owns the former compiler-lane feature and Wasm
  obligations; the stable MSRV status reuses its results. Older compiler support
  is retired; package release groups remain independent.
  [#313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313).

- Benchmark marker formatting and canister emission now have one checked path:
  `format_marker` and `performance::measure` return grammar errors. Replace
  `try_format_marker`/`try_measure` with those names and handle their results.
- `pair_benchmark_spans` pairs only within the same prefix, stream, suite and
  label. Replace `pair_benchmark_spans_by_origin` with that canonical function;
  independent producers sharing those fields still need caller separation.
- Parsing, pairing, aggregation and comparison are native-only. Wasm retains
  marker counters, checked formatting, emission and deterministic principals;
  its library graph no longer selects Metrics. Move analysis to the host.

### 🔧 Changed

- Adopt Metrics 0.26 with the already-selected Rust 1.99.1 compiler. Preserve
  native arithmetic-only benchmark averages, existing errors and the Metrics-free
  Wasm graph; no transport features are enabled.

- Select published Types 0.17 directly for the external Principal identity.
  Select Runtime 0.18 directly for Wasm IC0 and the private performance probe
  entrypoints. Testkit and probe Wasm graphs now exclude the Platform facade;
  raw Testkit remains executor-free. Native Candid/PocketIC adapters, marker
  formats and caller-owned policies are unchanged.

### 🧹 Cleanup

- Require shared public caller contracts in private Dev qualification and reuse
  those tests for registry-only post-publication verification. Remove the manual
  integration fixture's duplicate release rendering and obsolete peer assumptions.
- Reuse bounded process-test isolation for server-asset qualification. Keep
  setup cancellation, authentication, retained failure evidence and child reaping.
- Keep current host guidance separate from the preserved qualification archive,
  and use one installation example without hand-maintained dependency versions.

## [0.78.0] - 2026-11-01

### ⚠️ Breaking

- Align optional Wire transport with IC Wire 0.14. Consumers passing limits to
  `WireCallSpec` or inspecting codec errors must select Wire 0.14 too. This also
  aligns the optional Platform Types bridge with Platform 0.17's owned types;
  frame formats and caller-selected limits remain unchanged.

### 🔧 Changed

- Adopt Metrics 0.25 for benchmark averages with default features disabled.
  Preserve exact scaled averages and Testkit's existing error types without
  enabling Metrics' native Wire metadata.

## [0.77.0] - 2026-11-01

### ⚠️ Breaking

- Adopt IC Platform 0.17.0 and its maintained PocketIC client. Consumers must
  use `ic_platform::pocket_ic` from Platform 0.17 for client and client-owned DTO
  identities passed to Testkit helpers. Method signatures, simulator behavior
  and the PocketIC 16.1.0 server pin remain unchanged.

### 🔧 Changed

- Select Platform's independent Runtime 0.17.1 for Wasm system calls and the
  private performance probe; preserve the executor singleton and native-only
  client boundary.

## [0.76.2] - 2026-11-01

### 🔧 Changed

- Release Host Tools and the Testkit family independently in the existing workspace.
  Select `RELEASE_GROUP=testkit` for this group; package versions and public
  contracts are unchanged. Existing integration journals retain their original
  source, policy and recovery context. ([#299](https://github.com/dragginzgame/ic-tooling/issues/299))
