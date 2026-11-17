# Tools changelog

Packages: `ic-cargo-qualification`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.1.3] - Unreleased

### 🔧 Changed

- Run focused Sign admission/cancellation checks on the existing three native CI
  hosts. Keep disposable credential-session qualification explicitly selected;
  compilation and library tests do not establish native keychain acceptance.
- Add native provider jobs using pinned ICP 1.7.0 disposable identities. Linux
  records only socket/prompt-header metadata; macOS runs inside a network-denying
  sandbox. Retain selected evidence and sanitized setup failures, excluding all
  temporary credential directories. Actual provider acceptance passes on Linux
  and native Intel/ARM macOS. Raise the separate Intel library compilation step
  to eight minutes after the provisional four-minute bound expires; retain the
  job's failure-evidence reserve and asynchronous hosted verification.
- Qualify extracted Sign archives, provider dependency isolation and the installed
  `ic-sign` command against disposable native credentials. Prepared manifest/helper
  paths select the candidate without changing product signing or key custody.

- Include Testkit's installed bounded-clock/expiry regression in controller and
  archive qualification, retaining independent deletion evidence for all five
  instances. See the [Testkit 0.109.0 change](../testkit/CHANGELOG.md).

- Retain bounded, sanitized secret-scanner installation failures through the
  existing setup collector. Preserve setup status and deadline, upload only its
  evidence directory, and report artifact links or upload failures separately.

- Admit library archive candidates by workspace identity, allowing a separate
  registry dependency with the same package name. Resolve the secret-scan false
  positive with one historical fixture fingerprint; keep future findings scanned
  and preserve private-key redaction coverage.

- Bound CI jobs and individual setup/qualification steps using observed hosted
  durations, leaving time for failure evidence uploads. Keep compiler/tool timeout
  diagnostics inside the existing sanitized collector and preserve failure status.

- Reuse disposable Cargo compilation directories across archive qualification
  retries while keeping candidates, archives and evidence fresh. Refresh extracted
  source timestamps to prevent stale artifact acceptance; retain an opt-out for
  cold compilation and a changed/backdated-source rejection/recovery regression.
  Record matched local cold/warm measurements in the
  [cache contract](../../docs/qualification/archive-cache.md).

- Refresh Scheduling's current Testkit/Runtime admission and the reviewed Protocol
  replacement checksum. Add `admit-replacement` to qualify local driver graphs
  without requiring a prepared simulator; preserve source and lock-drift refusal.

- Ignore inherited `CDPATH` during qualification directory changes, preserving
  repository and relative source selection without extra path output. Add focused
  relative-invocation and source-capture regressions.

- Keep Canic qualification child modules in directory modules, including signer,
  staging and lock fixtures. Remove remaining lock/read-fixture path overrides;
  migration patches retain their original source-content checks.

- Retain bounded, sanitized compiler/common-tool setup failures from the CI
  scaffold job, including selected hidden attempt logs. Preserve original setup
  status, report missing/failed collection or upload separately, and link the
  seven-day diagnostic artifact from the job summary.

