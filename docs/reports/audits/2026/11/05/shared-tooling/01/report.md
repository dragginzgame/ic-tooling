# Shared Tooling 0.30.0 adoption and rule audit

Reviewed on 2026-11-05 in the ordinary IC Tooling checkout, against dirty main
based on `1401a36f`. The incoming async-only Testkit work and unrelated lock
selections were preserved before this batch. This is a focused development audit,
not complete release, security or native-host qualification.

## Adopted source

Shared Tooling `0.30.0`, commit
`07dce12cbe4a798fd4a6d263095518678c4f6b45`, was clean and matched remote HEAD
when selected. The canonical exporter performed the impact review and refresh;
170 managed payloads match that independently selected commit. The extra fleet
guide and its linked document closure keep the new source-classification contract
available locally. Scheduled maintenance was not activated.

## Rule coverage and disposition

| Rule family | Inspected surface and result |
| --- | --- |
| Authority, ownership and dirty work | Local instructions, contribution rules, extraction contract, process inventory and incoming diff/lock backups. No sibling source changes, delivery, publication or artifact retirement. |
| Snapshot and flow convergence | Exporter manifest, root Make includes, common catalogs, actual LOC caller and release-group parser caller. Source files stay snapshot-owned; generic producer suites were not replayed. |
| Rust workspace and dependency ownership | Root/member manifests, all six release groups, locked metadata, dependency declarations, external IC/native exceptions and inherited selections. Existing package identities, features and versions remain unchanged. |
| Source/test layout | Maintained package source inventory and module declarations. Four private helper owners moved into conventional test paths; no singular `test/` owner or module-path override remains. |
| Lint policy | Rust attributes, conditional attributes, generated-source text, Cargo lint tables and lint-related shell/workflow flags. Both actual Rust `allow(dead_code)` attributes became `expect` with reasons. No blanket allowance for unfulfilled expectations was introduced. |
| LOC roles | Private Dev declares build tooling, with workload/test overrides; perf probe declares benchmarks, with protocol tests kept separate. All Rust source has a category. |
| Errors and standard-library conventions | Handwritten error pairs reviewed by package. Minimal Host/Native crates do not already depend on thiserror; Testkit's computed labels, redaction and conditional source projection remain explicit, as does Build's execution/retention source precedence. The example's mechanical pair now derives Error without adding conversions. One rendered install-error assertion now matches the typed variant. The sole unnecessary core path became std. |
| Compiler/source fixtures | Build's inline Rust inputs exercise compiler flags, Cargo source discovery and package qualification. They are qualification data, not application/canister entrypoint injection. No production include/path indirection was found. |
| CLI and external input boundaries | Clap entrypoints and typed parsers; Host process capture budgets/custody; machine output parsing and diagnostic-only lossy rendering. Existing independent admission and recovery boundaries retained. |
| Encoding, metrics and storage | Candid remains at native/external-protocol adapters, including ledger evidence; owned application fixtures use Wire. Tooling has no Metrics dependency or Storage-specific allocation helper. No encoding, hash, gzip or stable-store ownership moved. |
| Native interfaces and source provenance | Linux/macOS cfg attributes, scoped native exceptions and imported-client policy. Platform-dependent expectations remain conditional. Existing donor/upstream source and license guards remain the owner of the earlier async cut. |
| CI, release and artifact custody | Workflow triggers, pinned actions, read-token scope, three native hosts, superseded-run cancellation, explicit group routing, source admission and separate publication adapters. Optional isolated final-candidate/reuse contracts were not enabled by adoption. Existing retained evidence was preserved. |
| Documentation and handoff | Group pending notes updated without version preparation. Current handoff replaces historical detail with canonical evidence links and preserves release/lock recovery facts. |

Public `testing` modules in Build, Host FS and Host Process remain normal
feature-selected library APIs. Moving or renaming them as private fixtures would
change their consumer contract. The eight LOC diagnostics were reviewed: three
report source macros and five report conditional attributes. They correspond to
ordinary code generation or explicit OS/imported-client gates, not skipped files.

## Focused evidence

Logs, before/after inventories, snapshot impact/adoption records and incoming
backups are under `target/shared-adoption-20261105/`.

- Strict all-target/all-feature Clippy passed for Artifacts, Process, Build,
  Testkit and Dev. Minimal Build/Testkit test-target Clippy also passed, checking
  the new expectations in both selections. Testkit was checked again after the
  example/error-assertion edit.
- Linux: 67 artifact tests, 161 Process tests, 207 Build artifact tests and five
  private statistics tests passed. The first Process attempt had 13 socket-bind
  permission failures in the sandbox; the native-access rerun passed all 161.
- The installed PocketIC 16.1 ownership/install-recovery case passed with the
  typed invalid-Wasm assertion (one explicitly selected ignored test).
- Snapshot integrity, dependency inheritance/pins, manifest/Rust formatting,
  product documentation links, actual `make cloc CLOC_SUBCRATES=1` wiring and
  the real Testkit group-plan parser were checked.
- The task's incoming Cargo.lock remains byte-identical. Native macOS 15 Intel
  and Apple Silicon were not executed; Linux evidence does not qualify them.

Schema 2 reports 33,253 library/application lines, 47,886 test lines, 907 example
lines, 625 benchmark lines and 1,510 tooling lines: 84,181 total, zero unclassified.
The baseline for this batch was 84,194 total. Moving 2,135 lines out of the
library category is a classification correction, not code deletion.

## Independent findings

The audit does not certify complete compliance. Two remaining ownership changes
have their sole actionable records in GitHub:

- [#333 — Application-owned PocketIC logging](https://github.com/dragginzgame/ic-tooling/issues/333):
  inherited constructors still create log files and attempt global subscriber
  setup. This needs an intentional canonical-logging migration, dependency and
  provenance changes, and consumer/lifecycle qualification.
- [#334 — Target-owned fixture scratch](https://github.com/dragginzgame/ic-tooling/issues/334):
  generated standalone qualification fixtures still default to `/tmp`. Move their
  defaults and actual callers together, preserving deliberate outside-root path
  cases and historical evidence; do not patch snapshot-owned helpers locally.
