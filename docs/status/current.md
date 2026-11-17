# Current implementation — 2026-11-17

## Decisions and scope

[Design revision 2](../DESIGN.md) is accepted. Required workflows determine owners
and interfaces; do not restore old exports or architecture to satisfy obsolete
callers. Read the affected [implementation contracts](../DESIGN.md#implementation-contracts)
before changing code. Build does not own archive qualification or release editing.

Stay in this repository on `main`. Ordinary fixes remain local; hosted CI runs
asynchronously. Qualify sibling consumers through [read-only current-source
snapshots](../qualification/local-sources.md), including working edits; refuse
capture drift and stale adapters. Consumer implementation stays with its owner.

Auth is on hold; Storage, Observability and Wire are frozen. Backup adoption is
user-owned. Profiler is retired. The [workload register](../workloads.md) retains
historical source identities and consumer boundaries; no current unfrozen caller
justifies a new snapshot-set/cache API without a defined partial-effect contract.

## Latest delivery

- Testkit 0.109.0 is published from `1fdd037` with verified registry readback.
  The exclusive controller supports bounded forward clock steps, processes expiry,
  and refuses further use after interrupted dispatch. Public error variants are
  added; exhaustive matches need updating. See the [clock contract](../testkit-held-replies.md#bounded-manual-clock-progression)
  and [release receipt](../release-ownership.md#testkit-01090-delivery).
- The complete Linux archive gate passes with committed dependencies, including
  all three installed fixture cases, five observed deletions and two installed
  server cases. Hosted native CI runs asynchronously; no Canic adoption is claimed.
- Concurrent Protocol/Clap/Nix/Zeroize lock edits are preserved in
  `target/deploy-clock/concurrent-Cargo.lock` and restored with the released
  Testkit version. They remain uncommitted. Private Tools 0.1.3 remains pending.

## Blockers and next actions

[IC Tooling #12 — Provide bounded external sign-only integration with exact expiry and existing identity custody](https://github.com/dragginzgame/ic-tooling/issues/12)
is in progress. The maintainer selected `canic-mainnet`; public metadata records
OS-keyring custody and secp256k1. Process's bounded pipe-input prerequisite passes
19 Linux tests, affected-caller Clippy and both macOS compilation checks. It is
prepared for Process 0.81.0 (`Input`/`Policy` lose `Copy`).
Sign now authenticates exact updates and independent original-ID status reads
using published Platform 0.34.15. The noninteractive provider refuses locked or
unauthorized credentials; unlocking stays with existing OS tools. Twelve library
tests and three disposable ICP 1.7.0 Linux keyring cases cover exact bytes/sender,
status binding, retained cancellation, missing/locked refusal and no prompt or
external sockets. The first macOS provider attempt compiled/passed network denial
but refused its unprepared credential ACL; the disposable fixture now preauthorizes
the helper. #12 stays open until native acceptance succeeds. See the [contract](../sign-only-integration.md).

Deployment is authorized on `main`: Process 0.81.0, Build 0.87.4, Testkit 0.110.0
and Sign 0.1.0. Build's Process boundary is private; Testkit exposes its report
type and therefore takes a minor. Process is published with verified readback;
remaining uploads are in progress. See the [cohort receipt](../release-ownership.md#signing-cohort--process-0810-first).
The scoped graph retains Protocol 0.3.3 and Nix 0.31.3; the concurrent 0.3.5 /
0.31.4 updates are preserved for restoration in `target/deploy-sign/` and bounded
durable recovery state under `.git/release-state/sign-0.1.0/`. New Sign suppliers
select current published versions. Optional unrelated refreshes stay deferred.

[IC Tooling #11 — Support bounded clock progression with held Testkit replies](https://github.com/dragginzgame/ic-tooling/issues/11)
is closed and delivered in Testkit 0.109.0. Canic's same-receiver acceptance remains
with Canic; adoption can now select the published supplier. Instance-wide
serialization and teardown after uncertain clock dispatch remain caller-owned.

The secret-scan fixture false positive and Linux archive name-collision refusal
are fixed. Full-history scanning, candidate admission/rejection and the four-group
Linux archive gate pass; see [failure causes and evidence](../qualification/ci-setup.md#reviewed-ci-failures--2026-11-17).
The delivered cache/timeout/CI fixes and handoff cleanup at `92655b1` pass
[hosted CI](https://github.com/dragginzgame/ic-tooling/actions/runs/44073531569).
The scanner failure-upload path, native test relocation and clock progression are
delivered; hosted observation remains outstanding and no new CI run is awaited.

On a delivery request, select affected groups under [release ownership](../release-ownership.md).
New product work requires a demonstrated current caller requirement.

Further consumer work requires current requirements and source capture. Canic's
rewrite owns its Platform adoption; old adapters correctly refuse changed owners.
Scheduling targets the rewrite at `../ic-scheduling` and preserves its Testkit
client/held-reply-fixture boundary. See the [workload decisions](../workloads.md#current-decisions-and-retained-evidence).

## Delivery and evidence boundaries

Foundation 0.80.3, Process 0.80.5 and Testkit 0.109.0 are published; exact commits,
checksums, tool selections and acceptance live in the [release receipts](../release-ownership.md).
Earlier publication and native-host evidence remain in that document, not here.

Consumer archive/deployment acceptance is separate from supplier qualification.
Async deletion is best-effort; cancellation during creation can leave remote
effects. Custody assumes cooperative writers/groups and exclusive child reaping.
Preserve retained recovery evidence; no fleet-wide completion is claimed.
