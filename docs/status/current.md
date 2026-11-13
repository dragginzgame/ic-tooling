# Current handoff — 2026-11-13

## Toko Tooling 0.41.0 — deployment preparing

Both roots select published Toko 0.41.0; the authenticated 115-file snapshot and
all recorded entrypoints remain. `make deps-list` reports all eleven packages
offline. Five unused product catalog aliases and their group dependency aliases
were removed; all nine product group members remain. Tooling adoption changed only
Toko lock blocks; deployment also includes the reviewed CC patch. Standard setup admitted the
new installer-policy receipts while retaining old tool generations.

Both roots now deny unused Cargo dependencies/workspace metadata, missing lint
inheritance and unknown Cargo lints, plus invalid Rustdoc tables. Removed unused
metadata and Testkit's redundant homepage; no dummy imports or lint allowances.
Both-workspace all-target/all-feature Clippy with warnings denied, strict Rustdoc,
portable Testkit/probe Wasm Clippy, snapshot/tool admission, pin checks and all
five release-group graph admissions pass. Evidence: `target/toko041.36z7NR/`.
The Build adapter's real metadata/rendering/receipt checks pass with an empty
alias roster; its setup/product gates were substituted. Native macOS is unrun.

The [publishing guide](../publishing.md) records independent native test suppliers
and optional-upgrade deferrals. Required gates/frozen admission remain; no deployment.
Rust 1.100's `DropGuard` replaces the test-only
produced-value destructor; `Path::trim_prefix` preserves optional manifest prefixes
and `Vec::from_fn` constructs indexed package fixtures.
[#355](https://github.com/dragginzgame/ic-tooling/issues/355) has Linux qualification
for clean/warning, rendered/plain JSON, cold/cached artifacts and compiler denial.
Native plain JSON preserves stderr ownership and compiler guards. Keep #355 open
for native macOS acceptance; focused evidence is `target/rust100-355.AQ2hSA/`.

The authorized frozen roster is Foundation 0.77.1, then Build 0.84.1. Preserve
entry source/locks at `.git/release-state/preserve/deploy041-20261113.jBRdpw/`;
current evidence is `target/deploy041.q8E0gh/`. Freeze current CC 1.11.2 and Toko
0.41.0 without further updates. Normalized Text/Tools/Testkit payload review shows
only development tooling/lints and homepage metadata; deliver this maintenance
through Git and retain their public versions. No new release/upload has run yet.

## Five-group deployment delivered

Frozen `batch-_HKWam5i` completed from reviewed source `918fd7bf`. All nine public
packages are published and non-yanked; downloaded registry archives match Cargo
index checksums and their exact group release commits. Remote main and annotated
tags were verified. The ordinary checkout reconciled cleanly to `bdb84eba`.

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Foundation (five packages) | 0.77.0 | `f366520eb6830d049c0622adec8e68ba7d329ced` | `foundation-v0.77.0` |
| Build | 0.84.0 | `a86d688ad39d69ca68843b64f7e2967e5352d673` | `build-v0.84.0` |
| Text | 0.75.0 | `42956d02b58d0ff5d1b9a68d34729e80cb5a7f39` | `text-v0.75.0` |
| Host Tools | 0.92.0 | `1c32f4c0df1b78aec118bc25cab871543d6f03ce` | `tools-v0.92.0` |
| Testkit | 0.102.0 | `bdb84eba0a382d65d87f6bbda5c6886df34ff2a7` | `testkit-v0.102.0` |

Private qualification packages remain 0.96.0. No sibling source or release was
included. No artifact or historical checkout was retired.

Retain `.git/release-state/frozen/batch-_HKWam5i/`, its managed checkout under
`target/release-frozen/`, and `target/deploy-minors-20261113.4mmynn/`.
`.git/release-state/preserve/deploy-20261113-minors.XUnm1U/` holds full entry
source/locks, untracked files, final logs and registry readback evidence. Both
lockfiles' non-product package blocks remain byte-identical to those entry copies;
release preparation changed only owned product identities. Do not reapply old
preservation locks over these supplier selections.

## Qualification and deployment cost

All five complete 32-stage Linux gates passed: strict Clippy/Rustdoc, native and
isolated feature profiles, Wasm, unsupported-filesystem compilation, installed
PocketIC, extracted archives and installed workspace tests. Every publication
reused its qualified full lint receipt and freshly checked source/archive custody.
Rounded gate totals were 610/467/451/439/426 seconds in supplier order, including
253/246/254/262/247 seconds of tooling fixtures. Later groups reused the four
unchanged invariant fixture stages. These are gate times, not total deployment
latency or controlled speedup measurements; preparation/Git/upload/index phases
lack separate timings. See [the observations](../design/release-validation.md#deployment-cost-observations).

[#353 — Reduce tooling-command qualification cost in deployments and avoid duplicate same-source CI](https://github.com/dragginzgame/ic-tooling/issues/353)
remains open for missing phase evidence and the documented collector-recovery
boundary. No collector-only recovery or whole-gate reuse was added.

First capture `batch-j5L1wRS7` stopped at archive qualification after 22 passing
stages. A Build fixture read Cargo's private build-script stdout path, changed
in 1.100. It created no prepared version, tag or upload. The fix in `918fd7bf`
compares cached JSON exports with a fixture-owned `OUT_DIR` marker; package IDs,
cached exports and custody remain checked. The exact regression and strict Build
Clippy passed before recapture; all subsequent archive gates passed. Retain the
failed journal, checkout and logs without relabelling them.

[CI on the final release](https://github.com/dragginzgame/ic-tooling/actions/runs/43418366917)
is asynchronous. Fresh native macOS 15 Intel and Apple Silicon qualification is
not claimed by the Linux evidence. Keep
[#354 — Adopt Rust 1.100.0 and the common lint policy across maintained workspaces](https://github.com/dragginzgame/ic-tooling/issues/354)
open until its native macOS acceptance is demonstrated; do not infer passes.

## Shipped compiler, tooling and suppliers

Both roots select Rust 1.100.0 and published Toko Tooling 0.40.1; all eleven
members inherit the owning compiler/lint policy. All 115 managed paths and
entrypoints remain. Cargo tools/editor are admitted for Rust 1.100.0, retaining
older generations. Safe roots forbid unsafe code; reviewed FFI and the pool's
Send contract keep their narrow scopes. Directory modules preserve public paths.
Earlier compiler, lint-consumer and tooling-rule proofs remain in
`target/toko040-rust1100.2MC9BA/` and `target/toko0401.XXEGwz/`.

Both roots select Platform 0.32.0 and Types 0.20.0; the private probe selects
Runtime/macros/guard 0.22.0. Wire resolves to 0.21.5 with optional Types integration
disabled. Native PocketIC remains upstream 16.1. Tools/Testkit minors cover exposed
Platform/Principal identities. Focused adoption evidence remains under
`target/platform-adoption.2onG3b/`; the final full gates cover the released graphs.

The Foundation ledger records bounded tool deferrals: keep Toko's reviewed
cargo-sort-derives 0.14.1, yq 4.47.2 and cloc 2.10 catalog, plus the admitted Build
0.73.0 release editor, for this batch. Revisit newer suppliers in tool maintenance.
Cloc 2.12 fixes archive expansion security; the selected LOC flow counts source
files without expanding archives. Do not hand-edit managed tool catalogs.

## Consumer acceptance and open work

[#352 — Compiler/wrapper identity or complete refusal facts for build reuse](https://github.com/dragginzgame/ic-tooling/issues/352)
is shipped in Build 0.84.0. `ConfigurationFile.compiler_overrides` now contains
`CompilerOverride::Build`/`Environment` facts, preserving empty, forced, relative
and shadowed declarations without values or a precedence/executable resolver.
Consumers must migrate vectors/matches and independently bind compiler identity
or refuse reuse. Canic owns adoption in
[Canic #247 — Bind effective Cargo-configured wrappers in build reuse evidence](https://github.com/dragginzgame/canic/issues/247).

[#263 — Cached snapshots and timer state](https://github.com/dragginzgame/ic-tooling/issues/263)
remains open for Canic's exact two-case acceptance of the correction first shipped
in Testkit 0.101.0. Native snapshots preserve timer/hook, globals, heap/stable,
Wasm, certified data and chunks through upstream PocketIC and Platform codecs.
Two slots per target and transfer cycles are required; stop/drain/start, sender,
funding and partial-failure recovery remain caller-owned. No client fork, timer
reset, second transport or time rewind. Installed regressions pass on Linux.
[Timers #87 — Snapshot regression qualification](https://github.com/dragginzgame/ic-timers/issues/87)
tracks separate coverage; no sibling edits or qualification are included here.

Next independent fixes remain outside this deployment:

1. Preserve exact paths in [Build tool probes](../../crates/ic-build/src/tools/probe/mod.rs)
   and check raw directory names before archive normalization.
2. Propagate filesystem errors from [Cargo configuration selection](../../crates/ic-build/src/cargo/environment/mod.rs).
3. Reject malformed successful [process observations](../../crates/ic-host-process/src/testing/mod.rs).

Canic's separate Build migration is
[Canic #762 — Explicit maintenance and pruning](https://github.com/dragginzgame/canic/issues/762).

## Retained historical evidence and contracts

The [pre-deployment handoff](https://github.com/dragginzgame/ic-tooling/blob/918fd7bffc93b51fabb573a18235085bff040978/docs/status/current.md)
records prior delivered Foundation 0.76.5 (`batch-eTIfLgJR`) and Testkit 0.101.0
(`batch-Aps5IIGp`), failed attempts, preservation refs and exact recovery paths.
Retain their original journals/adapters; never relabel old receipts as current.
Existing `preserve/*` refs and `.git/release-state/preserve/` remain intact.
The retired client archive is `/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`;
original source is `0c17900090f0c2d2ebf3182598b49e651223845e`.

Keep two-pass generated-root discovery, Darwin `File::sync_all` durability and
discovery error downcast identity. Retain the snapshot cleanup callback's explicit
future type; attempted `AsyncFnMut` alternatives failed its Send lifetime contract.
