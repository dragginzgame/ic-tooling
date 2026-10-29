# Current handoff — 2026-10-29

## Current integration state

The maintainer requested resolution of the ordinary checkout's pull conflicts.
Local main is `f53d4080` (Release 0.70.1), following `01e1b12c`; tag v0.70.1
exists. The existing merge brings in PR #265 through
`01bbf850f1f14c3678485da4601a80cd75dee88f`. Only CHANGELOG.md,
docs/changelog/0.70.md and this handoff conflicted. Rust and other documentation
changes merged automatically. This repair changes only those three files;
existing staged source changes and local release commits are preserved.

The complete local 0.70.1 root ledger and detailed-note bodies remain unchanged.
Incoming Process additions are recorded under compatible pending 0.70.2, rather
than retroactively adding them to finalized 0.70.1. Workspace versions and the
lockfile remain at their admitted 0.70.1 contents. This selects notes only: no
version change, new commit, push, tag, release or publication is performed.
The merge commit remains for the maintainer to complete after reviewing.

Conflict/index/stage backups and original source are retained at
`/tmp/ic-tooling-conflicts-0701.R6FwVw/`. Both sides, the common-base documents
and the marker-bearing files are saved. Earlier ordinary-checkout changes are
now in the maintainer's local commits, including Wire 0.7.22 adoption, the ten
complexity reductions and bounded Cargo grouping. Shared remains 0.27.1 at
`d08cbcafbaff4227e798a890232e2248bf167504`.

## Incoming Process ownership

[#264 — Raw process observations and Profiler workload mechanics](https://github.com/dragginzgame/ic-tooling/issues/264)
is supplied by [PR #265](https://github.com/dragginzgame/ic-tooling/pull/265).
Host Process owns raw procfs, Darwin libproc/Mach, clock and thread facts imported
from Profiler 70907ff. Profiler retains selection, creation-identity admission,
DTOs, cadence, aggregation, retention and diagnostics. Platform retains native
IC adapters and supplying build inputs. Bounded RSS-only sampling stays separate.

OwnedChild supports non-reaping observation, explicit relinquishment for callers
that keep a workload running, caller-selected graceful group termination followed
by synchronous reaping, and an argv/stdin-preserving launch gate. Readiness,
later reaping after relinquishment, retries and descendant containment remain
caller-owned. No IC dependency or second production owner is introduced.

Consumers use one immutable Host source family until registry supply is released.
Paired adoption is recorded in [Profiler #7](https://github.com/dragginzgame/ic-profiler/pull/7)
and [Platform #95](https://github.com/dragginzgame/ic-platform/pull/95).
Their c20dbd1 acceptance records do not automatically qualify later supplier heads.

## Validation and acceptance boundaries

This conflict repair is documentation-only. It checks removal of conflict
markers, exact preservation of finalized notes and unrelated source/index entries,
documentation links and diff consistency; no new Rust suite or broad gate runs.

PR reconciliation source `55252e66` passed all four workflows. CI run
40910712564 passed native Linux x86-64, native macOS 15 Intel and Apple Silicon,
MSRV and lint/security. Prior focused Process evidence includes 31 actual Rust
1.91 child/observation tests and strict Clippy. Merge evidence remains at
`/tmp/ic-tooling-merge-265.iHrIEC/`. These hosted results qualify that supplier
source, not the combined ordinary-checkout merge with later local 0.70.1 changes.

[#266 — Bound repeated Cargo resolution](https://github.com/dragginzgame/ic-tooling/issues/266)
uses at most 2n−1 observer calls, trying only the latest group while preserving
every isolated normal/build tree. Packing may be conservative; malformed trees
and mismatched roots have typed failures. Its prior local evidence is
`/tmp/ic-tooling-issues-263-267.9Ckcfb/`: ten Rust 1.91 grouping tests, strict
Clippy/Rustdoc, and the six-package frozen Canic graph probe. The probe preserved
groups with 15 versus 11 calls; single local timings are not a stable benchmark.
No new macOS or hosted qualification of that grouping/combined source is claimed.

## Preserved delivered fixes and evidence

[#263 — Snapshot timer coherence](https://github.com/dragginzgame/ic-tooling/issues/263)
and [#267 — Native Platform convergence](https://github.com/dragginzgame/ic-tooling/issues/267)
were closed with delivered supplier and consumer evidence. Canic's retained
four-case sequence passes on frozen Testkit 0.69.13 in
`/home/adam/projects/canic/target/adoption-qualification-20261029/`; original
failures remain under `target/session-release-0113/`. This does not qualify
later moving-root source or production backup/restore. Platform 0.70.0 adoption
still requires matching owned Principal/ledger types in consumers; Testkit's
external Principal API remains unchanged.

Prior Platform, Wire and cleanup records remain at
`/tmp/ic-tooling-platform-081.DQsDFy/`, `/tmp/ic-tooling-wire-0722.kvAbqM/` and
`/tmp/ic-tooling-cleanup-0701.bAD18Y/`. Earlier admitted handoffs retain package/
preparation failures, migration/retirement backups, original Host archive refs,
stale release-lock backups, release plans and stash obligations. No artifact
cleanup or sibling source change is performed.
