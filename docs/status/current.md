# Current implementation — 2026-11-18

## Decisions and scope

[Design revision 2](../DESIGN.md) is accepted. Required workflows determine owners
and interfaces; do not restore old exports or architecture to satisfy obsolete
callers. Read the affected [implementation contracts](../DESIGN.md#implementation-contracts)
before changing code. Build does not own archive qualification or release editing.

Stay in this repository on `main`. Ordinary fixes remain local; hosted CI runs
asynchronously. Qualify sibling consumers through [read-only current-source
snapshots](../qualification/local-sources.md), including working edits; refuse
capture drift and stale adapters. Consumer implementation stays with its owner.

Tooling-owned Auth, Storage, Observability and Wire migration lanes remain on
hold; this does not freeze their owning repositories. Backup adoption is
user-owned. Profiler is retired. The [workload register](../workloads.md) retains
historical source identities and consumer boundaries; no current unfrozen caller
justifies a new snapshot-set/cache API without a defined partial-effect contract.

The read-only predecessor is commit `a00369fc29fa837c6deb022cceb7e09aa447292c`
in this repository's history, not the current checkout. Historical inventory and
consumer observations are at `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3` under
`docs/design/consumer-evidence.txt` and `inventory.tsv`. Stage 1's owned-build
checkpoint passed; Stage 2's byte/filesystem slices have qualified Backup uses,
without claiming complete restore adoption. Stage 3's old Scheduling experiments
are historical; the rewrite's own current qualification determines its obligations.
See the [workload dispositions](../workloads.md#legacy-capability-disposition).

## Latest delivery

The Toko 0.45.8 hygiene cohort is published: Foundation 0.80.4 (both members),
Process 0.81.1, Build 0.87.5, HTTP 0.80.1, Text 0.77.3, Metadata 0.1.1,
Testkit 0.110.1 and Sign 0.1.1. All eight group tags are pushed; registry records
and exact archive readback match publication preflights. Linux archive, installed
simulator and disposable keyring gates pass. See the
[hygiene receipt](../release-ownership.md#hygiene-cohort--2026-11-18).

The preceding Process 0.81.0, Build 0.87.4, Testkit 0.110.0 and Sign 0.1.0 cohort has
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

Unrelated compatible Cargo.lock updates remain uncommitted with the released
local package versions. Recovery and delivery locks are retained under
`.git/release-state/hygiene-2026-11-18/`. Earlier signing evidence remains in the
cohort receipt and `.git/release-state/sign-0.1.0/`. Private Tools 0.1.3 remains
pending; its maintained workflow changes are delivered through Git.

## Blockers and next actions

Toko Tooling 0.45.8 adoption updates both workspaces, the authenticated
snapshot and CI action revision. The full hygiene pass groups Rust imports,
moves test declarations before implementation and labels dependency roles.
The private qualification CLI uses Clap with its existing command contract.
Ordinary Makefile Cargo commands keep `--locked`; `CARGO_NET_OFFLINE=true`
selects offline execution. Prepared qualification retains its offline contracts.
Workspace tests, warning-denied Clippy/docs, setup fixtures and tooling admission
pass; see the [audit report](../reports/audits/2026/11/18/code-hygiene-and-tt-adoption/01/report.md).
The optional setup module remains unselected pending
[Toko Tooling #396](https://github.com/dragginzgame/toko-tooling/issues/396) and
[Toko Tooling #397](https://github.com/dragginzgame/toko-tooling/issues/397);
the existing bounded collector preserves both guarantees. Compiler-policy edits
and the corrected combined Testkit/Sign archive feature selection are delivered.
Consumer requalification is not selected for this mechanical source batch;
production consumer adoption stays with its owner.

Producer acceptance has no outstanding blocker. Canic's thin Backup adapter and
installed acceptance remain in [Canic #5](https://github.com/dragginzgame/canic/issues/5).
[Tooling #11](https://github.com/dragginzgame/ic-tooling/issues/11) is closed;
Canic owns its same-receiver adoption of Testkit's bounded clock progression.

Earlier native-host acceptance and Intel's failed provisional timeout remain in
the [measured-budget record](../qualification/ci-setup.md#job-deadlines--2026-11-17).
This delivery's hosted CI runs asynchronously; no fresh macOS or whole-run pass
is claimed. GitHub has no separate application deployment target here.

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
