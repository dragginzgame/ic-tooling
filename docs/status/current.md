# Current handoff — 2026-10-29

## Source and scope

Nine of the ten accepted simplifications are implemented locally and unstaged
over `b0c94001` (Release 0.69.12). The incoming working/staged tree was clean;
no active Cargo/rustc process was visible. The prior handoff describes the
previous batch, already committed in this base. Package versions remain 0.69.12;
compatible pending notes are 0.69.13. Cargo.lock is byte-identical to admission.

No staging, commit, push, tag, release, publication, dependency update, tool
installation or sibling edit ran. Manifests and Shared snapshot are unchanged.
Shared Tooling remains `d08cbcafbaff4227e798a890232e2248bf167504` (144 files).

## Completed changes and retained decision

Cargo profile, lock/network and feature flags append to one argument buffer,
with the same ordering and feature encoding. Input capture has one private path
for admitted specifications: revalidation combines and sorts the roots without
cloning/normalizing exclusions again. Public entrypoints still admit inputs, and
source scans, old-root custody, absence authority and digest bytes remain intact.
Exclusion names reuse the existing single-normal-component predicate with their
own error context.

Diagnostic artifact discovery reuses Host's reader traversal, retaining its
8 KiB scratch, maximum representable aggregate count and Cargo's per-message
allowance. Interrupted/native IO, impossible counts, protocol/completion and
artifact selection remain independently checked.

Wasm paths carry the filename normalized during target admission, keeping raw
Cargo names for message matching and retaining collision admission. Cache stamp
validation reuses its prepared header; independent artifact hashing and complete
stamp-byte comparison remain. Extracted graph admission uses the existing
package directory-name renderer.

Benchmark server selection anchors the exact caller path and leaves the single
canonicalization to Host. An initial metadata observation preserves native path
errors; executable/type checks and the no-PATH contract remain. Port parsing
borrows trimmed text and allocates only invalid-value diagnostics.

Proposal 5, a typed workspace JSON envelope, is intentionally retained as-is.
Source review found that the current generic decoder accepts duplicate root keys
using the last value. A derived envelope would reject these; preserving the
behavior would require extra custom parsing code. No new decoder or issue was
introduced for a refactor that would increase complexity. Public APIs, feature/
dependency boundaries, report/cache formats and lifecycle contracts are unchanged.
No consumer migration or new tool selection is required.

## Validation and limits

Evidence: `/tmp/ic-tooling-cleanup-06913.uEdXff/`, selected by
`/tmp/ic-tooling-cleanup-06913-current`. Admission source/lock/handoff, commands,
attempted and final logs, final patch/source and identity records are retained.

Actual Rust 1.91 Linux focused tests pass: 67 cases across Cargo selection,
metadata and native invocation, standalone input evidence, diagnostic stream
handling, extracted package identity/manifests, exact cache stamps and artifact
paths, worker preparation/report handling and startup port parsing. Added
coverage checks a hyphenated cdylib example's projected filename, absolute and
relative explicit server paths, native missing-path errors and executable/type
refusals. Existing cases retain exact stamps, source drift/absence policy,
interrupted/impossible readers, output custody and command failure evidence.

Warning-denied all-target/all-feature Clippy passes for the three changed
packages on prepared Rust 1.99.1. After correcting the new server-path fixture,
its library/test lint and all 23 driver tests pass again. Initial line-limit
lint and fixture setup failures remain; final logs identify corrected evidence.
Formatting, local documentation links and diff checks pass. No full workspace
or release gate ran. This batch needed no live PocketIC execution. Native macOS
15 Intel and Apple Silicon qualification did not run; Linux is separate evidence.

## Preserved obligations and evidence

[#263](https://github.com/dragginzgame/ic-tooling/issues/263) retains its consumer
acceptance scope at `/tmp/ic-tooling-263.WQ4ch3/`. This batch establishes neither
Canic adoption nor its four-case sequence qualification and changes no issue state.
The earlier race-fixture evidence and failed package attempt remain at
`/tmp/ic-tooling-preparation-race.IMHoSv/` and
`target/package-qualification-1823269-0/`.

Previous batch evidence remains at `/tmp/ic-tooling-cleanup-06912.cqgTkd/`,
`/tmp/ic-tooling-cleanup-06911.Pznlfz/` and
`/tmp/ic-tooling-cleanup-06910.ZwEZDQ/`. Older evidence and supplying-owner
obligations retain the scope linked through those handoffs. Migration/retirement
backups remain under `/tmp/ic-tooling-migration/`; original Host archive refs,
`/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans, failed attempts
and earlier stash records remain preserved. No artifact cleanup ran beyond
existing owned regression-fixture behavior.
