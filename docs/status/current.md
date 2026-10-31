# Current handoff — 2026-10-31

## Admission and authority

Admission HEAD/main is released `5b658971c66f82e124b3efcbdfbc2efc77d8752c` (0.73.0).
The incoming dirty work held approved #276 groups and published Platform 0.15.1
adoption. The user now explicitly requested new Shared Tooling and deployment.
Shared 0.27.19 delegates the standard Git release and registry publication for
scoped crate deployment. Delivery selects Foundation patch 0.73.1 first, then
integration minor 0.74.0, preserving required gates and protections. At source
preparation all twelve package versions remain 0.73.0; no release effect has run.
Package locations, APIs, features, edition 2024 and MSRV 1.91 stay.

Fresh admission and the canonical 0.27.19 exporter are retained at
`/tmp/ic-tooling-deploy019.6xOjTH/`, pinned to
`54fc49e12f439dbe18d232f622a6d78e61773353`. Its changes from 0.27.18 clarify
delegated delivery; executable payloads and tool pins are unchanged. Admission
confirmed remote main matches HEAD, no open PRs or enforced branch protections,
and neither candidate tag nor registry primary version exists. Sibling release
processes are preserved; no Tooling build was active. No artifact cleanup is
included in delivery.

Admission, exporter impact/refresh and qualification are retained at
`/tmp/ic-tooling-shared018.eFTK4d/`. The clean isolated source pins Shared 0.27.18
at `ceaa1937c95c3d40c4453ca55d8655ebfcdf73f8` (151 payloads). This includes the
version-label correction after reviewed rule commit `99d04d1`; that correction
changes VERSION only. No sibling working checkout was edited or tool downloaded.

## Current release ownership

Both groups use literal package versions, with no root workspace.package.version.
`ic-build` owns seven public Foundation packages; `ic-testkit` owns Tools/Testkit/
Native and the two private Dev/probe members. One workspace, dependency catalog,
lockfile and existing Shared release runner remain. AGENTS, root Cargo comments,
policy and [release ownership](../design/release-groups.md) describe the same map.

Root CHANGELOG.md is an index. Canonical ledgers are
`releases/foundation/CHANGELOG.md` and `releases/integration/CHANGELOG.md`;
new integration detail is `releases/integration/0.74.md`. Coordinated releases
through 0.73.0 are preserved once in `releases/history.md`. Earlier Testkit-specific
notes remain in the integration ledger with their scope intact. The crate-local
changelog is a packaged navigation pointer. Historical headings/dates/prose were
compared against admission; only relative link targets changed. Maintained links
and the doc-check roster now include the canonical release paths.

Foundation's compatible tooling notes select 0.73.1; integration's exposed
Platform/Wire migration notes select 0.74.0. Standard commands own numeric
preparation. No old tag, plan, receipt or release identity was renamed.

Public release/resume/publication Make and consumer CLI commands reject missing,
empty and unknown RELEASE_GROUP before effects. Explicit environment selections
work; PACKAGE never infers a group and subsets must remain inside it. There is
no publication default or cross-group upload batch. The small group selector
only admits names; Shared retains complete group/graph admission and transactions.

Combined native CI uses `workspace.sh qualify`: explicit read-only all-package
qualification, one full lint pass and Cargo dry run. It cannot upload or select a
subset. Public publish/publish-check still require a group. The managed published
Build 0.73.0 cli editor and the per-package archive driver remain unchanged.

## Qualification and delivery

Actual Make routing/refusal fixtures pass, including absent/empty/unknown groups,
PACKAGE non-inference, explicit environment selection and one CI lint pass.
Publication fixtures pass both groups, scoped subsets, mixed exact versions,
licenses/provenance, failures/interruption and all-group dry-run qualification.
Both group adapter transactions pass with real disposable Git and admitted editor,
correct manifest/ledger files, unselected preservation, rollback, source/receipt
custody and committed checks. Setup and complete CI/MSRV gates are substitutes.

Actual formatting/hooks and dependency-order perturbation pass. Snapshot integrity,
group-aware declarations, manifest sorting, ShellCheck/Bash syntax, historical prose
comparison and document links pass. Real read-only Make queries return 0.73.0 for
both groups and their correct literal-version manifest/ledger file rosters.

`/tmp/integration-cohort.jsLHQJ/` retains current candidate 0.74 Tools/Testkit/Native
source and normalized archives, with real Rust 1.91 consumers against published
Foundation 0.73. Build errors, canonical registry/facade types, core Wire frame
round trips/limits and Native memory compose. No real versions or registry changed.
Retention qualification passes the selected caller fixtures under aliased relative
temporary parents, including premature completion and controlled failure probes.

[#276 — Reduce unrelated Host and Build version churn](https://github.com/dragginzgame/ic-tooling/issues/276)
has local implementation and remains open for delivery/native qualification.
No full workspace/release gate or new native macOS execution ran. All 14 checks,
including actual macOS 15 Intel/Apple Silicon, passed for released 0.73.0 only.
#268, #274 and #275 closed against that delivered release, not this new batch.

## Supplier and recovery custody

Selections remain Platform/macros/guard 0.15.1, Types 0.15.0, arithmetic-only
Metrics 0.23.1 and Wire/Derive 0.12.1. Platform independently versions Types again.
Published Wire's optional Types schema bridge still targets 0.1; matching work
at Wire #36 is unpublished. It is not selected or patched into Tooling's graph.
`/tmp/ic-tooling-platform015.v8GsIj/` retains its actual rejection plus 18 Rust
1.91 registry/Wire/memory/live-PocketIC tests, source/archive consumers, selected
Wasm, affected strict Clippy, graphs and supplier provenance. Both live tests pass
with localhost permission; the sandbox listener failure is retained separately.

Preserve `/tmp/ic-tooling-276-activation.Gjb80U/`,
`/tmp/ic-tooling-268-274-276.m4ejqy/`, and all prior cohort/adoption failures.
Preserve `target/package-qualification-2233002-0/`,
`/tmp/ic-tooling-package-fixture-fix.U8nZ5o/` (281 extracted Build tests) and
`/tmp/ic-tooling-merge277.3LQZJ9/` (Bash 3.2 repair/merge). Old locks, journals,
receipts, notes, failed targets and migration/retirement archives remain untouched.
Legacy attempts retain their original source/policy/file plan; never reinterpret
them after this layout change. Earlier handoff remains in admission evidence and
the [2026-10-30 archive](archive-2026-10-30.md). GitHub is the sole follow-up tracker.
