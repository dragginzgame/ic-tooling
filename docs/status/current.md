# Current implementation — 2026-11-19

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

Toko 0.45.11 and shared setup diagnostics are delivered through the private Tools
maintenance path. The 50-file snapshot, both workspace selections and six CI action
references agree. Package comparison confirms only the disabled development-tool
selector changed in public archives; crate versions and runtime contracts stay
at the published cohort below. See the
[tooling delivery record](../release-ownership.md#toko-04511-delivery--2026-11-19).

The latest published crate cohort uses Toko 0.45.8: Foundation 0.80.4 (both members),
Process 0.81.1, Build 0.87.5, HTTP 0.80.1, Text 0.77.3, Metadata 0.1.1,
Testkit 0.110.1 and Sign 0.1.1. All eight group tags are pushed; registry records
and exact archive readback match publication preflights. Linux archive, installed
simulator and disposable keyring gates pass. See the
[hygiene receipt](../release-ownership.md#hygiene-cohort--2026-11-18).

Process 0.81 owns finite pipe input; `Input`/`Policy` are `Clone`, not `Copy`.
Testkit 0.110 shares its report type identity. See the
[feature receipt](../release-ownership.md#signing-cohort--process-0810-first) and
[bounded clock contract](../testkit-held-replies.md#bounded-manual-clock-progression).

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

Toko 0.45.13 adoption updates both workspace selections, the authenticated
50-file snapshot and all six CI action references. Its rules/hygiene pass puts
child modules before imports, strengthens distinguishing refusals and preserved
state checks, and compiles the canonical server usage example through Rustdoc.
Pending ordinary tooling dev-dependencies are preserved; all-target/all-feature
Clippy needs no marker exceptions. Intentional dependency-scan exemptions are
declared. Shared diagnostics still pass 69 checks with unchanged helper bytes.
Fresh workspace tests, Clippy, Rustdoc, portable Wasm, dependency-use and tooling
checks pass; see the
[current audit](../reports/audits/2026/11/19/toko-0.45.13-rules-and-hygiene/01/report.md).
All non-Toko lock selections and existing member-manifest values are preserved.
The maintainer authorized deployment: Foundation 0.80.5, Process 0.81.2,
Sign 0.1.2 and Testkit 0.110.2 are selected patches. Other product groups have
private test/tooling changes only and keep their versions. The scoped delivery
graph and Linux core archive gate pass; remaining registry-backed leaf archive
qualification and publication are active. Unrelated external lock updates are
retained under `.git/release-state/toko-0.45.13/` for restoration. Consumer
implementation and qualification remain with their owners.

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
