# Current handoff — 2026-10-29

## Scope and source

All ten accepted complexity reductions are implemented locally and unstaged over
admission HEAD `ce660e30e20ec407baf446fc7c4f2b7587779761` (Release 0.69.9).
The working tree and index were clean at admission; the previous handoff's
unstaged snapshot fix is already in this released source. Package versions remain
0.69.9; compatible pending notes are 0.69.10. No staging, commit, push, tag,
release, publication or tool download ran. Cargo.lock, manifests, Shared snapshot,
index and sibling working checkouts remain unchanged. No unrelated Cargo/rustc
process was visible at admission. Existing artifacts are preserved.

## Completed changes

Testkit owns the new additive `try_pocket_ic_call` adapter: exact structured
rejections and recognized transport panics are returned without retry, while
unrelated panics resume with their original payload. Recipes, the maintained
example, integration fixtures and Wire calls project that result. Existing
internal capture-all boundaries retain their separate panic policy.

Snapshot rollback traverses known temporary and completed captures once, keeping
cleanup order, exact IDs/senders and all failed deletion causes. Bounded transfer
reads/writes borrow one source/target identity context. Complete-state admission,
funding, timer/hook restoration and caller-owned lifecycle remain unchanged.

Benchmark phase summaries have typed fields instead of string-keyed lookups.
Required fields need no access-time assertions, and optional phases retain nulls.
Successful JSON field names, alphabetical phase order and statistics remain intact.
Artifact batch execution retains partial causes internally and attaches final
phase/total timings once when constructing public failures.

Build observation keeps its callback, configuration and last-event time in one
optional state, independently of failure evidence. Cache dispatch narrows to
isolated/shared after handling scheduled shared maintenance. Lock ordering,
source revalidation, heartbeat behavior and retention boundaries remain intact.

Package qualification prepares admitted paths, selection and limits before
command effects. Its execution retains the same ordered before/after custody
checks. Command output retention uses the exact paths later carried by errors,
keeping both primary execution failures and all output-write failures. Process
exchange retains capture/communication/drain meaning through admission and
completion, preserving deadlines, pipe handling and cleanup distinctions.

Existing public APIs and dependency/feature boundaries remain intact; the call
adapter is additive. No cache identity, recovery format, tool selection or
consumer migration changes are introduced by this cleanup.

## Validation and limits

Evidence: `/tmp/ic-tooling-cleanup-06910.ZwEZDQ/`, selected by
`/tmp/ic-tooling-cleanup-06910-current`. Admission source/lock, previous handoff,
commands, attempted and final logs, benchmark JSON and final patch/source capture
remain there. Initial compile/lint failures are retained beside their corrections.

Actual Rust 1.91 Linux focused tests pass: 248 cases covering process execution,
Wasm cache acquisition/observation, artifact batch failures, package qualification,
transport/Wire calls, snapshot rollback/restoration, baseline pools and driver
reports. This includes actual Cargo archive qualification, the live PocketIC
16.1.0 timer/hook regression, 100 consecutive pool restores and the explicitly
selected dedicated-server recovery test. Existing package tests retain primary
command evidence even when both output-log writes fail.

The actual driver also completes two fresh and two pooled tasks with one worker,
capacity one and zero seeded state bytes. This is functional smoke evidence,
not a performance comparison. Warning-denied all-target/all-feature Clippy passes
for Build, Host Process, Testkit and Testkit Dev on prepared Rust 1.99.1.
Formatting, local documentation links and diff checks pass. No full workspace or
release gate ran. Native macOS 15 Intel and Apple Silicon qualification did not
run; Linux evidence is not native macOS evidence.

## Preserved obligations and evidence

This cleanup does not establish publication, Canic adoption or consumer sequence
qualification for [#263](https://github.com/dragginzgame/ic-tooling/issues/263).
The previous handoff records the lifecycle changes needed in both Root baseline
owners and the retained four-case sequence. Its source/evidence remains at
`/tmp/ic-tooling-263.WQ4ch3/`; Canic diagnostic work remains untouched. No external
issue-state inspection or mutation ran during this cleanup.

Earlier evidence remains under `/tmp/ic-tooling-cleanup-0698.LehyNb/`,
`/tmp/ic-tooling-cleanup-0697.OHn8IL/`, `/tmp/ic-tooling-262.zTQbiB/`,
`/tmp/ic-tooling-cleanup-0696.7aCypD/` and `/tmp/ic-tooling-deps-0696.OJreg2/`.
Supplying-owner and optimizer qualification obligations retain their recorded
scope. Shared Tooling remains revision
`d08cbcafbaff4227e798a890232e2248bf167504` (144 files).
Migration/retirement backups remain under `/tmp/ic-tooling-migration/`. Original
Host archive refs, `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans,
failed attempts and earlier backup/stash records remain preserved through prior
handoffs. No cleanup ran outside existing owned regression-fixture behavior.
