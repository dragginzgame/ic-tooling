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

Process 0.81.0, Build 0.87.4, Testkit 0.110.0 and Sign 0.1.0 are published with
verified registry records and exact archive readback. See the
[cohort receipt](../release-ownership.md#signing-cohort--process-0810-first) for
source commits, tags, checksums and source-bound qualification.

Process adds bounded finite pipe input; `Input`/`Policy` lose `Copy`, retaining
`Clone`. Build keeps that boundary private. Testkit exposes the Process report
identity and takes a minor. Its earlier bounded clock/expiry contract remains;
see the [clock acceptance](../testkit-held-replies.md#bounded-manual-clock-progression).

[Tooling #12](https://github.com/dragginzgame/ic-tooling/issues/12) is closed.
Sign uses published Platform 0.34.15 to authenticate exact updates and independent
original-ID status reads. It removes caller pipe-writer threads and detached
signing workers while retaining original bytes and cancellation/cleanup custody.
The selected profile requires an already-unlocked, OS-authorized credential and
never prompts. Only public `canic-mainnet` metadata was inspected; no operator
secret was accessed or used. See the [signing contract](../sign-only-integration.md).

The final Linux archive/default graph/installed CLI gate passes, including twelve
library cases and three disposable encrypted-keyring cases. All three actual
provider jobs pass on Linux and Intel/ARM macOS, including locked refusal,
network denial and owned cleanup. This qualifies the supplier, not Canic adoption.

Concurrent compatible Cargo.lock updates are restored with the released package
versions and remain uncommitted. Recovery copies and scoped publication evidence
live in `target/deploy-sign/` and `.git/release-state/sign-0.1.0/`. Private Tools
0.1.3 remains pending; its maintained workflow changes are delivered through Git.

## Blockers and next actions

Producer acceptance has no outstanding blocker. Canic's thin Backup adapter and
installed acceptance remain in [Canic #5](https://github.com/dragginzgame/canic/issues/5).
[Tooling #11](https://github.com/dragginzgame/ic-tooling/issues/11) is closed;
Canic owns its same-receiver adoption of Testkit's bounded clock progression.

Linux and ARM macOS's separate Sign library CI jobs pass. Intel's separate
library compilation exceeded its provisional four-minute bound; it now has an
eight-minute step in a thirty-minute job. Corrected hosted CI runs asynchronously;
no whole-run pass is claimed. See [measured budgets](../qualification/ci-setup.md#job-deadlines--2026-11-17).
The actual-provider Intel job passed independently.

Further product work needs a demonstrated current caller requirement. Canic's
rewrite owns its Platform adoption; old adapters correctly refuse changed owners.
Scheduling targets the rewrite at `../ic-scheduling` and preserves its Testkit
client/held-reply-fixture boundary. See [current workload decisions](../workloads.md#current-decisions-and-retained-evidence).

## Delivery and evidence boundaries

Earlier publications and CI/cache/setup repairs remain in the owning
[release receipts](../release-ownership.md) and [CI diagnostics](../qualification/ci-setup.md).
Consumer archive/deployment acceptance is separate from supplier qualification.
Async deletion is best-effort; cancellation during creation can leave remote
effects. Custody assumes cooperative writers/groups and exclusive child reaping.
Preserve retained recovery evidence; no fleet-wide completion is claimed.
