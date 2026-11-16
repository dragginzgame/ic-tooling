# Testkit changelog

Package: `ic-testkit`. Consumer adoption remains separate from publication;
each qualification record identifies its actual supplier and source capture.

## [0.107.3] - 2026-11-16

### 🔧 Changed

- Adopt Platform Protocol 0.2.6 with only `http-request`; retain compatible Types
  0.21.2. The HTTP encoder is unchanged, preserving fixture request bytes, bounds
  and unspecified pricing; new Caffeine and snapshot features remain disabled.

### ✨ Added

- Optional `managed-server` capability for explicit PocketIC 16.1.0 archive
  preparation, offline verification, bounded owned startup/output and retained
  server/instance lifetime. Reuse Process and FS owners; preserve original client
  types, caller-owned runtimes and localhost gateways. Explicit deletion reports
  attempts, while server stop retains process/cleanup and abandonment evidence.
  [Tooling #6 — Prepared simulator assets and server lifecycle](https://github.com/dragginzgame/ic-tooling/issues/6).

## [0.107.2] - 2026-11-16

### 🐛 Fixed

- Permit compatible Platform Types patches (`>=0.21.2, <0.22.0`) instead of
  requiring one exact patch. Testkit can share Types 0.21.2 with Runtime 0.23.13
  in a native-driver/Wasm-probe workspace, removing the supplier conflict from
  [#5](https://github.com/dragginzgame/ic-tooling/issues/5). Keep PocketIC's original
  types and assert exact Principal bytes across the explicit conversion boundary.

### 🔧 Changed

- Adopt Platform Protocol 0.2.5 with only `http-request` and Types 0.21.2;
  preserve original PocketIC types and unspecified HTTP pricing. The new optional
  pricing-version encoder and Registry capabilities are not selected.
  Unit and installed module/controller checks, observed instance deletion,
  Clippy and default-feature isolation pass on Linux.

## [0.107.1] - 2026-11-16

### ⚠️ Breaking

- Move the public fixture Principal boundary to Platform Types 0.21.1. Explicit
  Principal callers must select the same cohort; PocketIC's original client/type
  identity and canonical text conversion are unchanged.
- Replace the legacy Platform HTTP DTO/general encoder with Protocol 0.2.3's
  bounded HTTP GET encoder. `CompileError::Encoding` carries its output-limit
  or allocation refusal. Generated Candid/Wasm bytes may differ; management
  request semantics and immutable callback payloads are preserved.

- Replace the legacy Testkit surface with original PocketIC types and the
  optional `held-reply-fixtures` catalog, generator and exclusive controller.
  Consumers migrate explicitly; no legacy server launcher, snapshot helpers or
  compatibility facade is shipped in this package. Callers own servers and instances.

### 🧪 Testing

- Independently observe the migrated HTTP request's URLs, method, headers,
  slot bodies, response budget, replication and pricing through PocketIC.
  Retain reverse release, original replies, refusal and deletion checks.

- Prepare registry metadata and archive qualification for default and held-reply
  features, including installed module/controller checks and observed instance
  deletion with an explicitly prepared external server owner.

- Qualify current Scheduling rewrite pressure and held-upgrade against the local
  controller through an export-only migration patch. Both drivers share one local
  supplier; external lock admission, driver Clippy, endpoint refusal and observed
  instance deletion pass.
- Add focused watchdog admission/pressure and Jobs pressure selections, including
  empty HTTP inventory between subjects. Released and replacement suppliers pass;
  independently captured source identities are recorded because unselected
  retention fixtures changed between captures. Other modes remain unqualified.
- Add a single-capture released/replacement comparison for watchdog trap recovery,
  owner-command precedence, delivery-refusal backoff and upgrade reconstruction.
  Existing assertions pass for both suppliers, with endpoint refusal and observed
  instance deletion. Consolidate installed selectors into one qualification patch;
  no product API or sibling source change is needed.
- Compare the actual synchronous executor driver on one capture: three lifecycle
  and three admission checks pass with released and replacement suppliers. Verify
  the replacement's selected default graph excludes WAT and Platform fixture
  dependencies; keep endpoint/instance ownership external and check deletion.
- The 0.107.0 checkpoint remains unpublished; 0.107.1 is the first replacement
  release. The sibling source and its supplier selection remain unchanged.
- Compare current Scheduling retained-callback custody and dropped-handle
  settlement with both suppliers on one capture. Preserve existing consumer
  assertions and focused selectors; endpoint refusal, instance deletion, external
  lock admission and driver Clippy pass without a product API change.
- Qualify current Jobs calendar results, owner precedence, upgrade reconstruction
  and downtime with released and replacement Testkit on one source capture.
  Preserve consumer assertions and retry selectors; no new shared API is needed.
- Compare 29 current Jobs retry, reconciliation and completion scenarios with both
  suppliers. Preserve budgets, effect identity, explicit recovery authority and
  completion-relative deadlines; both runs pass endpoint refusal and deletion.
- Compare eight transient-lifetime and seven unresolved-upgrade scenarios with
  both suppliers, preserving trapped custody, stale-delivery refusal and explicit
  reconciliation before retry. No product implementation changes are needed.
- Qualify initial and one-shot deadlines, schedule/progress boundaries and bounded
  catch-up against both suppliers on one current capture. Preserve exact occurrence
  bytes, non-replay, paused upgrades and catch-up quotas across trap recovery.
- Follow the verified Scheduling rewrite relocation to `../ic-scheduling`, with
  ancestry admission rejecting legacy consumer source. Qualify completion cadence,
  committed diagnostics and low-cycle delivery refusal with both Testkit suppliers.
- Qualify the complete current Once aggregate with released and replacement
  Testkit on one capture, including inline authentication/lifecycle and
  control/dispatch cost probes. Require explicit prepared launcher/server paths
  after legacy tool locations were removed; matching tool bytes preserve identity.
- Qualify the paired metrics-disabled/enabled probe with both suppliers on source
  bytes identical to the aggregate run. Preserve callback behavior, exact counter
  results and probe bytes; retain observations without a performance-gain claim.
- Refresh the six executor lifecycle/admission checks on those same source bytes,
  retaining default-feature isolation. Once aggregate, metrics-feature and executor
  comparisons now share an exact current source identity with both suppliers.

## [0.107.0] - GitHub checkpoint 2026-11-15 (unpublished)

### Added

- Establish the original PocketIC 16.1.0 client/type re-export as the shared test
  boundary. No runtime, server launcher, instance pool or consumer scheduler.
- Add an owned immutable update-reply catalog behind `held-reply-fixtures`.
  Complete admission checks duplicate-slot aggregate limits, method names and
  Wasm32 representation before copying; ownership allocations are fallible.

### Dependency selection

- Select PocketIC 16.1.0 at the shared type boundary. Its exact thiserror 2.0.18
  requirement selects that permitted version for the workspace; previous external
  identities are otherwise retained. Build/bytes do not acquire the simulator graph.

### Validation and remaining work

- Focused tests cover exact/over-budget aggregates, later invalid names, empty
  catalogs, duplicate order and opaque page-spanning reply ownership.
- Scheduling continues using released Testkit; its installed
  baseline is not replacement acceptance. No old source or test tree was imported.

### Generated module follow-up

- Add bounded `Catalog::compile` using Platform-owned HTTP encoding and escaped
  WAT. Duplicate update methods consume immutable single-use reply slots; failed
  barriers and exhausted methods reject. No simulator operations or release
  authority are created by compilation.
- Five catalog/module tests and one installed two-target test pass: exact caps,
  independent Wasm validation, page-spanning bytes, reverse release, differing
  replies, HTTP rejection and exhaustion. Add `make test-testkit-module` for the
  explicitly owned-server check. Consumer migration remains open.

### Exclusive controller follow-up

- Add `Fixture` and opaque `Request` values over the original async PocketIC
  client. Construction binds a distinct namespace without installing or ticking;
  pending observations filter admitted slots, and release verifies exact identity.
- Consume exclusive local release authority before dispatch. Cancellation or an
  upstream panic cannot authorize retry; drop has no simulator effects.
- Six unit tests and two independently selected installed tests now pass. The
  controller checks different replies in reverse order, wrong server/instance,
  foreign or altered tokens, expiry, repeated release and automatic progress.
  Local fault injection covers cancellation/panic uncertainty; it is not a remote
  rollback guarantee. Add `make test-testkit-control` for the owned-server check.
- Scheduling still uses the released supplier pending current-source replacement
  qualification. macOS and publication remain outside this Linux development slice.
