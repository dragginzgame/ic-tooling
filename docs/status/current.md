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
