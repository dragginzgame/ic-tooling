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

- Secret-scanner setup now uses bounded sanitized diagnostics, failure uploads
  and artifact reporting. Actionlint and 57 focused checks pass, including the
  actual pinned installer's controlled download failure. See [setup evidence](../qualification/ci-setup.md).
- Process's inline macOS test and Testkit's fixture constructor now live in
  test-only files through ordinary directory modules. Both macOS Process targets
  pass all-target Clippy; Testkit's managed-server Clippy and four ordinary server
  tests pass. Both release archive gates pass, including Testkit's installed
  CLI/simulator checks; native macOS execution remains with asynchronous CI.
- Process 0.80.5 and Testkit 0.108.1 are published from `02ac139` with verified
  registry readback. Both archive gates used committed dependencies; see the
  [release record](../release-ownership.md#process-0805-and-testkit-01081-delivery).
- Concurrent Protocol/Clap/Nix/Zeroize lock edits are preserved in
  `target/deploy-scanner/concurrent-Cargo.lock` and restored with the two released
  member versions. They remain uncommitted. Private Tools 0.1.3 remains pending.

## Blockers and next actions

[IC Tooling #11 — Support bounded clock progression with held Testkit replies](https://github.com/dragginzgame/ic-tooling/issues/11)
is implemented locally: the exclusive controller admits bounded forward clock
steps, processes expiry, and refuses further use after interrupted dispatch.
Eight fixture tests, Clippy/Rustdoc and three installed cases pass; all five
instances are independently observed deleted. See the [clock contract and evidence](../testkit-held-replies.md#bounded-manual-clock-progression).
Deployment now selects Testkit 0.109.0 for its public error variants; exhaustive
matches need updating. The complete archive gate passes with committed dependencies;
see the [release record](../release-ownership.md#testkit-01090-delivery).
Concurrent lock edits are saved under `target/deploy-clock/` for restoration with
the new member version. Canic's same-receiver acceptance remains with Canic.

The secret-scan fixture false positive and Linux archive name-collision refusal
are fixed. Full-history scanning, candidate admission/rejection and the four-group
Linux archive gate pass; see [failure causes and evidence](../qualification/ci-setup.md#reviewed-ci-failures--2026-11-17).
The delivered cache/timeout/CI fixes and handoff cleanup at `92655b1` pass
[hosted CI](https://github.com/dragginzgame/ic-tooling/actions/runs/44073531569).
The new scanner failure-upload path and native test relocation have been pushed;
hosted observation remains outstanding and no new CI run is awaited.

On a delivery request, select affected groups under [release ownership](../release-ownership.md).
Current work removes reliance on ephemeral scanner logs and separates test-only
code without changing runtime behavior or public contracts.

Further consumer work requires current requirements and source capture. Canic's
rewrite owns its Platform adoption; old adapters correctly refuse changed owners.
Scheduling targets the rewrite at `../ic-scheduling` and preserves its Testkit
client/held-reply-fixture boundary. See the [workload decisions](../workloads.md#current-decisions-and-retained-evidence).

## Delivery and evidence boundaries

Foundation 0.80.3, Process 0.80.5 and Testkit 0.108.1 are published; exact commits,
checksums, tool selections and acceptance live in the [release receipts](../release-ownership.md).
Earlier publication and native-host evidence remain in that document, not here.

Consumer archive/deployment acceptance is separate from supplier qualification.
Async deletion is best-effort; cancellation during creation can leave remote
effects. Custody assumes cooperative writers/groups and exclusive child reaping.
Preserve retained recovery evidence; no fleet-wide completion is claimed.
