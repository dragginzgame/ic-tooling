# Current handoff — 2026-10-27

## Source and scope

The user reported 0.69.2 live. The checkout was clean at
`1e245a9e8791b04daa0bf971124ae8908d956821` before this accepted cleanup batch.
Current changes are local and unstaged. Pending notes select 0.69.3 for compatible
internal cleanup; package versions remain 0.69.2. Public APIs, cache formats,
fingerprint framing, dependency selections and product-tool pins are unchanged.
No commit, push, release, publication, setup/download or artifact cleanup ran.

Shared Tooling remains the exact 144-file snapshot at
`d08cbcafbaff4227e798a890232e2248bf167504` (0.27.1 plus the fixture correction).
This batch does not refresh Shared Tooling or modify sibling working checkouts.
Host, Build and Testkit retain their extraction boundaries and consumers retain
installation, budgets, source custody and recovery policy.

## Completed simplifications

Startup owns one remaining-budget/poll-interval calculation across sync and async
readiness and instance creation. Owned servers keep periodic liveness inspection;
borrowed servers retain the full remaining creation deadline. Each execution
path retains its waiting, panic, cancellation and cleanup behavior.

Artifact-cache hits and commits converge after their own required validation and
materialization. Usage recording, scheduled maintenance, total timing and retained
record construction have one completion owner. The transaction no longer stores
a redundant namespace path. Namespace/content lock custody, post-materialization
hit revalidation and primary/cleanup error retention keep their existing order.

Batch maintenance owns conflicting-policy rejection and per-target deduplication
together. The batch loop projects its preparation error once, preserving indexed
specification failures and collect-all behavior. CLI server selection is an
explicit directory/environment/default match with unchanged conflict diagnostics,
URL precedence and spawn-only options.

Wasm target admission and captured output projection have a dedicated private
owner under `wasm_cache/outputs/`. Generic input discovery delegates the cdylib,
name-collision and target-kind decisions instead of implementing them inline.
Snapshots intentionally retain the projection or its error: the public borrowed
`artifact_paths()` contract reads captured data without new observations, and
non-Wasm generic evidence remains usable. This is a bounded private separation;
it does not remove the output API or introduce lazy state, metadata copies or a
second generic Cargo specification. The larger public redesign is unnecessary
for this compatible batch.

## Validation and preservation

Evidence is under `/tmp/ic-tooling-cleanup-0693.JiPDSR/`: starting HEAD/archive,
incoming patch, focused logs and final changed-source capture. All commands use
locked/offline Cargo and prepared tools. No failed validation attempt occurred.

Actual Rust 1.91 Linux evidence passes: 36 artifact-transaction cases, 13 Wasm
batch cases, 75 Wasm-cache cases, 28 ordinary startup cases and 13 ordinary
server-runner cases. Six startup and four server-runner live/helper cases remain
ignored in their ordinary suites. Selected real PocketIC 16.1 cases also pass:
synchronous owned-instance topology/lifetime, asynchronous owned and borrowed
lifetimes, and the CLI's default prepared-bundle path without environment selection.
Native tests used loopback access; they did not provision or download a server.

Warning-denied all-target/all-feature Clippy passes for Build and Testkit.
Formatting, local documentation links and final diff checks are recorded beside
those logs. These are focused checks, not a full workspace or release gate.
Native macOS Intel and Apple Silicon qualification did not run for this changed
source and must remain separate from the Linux evidence.

The [0.69.2 handoff](https://github.com/dragginzgame/ic-tooling/blob/1e245a9e8791b04daa0bf971124ae8908d956821/docs/status/current.md)
retains the previous cleanup and Shared contribution evidence. Its source and
logs remain under `/tmp/ic-tooling-cleanup-0692.DCU3sj/`; older preservation and
qualification records remain linked from that historical handoff.

## Retained obligations

This batch supplies no crypto correction or security exemption. Historical
[Tooling #240](https://github.com/dragginzgame/ic-tooling/issues/240) and supplying
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) retain graph
and qualification evidence. No consumer migration or publication is claimed here.

Migration/retirement archives remain under `/tmp/ic-tooling-migration/`. Original
Host refs, stale-lock evidence at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`,
release plans, failed attempts and earlier backup/stash evidence remain preserved.
