# Platform adoption — 2026-11-16

## Current Protocol 0.2.6 refresh

Testkit 0.107.3 selects Protocol 0.2.6. Its non-yanked crates.io archive has
SHA-256 `758e0b8455b2ab340a4503f211357b7725f52a55a2c8c209ee2f01b122763b7e`
and identifies source `a813d4ea5378515984a547a9f4e9d37f2124cf14`. Types 0.21.2
remains the latest published release; its compatible requirement and checksum
below are unchanged. Only Protocol's version/checksum changed in the resolved
dependency selections. Pending managed-server work is preserved separately.

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
comparisons remain excluded. The prepared 0.106.0 launcher is still an external
qualification tool with older transitive Platform dependencies; there is no Rust
type exchange with it. Replacing its setup/CLI wiring is separate Testkit tooling
work, now enabled by the local [Tooling #6 capability](../testkit-server.md), not a reason
to add the Platform facade or Runtime to Tooling's product graph.

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

## Repository inventory

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

## Current-source review captures

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
