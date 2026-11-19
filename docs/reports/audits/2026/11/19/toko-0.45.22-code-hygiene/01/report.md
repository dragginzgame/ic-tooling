# Toko 0.45.22 adoption and code hygiene

**PASS** for local adoption and the maintained Rust workspaces. Reviewed the
working tree on `main` at `3baa160`, including the pending Foundation method
inventory and Text byte-size formatter.

## Selection and rules

Both Cargo workspaces and all six workflow action references select 0.45.22.
The registry archive SHA-256 is
`4c4177a70760ea774fc0e14938593658b7b87facb3ee3f8848f65a402f778c7b`.
The snapshot exporter retained all 52 payloads byte for byte, including the
compiler installer and its manifest reader. Both lockfiles match their
pre-adoption contents outside the Toko package version and checksum.
[Producer revision](https://github.com/dragginzgame/toko-tooling/tree/3012643f1345ea2e67f31822fda09d4724391b58)

The new on-demand lint auditor checked 13 manifests across both workspaces,
including the effective policy of inheriting packages. It reported zero
discrepancies. The new Git-only release adapter requires a private workspace
with one inherited version. IC Tooling publishes independently versioned groups.
Its existing release adapters remain appropriate.

## Source review

The [local audit overlay](../../../../../../../audits/code-hygiene.md) and the
[previous pass](../../toko-0.45.21-code-hygiene/01/report.md) supply the scope
and comparison. The shared hygiene method and Rust baseline have identical
contents in 0.45.21 and 0.45.22. The module/import order check covered all
71 maintained Rust files, including the pending methods module and tests.

Reviewed export admission from the bounded reader through complete Wasm
validation, function-type lookup, exact method names and duplicate detection.
The validator checks function indices and types before the signature lookup.
The all-feature tests exercised signature, ambiguity, size and format refusals.
Reviewed Text's integer rounding through the largest input. Added a regression
test covering exact half-up rounding, trailing-zero trimming, both sides of the
rounded KiB-to-MiB boundary, and `u64::MAX`. Applied spacing in the pending
source. The maintained product/probe trees contain no broad source lint
allowances, TODO/FIXME markers or source symlinks.

## Fresh verification

| Check | Linux result |
| --- | --- |
| Snapshot admission, formatting and installed tool checks | Passed |
| Workspace Clippy with all targets and features, warnings denied | Passed |
| Workspace tests with all features | 114 tests and two doctests passed. Nine opt-in tests were ignored. |
| Workspace Rustdoc with warnings denied | Passed |
| Qualification workspace Clippy and release Wasm build | Passed |
| Wasm checks for Artifacts with method inventory, Text and HTTP | Passed |
| Workflow actionlint and setup-diagnostics tests | Passed, including 69 diagnostics assertions |
| Dependency usage check across both source roots | No unused dependencies reported |

Local logs and the original lockfiles are under `target/toko-04522/`. This pass
covers the local rules and hygiene checks. Installed simulator, live credential,
release archive and hosted macOS qualification have separate owners.
