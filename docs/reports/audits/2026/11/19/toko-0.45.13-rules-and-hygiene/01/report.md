# Toko 0.45.13 rules and hygiene — 2026-11-19

Verdict: **PASS** for the declared local rules/adoption and code-hygiene scope.
Required checks pass after the corrections below; no actionable finding remains
in that scope. This does not establish fresh native-macOS, installed-simulator,
credential-provider, deployed, security-freshness or performance acceptance.

## Identity, method and scope

Trigger: maintainer request to adopt the latest Toko Tooling and perform a rules
and hygiene pass. Starting source: `2a3f9450f4e2347e0f2d392c4172729461175b39`,
on `main`. The root lock and eleven member manifests were already dirty: the
members were moving the tooling marker into ordinary dev-dependencies. Those
values are retained, with intentional dependency-scan metadata added. No staging,
commit, tag, push, product version change or sibling write was performed.

Selected package: Toko 0.45.13, source
`aa962a23937b9daea4b15521b7af7e5c15eac54f`, verified archive SHA-256
`8e5ed346b3e29656bd1521a24ec326e40236bd1ba9101e807684cfd77d8dcbe9`.
The canonical exporter retains all 50 files and changes only baseline bytes.
The six CI cache-action pins advance; action, tool-catalog and shared diagnostic
helper bytes remain unchanged. Both maintained workspaces select the same package.

