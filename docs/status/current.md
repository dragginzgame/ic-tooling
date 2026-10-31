# Current handoff — 2026-10-31

## Source and accepted scope

HEAD remains `e2f5e610831175c020d486b1a84a37bffb06a10c` (maintainer commit
`0.73.0`). Root/package versions still read 0.72.1; pending notes select 0.73.0.
The latest task adopts Platform and reviews more issues, preserving all pending
#268/#275 fixes. Only three Platform lock records change from this step's dirty
admission. Real index and HEAD are unchanged; no staging, commit, own version
bump, release, tag or publication ran. Earlier startup evidence is preserved at
admission Git and in the [2026-10-30 archive](archive-2026-10-30.md).

Platform is now published 0.13.0 with matching macros/runtime guard. It removes
the gzip facade and Host Artifacts dependency; selected implementation sources
otherwise match 0.12. Tooling already uses codecs directly, so no caller moved.
The publication guide now makes normal current publication distinct from the
retained legacy Host-first bootstrap and partial-upload procedures.

Tools' public registry principals require direct Platform 0.13 alignment. Wire
remains on published 0.11.0 for the core codec. Its optional schemas/endpoints
remain disabled and target Platform 0.12; they do not supply Platform 0.13 schema
identities. No extra Platform generation or schema compatibility shim is added.

## Latest evidence and limits

Evidence is `/tmp/ic-tooling-platform013.KYEMoV/`, retaining dirty admission,
lock/index/HEAD, registry responses, metadata/edge inventory, focused logs and
prepared server admission/hash. Platform 0.13 names supplier source
`bb0e2ae0bc851f48fcbcd071dbacf7236bbc2b1b`.

Rust 1.91 passes 19 tests: three registry, nine Wire, four native memory, one
private protocol and two real PocketIC codec/refusal/rejection scenarios using
the prepared 16.1.0 server. Selected Testkit/Native/probe Wasm compilation passes.
Affected-package strict all-target/all-feature Clippy, complete Testkit and
Host/Tools dependency policies, manifest sorting, pins, docs and diff checks pass.
Only three Platform lock records differ from admission. Native macOS 15 Intel/
Apple Silicon, full workspace/release gates and hosted dispatch were not run.
Linux execution and Wasm compilation do not establish native macOS qualification.

## Preserved fixes and qualification

#268 delegates scaled benchmark averages to published Metrics 0.22.0 with default
features disabled. Error variants/messages, exact/floor arithmetic, zero-run
precedence, atomic grouped updates and reporting retain the Testkit contract.
Metrics metadata/recording remain disabled and its normal dependencies are Serde.
Host/Build still exclude Metrics; default native and Wasm Testkit exclude Wire.

#275 aligns encoder/parser 0.263 and Wat 1.263, retaining incoming Wat and unrelated
lock edits while removing duplicate 0.262 records. Wat stays within the 1.263
patch line to prevent routine drift into another encoder family. The complete
Testkit graph policy now passes without relaxing identity/feature requirements.

Their evidence is `/tmp/ic-tooling-268-275.1UBPlt/`: 60 focused Rust 1.91 tests,
including 23 benchmark, 31 artifact Wasm, one encoder consumer, three held-fixture
and two real held-reply scenarios. It also records selected Wasm compilation,
strict affected-package Clippy, graph policies and script/format/docs checks.
That prior runtime evidence stays bound to its original inputs; latest Platform
qualification above covers the newly affected profiles. Metrics supplier source
is `402168f1b08dd09502cc0a5fd713d41344cf461f`.

## Issue and delivery boundaries

[#268 — Delegate Testkit scaled benchmark averages to ic-metrics](https://github.com/dragginzgame/ic-tooling/issues/268)
and [#275 — Keep Wat and direct Wasm encoder selections in one graph identity](https://github.com/dragginzgame/ic-tooling/issues/275)
have local fixes; closure needs delivered source and applicable qualification.
[#274 — Align Testkit typed Wire calls with the Platform 0.12-compatible Wire release](https://github.com/dragginzgame/ic-tooling/issues/274)
has local adoption, but downstream Tooling publication remains pending. Public
integration guidance now reflects Platform 0.13/core Wire 0.11 without claiming
that Wire's optional schema integration targets 0.13.

[#276 — Reduce unrelated Host and Build version churn from integration-only Tooling changes](https://github.com/dragginzgame/ic-tooling/issues/276)
now records the actual internal requirements, public error/digest identities and
release/publication constraints. Platform #118 was administratively closed without
its stable-identity implementation; Wire #36 remains an upstream boundary decision.
Keep the agreed coordinated workspace policy. No independent-version split or
wide-range dependency workaround was made. Metadata evidence is in this task's
`cohort-edges.json`; the issue owns the decision and acceptance, not this handoff.

Shared remains 0.27.11 / `62edfbd43087a718bf0283e9040676a49211976d`; PR #327 merged
as `81ee6b59a31f6789c85caa4f9343e57ae5824c3e`. Consumer provenance/licensing/source
admission keeps the lower publication runner. Previous adoption evidence is under
`/tmp/ic-tooling-wire-adoption.2VwCQY/` and `/tmp/ic-tooling-platform-adoption.6oO1jt/`;
cleanup evidence is `/tmp/ic-tooling-cleanup-0722.N0q9MJ/`. Preserve failed cases,
package artifacts, release plans/locks/receipts, supplier clones, migration/
retirement archives and original Host refs. No tool download, sibling checkout
edit or artifact cleanup ran. GitHub remains the sole follow-up tracker.
