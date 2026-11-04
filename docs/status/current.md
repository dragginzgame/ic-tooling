# Current handoff

## Local Platform 0.22 adoption

The root catalog now selects published Platform 0.22.0. Tools' certificate/tree
API uses the upstream identity shared directly with native Agent; its three
certificate graph profiles now require one `ic-certification` package. Pending
Tools 0.82.0 notes cover that identity change. Testkit's native Candid/management
adapter adoption is compatible, with pending 0.85.1 notes; Types 0.18, Runtime
0.19 and the exact management DTO pin remain unchanged. Deployment is authorized
for Tools 0.82.0 and Testkit 0.85.1, through main and the normal group release/
publication adapters. Tools 0.81.1 was never published; the public certificate
identity change requires the selected minor release instead of that patch.

Focused Linux evidence in `target/platform022-adoption/` records 30 certificate/
Agent tests, the management DTO round trip, and the installed PocketIC 16.1
management/snapshot test. Certificate-only, certificate/interface and combined
Candid graphs passed, alongside Testkit native/Wasm exclusion graphs. Formatting,
ShellCheck and local document links passed. Native macOS qualification of this
adoption has not run. The incoming lock backup is retained there; unrelated
package versions/checksums are preserved while Cargo repairs stale Windows edges.

## Delivered cleanup and package releases — 2026-11-04

The test-layout and complexity cleanup is integrated at
`c53f05954ffa35f7393f7055d69826e2bcea033a`. Shared Tooling 0.29.4 is adopted from
committed source `628ccc75d813bfd9bae777bfbaef55838d25b1b2`.
The ordinary checkout remains the implementation and release owner on main.

| Group | Published version | Release commit |
|---|---|---|
| Foundation | 0.74.3 | `7516f1091b6eaea7fc252bbcc3a4292c3066c20d` |
| Build | 0.74.3 | `ab8879a313845217caf41e264a8c92e990ce9404` |
| Text | 0.74.2 | `6cf984a0b60670a566854f4aa1e159c26c3e9d40` |
| Testkit | 0.85.0 | `91373903db4d208f53cb5ffe22acfe3adf6532fc` |

All four group commits and tags are pushed. All eight affected public packages
are uploaded to crates.io. Tools remains 0.81.0; Native remains 0.80.0. Private
Dev/probe packages follow Testkit 0.85.0 and were not uploaded. Each release passed
its complete source-bound 35-obligation Linux gate, publication dry run and upload.
Independent sparse-index/archive readback verified checksums, upload byte identity,
exact release VCS and license/provenance contents. A registry-only consumer ran
against the new packages without local path dependencies.

The cleanup moved inline tests into ordinary test modules without widening
production visibility. Runtime forwarding and cache failure finalization are
simpler. The maintained client now requires an explicit server URL;
`PocketIc::new(url)` and `list_instances(url)` replace implicit server discovery.
`pic` retains provisioning and managed startup. The separately checksummed
maintenance patch reverses before original donor/upstream reconstruction, retaining
licenses, protocol/crypto identities and fallible/no-replay/snapshot contracts.
The [group ledgers](../../CHANGELOG.md) own release notes and upgrade instructions.

## Native qualification

Linux x86-64, macOS 15 Intel and macOS 15 Apple Silicon all passed native CI,
optimizer, runtime and browser qualification at cleanup source
`7516f1091b6eaea7fc252bbcc3a4292c3066c20d`:
[CI 41815128302](https://github.com/dragginzgame/ic-tooling/actions/runs/41815128302),
[optimizer 41815137023](https://github.com/dragginzgame/ic-tooling/actions/runs/41815137023),
[runtime 41815141697](https://github.com/dragginzgame/ic-tooling/actions/runs/41815141697),
and [browser 41815147044](https://github.com/dragginzgame/ic-tooling/actions/runs/41815147044).
These results precede the later group-version metadata and pipeline integration;
retain their actual source identity. Final pipeline qualification is dispatched
asynchronously after Git delivery, with run identities recorded in deployment evidence.

## Release pipeline integration

Tooling now adopts Shared's stage runner as an additional managed entrypoint;
all 164 snapshot files match the committed supplier revision. Releases select
one adapter group, while both ordinary CI entrypoints retain all six groups and
the complete 35-obligation roster. The retention-prefix fixture reuses an exact
input match across groups, with fresh source/group admission and immutable
original execution proof/log custody. Rust and archive stages still execute.
See [release validation](../design/release-validation.md#selected-adapter-cases-and-unchanged-stage-reuse).

Focused Linux checks cover actual retention execution/reuse/invalidation, selected
adapter routing, full-CI routing, source drift and tampered stage evidence rejection.
The Testkit adapter case passed with product qualification effects substituted;
these fixture passes are distinct from native product execution above. Pipeline
changes are delivered through Git only; pending Build notes do not bump package
versions or authorize an additional crate upload.

## Lockfile and evidence custody

Unrelated user Cargo.lock selections were excluded from release commits and
publication. Three observed lock states and named stashes are retained under
`.git/release-state/deployment-cleanup-20261104/`; `post-release-Cargo.lock` is the
latest incoming selection. Restore it with only workspace-owned version alignment
after source-sensitive delivery checks. Keep the original bytes and stashes.

Development evidence remains in `target/loc-cleanup-20261104/` and
`target/complexity-reduction-20261104/`. Release logs, archive downloads, registry
consumer and pipeline fixtures are under `target/deploy-20261104/`. Compact
receipts, registry observations, native observations and lock custody are under
`.git/release-state/deployment-cleanup-20261104/`. Failed attempts remain retained:
one Testkit gate rejected input drift; publication admission later rejected another
lock rewrite. Fresh qualification/clean-source publication succeeded afterward.

The obsolete nested release checkout was reviewed for active work and unique refs,
then removed with explicit user approval. Its 760 KB of completed plans/receipts
remain byte-for-byte at
`.git/release-state/deployment-tools-0.77.0-testkit-0.77.0/historical-release-records/`,
alongside ref/status inventories and the review record. Retained-state admission
passes with the shared limits unchanged. No other artifact cleanup was performed.
