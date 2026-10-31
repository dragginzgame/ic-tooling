# Current handoff — 2026-10-31

## Release-group editor contribution

The isolated Shared #328 contribution adds bounded `group-manifest` rendering
with explicit version/requirement selections; it changes no package versions or
dependency selections. Rust 1.91 passes seven renderer and six CLI tests. Strict
affected library/binary/CLI Clippy passes on the pinned 1.99.1 compiler; 1.91
Clippy encounters an existing unrelated batch-error lint expectation. Shared's
group preparation/rollback/resume fixtures use this compiled candidate, not a
published tool. Native macOS and publication remain unqualified by this batch.
See [Shared #328](https://github.com/dragginzgame/shared-tooling/issues/328).

## Source and local result

HEAD remains `85f759d7e964568dffecfdef1d6c8a7c6872be52` (Release 0.72.1).
Own versions remain 0.72.1; pending notes select 0.73.0 for exposed Platform/Wire
identities. Changes remain local and unstaged. Prior cleanup and incoming lock
edits are preserved, with authorized updates to three Platform and two Wire
records. The previous long handoff is preserved verbatim in
[the 2026-10-30 archive](archive-2026-10-30.md), including prior source-bound
qualification, migration/retirement backups and outstanding owner obligations.

The latest task adopts published Wire 0.11.0 plus its derive crate alongside
Platform 0.12.0/macros/runtime guard. Wire core Rust sources match 0.10.0; Platform
facade sources match 0.11.1. Platform's optional gzip supplier changed, but Tooling
does not select it. Tools' public principals require direct Platform 0.12 alignment;
Testkit's public Limits/codec causes require Wire 0.11. Wire's optional schemas and
endpoints remain disabled. Codec bytes, budgets and transport policy are unchanged.

Native CI uses `ci-publication`: prerequisite admission, publication source
admission and full workspace lint once, the dry run, then remaining checks offline.
Standalone CI/publication retain independent full gates, without a lint bypass.
Testkit Dev's direct Wire use is test-only; Platform remains production-owned for
workload limits. Build carries admitted cache targets without re-reading cache
mode. Cargo-discovery errors retain ToolError; fake Cargo preserves native paths.
Startup/driver share the PocketIC range message. Candid parser errors use derive
while retaining display and the intentionally terminated source chain.

## Latest evidence and limitations

Wire evidence is `/tmp/ic-tooling-wire-adoption.2VwCQY/`, retaining dirty admission,
index response, lock/index/HEAD, source provenance, server admission/hash and logs.
Published Wire names source `568196014214a27b1a679b119ffde45284ed5eae`.
Rust 1.91 passes nine Wire contract tests, one private protocol test and both real
PocketIC codec/refusal/rejection integration tests against the prepared 16.1.0
server. Strict all-target/all-feature Clippy passes for Testkit, Dev and the probe.
Selected Testkit/probe Wasm compilation and native/Wasm/runtime graph policies
pass. Manifest sorting, links and diff checks pass. Only the two Wire lock records
change from this step's admission; real index, HEAD and own versions are preserved.
No new native macOS execution or full workspace/release gate ran. Wasm compilation
and Linux installed tests do not qualify native macOS or live IC production.

Platform evidence is `/tmp/ic-tooling-platform-adoption.6oO1jt/`, retaining its
admission, index responses, provenance and logs. Published Platform names source
`7d9927f49320b06922fcd4dc745f8229f5a371fc`. Rust 1.91 passed 17 focused tests and
selected Testkit/Native/probe Wasm compilation; affected-package strict Clippy,
Host/Tools graphs and selected Testkit policies passed. That step changed only
three Platform lock records beyond its admission.

Prior cleanup evidence is `/tmp/ic-tooling-cleanup-0722.N0q9MJ/`, including original
failures and corrected logs. It records MSRV parser/server/driver/cache tests,
affected-package strict Clippy and Make ordering/refusal fixtures, plus script,
workflow, format, docs and Rustdoc checks. Reuse is bound to unchanged inputs.

The incoming lock's Wat 1.263 brings encoder 0.263 beside direct encoder 0.262,
failing the held-reply single-identity policy. It was preserved and policy was
not relaxed. The prior isolated copy passed with committed lock and changed Dev
manifest; current selected graphs do not override that known full-graph failure.
[#275 — Keep Wat and direct Wasm encoder selections in one graph identity](https://github.com/dragginzgame/ic-tooling/issues/275)
owns reconciliation. Preserve old package failures, release plans, locks, receipts
and retirement backups. No current-repository Git mutation, own version bump,
tag, publication, tool download, sibling edit or artifact cleanup ran.

## Reviewed upstream integration and issue ownership

[Shared Tooling PR #327](https://github.com/dragginzgame/shared-tooling/pull/327)
merged as `81ee6b59a31f6789c85caa4f9343e57ae5824c3e`. Its isolated snapshot fixture
repair includes documentation companions; focused publication/attempt/snapshot,
shell and docs checks passed on Linux. Hosted/native evidence remains bound to
its actual revision and run. The adopted snapshot remains Shared 0.27.11 at
`62edfbd43087a718bf0283e9040676a49211976d`. Tooling retains the lower publication
runner because consumer license/provenance/roster/source policy still needs an
owner; the higher adapter would introduce another admission flow.

[Observability PR #102](https://github.com/dragginzgame/ic-observability/pull/102)
merged as `691fb85ace5125e0f941def3acb95a21d3ec1a1d`. Metrics arithmetic adoption
remains owned by
[#268 — Delegate Testkit scaled benchmark averages to ic-metrics](https://github.com/dragginzgame/ic-tooling/issues/268),
with its published-profile prerequisite; no Git/path workaround was added.

The preceding PR review found no open Tooling PRs. PR #269 was closed as
superseded by the released ordinary-module test-support owner. Issues #270/#271
were closed against published 0.71/0.72 and recorded Auth installed evidence.
[#274 — Align Testkit typed Wire calls with the Platform 0.12-compatible Wire release](https://github.com/dragginzgame/ic-tooling/issues/274)
now has local Platform/Wire adoption implemented and qualified. It remains open
for coordinated Tooling publication; downstream availability is not established
by these uncommitted changes. GitHub remains the follow-up tracker.
