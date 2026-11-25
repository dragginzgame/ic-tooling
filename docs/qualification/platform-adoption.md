# Platform adoption

## Platform 0.35.1 adoption

On 2026-11-25 the live crates.io index confirmed non-yanked Platform 0.35.1,
Protocol 0.3.10 and Types 0.21.5. Their archive checksums match the locked
selections. Sign retains Platform's native secp256k1 update and status signing
boundary. Testkit's held fixtures select only Protocol's `http-request` feature
and one Types identity. Default Testkit excludes these dependencies and WAT.
The independent owned-Wasm-probe workspace has no Platform dependency. The
existing product APIs and fixtures needed no changes.

Both workspaces passed locked metadata. On Linux, all-target/all-feature Clippy
passed for Sign and Testkit. Sign passed 12 library tests and Testkit passed eight
library tests with held fixtures enabled. Three installed Testkit cases were
ignored by that selection. Evidence is in `target/platform-0351.v7BzF14o/`.

The Scheduling replacement uses the current consumer lock directly and switches
only its Testkit dependency. The captured lock already includes Protocol 0.3.10,
Types 0.21.5 and Runtime 0.23.27. Source, replacement graph, external lock and
both host-driver Clippy checks passed against HEAD
`b4f31db4ec957be0e8d8cd8ed130f627561e01c3`, with source tar SHA-256
`66286db76d179488a3bb7e97c6dc1d528de1047d7187da4e4d8f5602b8966cf2`.
The capture is in `target/scheduling-new.4vflCifB/`. The Once installed selector
patch refused the capture, so the result covers host drivers only.

## Testkit 0.108.0 delivery

