# Current handoff — 2026-10-28

## Source and authorized scope

The upstream dependency adoption is local and unstaged over released 0.69.5,
HEAD `c5b6fae87c67` (full identity in evidence). Pending notes remain 0.69.6,
covering the compatible cleanup batch and this adoption. Workspace versions
remain 0.69.5. No staging, commit, push, release, tag or publication ran.

Incoming work included the complete seven-improvement cleanup, its two new
regression files and numbered notes, plus a Cargo.lock update to Platform/macros
0.7.2 and Wire/derive 0.7.19. Preserve all of it. This adoption leaves that
lockfile byte-for-byte unchanged and changes no cleanup implementation. The real
index, sibling checkouts, retained artifacts and Shared Tooling snapshot remain
untouched. No unrelated active Cargo/rustc process was visible at admission.

## Dependency selection and review

The root catalog now requires Platform 0.7.2 and Wire 0.7.19. Child manifests
continue inheriting those requirements; features, dependency roles and Rust 1.91
support are unchanged. Cargo's refreshed official registry index identifies those
as the latest non-yanked releases. Its actual Rust 1.91 targeted update dry run
reports zero further changes; eight unrelated dependencies were left alone.
Direct crates.io API access was unavailable, so selection used Cargo's index and
checksum-bound published registry sources rather than sibling working trees.

Published source identities are Platform/macros
`95b7caf960ec3af045923d85bad4801e9e463782` and Wire/derive
`793134aa4476b5cf7501004d6aab271687c7235f`. Source and normalized manifest diffs
were reviewed. Platform macros are unchanged in source. Platform's Candid change
only documents the existing Wasm nesting limitation. Its new owned DER framing
guard precedes the imported signature parser; that optional capability is not
enabled here, and the cryptographic bodies remain unchanged.

Wire adds caller-storage encoders and Serde crate-path derive support, preserving
existing codec/frame contracts. TypeScript changes are outside the selected
consumer features. Testkit keeps its existing owned request buffers because the
PocketIC transport consumes Vec values. No new adapter, schema or consumer API
migration is introduced solely to use the added upstream surface.

## Validation and evidence

Evidence is under `/tmp/ic-tooling-deps-0696.OJreg2/`, selected by
`/tmp/ic-tooling-deps-0696-current`. It retains admission HEAD/status, the complete
incoming patch/source archive, exact manifest/lock backups, previous handoff,
registry-update preview, published identities/source diffs, selected Cargo
metadata/features, focused logs and final changed-source capture. No host-tool
setup/download or consumer-artifact cleanup ran.

Actual Rust 1.91 Linux tests pass for Candid service/decode integration (11),
Testkit Wire admission/error behavior (9), and optional Native stable-memory
observations (4). The locked/offline Wasm compile passes for Testkit, Native and
the private performance probe, including its Platform CDK macros and Wire codec.
This retains Candid/host exclusion on Wasm and optional dependency boundaries.
Dependency ownership/inheritance and native/Wasm Testkit graph checks pass.

Two live prepared PocketIC 16.1 Wire tests pass: codec refusal/error identity and
structured query/update rejection. They used native Linux loopback access after
rebuilding the selected CLI on Rust 1.91 and performing offline bundle admission;
no server provisioning ran. Four-package all-target/all-feature warning-denied
Clippy passes for Testkit, Dev, Native and the private probe. Formatting, local
documentation links and final diff checks are recorded beside those logs.

These are focused adoption checks, not a full workspace/release gate. Native
macOS 15 Intel and Apple Silicon qualification did not run for this dependency
set; report it separately from Linux evidence. No release or consumer deployment
is claimed. Existing decoder byte/work allowances are not a Wasm stack guarantee;
no advisory exemption or blanket crypto qualification is supplied by this task.

## Preserved cleanup and history

The preceding cleanup remains pending in the same worktree and 0.69.6 notes.
Its evidence is `/tmp/ic-tooling-cleanup-0696.7aCypD/`, including the preserved
90-line handoff and 239 focused Linux test results. Successful observations stay
scoped to one batch context/pass; per-spec exclusions and fresh mutation guards
remain intact. Output inspection/scaffolding, pipe scratch, directory sizing and
borrowed response hex changes remain as qualified. Progress rendering, prepared
snapshot indexing and selective status parsing remain deliberately unchanged.
The admission copy of that handoff is also `previous-handoff.md` in this adoption's
evidence directory; earlier cleanup/adoption records remain linked there.

Shared Tooling remains the exact 144-file snapshot at
`d08cbcafbaff4227e798a890232e2248bf167504`. Supplying [Platform #50](https://github.com/dragginzgame/ic-platform/issues/50)
and historical [Tooling #240](https://github.com/dragginzgame/ic-tooling/issues/240)
retain their existing qualification evidence and obligations; this task does not
assert their resolution. Migration/retirement archives remain under
`/tmp/ic-tooling-migration/`. Original Host refs, stale-lock evidence at
`/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans, failed attempts
and earlier backup/stash evidence remain preserved.
