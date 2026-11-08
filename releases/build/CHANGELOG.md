# Build changelog

This ledger owns ic-build: Cargo/Wasm builds, exact inputs and artifact caches.
Its unchanged published baseline is 0.73.6, from the
[original Foundation release](../foundation/CHANGELOG.md#0736---2026-11-01).
Earlier history remains there once, under its original package scope; no version
reset, release, tag or upload is implied by establishing this group.

## [0.79.1] - Unreleased

### ➕ Added

- Extract Canic tool staging/publication, exact Cargo producer exports, private
  environment comparisons, Git observations and compiler sysroot input discovery.
  Callers retain acquisition/retry policy, pins, private key custody, selected
  targets, recipe identities and evidence publication. No new crate is introduced
  ([#344 — Extract reusable Canic build machinery](https://github.com/dragginzgame/ic-tooling/pull/344)).

## [0.79.0] - 2026-11-08

### ⚠️ Breaking

- Remove `WasmBuildBatchConfig` and its batch argument. Configure maintenance on
  each `WasmBuildSpec`; batch execution applies matching policies once per
  canonical target and reports conflicting policies before that entry builds.
  Standalone and batch builds now share one policy owner. See the
  [migration guide](../../docs/build/artifacts.md#pending-build-079-migration)
  and [Canic #742](https://github.com/dragginzgame/canic/issues/742).
- Share built/reused outcome projection through `CacheOutcome<Record>` while
  retaining distinct artifact/Wasm records and domain type names. Import enum
  variants from `CacheOutcome` rather than through the domain aliases.

### 🧹 Cleanup

- Share caller-labeled specifications through `LabeledSpec<S>`, retaining the
  artifact and Wasm domain names and their constructors/accessors.
- Derive release-group admission and fixture iteration from the existing group
  catalog; qualification coverage remains explicit. Publication reads Tooling's
  preflight release context and re-admits it through the sealed receipt, removing
  the second parser for Shared's private release journal.
- Reuse unchanged workflow-selection and scratch-routing fixture evidence across
  release groups, alongside evidence/retention fixtures. Group metadata admission
  and metadata-dependent release/publication tests retain fresh execution.
- Publication reuses a committed release's sealed full workspace lint when its
  payload, source, environment, tools, host and evidence still match. Missing,
  stale, corrupt or focused-only proof falls back to fresh lint; archive, license
  and provenance admission remain fresh.

## [0.78.0] - 2026-11-07

### ⚠️ Breaking

- Remove the compiler-recipe wrapper and its executable. Consumers own their
  Cargo compiler configuration; generic Build no longer records wrapper recipes.
- Remove cross-call prepared input snapshots and source leases. Resolve each
  independent batch through the ordinary batch entrypoint; compatible entries
  still share resolution within that call. Warm-cache and post-build source
  revalidation, exact publication and shared-target lock ordering remain.
- Emit phase-aware heartbeats only. Remove output/lifecycle progress variants,
  output toggles and disabled-heartbeat settings; the interval getter returns
  `Duration`. Use silent entrypoints or batch `None` to disable observation,
  and returned results for completion, cache status and bounded diagnostics.
  See the [migration guide](../../docs/build/artifacts.md#pending-build-078-migration)
  and [IcyDB #630 — Adopt Build 0.78 heartbeat-only progress configuration](https://github.com/dragginzgame/icydb/issues/630).

### 🧪 Testing

- Admit the empty standalone-script source root after retiring the compiler
  fixture, while retaining Candid admission for new script fixtures and symlinks.

## [0.77.0] - 2026-11-07

### ⚠️ Breaking

- Read cache acquisition, input-resolution and failure timing records through
  documented public fields. Remove their forwarding getters without aliases;
  records and reports still expose timings through their existing owners.

### 🧹 Cleanup

- Share the isolated/shared/scheduled cache fixture matrix across warm output,
  retained-entry corruption and source-mutation tests. Keep each scenario's
  assertions and every mode combination.

## [0.76.0] - 2026-11-06

### ⚠️ Breaking

- Use `CacheBatchEntry<T, E>` for both cache batch engines instead of their
  separate entry types. Wasm failures retain their error and phase/timings
  together in `WasmBuildBatchFailure`; consuming an entry returns one complete
  result. Reports retain their distinct domain metrics and collect-all behavior.
- Pass the package name explicitly to the test-support fixture writer; named
  packages, nested workspaces and cache fixtures now share that writer.

## [0.75.0] - 2026-11-06

### ⚠️ Breaking

- Remove unused standalone shared-target inspection and configuration readers.
  Cache ownership mode is now internal; select it through the existing shared
  or isolated builders. Retention builders and their strict/best-effort behavior
  remain supported. Existing maintenance outcomes retain sizes, use times and
  coordination details; no compatibility inspection wrapper remains.

## [0.74.4] - 2026-11-05

### 🩹 Fixed

- Keep Cargo discovery and root-package fixtures valid when caller-selected
  temporary directories are beneath another workspace. Preserve real ancestor
  discovery and explicitly isolate the root-package Cargo fixture. Keep the
  implicit-exclusion fixture outside enclosing workspaces, documenting that
  specific temporary-directory exception.

### 🧪 Testing

- Stop optimizer qualification at a failed tool/dependency setup pipeline, retaining
  the original failure instead of masking it with successful log capture.
- Add Tools and Build focused release contracts, selected only after a matching
  full/focused comparison on Linux and both native macOS hosts. Retain owner
  feature profiles, declared consumers, fresh archives and conservative fallback
  for shared/mixed changes, new callers or missing qualification.
- Admit local dependency archives for selected-package qualification without
  executing unrelated support-package tests. Preserve external selections,
  source/provenance guards, typed command-failure diagnostics and the complete
  default archive gate.
- Move private artifact/Candid helpers into `support_tests.rs` and `tests/`,
  and make integration helper dead-code suppressions checked lint expectations.
- Reuse unchanged product-evidence fixture execution across release groups, with
  new source/group admissions and original proof/log custody. Ordinary CI still
  runs the fixture; Rust, runtime and archive qualification remain fresh.
- Run common adapter corruption/refusal probes once per six-group fixture, while
  retaining each group's preparation, rollback, isolation and committed checks.
  Selected-group fixtures keep the complete common probe set.
- Keep adapter preparation/recovery probes on a compact substituted gate. The
  separate validation fixture still checks the complete production roster.
- Release qualification checks its selected adapter group; full CI retains all
  six groups. Reuse unchanged fixture-retention execution through Shared's
  stage runner, retaining original proof/log custody and fresh source/group
  admission. Rust and archive obligations continue to execute normally.
- Keep the selected-group routing fixture independent of its caller's full-CI
  marker and release admission destination. Nested validation fixtures own their
  admission records. Replay stage-reuse diagnostics on fixture failures.
- Parse stage admission records with portable extended regular expressions so
  macOS's native sed admits the same execution/reuse records as Linux.
- Keep release-stage arguments nonempty for macOS Bash 3.2's nounset behavior.

### ➕ Added

- Allow callers to package and qualify fresh admitted archives with their own
  checks and compiler-cache policy. Preserve the existing default qualification
  behavior and all source, archive and graph custody checks.

## [0.74.3] - 2026-11-04

### 🧹 Cleanup

- Move inline unit tests into ordinary test modules while preserving their
  module paths, fixture contents and production visibility.
- Derive routine cache error formatting while preserving diagnostics and sources.
- Share acquisition finalization between the Wasm and generic artifact caches.
  Preserve published entries after later failures and retain both causes when
  incomplete-entry cleanup fails. Generic commit cleanup now makes one explicit
  attempt, leaving recovery to its returned error rather than retrying on Drop.

### 🥾 Governance

- Adopt Shared Tooling 0.29.4, including test-layout guidance and bounded
  retained-release-state admission, from committed source `628ccc75`.
- Prepare the CI classifier fixture from its declared companion closure so
  new shared parser dependencies remain available in the consumer check.

## [0.74.2] - 2026-11-03

### 🛠️ Fixes

- Qualify dependency-free package archives while still rejecting empty package
  graphs, malformed locks and changed external identities/checksums.
- Let explicit CI qualification dispatches finish independently of newer routine
  pushes, preserving normal newest-revision cancellation and all native checks.
  Accept a fully qualified main-branch dispatch as a release-plan baseline, with
  the same source, host, step and recovery requirements. This prevents the
  matched-input comparison from being repeatedly cancelled during delivery
  ([#314 — Qualify group-scoped release validation](https://github.com/dragginzgame/ic-tooling/issues/314)).

### 🧹 Cleanup

- Keep prepared source leases and batch input resolution in one internal owner,
  separate from cache acquisition and publication, without changing fingerprint
  or public API contracts. Check fixture retention and relative temporary paths
  at the actual initialization/EXIT boundaries without repeating product suites.
  Keep the Make roster-order and failure-stopping fixture aligned with that
  explicit membership, with a focused roster entrypoint.
- Adopt Shared Tooling's pending 0.29.0 snapshot with its shared validation-plan
  parser, evidence interfaces and fail-closed tree inventory, including the UTF-8
  fix in [Shared PR #350 — Fix UTF-8 runner evidence verification](https://github.com/dragginzgame/shared-tooling/pull/350).
  Run full-versus-focused comparisons
  only for explicit qualification; routine CI retains the complete native gate
  without repeating both focused contracts on every push.
- Replace local execution-event and timing parsing with Shared's canonical
  validation proof. Seal live evidence before version preparation; recovery
  retains the same frozen proof, complete artifact inventory and exact payload
  checks. Version-4 receipts refuse older formats rather than reinterpreting them.
- Share standalone/batch input hashing and artifact verification decisions while
  preserving fingerprints, mutation observation points, lock order and failure
  attribution. Carry each configured shared target through its maintenance
  operation while retaining fresh filesystem admission.
- Bind release verification to exhaustive group plans and retained stage evidence.
  Native/Text-only releases can narrow from a complete three-host baseline;
  shared changes and the four wider groups retain the full gate.
  [#314 — Qualify group-scoped release validation](https://github.com/dragginzgame/ic-tooling/issues/314).
- Keep release-stage inventory and three-host baseline admission with one owner
  each. Full CI and release selection now use the same admitted policy instead
  of duplicated Make lists and recipe parsing, preserving all 35 checks, one lint
  pass and failure stopping before later publication/checks. Missing or unmapped
  stage coverage refuses; saved-evidence revalidation remains required.
- Compare full and focused qualification on the same native CI inputs, retaining
  stage timing and installed-archive evidence. Narrow release admission requires
  that proof on Linux and both macOS hosts.
- Trim unused Shared producer regression fixtures from the consumer snapshot;
  retain the same adopted revision and local consumer qualification suites.

## [0.74.1] - 2026-11-02

### 🧹 Cleanup

- Keep Wasm progress and shared-target maintenance with their private owners,
  and use one input-identity comparison across cache acquisition, transactions
  and retained evidence. Preserve mutation checks, lock order, failure attribution
  and public receipts.
- Share successful batch outcome counting while retaining separate artifact and
  Wasm reports. Build fixtures now declare their protocol output and script hooks
  directly, avoiding edits tied to the generated shell script's layout.
- Derive Wasm stamp admission length from its encoder so cache reads and writes
  follow the same format.

## [0.74.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 and pin contributor/release compilation to that version.
  Older compiler support is retired.
  [#313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313).

### 🧹 Cleanup

- Own one independent release group, ledger and tag namespace. Unchanged
  Foundation, Text and integration packages need no coordinated bump.

- Carry admitted shared-target paths and maintenance policy through Wasm cache
  acquisition without reinterpreting raw settings. Preserve input revalidation,
  lock order, retained artifacts and maintenance scheduling.
