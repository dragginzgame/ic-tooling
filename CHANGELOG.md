# Changelog

## Next development — Unreleased

### ➕ Added

- Add unpublished `ic-host-http` 0.80.0 for complete bounded response bodies, with
  typed size/transport/allocation failures and capped fallible growth. Consumers
  select Reqwest and TLS directly; no client re-export or request policy wrapper.

- Add unpublished `ic-cargo-qualification` 0.1.0 with a read-only external-lock
  admission command, separate from Build. Preserve exact external tuples,
  registry checksum requirements, duplicate refusal and bounded no-follow reads.

### 🔧 Changed

- Start `ic-build` 0.87.1 and `ic-host-artifacts` / `ic-host-fs` 0.80.1 after
  the Stage 1–2 GitHub checkpoint. Update local dependency selections and the
  pinned Backup qualification together. All packages remain unpublished.
- Admit `thiserror` 2.0.18 for Scheduling's exact PocketIC requirement while
  retaining 2.0.21 in the product lock.
- Add held-file identity-checked removal for Jobs producer cleanup and use the
  same owner for publication cleanup; parent synchronization remains explicit.
- Remove legacy Build from the Scheduling qualification graph. Keep bounded
  configuration admission and request-derived recipe evidence private to Timers.
  Version recipe identities so legacy/new reports refuse silent comparison;
  report schema and application bytes remain unchanged.

### 🧪 Testing

- Move Backup qualification’s bounded reads and exact payload streams to the
  existing byte owner. Keep commitment comparison in Backup, preserve persisted
  bytes and test refusals, and reuse the admitted manifest identity.

- Refresh Backup qualification for current local metadata and transfer-binding
  work. Preserve its record budget and retained evidence; qualify cold replay,
  uncertain writes and damaged/orphan evidence through the new filesystem owner.

- Select current local sibling snapshots for active consumer qualification,
  preserving local edits and rejecting source drift or stale migration patches.
  Qualify Canic’s current retained-record path; older runs remain historical.

- Qualify Canic retained-record admission against the new byte/filesystem owners
  using a pinned committed export, bounded-read refusals and original plan and
  signed-recovery tests. Keep publication and build migration boundaries explicit.

- Qualify Auth's pinned package checker against the new executable, including its
  original refusal fixtures and exact public archives compiled with all library
  features on native and Wasm targets. Archive and consumer inputs remain unchanged.

- Migrate Jobs capture to held filesystem staging with retained failure copies,
  preserving optimizer flags, status and logs. Qualify original and optimized
  Wasm through the installed management/upgrade/watchdog subject.

- Add a pinned Scheduling migration from manual Cargo execution, shared-target
  locking and path capture to the owned-build operation. Preserve compiler/source
  observations and receipts; qualify 13 host tests and two installed PocketIC
  subjects covering watchdog trap/upgrade recovery and invalid Wire rejection.

## Stage 1–2 checkpoint — 2026-11-15 (unpublished)

Development notes for the redesigned, unpublished `ic-build` package. The next
minor follows published 0.86.0 because the Rust API is incompatible. Historical transplant
notes remain in Git at `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`.

### ⚠️ Breaking

- Retain the `ic-build` crates.io name for the redesigned API at 0.87.0. Existing
  callers require explicit migration; no legacy helper API is carried forward.
  Publication remains disabled during development.

### ➕ Added

- Introduce unpublished `ic-host-artifacts` and `ic-host-fs` 0.80.0 with bounded
  byte admission, held-directory custody and consuming publication states.
  Publication preserves pre-effect refusal and visible-but-unconfirmed outcomes;
  retained recovery files have a separate explicit lifecycle.

- One offline Wasm build operation owns execution, bounded Cargo decoding, exact
  selection and capture into immutable owned bytes. Failure retains process,
  protocol, diagnostic and cleanup evidence; mutable Cargo paths do not escape.

### 🧪 Testing

- Qualify the filesystem supplier against pinned Backup store, payload, capture
  and selected ledger tests in an isolated export, preserving persisted bytes.
  Add focused publication race, retention, identity and sync-failure coverage.

- Add focused byte/custody/process/protocol failure qualification and a real Wasm
  fixture in an independent qualification workspace. Development commands now
  target this workflow instead of the retired copied packages.

### 🔧 Changed

- Share byte admission and filesystem custody with Build; explicitly create its
  private target with mode 0700 instead of relying on umask-dependent defaults.

- Adopt Toko Tooling 0.44.3 in both workspace locks and the verified snapshot;
  LOC reporting now expands levels relative to the selected scope.
