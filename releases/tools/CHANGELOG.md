# Tools changelog

Packages: `ic-cargo-qualification`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

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

- Advance Build/Text recipe selections for the next development versions. Refresh
  Query only after its stale patch refuses the changed current source.

- Active consumer runners capture current local sibling working trees, preserve
  edits and refuse capture drift or stale patches. They never modify the sibling.
- Platform and DB targets select prepared local Scheduling CLI tools by default,
  retaining explicit overrides and offline asset checks.

### 🧪 Testing

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
