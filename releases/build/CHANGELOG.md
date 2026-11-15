# Build changelog

Packages: `ic-build`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.87.3] - Unreleased

Continues the unpublished development line after GitHub checkpoint
[aaebfa4](https://github.com/dragginzgame/ic-tooling-new/commit/aaebfa4db99593c3709a0ee92f71bc691f6ac4b6).
This version advance changes no public behavior; prior qualification evidence
remains tied to its recorded inputs.

### ⚠️ Breaking

- The redesigned Build API replaces manual process/message/path composition with
  one operation returning owned Wasm bytes. Consumers must migrate explicitly;
  mutable target paths and legacy helper APIs are not retained.

### 🔧 Changed

- Continue the unpublished 0.87 line after the Stage 1–2 checkpoint. Build uses
  the shared byte and filesystem owners and a private target with mode 0700.
- Admit thiserror 2.0.18 for Scheduling's exact PocketIC requirement while the
  product lock retains 2.0.21.

### 🧪 Testing

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
