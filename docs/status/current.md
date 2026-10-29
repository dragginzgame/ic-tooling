# Current handoff — 2026-10-29

## Accepted scope and source

The maintainer requested work on open issues #263, #266 and #267. Snapshot and
Platform corrections were already delivered; their current evidence was verified
and both issues closed. The Cargo grouping correction is complete locally,
unstaged, with its own checks and notes. It remains open for delivery and native
qualification. No commit, push, version transaction, release, publication, tool
installation, sibling source edit or artifact cleanup ran in this issue batch.

The ordinary checkout remains on `ecc8d61d` (Release 0.70.0), with its admitted
35 dirty files preserved. They contain Wire 0.7.22 adoption and all ten previous
complexity reductions. Cargo.toml, Cargo.lock, the real index and HEAD retain
their admission contents. Own versions remain 0.70.0; pending notes stay 0.70.1
because the combined local batch preserves published APIs and feature boundaries.
Shared remains 0.27.1 at `d08cbcafbaff4227e798a890232e2248bf167504`.

PR #265 was separately merged normally as
`01bbf850f1f14c3678485da4601a80cd75dee88f`; it is not pulled into this dirty
checkout. Its reconciliation head `55252e66` now has all four workflows passing.
Native Linux x86-64 and macOS 15 Intel/Apple Silicon, MSRV and lint/security pass
in CI run 40910712564. This does not qualify the later local grouping changes.
Merge evidence remains at `/tmp/ic-tooling-merge-265.iHrIEC/`.

## Bounded Cargo observations

Build tries each input package only against the latest group, starting another
group on a feature conflict. Including isolated trees, nonempty selections use
at most `2n−1` observer calls. Earlier groups are not searched; packing can be
conservative. Every proposed group is still observed through actual Cargo and
must preserve all isolated normal/build trees. Feature/target/locked/offline
flags, the caller observer and its typed execution errors remain unchanged.
Malformed forests and missing/additional selected roots return CargoTreeError.
No cross-call cache, dependency, duplicate consumer owner or metadata substitute
was added. The bound covers observer calls, not caller-selected output/time.

Evidence: `/tmp/ic-tooling-issues-263-267.9Ckcfb/`. Admission source/patch/status,
failed and corrected Clippy logs, actual Rust 1.91 checks and the comparative
probe source/results are retained. Ten focused grouping tests pass, covering
compatible/incompatible work bounds, conservative order, actual Cargo feature
unification, graph selections, malformed forests and typed process failures.
Strict all-target/all-feature Build Clippy passes on prepared Rust 1.99.1;
strict Build Rustdoc passes on actual Rust 1.91. Formatting and doc links pass.

The comparative probe uses the frozen Canic adoption workspace's six selected
canister packages with locked/offline Wasm normal/build trees on Rust 1.91.
Previous and bounded algorithms produce identical groups: 15 versus 11 Cargo
calls, 75,407,408 versus 51,465,988 stdout bytes. Single sequential local samples
take 5,016 versus 3,586 ms; this is not a stable benchmark or whole-build claim.
The frozen root manifest/lock remain unchanged. This is a graph probe, not a
Canic build/release or full adapter qualification. New grouping source has no
native macOS execution or hosted CI yet; no full workspace/release gate ran.

## Delivered corrections and preserved evidence

[#263 — Snapshot timer coherence](https://github.com/dragginzgame/ic-tooling/issues/263)
is closed after checking the delivered metadata-upload snapshot implementation,
installed regression coverage and the retained Canic four-case sequence. All four
labels pass on frozen Testkit 0.69.13; the governed suite log confirms success in
`/home/adam/projects/canic/target/adoption-qualification-20261029/`. No later
moving-root or production backup qualification is inferred. Original failing
Canic evidence under `target/session-release-0113/` remains untouched.

[#267 — Native Platform convergence](https://github.com/dragginzgame/ic-tooling/issues/267)
is closed against published Tooling 0.70.0 and the existing Timers one-Platform
consumer evidence. Platform adoption evidence remains at
`/tmp/ic-tooling-platform-081.DQsDFy/`. Its owned Principal/ledger migration still
requires consumer alignment; Testkit preserves its external Principal API.

Earlier cleanup evidence remains at `/tmp/ic-tooling-cleanup-0701.bAD18Y/`:
110 focused Rust 1.91 cases and four installed Linux/PocketIC 16.1.0 cases,
selected Wasm, graphs, Clippy and docs passed for that source. Wire adoption
evidence remains at `/tmp/ic-tooling-wire-0722.kvAbqM/`. No native macOS proof
is attributed to these uncommitted changes. Admission handoffs retain earlier
preparation/package failures, migration/retirement backups, original Host archive
refs, stale release-lock backups, release plans and stash obligations; preserve
them. No consumer artifacts or migration evidence were removed.
