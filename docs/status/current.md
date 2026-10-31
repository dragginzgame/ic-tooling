# Current handoff — 2026-10-31

## Current source and repair

Admission is clean `5b78f75288f6dfb8caf83712f4df5f7157e9ea24` (maintainer commit
`0.73.0`). Root/package versions still read 0.72.1, with pending 0.73.0 notes.
The latest task repairs the archive-qualified Build library failures reported
from `target/package-qualification-2233002-0/`. Original source, archives, target,
logs and release state remain preserved. No Git mutation, release, own version
bump, tag, publication or tool download was performed in this repair.

Six Build tests failed because the fake Cargo producer required `REAL_CARGO`,
which disappeared when callers replaced the complete child environment. Four
failures reported the missing variable; two surfaced earlier/wrong phases as a
result. The package-cache wait messages were not the cause.

The private fixture now executes the already captured native `CARGO` value,
falling back to `cargo`, instead of requiring a separate injected variable.
Native path bytes remain intact. Production APIs and the complete-environment
replacement contract are unchanged. The repair and release notes are unstaged.

## Evidence and qualification

Evidence is `/tmp/ic-tooling-package-fixture-fix.U8nZ5o/`, including admission
lock/index/HEAD, original test stdout/stderr, six corrected source-test logs,
strict Build library Clippy and a newly packaged Build archive.

All six failing cases pass from source on the declared development compiler.
A copy of the original extracted qualification cohort replaces only Build with
the fresh archive; all 281 Build library tests pass there with all features on
actual Rust 1.91. The copied harness preserves the original package selections;
this does not relabel the failed original attempt as passed. Artifact sources and
failed-attempt compilation evidence remain in their original directory.

Rustfmt, the actual `cargo sort --workspace --grouped --check`, docs and diff
checks pass. Real index, HEAD and Cargo.lock match admission. The complete
coordinated archive suite, remaining release gates and native macOS 15 Intel/
Apple Silicon have not been rerun. No cleanup or manual release-state reset ran;
retain interrupted plans, failed receipts and package artifacts when continuing.

## Integrated source and remaining delivery

Local main includes merge `1c52432978bee318acbe0a92a34f6463cba61efd`, combining
maintainer `ead0b753` with PR #277 at `20ca31036e0ea22dbc39c2256f1ace7362b0826d`.
The handoff conflict retained both contributions. The merged pure `group-manifest`
renderer passed seven library and six CLI tests on Rust 1.91. Its native macOS
qualification remains separate; Shared consumer activation needs a published
supplying editor. Merge evidence is `/tmp/ic-tooling-merge277.3LQZJ9/`.

That merge also fixes macOS Bash 3.2 publication fixtures by using a nonempty
complete expected-argument array instead of expanding an empty optional array
under nounset. Linux fixture/shell checks passed; native macOS reruns remain
outstanding. The subsequent Testkit manifest ordering fix passed the exact
workspace-wide preflight sorter and is committed in admission source.

Platform 0.13.0/macros/runtime guard and core Wire 0.11.0 are adopted. Tools'
registry principals require direct Platform 0.13 alignment; typed Testkit limits/
errors require direct Wire 0.11. Optional Wire schemas/endpoints target Platform
0.12 and remain disabled here. Platform no longer has the Host gzip dependency;
current publication needs no Host-first bootstrap. Platform evidence is
`/tmp/ic-tooling-platform013.KYEMoV/` (19 focused MSRV/native tests, Wasm, Clippy
and dependency policies); earlier Wire evidence is `/tmp/ic-tooling-wire-adoption.2VwCQY/`.

[#268 — Delegate Testkit scaled benchmark averages to ic-metrics](https://github.com/dragginzgame/ic-tooling/issues/268)
uses arithmetic-only Metrics 0.22.0 with unchanged typed errors and atomic grouped
counter semantics. [#275 — Keep Wat and direct Wasm encoder selections in one graph identity](https://github.com/dragginzgame/ic-tooling/issues/275)
aligns encoder/parser 0.263 and Wat ~1.263.0 without policy relaxation. Their prior
60-test evidence remains `/tmp/ic-tooling-268-275.1UBPlt/`; latest archive Build
checks above cover this fixture repair. Closure requires delivered qualification.
[#274 — Align Testkit typed Wire calls with the Platform 0.12-compatible Wire release](https://github.com/dragginzgame/ic-tooling/issues/274)
still requires actual downstream Tooling publication.

[#276 — Reduce unrelated Host and Build version churn from integration-only Tooling changes](https://github.com/dragginzgame/ic-tooling/issues/276)
records the internal/public identity edges and release-policy decision. Platform
#118 is reopened; Wire #36 and Shared #328 own prerequisite boundary/release-group
work. Tooling retains coordinated versions; no independent groups or compatibility
facades were introduced. Shared remains 0.27.11 at
`62edfbd43087a718bf0283e9040676a49211976d`; merged PR #327 does not replace
consumer-owned license/provenance/source admission.

Preserve earlier `/tmp/ic-tooling-*` evidence, supplier clones, failed cases,
release plans/locks/receipts, migration/retirement archives and original Host refs.
The older handoff remains in admission Git and
[the 2026-10-30 archive](archive-2026-10-30.md). No sibling edit or artifact cleanup
ran. GitHub remains the sole follow-up tracker.