Method: [code hygiene](https://github.com/dragginzgame/toko-tooling/blob/aa962a23937b9daea4b15521b7af7e5c15eac54f/audits/code-hygiene.md),
[common audit contract](https://github.com/dragginzgame/toko-tooling/blob/aa962a23937b9daea4b15521b7af7e5c15eac54f/audits/README.md)
and the [consumer overlay](../../../../../../../audits/code-hygiene.md) updated
in this batch. The new baseline's required Rust, dependency, workspace and
adoption guides were read at their exact linked revisions `a716b501274678a8ebf163ef576bbb549e974acd`
and `e873b124c1b23fa8e25718c6931b596e908c640f`. Module placement and distinguishing
negative assertions add criteria beyond the
[previous audit](../../toko-adoption-and-code-hygiene/01/report.md): comparisons
for those criteria are N/A (method change). Unchanged production function bodies
and preserved configuration values are separate comparable anchors.

Scope: all root product/private tooling packages and their tests/examples, the
independent owned-Wasm probe, documentation examples, current target guards,
selected tooling closure, shared CI diagnostics and workflow wiring. Historical
source-bound consumer adapters remain evidence; their held migrations and sibling
implementations were not refreshed or requalified. Generated WAT is data, not an
embedded Rust entrypoint. No additional public owner or generic tooling engine
was introduced.

## Review and corrections

| Obligation | Evidence and disposition |
| --- | --- |
| Child modules before imports | Move late declarations without changing names, visibility, attributes or public paths. Restore blank lines at import boundaries. Syntax-aware inspection covers all 68 maintained Rust files, including tests/examples. |
| Meaningful negative tests | Sign binding/configuration and Cargo lock/version tests assert distinguishing error variants/reasons. Build capture tests inspect typed no-follow causes and invalid-root admission. FS tests assert exact size refusals, cleanup outcomes and preserved directory identity. Existing foreign-file/output preservation remains asserted. |
| One checked complete example | Move the complete managed-server lifecycle example from the Markdown copy into `Server` Rustdoc and link it from the guide. Cargo compiles that exact `no_run` example. Conceptual design fragments remain explicitly illustrative. The existing executable preparation example stays with its Cargo target. |
| Ordinary tooling marker | The pre-existing ordinary dev-dependencies pass all-target/all-feature workspace Clippy. No package needs an always-false marker exception. Add only the intentional `toko-tooling` cargo-machete exemption to each member; add no dummy imports or blanket lint allowances. |
| Portable/target-specific work | Retain the previous concrete native boundaries: Reqwest's selected chunk API, Darwin libproc cleanup, Linux procfs observations and native credential backends. No compound test/Wasm guard was introduced. Artifacts/Text and the independent probe compile unconditionally for Wasm. |
| Catalog/compiler boundaries | Both workspaces retain Rust 1.100.0 and inherited lint tables. Used dependency roles, release-group membership and the separate qualification workspace remain coherent. Non-Toko lock bytes and existing member-manifest values are preserved. |

A syntax-aware comparison of 38 production/fixture files against HEAD preserves
import bindings, function signatures/bodies and module attributes/visibility;
it includes the moved native keyring fixture declaration. This is not a complete
ABI comparator. Intentional test-assertion edits and documentation changes are
reviewed separately. Public signatures and runtime behavior are unchanged.

The first import-spacing inspection found one remaining missing blank before
Process's re-export; that failure is retained in
`module-order-before-final-spacing.log`. The corrected complete inspection passes.
The disposable inspector refuses both a missing required source and an empty
inventory; it is audit evidence, not a new installed scanner or CI policy engine.

No path overrides, inline executable Rust injection, broad source lint allowances
or TODO/FIXME markers were found in the maintained product/probe roots. No tracked
or reviewed working-tree source symlinks were found. Installer-owned ignored tool
pointers remain separate. Existing typed errors, bounded byte/body/frame admission,
native cleanup custody and documented Testkit live-instance invariants are retained.
Generic timeout/panic/closed-endpoint observations and intermediate writer refusals
whose final typed admission cause is already asserted are not converted mechanically
into arbitrary errno/string expectations.

## Fresh verification and preservation

Host: Linux x86_64. rustc 1.100.0 (`0ef1bc386 2026-11-09`), Cargo 1.100.0
(`3acc79c0c 2026-11-05`). The preserved lock selects Platform 0.34.17 and
Protocol 0.3.6. Existing cargo-machete 0.9.2 was used for the focused dependency
inspection; no new shared tool catalog or CI gate was installed.
Disposable evidence is under `target/toko-rules-refresh-2026-11-19/`.

| Check | Result / evidence |
| --- | --- |
| Ordinary-marker baseline Clippy, workspace/all targets/all features | Pass (`ordinary-marker-clippy.log`), before adding exemptions or changing Rust assertions. |
| Workspace Clippy, all targets/all features, locked, warning-denied | Pass after the coherent edit batch (`clippy.log`). |
| Workspace tests, all features, locked | 110 native tests and one compile-only doctest pass; nine opt-in cases ignored (`tests.log`). |
| Build Clippy/tests after strengthening capture refusals | Pass; 19 library tests (`build-refusal-clippy.log`, `build-refusal-tests.log`). |
| Independent qualification workspace Clippy, all targets, locked | Pass (`qualification-clippy.log`). |
| `cargo machete --with-metadata crates qualification/owned-wasm-probe` | No unused dependencies (`machete.log`). |
| Module/import-order inspection | All 68 files pass (`module-order.log`); missing/empty inventories refuse. |
| Production layout comparison | 38 files pass (`production-layout-proof.log`). |
| Snapshot, independent selection, prepared-tool and format admission | Pass (`snapshot-check.log`, `qualification-selection.log`, `tools-check.log`, `fmt-check.log`). |
| CI setup fixture and Actionlint | 69 checks pass (`setup-diagnostics.log`); workflow syntax passes. |
| Artifacts/Text and independent probe Wasm checks | Pass (`portable-wasm-check.log`, `qualification-wasm-check.log`). |
| All-feature workspace Rustdoc, warning-denied | Pass (`docs.log`). |

Member preservation reconstructs the initial captured patch under `target/`,
verifies its original hashes, then compares complete TOML values after removing
only the new intentional exemption metadata (`member-preservation.log`). Reversing
only the Toko version/checksum projection reproduces both original lockfiles
byte-for-byte. No unrelated dependency or pending member choice was reset.

The later source change after the runtime gates is Process import whitespace;
the function-body proof admits it. No unchanged broad runtime suite was repeated
for that whitespace or these notes. The nine ignored native/installed cases,
native macOS, current sibling execution, archive release gates, hosted CI and
publication were not run. Earlier receipts retain their original source/host
identities. At the audit handoff product versions were unchanged and maintenance
notes were pending. Subsequent authorized delivery is recorded in the
[cohort receipt](../../../../../../../release-ownership.md#toko-04513-ruleshygiene-cohort--2026-11-19);
its additional checks do not change this audit's execution scope.
