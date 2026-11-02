# Current handoff — 2026-11-02

## Delivery scope

The maintainer authorized the five-group cleanup delivery, approved corrective
Foundation 0.74.1, and held Tools/Testkit for Types 0.17. Matching suppliers are
now published; the scoped upper-group release authorization continues.
No sibling releases or artifact cleanup are authorized.

Already published and independently registry-readback verified:

| Group | Version | Release/source commit |
| --- | --- | --- |
| Foundation | 0.74.1 | `7992cfd183888096c2ec922479836dc1ab0fbef1` |
| Build | 0.74.0 | `5bff8453ac5c5d42a162c68486c498f5c1d41944` |
| Text | 0.74.0 | `a60051341edc59216143c5dcce94db4c2c6da26b` |

Do not repeat these increments/uploads. Foundation 0.74.0 remains immutable,
Git-only and unpublished. Its dry run exposed the repaired FS/Process dev cycle.
The test-only macOS socket-readiness repair has pending 0.74.2 notes; it does
not require another Foundation publication for this delivery.

## Qualified upper adoption; releases next

Tools 0.77.0 has pending 0.78.0 notes. Testkit/Native and private Dev/probe remain
0.78.0 with pending 0.79.0 notes. Use standard minor release, publication dry run,
publication and independent registry readback for Tools, then Testkit.
Full release gates remain mandatory; no group validation mapping is activated.

The source selects Types 0.17 and Runtime/facade/PocketIC client 0.18.
Real-source strict Clippy and focused public caller tests pass. Tools feature
and Testkit native/Wasm graphs pass: one Types identity, executor only in the
private Wasm probe, arithmetic-only Metrics 0.26 on native and none on Wasm.
The PocketIC server remains 16.1.0. Native client imports remain through the
current facade; independent direct client adoption is separate.

Platform 0.18 removed stable-structures. Native's optional memory profile now
uses IC Memory 0.63's published backing Memory interface. Its selected legacy
facade 0.17 is stable-only, with no Types/runtime/client capabilities. IC Memory's
core codec dependencies are confined to that optional native profile. Native's
default and Wasm profiles remain dependency-free. Do not hide duplicate Types
or relax the executor singleton to admit incompatible suppliers.

Private Dev now requires shared public error, principal, certificate, Wire and
memory contracts. The standard package gate still qualifies all ten public
archives. After both upper groups publish, run the opt-in registry-only
`CARGO_NET_OFFLINE=false bash scripts/release/test-integration-cohort.sh` for the
same tests with exact published inputs; retain its target evidence.

## Checkouts and custody

The isolated delivery checkout is `/tmp/ic-tooling-five-groups.iOEuyP/checkout`.
Use its scoped lock for delivery. The ordinary checkout's unrelated Cargo.lock
selections must stay outside commits and survive final fast-forward. Its current
local handoff also records the previous hold; preserve any new user edits when
updating it. No cleanup is authorized.

Durable logs, plans/receipts, upload/readback checksums, supplier observations and
preservation evidence live under
`.git/release-state/deployment-five-groups-20261102/` in the ordinary checkout.
Its `release-checkout-path` points to the isolated checkout. The prepared upper
controller may run after the scoped adoption is committed; it must not rerun
completed core releases. Keep failed artifacts and saved release state.
The historical stash `bc5e544bb9adfea1e0b0a0aac77945ca573321d2` is not a patch
to apply over current release metadata.

## Policy and native evidence

Shared Tooling 0.27.27 at `651f5f7acb2e3055abd51ba5f835a2112801666a` is
adopted with all 162 managed payloads intact. Rust floor/development pin is
1.99.1. Build and Text have independent groups. Full Linux release gates include
native/Wasm/installed/package qualification; publication is a separate effect.

Native macOS evidence is separate. Core-source CI run 41448255449 at `a6005134`
passed Linux and macOS 15 Intel. Apple Silicon failed the Process elapsed-deadline
socket test on an immediate nonblocking accept (`WouldBlock`); the new test-only
repair bounds that accept and verifies the original peer. Its Linux-focused test
passes; fresh hosted native evidence must still be observed asynchronously.
Do not call pending/cancelled jobs passes or substitute Linux for native macOS.
