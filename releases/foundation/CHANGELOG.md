# Foundation changelog

Packages: `ic-host-artifacts`, `ic-host-fs`.

This group starts with the replacement implementation. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.80.4] - 2026-11-18

### 🔧 Changed

- Refresh the package development tooling selection to Toko Tooling 0.45.8.
  Public APIs, persisted bytes and runtime behavior remain unchanged.

## [0.80.3] - 2026-11-17

### 🧹 Cleanup

- Use directory modules for code with child tests, following the installed Toko
  layout rule. Public module paths, source bytes and filesystem behavior are unchanged.

## [0.80.2] - 2026-11-16

### ➕ Added

- Admit an original retained file for create-only publication after reopening,
  verifying its held identity, permissions and complete expected bytes before
  using the existing seal/commit lifecycle. Refusal retains recovery evidence;
  no rewriting, permission changes or automatic retries occur.
  [#2 — Support publication of original retained files after reopen](https://github.com/dragginzgame/ic-tooling/issues/2).

### 🔧 Changed

- Advance Artifacts with the coordinated foundation group; its byte API and
  implementation are unchanged. Backup adoption remains consumer-owned work.

## [0.80.1] - 2026-11-16

### 🔧 Changed

- Fix macOS creation-mode compilation by converting validated permission bits
  once to Rustix's native mode type; reject invalid bits before conversion.
  Prepare FIFO refusal fixtures through native `mkfifo` on both hosts because
  Rustix does not expose `mkfifoat` on macOS.
- Separate exclusive sidecar creation from opening an occupied entry to avoid
  Darwin's concurrent create-or-open `ENOENT`; retain no-follow admission,
  unchanged existing files and nonblocking contention.
- Prepare crates.io-only candidate metadata for Artifacts and FS. Extracted
  archives pass Linux byte/custody/publication tests and the Build consumer;
  native macOS CI remains required before release readiness.
### ⚠️ Breaking

- The replacement byte and filesystem APIs require explicit consumer migration
  from the production packages. Filesystem publication distinguishes refusal
  before effects from visible-but-unconfirmed outcomes; retained recovery files
  have an explicit lifecycle. Existing stored bytes remain unchanged.

### 🔧 Changed

- Add `Directory::lock` to own held-parent sidecar open/create, regular-file
  admission and nonblocking acquisition. Preserve existing bytes/modes/inode,
  original contention and explicit release; no sync, replacement or cleanup.
- Add no-replace publication of a caller-prepared held directory. Return distinct
  pre-publication and visible-but-unconfirmed failures with namespace evidence;
  retain staging on refusal and leave tree validation/recovery with the caller.
- Add held-directory preparation for explicit child components. Validate names
  before effects, return the held leaf, and retain created prefixes with the
  failed component and original cause. Production consumer migration remains separate.
- Add `hash_with` for complete bounded producer commitments without buffering
  payloads or exposing digest snapshots. Preserve producer errors and refuse
  swallowed overflow; producer completion and length admission gate identity.
- Qualify the Backup assembled-payload adapter with four current fixed-capture
  completion/refusal tests using both suppliers, retained external locks and
  warning-denied Clippy. Restore remains blocked by 15 earlier baseline failures.

- Share bounded complete-stream admission and held-directory custody with Build
  and Backup. Add held-file identity-checked removal for producer cleanup while
  keeping parent synchronization explicit.
- Continue the unpublished 0.80 line after the Stage 1–2 checkpoint; update
  Backup's dependency selection with the suppliers.

### 🧪 Testing

- Move Canic Host's direct released-FS dependency to test-only use in the export;
  verify library compilation and exact dependency kind while retaining independent
  test oracles. The released Process transitive edge remains explicit.
- Qualify Canic's private output-path comparison with missing/dangling aliases,
  traversal refusal and active-depth limits, preserving pre-lock app admission.
  The composed comparison passes 52 paired cases without a new public path API.
- Qualify Canic artifact inspection through held regular-file admission and bounded
  bytes, preserving alias/hard-link behavior, exact identity and refusal of invalid,
  oversized or special inputs. The composed comparison now has 49 paired cases.
- Extend Canic held preparation to recovery/release records and dispatch deadlines;
  pass 47 paired cases plus two custody tests. Preserve authority, persisted JSON
  and recovery ordering; live signing/submission remains outside runtime evidence.
- Qualify fleet staging member writes through retained root/child handles, keeping
  preparation order, slot mapping, JSON bytes and create-only publication in the
  composed 41-case comparison plus two custody tests.
- Qualify starter member preparation and publication through held staging custody,
  removing its released parent helper and pathname-based writes while preserving
  generated content, create-only behavior and retained partial staging.
- Qualify held starter/fleet staging allocation in the 41-case Canic comparison
  plus two collision/identity cases; retain the created handle through publication
  without adding public allocation policy or importing consumer product code.
- Qualify Canic Store/plan preparation with 41 composed cases: use prepared custody
  directly for locking and both plan writes, preserving permissions, path refusal,
  bounds and retained recovery behavior.
- Qualify Canic Store/configuration sidecar locks in a 39-case combined comparison,
  preserving contention with the released helper, inode/bytes/modes, hard-link
  policy and release on scope exit. Native macOS remains unqualified.
- Pass four independent sidecar-lock tests for native interoperability,
  concurrent creation, parent rename, policy refusal and preserved foreign bytes.
- Pass a combined 37-case current Canic retained-read/publication comparison with
  both suppliers, preserving ancestor aliases, final-link refusal, bounds and
  missing-file causes alongside config/record/starter/fleet recovery assertions.
- Pass five independent directory-publication cases covering atomic competition,
  occupied/link refusal, identity substitution, parent rename and sync failures.
- Qualify 16 current Canic starter/fleet cases with both suppliers. Preserve
  content verification, no-replace publication, retained approvals, cancellation,
  expiry and slot mapping; staging allocation and member writers stay unchanged.
- Pass four independent directory-preparation tests, including root rename,
  link/permission refusal and retained prefixes after child/parent sync failures.
- Qualify current Canic artifact publication through held parent preparation and
  stage/write/seal/commit, preserving final-entry replacement, refusal, existing
  modes and cleanup. Other Canic writers remain unmigrated.
- Compare six current Canic configuration cases with both suppliers, preserving
  create-only initialization, locked replacement, bounds and symlink refusal.
  The original sidecar locking and retained-record reads remain selected.
- Compare retained-record publication and signed recovery with both suppliers;
  preserve create-only evidence, original parent permissions, authority/expiry
  behavior and bounds before effects. Signing and lock protocols stay unchanged.
- Qualify current Backup metadata, payload and transfer-binding behavior, retaining
  its budget, commitment comparison, recovery evidence and uncertain-write refusals.
- Qualify Canic retained-record admission and signed recovery. Current local-source
  evidence supersedes earlier pinned runs without extending build/publication scope.
- Qualify Jobs capture through retained staging and producer-identity cleanup,
  preserving optimizer flags, status, logs and failed-capture evidence.
