# Current handoff — 2026-11-02

## Completed five-group delivery

The maintainer authorized the cleanup delivery, corrective Foundation 0.74.1,
and the upper groups once matching Types 0.17 suppliers were published.
All five groups are now live, unyanked and independently registry-verified:

| Group | Version | Release/source commit | Public packages |
| --- | --- | --- | --- |
| Foundation | 0.74.1 | `7992cfd183888096c2ec922479836dc1ab0fbef1` | Artifacts, FS, HTTP, Process, Wasm inspector |
| Build | 0.74.0 | `5bff8453ac5c5d42a162c68486c498f5c1d41944` | ic-build |
| Text | 0.74.0 | `a60051341edc59216143c5dcce94db4c2c6da26b` | ic-host-text |
| Tools | 0.78.0 | `3c5ce4c732275922f404069ea2def690453dd74b` | ic-host-tools |
| Testkit | 0.79.0 | `f64b6c719438ba56c35d53f68e14e410927673e2` | ic-testkit, ic-testkit-native |

Every standard release passed its complete Linux gate and atomically delivered
main plus its annotated group tag. Every upload followed a publication dry run;
independent sparse-index/archive readback matches the uploaded checksum and clean
release source. Do not repeat increments/uploads. Foundation 0.74.0 remains
immutable, Git-only and unpublished. Its dry run exposed the repaired FS/Process
dev cycle. Build and Text have independent release groups.

Upper adoption is in `cd40bdb5`. Tools/Testkit select Types 0.17 and
Runtime/facade/PocketIC client 0.18. Consumers exchanging public principal,
certificate or client-owned DTO identities must align those suppliers. The
PocketIC server remains 16.1.0. Independent direct client adoption stays separate.
Native's optional memory feature uses IC Memory 0.63's published backing interface.
Its legacy facade 0.17 backend is stable-only, selecting no Types/runtime/client;
IC Memory's codec dependencies stay in this optional native profile. Native's
default and Wasm profiles remain dependency-free.

## Evidence and pending hosted qualification

Both upper releases passed full Linux/native/Wasm/installed/package qualification.
Public caller tests pass in private Dev. The same tests pass against exact current
registry packages, without source patches: error/principal/certificate identities,
core Wire frames/limits and backing-memory traits. Evidence is retained at
`/tmp/ic-tooling-five-groups.iOEuyP/checkout/target/published-integration.LagfoN`.
The verifier accepts string/table dependencies; its initial failed fixture is
retained separately at `published-integration.eiaS3h`. No repeat upload was needed.

Native macOS evidence is separate. Older core source `a6005134` passed Linux and
macOS 15 Intel in CI run 41448255449. Apple Silicon exposed a Process test's
immediate nonblocking accept assumption. The bounded accept/peer-identity repair
passes Linux and is delivered in `cd40bdb5`; Foundation 0.74.2 has test-only pending
notes and is not released. Testkit 0.79.1 notes cover the verifier-only repair;
no further package release is implied by these pending notes.

Hosted qualification of the latest delivered source remains asynchronous.
The Testkit release's CI run is 41477500722; at observation it was queued,
Browser passed, and optimizer/runtime were running. Final bookkeeping/verifier
source delivery starts a superseding run. Report its actual status; do not wait
solely for the matrix or treat pending/cancelled jobs as passes. The older Intel
pass does not qualify the new SDK source on either macOS architecture.

## Checkouts and recovery custody

The retained release checkout is `/tmp/ic-tooling-five-groups.iOEuyP/checkout`.
Its scoped release lock excludes the ordinary checkout's unrelated dependency
refreshes. The ordinary checkout is being fast-forwarded after final delivery;
retain its incoming lock selections while resolving the intended SDK/group updates.
Its lock backup and exact unrelated delta are under `preservation/before-upper-sync/`.
Preserve any new user work. No artifact cleanup or sibling release is authorized.

Durable logs, plans/receipts, registry archives/readback checksums, supplier
observations, CI observations and preservation evidence live under the ordinary
checkout's `.git/release-state/deployment-five-groups-20261102/`.
`release-checkout-path` identifies the retained checkout; its isolated release
state is copied into that evidence. Completed controllers must not be replayed.
The historical stash `bc5e544bb9adfea1e0b0a0aac77945ca573321d2` is not a patch
to apply over current metadata. Preserve failed artifacts and saved source custody.

## Adopted policy

Shared Tooling 0.27.27 at `651f5f7acb2e3055abd51ba5f835a2112801666a` is
adopted with all 162 managed payloads intact. Rust floor/development pin is
1.99.1. Metrics 0.26 stays native/arithmetic-only and absent from Testkit Wasm.
Full release gates remain mandatory: no narrower group mapping is activated.
Hosted CI runs asynchronously; local release qualification and publication are
separate effects. No Platform, Memory, Wire or other sibling release is authorized.
