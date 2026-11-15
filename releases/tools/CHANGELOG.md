# Tools changelog

Packages: `ic-cargo-qualification`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.1.0] - Unreleased

### ➕ Added

- Introduce the unpublished external-lock admission executable, separate from
  Build. Preserve exact external tuples, registry checksums, duplicate refusal
  and bounded no-follow reads. Auth's pinned package/archive qualification remains
  historical; further Auth migration is on hold.

### 🔧 Changed

- Active consumer runners capture current local sibling working trees, preserve
  edits and refuse capture drift or stale patches. They never modify the sibling.
- Platform and DB targets select prepared local Scheduling CLI tools by default,
  retaining explicit overrides and offline asset checks.

### 🧪 Testing

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
