# Current handoff — 2026-10-31

## Source and accepted scope

This integration combines maintainer commit `ead0b753494e689b5d2982057c936848c1050fc4`
(`fixing`) with merged PR #277 at `20ca31036e0ea22dbc39c2256f1ace7362b0826d`.
The user authorized conflict resolution and merging. The only textual conflict
was this handoff; both contributions are retained. Root/package versions remain
0.72.1 with pending 0.73.0 notes. No release, tag, publication, history rewrite,
tool download, sibling edit or artifact cleanup ran. Cargo.lock matches merge
admission. Earlier evidence remains at its existing owners and in the retained
[2026-10-30 archive](archive-2026-10-30.md).

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
are included in the maintainer source; closure needs delivered source and applicable qualification.
[#274 — Align Testkit typed Wire calls with the Platform 0.12-compatible Wire release](https://github.com/dragginzgame/ic-tooling/issues/274)
has local adoption, but downstream Tooling publication remains pending. Public
integration guidance now reflects Platform 0.13/core Wire 0.11 without claiming
that Wire's optional schema integration targets 0.13.

[#276 — Reduce unrelated Host and Build version churn from integration-only Tooling changes](https://github.com/dragginzgame/ic-tooling/issues/276)
now records the actual internal requirements, public error/digest identities and
release/publication constraints. Platform #118 was reopened for the unresolved
stable-identity decision; Wire #36 remains conditional on that boundary.
Keep the agreed coordinated workspace policy. No independent-version split or
wide-range dependency workaround was made. Metadata evidence is in this task's
`cohort-edges.json`; the issue owns the decision and acceptance, not this handoff.

Shared remains 0.27.11 / `62edfbd43087a718bf0283e9040676a49211976d`; PR #327 merged
as `81ee6b59a31f6789c85caa4f9343e57ae5824c3e`. Consumer provenance/licensing/source
admission keeps the lower publication runner. Previous adoption evidence is under
`/tmp/ic-tooling-wire-adoption.2VwCQY/` and `/tmp/ic-tooling-platform-adoption.6oO1jt/`;
cleanup evidence is `/tmp/ic-tooling-cleanup-0722.N0q9MJ/`. Preserve failed cases,
package artifacts, release plans/locks/receipts, supplier clones, migration/
retirement archives and original Host refs. GitHub remains the sole follow-up tracker.

## Release-group editor integration

PR #277 already merged remotely as `20ca31036e0ea22dbc39c2256f1ace7362b0826d`.
The local pull merges that source with maintainer commit `ead0b753` (`fixing`),
which contains the pending adoption and #268/#275 fixes above. The sole textual
conflict was this handoff; both contributions are preserved. The merge introduces
bounded `group-manifest` rendering without changing release groups or versions.
Shared retains group/graph admission, transactional installation and resume.
Its candidate preparation/rollback/resume fixtures used the unpublished compiled
editor; consumer activation still requires a published supplying tool.

Merge evidence is `/tmp/ic-tooling-merge277.3LQZJ9/`, retaining the conflicted
source/index, native CI logs and focused checks. On Rust 1.91 seven renderer and
six CLI tests pass; selected Build strict Clippy, Testkit graphs and doc links
pass. Linux CI failed the duplicate-encoder policy already repaired in `ead0b753`.
Both macOS jobs failed publication fixture expansion of an empty array under Bash
3.2 nounset. The fixture now constructs a nonempty complete argument array for
both modes; its actual substitute-effect suite, ShellCheck and syntax checks pass
on Linux. Native macOS reruns and full hosted qualification remain outstanding;
the old failed runs are not passes. PR #277 is already merged remotely; there
were no open Tooling PRs at this integration review.