The Protocol 0.3 adoption below is published in Testkit 0.108.0. Release resolution
selects non-yanked 0.3.3 within the reviewed `>=0.3.2, <0.4.0` range; its HTTP
encoder is byte-identical to 0.3.2. Linux archive and fresh installed Testkit/CLI
checks pass, alongside versioned Scheduling driver admission. See the
[release receipt](../release-ownership.md#foundation-0803-process-0804-and-testkit-01080-delivery)
for source, hashes and validation boundaries. The earlier local evidence below
retains its original scope.

## Local Protocol 0.3.2 adoption

The registry review on 2026-11-17 found non-yanked Protocol 0.3.2, Types 0.21.2,
Platform 0.34.11 and Runtime 0.23.15. Testkit is the only direct Platform-family
consumer in this repository: its optional held-reply feature now selects Protocol
`>=0.3.2, <0.4.0`, still with only `http-request`, and retains current Types 0.21.2.
The facade and Runtime are not product dependencies. The separate owned-Wasm-probe
workspace has no Platform-family dependency. Historical consumer patch context is
not an independent Tooling supplier selection; frozen comparisons remain excluded.

Protocol 0.3.2's downloaded archive matches registry SHA-256
`b2b1db444c839e816c1f2b6edec5ef8966a3f022a73a12819cc0db8477e5b560`
and identifies source `80a5b488666629dd28152f6bcb6d489091d55994`. Its
`src/http_request/` tree is byte-identical to the previously locked 0.2.8 archive.
No encoder adaptation is needed. Nevertheless, Testkit exposes Protocol's
`EncodeError` through `CompileError::Encoding`; its new Cargo type identity needs
the pending Testkit 0.108.0 minor release. Consumers naming that payload type must
select Protocol 0.3. Package versions and published releases have not advanced.

Evidence under `target/platform-adoption-latest/` records six passing held-fixture
unit tests, warning-denied Testkit Clippy, sole `http-request` feature admission,
default-graph exclusion and independent-workspace metadata. The lock delta from
the starting working tree contains only Protocol's version and checksum; earlier
unrelated lock changes are preserved. The two installed tests remain ignored;
unchanged encoder evidence does not constitute a fresh installed or native-host run.

The Scheduling adapter now admits the captured consumer's registry Testkit 0.107.4
and Runtime 0.23.15 context. Its local replacement admits only the reviewed new
Protocol checksum while retaining refusal of unrelated external lock changes.
`bash scripts/qualification/scheduling-consumer.sh admit-replacement` passes
source capture, graph/lock admission and both driver Clippy checks without requiring
a server. Capture `target/scheduling-new.lc7k0gVW/` has HEAD
`df5d14a482973e427ea79cae022bd38971b39fa7` and source-tar SHA-256
`90ece655056c467ba7653319368a1cf1471f34aec3beb09ac34982b882ac3fe2`.
Sibling implementations are unchanged. This removes stale supplier admission and
the need to start a simulator merely to qualify the replacement driver graph.
The changes are local; publication and installed consumer delivery are separate.

## Protocol 0.2.7 deployment

Testkit 0.107.4 selects the compatible requirement `0.2.7` (`>=0.2.7, <0.3.0`),
with the root lock selecting the latest non-yanked Protocol 0.2.7. Its verified
archive SHA-256 is `db4ea27c2705897b24b1c7c239db916abbb4c15eb97c1adcda896d0ca3891ff4`,
source `26834168a74cf76a4b8f93d4450c3da4f9e3f5db`. The published 0.2.6/0.2.7
HTTP encoder source trees are byte-identical. Only `http-request` is enabled;
Types and original PocketIC identities remain unchanged. No product adaptation
or compatibility layer is necessary.

This removes the exact patch-cohort conflict tracked in
[Tooling #9](https://github.com/dragginzgame/ic-tooling/issues/9), while Cargo's lock
still records exact admitted bytes. The independent qualification workspace has
no Protocol dependency. Scheduling's local supplier adapter explicitly admits
only Protocol 0.2.7's reviewed checksum in addition to its captured external
identities; unrelated lock drift still refuses. Consumer source stays untouched.
Release evidence is recorded in the [Testkit receipt](../release-ownership.md#testkit-01074-publication).

## Earlier Protocol 0.2.6 refresh

Testkit 0.107.3 selects Protocol 0.2.6. Its non-yanked crates.io archive has
SHA-256 `758e0b8455b2ab340a4503f211357b7725f52a55a2c8c209ee2f01b122763b7e`
and identifies source `a813d4ea5378515984a547a9f4e9d37f2124cf14`. Types 0.21.2
remains the latest published release; its compatible requirement and checksum
below are unchanged. Only Protocol's version/checksum changed in the resolved
dependency selections. The managed-server capability was also published in 0.107.3.

The verified 0.2.5 and 0.2.6 `src/http_request/` trees are identical. New Caffeine
and management-snapshot modules are gated by disabled features. Tooling still
selects only `http-request`, preserving unspecified pricing, exact request fields,
output limits and original PocketIC types. No caller/API adaptation or new
mechanism is needed; Protocol retains encoder ownership.

The repository inventory below was rechecked across both maintained workspaces,
product/dev/build declarations, scripts and native tool selections. Only Testkit
directly selects Platform crates. The independent owned-Wasm-probe workspace has
no Platform dependency. Captured sibling manifest context is not a Tooling pin;
consumer migrations remain in their owners. Auth and the other expressly frozen
comparisons remain excluded. The optional [product CLI](../simulator-ownership.md)
now composes Testkit managed-server instead of the old 0.106.0 launcher; the
private qualification prototype is removed. The CLI graph does not select Platform;
the product's held fixtures retain their explicit Protocol/Types boundary.

Focused evidence in `target/platform-adoption-current/` passes six held-fixture
unit tests, all-feature/all-target Testkit Clippy with warnings denied, locked
metadata for both workspaces, exact Protocol feature admission, and exclusion of
fixture dependencies from default Testkit. The two installed held-fixture checks
below retain applicability to the unchanged encoder and Types code; they were
not rerun or relabeled as new simulator evidence. Formatting and installed Toko
admission pass. This refresh was subsequently published in Testkit 0.107.3; the
[release receipt](../release-ownership.md#testkit-01073-publication) adds fresh
archive acceptance and registry readback. No sibling edits or new native macOS
qualification are claimed. Toko's separate 0.44.11 adoption is also delivered.

## Earlier Protocol 0.2.5 adoption

The earlier adoption selected registry Protocol 0.2.5 and Types 0.21.2.
Both verified archives identify source `9dfdc0083fa5d6e56da7b90c04c20de4b2d3cc9f`:

| Package | Archive SHA-256 |
| --- | --- |
| Protocol 0.2.5 | `93ed6052f78c99bf45bcf1ffdd11bd6eb4de4463dbe425a7b63e093b8f7a0ed2` |
| Types 0.21.2 | `3b08201b2ebbb9a9f0e1c8ee2854f48ff3f68a777c441147273c600bf6723d7d` |

The registry also advertises Platform 0.34.6 and Runtime 0.23.13. Tooling needs
neither facade nor canister runtime; its fixture uses only the narrow HTTP encoder
and Principal type. Protocol adds a separate version-one pricing encoder while
the existing encoder preserves unspecified pricing. Types source is unchanged
from 0.21.1. No product implementation or compatibility layer is added.

## Historical inventory at the 0.2.5 refresh

| Surface | Selection and disposition |
| --- | --- |
| Product workspace and lock, including dev/build edges | Only Testkit's optional fixture edges select Protocol 0.2.5 / Types 0.21.2. Default Testkit has neither; no legacy Platform or Runtime remains in this workspace. |
| Independent `qualification/` workspace and lock | The owned Wasm probe has no Platform dependency. |
| Scheduling adapter | Refreshed manifest context for Runtime 0.23.13 / Types 0.21.2. Host drivers pass with local Testkit; installed migration remains blocked by consumer probes and the published Testkit Types pin. See the [source-bound record](../../qualification/scheduling/README.md#platform-runtime-refresh--2026-11-16). |
| Old Platform raw_rand/Agent adapters | Removed with their three exclusive scripts and Make targets. Current Platform has removed every patched owner script; neither old API restoration nor historical-source fallback is appropriate. [Dated results](../../qualification/platform/README.md) remain historical. |
| Native simulator bootstrap | Retain explicitly prepared `ic-testkit-server` 0.106.0 and PocketIC 16.1.0. The legacy launcher can contain old Platform transitively; it is an external tool, not a migrated product graph. New Testkit deliberately has no launcher. Replacing this tool requires a separately owned server-lifetime/preparation capability and its acceptance; this adoption does not claim that work is complete. |
| Query, DB and Canic adapters | Their Platform references belong to captured consumer manifests or patch context, not independently maintained Tooling dependency selections. Product migration remains in those owners. Review captures are listed below; earlier runtime evidence is not relabeled as current acceptance. |
| Backup / Auth / Storage / Observability / Wire | Backup adoption is user-owned, Auth is on hold, and the remaining three are frozen by maintainer direction. Their old captures/adapters are excluded from migration and are not current acceptance. |

The removed obligation is maintaining executable adapters for vanished Platform
fixture scripts. Testkit continues to delegate encoding to Protocol and retains
its fixture bounds and original PocketIC identity. Delivery of the new pins and
completion of Scheduling's installed qualification remain outstanding; this is
not a fleet-wide completion claim.

## Historical current-source review captures

All captures use the shared double-capture helper, including tracked edits and
non-ignored additions, under `target/platform-adoption-next/<owner>-review/`.
Each directory retains `source-head`, `source.sha256` and the original source tar.
No sibling source was edited.

| Owner | HEAD | Observed selection / boundary |
| --- | --- | --- |
| Platform | `7e0e3af86372bc77729c808b42a04e154d801147` | Current owner has its own replacement qualification workspace; old raw_rand/Agent script paths are absent and both old patches refuse. |
| Scheduling | `3d2b3fd3775252e779181c4175a8ecf661afb60b` | Runtime 0.23.13 / Types 0.21.2 manifests; stale independent lock and unmigrated probes, tracked in Scheduling #2. |
| Query | `4a7305c4c804dcd0a0ce3e3433c6a8662d81fd75` | Consumer still selects Platform 0.33.7 alongside 0.34.5, Protocol 0.2.4 and Types 0.21.1. Its product migration is outside this repository. |
| DB | `bf586ae4c7b1834102278624033c8398ac354cbe` | Locks already contain Types 0.21.2 and qualification Runtime 0.23.13; older Runtime/Types also remain in consumer-owned graphs. No installed rerun claimed. |
| Canic | `e84702b3bdcf4eb209476aadd053df21f18dd8eb` | Manifest retains legacy Platform 0.33.0 / Runtime 0.22.1 requirements. Its rewrite determines future requirements; no bulk migration of historical adapters. |

## Focused acceptance

`target/platform-adoption-next/` retains six passing Testkit unit tests, all-target
warning-denied Clippy, both separately selected installed module/controller tests,
and independent inventory showing all three instances deleted. The installed
checks observe exact request fields, unspecified pricing, original callback bytes,
reverse release and refusal behavior. Default-graph exclusion and the sole
Protocol feature `http-request` are checked from Cargo metadata/tree output.
Recorded source and prepared-tool hashes remain unchanged across execution.

Scheduling's host checks pass with the local supplier; its Wasm build failure is
recorded without changing consumer code. Old Platform patches refuse the current
source before retirement. Formatting, snapshot/lock checks, script lint and Make
dispatch checks cover the local changes. No new archive qualification, publication,
native macOS acceptance or production consumer migration is claimed.
