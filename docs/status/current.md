# Current handoff — 2026-11-02

## Authorized Metrics removal deployment

The maintainer requested removing Tooling's Metrics dependency and filing issues
at Shared/Metrics during the ongoing deployment. Ship the cleanup as Testkit
0.80.1; 0.80.0 is already immutable and published. The full standard release gate,
publication dry run/upload and independent registry verification remain required.
No other repository source release or deployment is authorized.

Only native benchmark analysis previously used Metrics, via one checked ratio.
Its private average function now computes the same exact floor with quotient
and remainder, preserving zero-denominator precedence and final-result overflow,
including representable results whose naive intermediate multiplication overflows.
Its public error/API is unchanged. The added regression covers final-addition
overflow. All 23 benchmark tests, strict affected-package Clippy, full metadata,
Metrics-free native/Wasm graph policies, formatting and docs pass before release.
No recording/instrumentation engine or extra arithmetic crate was introduced.

The maintainer-approved narrow ownership exception is recorded in local AGENTS;
managed Shared payloads remain unchanged. Broader ownership/policy belongs to
[Shared #347](https://github.com/dragginzgame/shared-tooling/issues/347) and
[Metrics #127](https://github.com/dragginzgame/ic-observability/issues/127).
Those issues were explicitly requested, searched and opened. Do not close them
merely because this consumer removes its dependency.

## Already published in this deployment

| Group | Version | Immutable release/tag source |
| --- | --- | --- |
| Testkit | 0.80.0 | `5cd77b223e8f35cdd84ff733e7c3e27edd3d1bc0` |
| Native | 0.80.0 | `95820f0293b115782a48a6472fc40d095bf474d4` |

Both passed complete local Linux/native/Wasm/installed/package gates, publication
checks and live index/archive readback against uploaded checksums and clean source.
Testkit's registry-only combined caller tests pass; Native's two allocation tests
pass against its actual dependency-free registry package. Do not repeat uploads
or increments. Native is now independently released, allocation-only and has
removed its memory module/feature. Its generic allocation API is unchanged.

Native was initially held for Storage. During delivery, the owner published
Storage 0.65.0 with the metering helper, unblocking Native. Its checksum-verified
archive source is `84a7ecf6ed6517b989848c8e953d93fb5f561505` and exports native
`ic_storage::testing::{MeteredMemory, MemoryIoCounts}` under `test-support`.
The prior proposed Storage 0.64.1 contribution is historical; do not release it.
The old Scheduling patch needs rebasing for actual Storage 0.65 and its owner's
SDK choices. That consumer release remains outside this Tooling deployment.

## Checkouts, pending work and evidence

The retained release checkout is `/tmp/ic-tooling-five-groups.iOEuyP/checkout`.
It uses a scoped lock with the previous external patches. Ordinary checkout
`/home/adam/projects/ic-tooling` is at Testkit's `5cd77b22` with the approved
Metrics cleanup and unrelated lock refreshes still dirty. Leave those source
changes in place until the cleanup is delivered, then stash only the verified
task-owned paths, fast-forward and restore the unrelated lock selections.
Do not apply the entire task stash: its source changes will already be delivered.

Cleanup inputs/evidence are `/tmp/ic-tooling-no-metrics.jRr1eX/` (including the
source-only patch). It removes exactly Metrics 0.26.1 and preserves all 455 other
ordinary registry records. Testkit patch metadata will advance only its three
owned package versions; Native must stay 0.80.0. The ordinary lock stays outside
release commits. Preserve any new user work and all failed evidence/artifacts.

Durable evidence directories in the ordinary Git directory are:
`release-state/deployment-testkit-0.80.0/`, `release-state/deployment-native-0.80.0/`
and the new `release-state/deployment-testkit-0.80.1/` for this attempt. Copy the
isolated plans/receipts after delivery. Never replay the two completed attempts.
No artifact cleanup is authorized. Shared's group validation mapping remains
inactive, so this patch still uses the full configured gate.

## Hosted qualification and earlier delivery

Hosted CI runs asynchronously. New-source native macOS evidence remains separate
from local Linux gates; report actual results without waiting solely for matrices.
Previous base `b248f5d4` passed Linux x86-64 and both macOS 15 architectures in
run 41477907607; that does not qualify the later Wire, Native or Metrics changes.

Earlier Foundation 0.74.1, Build/Text 0.74.0 and Tools 0.78.0 remain published
and unchanged. Their evidence remains in `release-state/deployment-five-groups-20261102/`.
Shared Tooling 0.27.27 (`651f5f7`) and Rust minimum/development pin 1.99.1 are
unchanged. Six independent groups retain literal versions and historical custody.
