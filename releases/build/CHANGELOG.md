# Build changelog

Packages: `ic-build`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.87.2] - Unreleased

Continues the unpublished development line after GitHub checkpoint
[0352b4e](https://github.com/dragginzgame/ic-tooling-new/commit/0352b4e6fb1996e329e9ebf837ea38e40348e1bf).
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
