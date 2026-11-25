# Current implementation — 2026-11-25

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

## Current dependency adoption

Both workspaces, the installed snapshot and CI select Toko Tooling 0.46.4.
`make check` passes. The [Platform adoption](../qualification/platform-adoption.md#platform-0351-adoption)
qualifies Sign with Platform 0.35.1 and the Testkit held fixtures with Protocol
0.3.10 and Types 0.21.5. The Scheduling replacement verifies the existing
consumer lock directly, removing the previous Protocol exception. Its host
drivers pass Clippy. Installed Once qualification remains blocked by the stale
selector patch.

The Linux Sign provider now selects Secret Service 6.0.0. Clippy and disposable
keyring signing, cancellation and locked refusal passed against the selected
lock. The Linux keyring test dependency still selects Secret Service 5.2.0
through the Keyring backend. Native-run evidence is in
`target/ss.9zsVPp/target/sign-keyring.YAvAcGGk/`.

## Latest delivery

All nine public crates shipped with Toko 0.46.0, including Build 0.87.7. See the
[verified release receipts](../release-ownership.md#toko-0460-release).

Foundation 0.80.7 and Text 0.77.5 are published. See the
[release receipts](../release-ownership.md#toko-04522-foundation-and-text-releases).
The preceding nine-crate release shipped from `297c304`. See the
[verified publication receipts](../release-ownership.md#toko-04521-hygiene-releases).
The previous hygiene and CI review used
[Toko 0.46.0](../reports/audits/2026/11/21/toko-0.46.0-hygiene-and-ci/01/report.md).
The revised Intel archive timeout awaits hosted verification. The
[HTTP adapter](../http-body.md) has the only Rust Wasm target guard. The
unguarded compile reproduced the missing Reqwest method. Artifacts, Text and
the qualification probe build for wasm32; Testkit's generated WAT export test
passes. The repository has no Rust canister endpoint handlers or templates.

Process stdin and Testkit clock cases now share their parent test modules. The
installed clock selector runs exactly one case. Current Canic qualification
uses its own tests and lists each target before running exact selections.
All 11 renamed selectors passed on Canic 0.232.0. These changes ship through
Git because they affect test organization and qualification. See the
[Canic evidence](../../qualification/canic/README.md#current-cli-text).

The preceding tooling delivery selected Toko 0.45.16. Compared with the published cohort, this updates developer tooling,
test code and qualification. Product runtime code and dependencies are unchanged,
so this delivery goes through Git only. Linux passed the process-reaping test
and all three native signer cases with the delivery lock. Locked refusal now
runs in its own disposable session, independent of signing and cancellation
(`target/sign-keyring.o6bw22P1/`). Native macOS qualification runs in CI.

The Toko 0.45.13 rules/hygiene cohort is published: Foundation 0.80.5 (both
members), Process 0.81.2, Sign 0.1.2 and Testkit 0.110.2. All four group tags are
pushed and registry readback matches exact preflight bytes. Linux archive,
installed simulator and disposable provider gates pass. See the
[cohort receipt](../release-ownership.md#toko-04513-ruleshygiene-cohort--2026-11-19).
Build 0.87.5, HTTP 0.80.1, Text 0.77.3 and Metadata 0.1.1 keep their published
versions; their private test/tooling maintenance is delivered through Git.
Earlier [shared-diagnostic](../release-ownership.md#toko-04511-delivery--2026-11-19)
and [hygiene](../release-ownership.md#hygiene-cohort--2026-11-18) receipts retain
their original source/host evidence.

Process 0.81 owns finite pipe input; `Input`/`Policy` are `Clone`, not `Copy`.
Testkit 0.110 shares its report type identity. See the
[feature receipt](../release-ownership.md#signing-cohort--process-0810-first) and
[bounded clock contract](../testkit-held-replies.md#bounded-manual-clock-progression).

[Tooling #12](https://github.com/dragginzgame/ic-tooling/issues/12) is closed.
Sign first shipped with Platform 0.34.15 to authenticate exact updates and independent
original-ID status reads. It removes caller pipe-writer threads and detached
signing workers while retaining original bytes and cancellation/cleanup custody.
The selected profile requires an already-unlocked, OS-authorized credential and
never prompts. Only public `canic-mainnet` metadata was inspected; no operator
secret was accessed or used. See the [signing contract](../sign-only-integration.md).

Unrelated compatible Cargo.lock updates remain uncommitted with the released
local package versions. Recovery and delivery locks are retained under
`.git/release-state/toko-0.45.13/`. Earlier signing evidence remains in the
cohort receipt and `.git/release-state/sign-0.1.0/`. Private Tools 0.1.3 remains
pending; its maintained workflow changes are delivered through Git.

## Blockers and next actions

Qualification inherits Cargo’s network settings. Linux archive runs
verified both the default and caller-selected offline mode. The release-group
catalog declares the Text alias and passes the shared dependency-pin and
Cargo inheritance check. See [release ownership](../release-ownership.md).

The [Canic build and metadata targets](../../qualification/canic/README.md#current-build-and-metadata)
now exercise the captured CLI directly. Real Wasm publication and 17 pre-build
refusals passed against local Artifacts, FS and Process.

Canic owns its Store tests and filesystem publication code. The
[Foundation runner](../../qualification/canic/README.md#current-foundation-qualification)
now verifies local FS/Artifacts dependency edges and exact test outcomes.
The Linux runtime selection passed 54 Host and three CLI tests plus actual Wasm
publication and link refusal. The archive selection passed 39 Host cases.

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
