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

## Current maintenance batch

- Archive runners reuse disposable Cargo compilation output while keeping fresh
  candidates and evidence. Timestamp refresh prevents stale artifact acceptance.
  Cold/warm gates and changed/backdated-source rejection/recovery pass; see the
  [cache evidence and limits](../qualification/archive-cache.md). This qualifies
  the runners, not new consumer adoption.
- CI job/step deadlines reserve time for evidence uploads. Actionlint and 44
  diagnostic checks pass, including timed-out setup capture and redaction. See
  [budgets and evidence](../qualification/ci-setup.md#job-deadlines--2026-11-17).
  Hosted deadline/upload behavior has not been deliberately induced.
- The handoff cleanup moves the contract index into the design and retains
  completed evidence at its owning documents.
- Preserve concurrent Protocol/Clap/Nix `Cargo.lock` edits outside this delivery.
  Their current recovery copy is `target/ci-timeout-review/pre-delivery-Cargo.lock`;
  earlier release recovery evidence remains under `target/deploy-ranges/`.

## Blockers and next actions

The secret-scan fixture false positive and Linux archive name-collision refusal
are fixed. Full-history scanning, candidate admission/rejection and the four-group
Linux archive gate pass; see [failure causes and evidence](../qualification/ci-setup.md#reviewed-ci-failures--2026-11-17).
Hosted confirmation remains asynchronous; no new CI run is awaited.

The requested delivery covers this maintenance batch on `main`, through Git only.
Private tooling notes remain in [Tools 0.1.3](../../releases/tools/CHANGELOG.md);
no product crate source or metadata changes require publication. Future product
delivery follows [release ownership](../release-ownership.md).

Further consumer work requires current requirements and source capture. Canic's
rewrite owns its Platform adoption; old adapters correctly refuse changed owners.
Scheduling targets the rewrite at `../ic-scheduling` and preserves its Testkit
client/held-reply-fixture boundary. See the [workload decisions](../workloads.md#current-decisions-and-retained-evidence).

## Delivery and evidence boundaries

Foundation 0.80.3, Process 0.80.4 and Testkit 0.108.0 are published; exact commits,
checksums, tool selections and acceptance live in the [release receipt](../release-ownership.md#foundation-0803-process-0804-and-testkit-01080-delivery).
Earlier publication and native-host evidence remain in that document, not here.

Consumer archive/deployment acceptance is separate from supplier qualification.
Async deletion is best-effort; cancellation during creation can leave remote
effects. Custody assumes cooperative writers/groups and exclusive child reaping.
Preserve retained recovery evidence; no fleet-wide completion is claimed.
