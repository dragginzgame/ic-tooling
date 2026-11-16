# Tools changelog

Packages: `ic-cargo-qualification`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.1.3] - Unreleased

### 🔧 Changed

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
  [#1 — Complete hosted CI wiring for adopted Toko Tooling checks](https://github.com/dragginzgame/ic-tooling/issues/1).

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