- Replace the legacy Testkit 0.106.0 qualification launcher and implicit sibling
  tool lookup with Testkit's product CLI, removing the temporary private runner.
  Scheduling and archive checks use explicit prepared paths; bounded setup
  diagnostics, original command failures and server cleanup remain observable.
  Fetch the extracted archive's registry graph before offline feature checks.
  Retire DB's already-adopted direct-client/Text migration patches and runners;
  [Tooling #10](https://github.com/dragginzgame/ic-tooling/issues/10) supplies the
  command interface; archive qualification installs and tests its actual package.
  Reconcile completed Scheduling/Canic adoption with the workload register.

- Retire the obsolete Platform raw_rand/Agent qualification adapters and their
  Make targets after current-source capture confirms their owner scripts were
  removed. Preserve dated evidence without restoring old Platform APIs. Refresh
  Scheduling's manifest-only adapter for Runtime 0.23.13 / Types 0.21.2; its
  remaining probe migration is tracked in Scheduling #2.

- Adopt Toko Tooling 0.44.11 across both workspaces and all reviewed cache-action
  references. Supplier adoption includes qualification and native callers by
  default; explicitly frozen comparisons retain their recorded inputs. Keep
  compatible `thiserror` requirements and the existing single qualification
  package tier; installed executable bytes are unchanged.

- Qualify Testkit's optional managed-server feature from its Cargo archive,
  including prepared assets, gateway/held-reply behavior and failure cleanup.
  Hosted setup reuses its verified gzip archive with an explicit gzip executable;
  setup diagnostics and library evidence have seven-day artifact retention.

- Admit current Scheduling's registry Testkit 0.107.1 selection. Remove the obsolete
  fixture API migration from its local-supplier adapter; comparisons now change only
  the dependency while preserving consumer assertions and current-source capture.

- Report bounded, escaped PocketIC setup diagnostics and exact retained evidence
  paths through the qualification bootstrap. Preserve original failure status and
  offline artifact admission; retain setup evidence even before archive tests start.
  [Tooling #3 — Surface PocketIC setup download stderr](https://github.com/dragginzgame/ic-tooling/issues/3).

- Remove the pre-rewrite Scheduling runner, its four exclusive migration patches
  and two session helpers. Retain historical source identities and results without
  runnable instructions; current rewrite qualification keeps its own dispatch.
  [Tooling #4 — Remove the executable pre-rewrite Scheduling qualification runner](https://github.com/dragginzgame/ic-tooling/issues/4).

- Adopt Toko Tooling 0.44.6 in both workspaces through its verified exporter and
  align the hosted cache-action references with that release. Executable snapshot
  files are unchanged; hosted CI policy now explicitly covers registry publication.

- Add explicit HTTP/Text/Testkit archive readiness, preserving packaged-source
  identity and Testkit feature isolation. Reuse the prepared external simulator
  owner for installed checks and add the selected libraries to native CI.

- Retire the obsolete Canic report-allocation migration from the current handoff.
  Record its existing published-FS adoption from a read-only current-source
  capture and preserve the old report patch's refusal of changed source.

- Adopt Toko Tooling 0.44.5 in both workspaces through its verified exporter;
  align the hosted download-cache action with the release's source commit.
  The retained 49-file snapshot and cache-action implementation are unchanged.

- Run snapshot/pin admission and non-mutating formatting before the native CI
  matrix, including on tooling-only and documentation-only pull requests.
  Prepare all repository-required Rust components and common tools explicitly
  through the shared installer and pinned download cache; retain
  one history secret scan outside the matrix.
  [Tooling #1 — Complete hosted CI wiring for adopted Toko Tooling checks](https://github.com/dragginzgame/ic-tooling/issues/1).

- Add a four-crate archive readiness command and native Linux/macOS CI. Keep
  production publication separate; qualify extracted dependency candidates and
  current Canic metadata/build refusals without sibling edits.
- Start the next unpublished tools development line after GitHub checkpoint
  [353e37a](https://github.com/dragginzgame/ic-tooling-new/commit/353e37a9632c30f0912f7060eecdb12def6ee19e).
  The tools group owns qualification workflows; library versions and public
  behavior are unchanged. Earlier runtime evidence remains bound to its recorded
  inputs.
- Add `test-canic-runtime` to compose current Host custody with CLI viewer and real
  artifact/app build checks on one source capture. Pass 52 paired cases, 15 focused
  cases and 20 metadata/tool refusals with Host/CLI Clippy and graph admission.
  Preserve explicit JSON fixture output; fleet-report FS, profile/Store runtime
  and archive acceptance remain separate work.
- Extend the runtime export with held report-file publication, reusing the
  private output adapter. Two paired cases preserve rendered bytes, modes,
  occupied entries and link refusal; a separate replacement-only case proves
  the explicitly narrower parent-traversal policy. The complete 54-case paired
  comparison and existing runtime checks pass. Automatic report-directory
  allocation retains its released helper; no library API changes.

## [0.1.2] - Unreleased

Continues the unpublished development line after GitHub checkpoint
[aaebfa4](https://github.com/dragginzgame/ic-tooling-new/commit/aaebfa4db99593c3709a0ee92f71bc691f6ac4b6).
This version advance changes no public behavior; prior qualification evidence
remains tied to its recorded inputs.

### ➕ Added

- Introduce the unpublished external-lock admission executable, separate from
  Build. Preserve exact external tuples, registry checksums, duplicate refusal
  and bounded no-follow reads. Auth's pinned package/archive qualification remains
  historical; further Auth migration is on hold.

### 🔧 Changed

- Compose current Canic Host filesystem and Process qualification on one source
  capture. Verify the dev-only legacy-FS oracle and absence of legacy FS/Process
  from Host's native normal/build graph; compare 52 cases and run two custody
  plus nine focused Process checks. Preserve current registry Store inputs and
  ordinary module wiring for shared fixtures.
- Adopt Toko Tooling 0.44.4 in both workspaces through its verified snapshot.
  Rename the qualification fixture directory to `owned-wasm-probe` to match its
  existing package identity; update workspace membership and Build callers.

- Advance Build/Text recipe selections for the next development versions. Refresh
  Query only after its stale patch refuses the changed current source.

- Active consumer runners capture current local sibling working trees, preserve
  edits and refuse capture drift or stale patches. They never modify the sibling.
- Platform and DB targets select prepared local Scheduling CLI tools by default,
  retaining explicit overrides and offline asset checks.

### 🧪 Testing

- Add a focused Scheduling rewrite held-reply selection using retained Testkit:
  pressure/reverse release and held-upgrade refusal/settlement pass. The outer
  launcher owns server teardown; endpoint refusal and independent instance deletion
  are checked. Define the bounded shared Testkit contract and remaining failure
  gates without importing a replacement implementation.

- Correct active Scheduling source to `ic-scheduling-new`; admit retained Testkit
  types and held-reply fixtures and check its actual host drivers. Retire active
  old-Scheduling Jobs/Timers commands. The older Scheduling experiments recorded
  below are historical only and do not qualify the rewrite or justify removing
  Testkit. Preserve current-source requirements before new shared API design.

- Remove the Timers Testkit library dependency in the current qualification patch.
  Direct PocketIC creation/deletion, funding/install and caller-budgeted ticks
  preserve the uploaded-snapshot fix. Four focused subjects pass with five deleted
  instances; the native-snapshot negative control still reproduces its timer trap.
  Retained external-lock tuples are unchanged; no public simulator API is added.

- Fix local Timers snapshot restoration through PocketIC download/upload, preserving
  timer metadata with the heap. Unchanged and repeated mutated restores pass without
  timer resets or instance replacement; transfer files and both snapshot IDs are
  removed on success. Retain the plain take/load negative control and shared-local-
  filesystem boundary, without importing a generic snapshot implementation.

- Retain a focused negative Timers snapshot probe: native take/load snapshots trap
  on the first mutated timer rearm despite matching heap observations. Preserve the
  passing uploaded-snapshot adapter and record the remote-transport obligation;
  no Testkit snapshot implementation is imported into product crates.

- Add a focused Timers borrowed-server selection. Remove per-subject server/log
  setup in the qualification patch, retain explicit instance ownership, and verify
  watchdog/ingress/cached restores, missing-endpoint refusal and three deleted
  instances while the outer server remains reachable.

- Combine current Canic Build 0.87.3 and Text 0.77.2 in one app-workflow graph;
  qualify app partial effects and thirteen fleet/placements display cases. Refresh
  new display callers using the existing Text API and retain external selections.

- Verify Canic compilation failure preserves symlink/hardlink/dangling entries
  and foreign bytes; later success replaces each entry with exact built bytes
  without creating a missing referent. Publication remains owned
  by the consumer and its retained Host FS supplier.

- Qualify current Canic with Build 0.87.3: missing/unavailable prepared tools and
  compiler wrappers refuse before compilation, preserve output, and leave scratch
  empty. Controlled wrappers are never executed; no new tool-install policy is added.

- Qualify Canic metadata admission with a real-build control and ten refusals
  before compilation, retaining destination bytes and empty scratch. The adapter
  checks workspace membership and an absolute target directory.

- Add a focused Canic app-build selection, reusing the existing export/build runner
  without repeating the artifact-command suite.

- Qualify DB rollback with direct upstream PocketIC and explicit external-server
  ownership, preserving raw stable-memory oracles, deletion and server survival.
- Qualify Scheduling Jobs controller snapshot recovery, unauthorized-caller refusal
  and exact stable-memory restoration without a new simulator wrapper.
- Qualify Platform raw_rand and Agent fixtures with direct PocketIC. Preserve
  callback cleanup/cancellation, certificate rejection and Principal package
  identity admission across the upstream client boundary.

These development workflows belong to this ledger; they are not APIs or bundled
features of the external-lock executable. Detailed source identities, validation
results and remaining boundaries stay in the qualification documents.
