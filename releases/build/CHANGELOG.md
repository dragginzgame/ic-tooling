# Build changelog

Packages: `ic-build`.

This group starts with the replacement implementation. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.87.8] - 2026-11-25

### 🔧 Changed

- Select Toko Tooling 0.46.4 for package development and validation.

## [0.87.7] - 2026-11-21

### 🔧 Changed

- Build with Cargo’s configured network mode so it can retrieve missing locked
  dependencies. Callers can select offline mode through the request environment.

## [0.87.6] - 2026-11-19

### 🔧 Changed

- Adopt Toko Tooling 0.45.21 and its Rust source layout in the packaged crate.

## [0.87.5] - 2026-11-18

### 🔧 Changed

- Refresh the package development tooling selection to Toko Tooling 0.45.8.
  Public APIs, persisted bytes and runtime behavior remain unchanged.

## [0.87.4] - 2026-11-17

### 🔧 Changed

- Use Process 0.81.0 after its finite-input addition. Build keeps Process types
  private; its owned-artifact API and existing output/failure behavior are unchanged.

## [0.87.3] - 2026-11-16

### 🔧 Changed

Continues the unpublished development line after GitHub checkpoint
[aaebfa4](https://github.com/dragginzgame/ic-tooling-new/commit/aaebfa4db99593c3709a0ee92f71bc691f6ac4b6).
This version advance changes no public behavior; prior qualification evidence
remains tied to its recorded inputs.

### ⚠️ Breaking

- The redesigned Build API replaces manual process/message/path composition with
  one operation returning owned Wasm bytes. Consumers must migrate explicitly;
  mutable target paths and legacy helper APIs are not retained.

### 🔧 Changed

- Prepare the 0.87.3 first-release candidate with crates.io-only publication
  metadata and a package description; registry upload remains separate.
- Continue the unpublished 0.87 line after the Stage 1–2 checkpoint. Build uses
  the shared byte and filesystem owners and a private target with mode 0700.
- Admit thiserror 2.0.18 for Scheduling's exact PocketIC requirement while the
  product lock retains 2.0.21.

### 🧪 Testing

- Qualify extracted Build/Artifacts/FS/Process archives, including real Wasm
  capture and current Canic metadata/tool refusal checks. Native macOS CI is
  configured separately from the passing Linux evidence.
- Refresh current Canic 0.214.0 Build qualification while preserving its locked
  Store-source resolver and consumer-owned default destination. A tiny local
  Wasm supplier fixture verifies publication outside the supplier and unchanged
  supplier-tree bytes; this does not qualify the production Store implementation.

- Extend combined metadata acceptance with nonzero exit after valid JSON, stdout
  and stderr overflow, and invalid UTF-8. All four refuse before compilation and
  preserve the destination; ten selection and six tool/wrapper refusals also pass.
  Metadata capture still uses the retained supplier pending its migration.
- Follow up by moving metadata execution to local Process in the combined
  Process recipe; preserve the preceding refusal cases and original diagnostic
  assertions. The ordinary standalone Build recipe remains separate evidence.

- Refresh current Canic Build lock admission and qualify the combined Build,
  Text and Process graph for app preflight, lock initialization/refusal,
  existing-lock preservation, exact outputs and serial partial failure.

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

- Migrate Scheduling from manual Cargo execution, shared-target locking and path
  capture to owned builds. Keep bounded configuration, compiler/source observations
  and receipts consumer-owned. Version recipe identities to refuse silent
  legacy/new comparisons without changing report schemas or application bytes.
- Retain focused process/protocol/custody refusals, real offline Wasm acceptance,
  and selected installed watchdog/upgrade and invalid-request cases.
- Qualify current Canic’s build path with owned capture, retaining consumer-owned
  default/explicit publication. Check stale-output replacement, exact identity,
  failed-build retention, publication refusal and private-target cleanup.
- Extend current Canic qualification to app recipes: refuse duplicate outputs
  before effects, retain lock bytes, and preserve earlier successful outputs when
  a later recipe fails. App builds remain serial, not transactional.

### Shared execution follow-up

- Replace the private spawn/drain/cleanup engine with unpublished Process 0.80.0.
  Build retains its Cargo stream decoder, bounded diagnostics and error mapping;
  the 19 existing Build tests pass through the shared owner. The Build API and
  version stay on the current unpublished draft. Canic qualification must be
  refreshed before claiming this changed graph against that consumer.

- Refresh current Canic artifact/metadata qualification for the shared Process
  graph. Retain its newly required bounded/escaped compiler diagnostics and named
  missing-package refusal in the consumer adapter. Exact output, failure
  preservation, tool/wrapper refusal and cleanup checks pass; combined app/Text
  evidence is not implicitly refreshed.
