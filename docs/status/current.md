# Current handoff — 2026-10-29

## PR #265 integration

[PR #265 — Own raw native observations and workload mechanics in Host](https://github.com/dragginzgame/ic-tooling/pull/265)
is being reconciled and merged under explicit maintainer authorization.
Integration uses `/tmp/ic-tooling-merge-265.iHrIEC/integration`, starting from
PR head efb49570 and released main ecc8d61d (0.70.0). Normal merge ancestry
preserves the original feature commits and main's released dependency changes.

Own package versions inherit 0.70.0 from main; no independent version bump,
tag, release or publication is performed. Compatible pending notes move from
0.69.15 to 0.70.1. Released root/minor-line histories are preserved exactly.
No native Process implementation changes are needed for this reconciliation.
The ordinary checkout's Wire adoption and ten-item cleanup remain unstaged and
untouched. Its admission patch/source/index/head are preserved in merge evidence;
those changes are not silently included in this PR.

## Ownership and consumer boundary

[#264 — Own raw process observations and Profiler workload mechanics in Host](https://github.com/dragginzgame/ic-tooling/issues/264)
adds raw procfs, libproc/Mach, clock and thread observations to Host Process.
Source provenance identifies Profiler 70907ff. Profiler retains process selection,
creation-identity admission, serialized DTOs, cadence, aggregation, retention and
diagnostics; Platform retains native IC adapters and matching build inputs.
Existing bounded RSS-only sampling stays separate. No IC dependency, automatic
resource allowance, cryptographic change or second production owner is added.

OwnedChild keeps non-reaping observation, explicit relinquishment for a caller's
keep-running decision, caller-selected graceful group termination followed by
synchronous reaping, and an argv/stdin-preserving launch gate. Existing bounded
cleanup and successful handoff stay intact. Readiness, later reaping after
relinquishment, retries and descendant containment remain caller-owned.

Consumers use one immutable Host source family until registry supply is released;
no local Cargo patch is a distribution mechanism. Paired adoption lives in
[Profiler #7 — Host process adoption](https://github.com/dragginzgame/ic-profiler/pull/7)
and [Platform #95 — Host process supplying inputs](https://github.com/dragginzgame/ic-platform/pull/95).
Their c20dbd1 acceptance records do not automatically qualify later supplier heads.

## Checks and merge evidence

Merge evidence is `/tmp/ic-tooling-merge-265.iHrIEC/`, including current ordinary
source preservation, PR/main records, conflict resolutions and focused logs.
GitHub verified the PR author's current admin access. No review submissions or
unresolved review threads existed at inspection. Main has no enforced branch
protection/ruleset requirements; no protection or review setting is changed.

All 14 checks passed at efb49570. Native macOS 15 Intel and Apple Silicon both
passed native CI and archive verification in CI run 40886165029; Linux native,
MSRV, security, browser, optimizer and simulator jobs also passed. These are
hosted observations for that source, separate from local Linux evidence.
The later main reconciliation retains identical Process source. Focused strict
Clippy, 31 Rust 1.91 child/observation tests, formatting and doc links pass.
Updated hosted CI runs asynchronously; no full local workspace/release gate or
new native macOS execution is inferred from focused Linux checks.

## Preserved obligations

[Testkit snapshot consumer acceptance (#263)](https://github.com/dragginzgame/ic-tooling/issues/263)
retains its separate consumer scope. No Canic adoption or retained four-case
sequence qualification is established by this PR merge.

The ordinary checkout's latest cleanup handoff and source are retained in this
merge evidence, along with links to earlier Wire/Platform/cleanup records.
Earlier migration/retirement backups, original Host archive refs, failed package
attempts, release plans and stale-lock/stash evidence retain their recorded
scope. No artifact cleanup, sibling edit or unrelated issue change is performed.
