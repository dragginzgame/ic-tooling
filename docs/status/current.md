# Current handoff — 2026-11-13

## Foundation 0.76.5 delivered

Foundation 0.76.5's ledger owns its bounded-read, permission, lock and HTTP APIs.
Consumers retain byte/path limits, permissions, trust and recovery policy.

Release commit `963df312794cf90e4756d48420a2462e963e6c69` and annotated tag
`foundation-v0.76.5` match GitHub main. All five Foundation packages are present
and non-yanked on crates.io. Frozen `batch-eTIfLgJR` completed and was reconciled
into ordinary main. Source review commits are `715af316` and `bb4df402`.
The complete 32-stage Linux gate passed, including strict Clippy/docs and native,
Wasm, installed PocketIC and extracted-package checks. Fresh native macOS
qualification remains asynchronous and is not claimed here.

Retain `.git/release-state/frozen/batch-eTIfLgJR/`, its managed checkout under
`target/release-frozen/`, and `target/deploy-foundation765/` logs and registry
observations. Gate evidence is `target/release-validation/foundation-gate.XaK96U/`
within that checkout. No sibling release or cleanup was performed.

Frozen `batch-lc826t2Z` stopped before preparation at a stale Make fixture's
isolated-feature expectation. Clippy and formatting passed; no release/tag/upload
was created. Its inputs and evidence remain retained. The fixture's corrected
two-command expectation passed focused and full-gate qualification in the new batch.

Build 0.83.0, Text 0.74.5, Tools 0.91.0 and Testkit 0.101.0 remain published.
Pending compiler-floor minors are Foundation 0.77.0, Build 0.84.0, Text 0.75.0,
Tools 0.92.0 and Testkit 0.102.0. Build also includes the refusal API change below.
Private qualification packages remain 0.96.0. Do not release unchanged groups.

## Pending Build 0.84.0 — compiler refusal facts

[#352 — Compiler/wrapper identity or complete refusal facts for build reuse](https://github.com/dragginzgame/ic-tooling/issues/352)
is closed for the local typed `Build`/`Environment` refusal facts. Empty, forced,
relative and shadowed declarations remain; values stay private, invalid tables
refuse and no precedence/executable resolver is added. Build 0.84.0 requires
vector/match migration; `is_empty()` remains valid. Canic owns adoption in
[Canic #247 — Bind effective Cargo-configured wrappers in build reuse evidence](https://github.com/dragginzgame/canic/issues/247);
No sibling edits or delivery. All 14 configuration tests, Clippy/Rustdoc and
links pass on Linux (`target/issue352/`); no full gate or native macOS proof.

## Pending Rust 1.100.0 and Toko Tooling 0.40.1

[#354 — Adopt Rust 1.100.0 and the common lint policy across maintained workspaces](https://github.com/dragginzgame/ic-tooling/issues/354)
is implemented locally for Linux. Both roots select 1.100.0 and published Toko
0.40.1; all eleven members inherit their owning lint table and compiler minimum.
All 115 managed paths/entrypoints remain. Cargo tools/editor are admitted for
1.100.0; older generations stay retained. Common/pedantic lints, safe-root forbids
and scoped FFI/Send boundaries are qualified in `target/toko040-rust1100.2MC9BA/`.
That batch passed both-workspace strict Clippy/Rustdoc, minimal/portable graphs,
focused runtime and positive/negative lint consumers; Windows proof is compile-only.
The owning issue records cases. Keep #354 open for native macOS 15 Intel/ARM.
0.40.1's [deployment-rule review](../publishing.md), admission and one-lint CI/
release/publication routing checks are under `target/toko0401.XXEGwz/`.
Directory-module provenance and prior 0.39.x evidence remain; the optional Rustdoc
checker is unselected. The authorized five-group frozen deployment is preparing;
no new release, full gate or upload has completed yet.

## Pending Platform adoption

Both roots now select Platform 0.32.0 and Types 0.20.0; the private probe selects
Runtime/macros/guard 0.22.0. Tools 0.92.0 and Testkit 0.102.0 cover the exposed Rust
identity transition; native PocketIC remains 16.1. Wire stays at 0.21.5 with its
optional Types integration disabled. Adoption changed only Platform-family blocks.
Strict affected-caller Clippy, isolated graphs, Wasm/probe and public-contract
checks pass. Installed snapshot restoration and Wire state/failure calls pass on
Linux. The sandbox denied one Agent loopback fixture; only that case was retried
with local-network access before continuing the remaining composition stages.
Evidence: `target/platform-adoption.2onG3b/`. No caller edits, macOS proof or delivery.
Keep historical preservation refs, but do not reapply their old Platform blocks.

## Pending deployment qualification work

[#353 — Reduce tooling-command qualification cost in deployments and avoid duplicate same-source CI](https://github.com/dragginzgame/ic-tooling/issues/353)
has local fixture reductions and timing/admission/fallback diagnostics. All four
production fixture stages and the 32-stage gate remain; focused Linux adapter
samples are 114.247/108.110 seconds, not deployment/macOS speedup evidence.
Checks are under `target/issue353/`. The [release design](../design/release-validation.md#deployment-cost-observations)
owns timing details. Its [collector boundary](../design/release-validation.md#collector-failure-boundary)
has no post-processing-only recovery; explain the failed phase, missing admission
and rerun scope before retry. Keep #353 open for real deployment measurements.
The deployment will supply actual phase evidence; no sibling delivery is included.

## Released tooling and selected inputs

Foundation 0.76.5 captured Toko Tooling 0.39.15's 115-file snapshot. Prior proofs
remain under `target/toko03913/` through `target/toko03917/` and
`target/toko-adoption.ZfdcMi/`; current commands are in [the host guide](../hosts.md).

The released graph retains Platform 0.31.2, Types 0.19.4, Wire 0.21.0 and
qualification Runtime/macros/guard 0.21.1. Its metadata/freshness evidence remains
under `target/deploy-foundation765/`; the Foundation ledger records deferrals.

Unrelated ordinary lock updates were saved outside the release graph at
`.git/release-state/preserve/deploy-20261112-foundation765/`, including both full
locks, their patch and the prior handoff. The durable branch
`preserve/deploy-20261112-foundation765-dependencies` retains the latest deferred
choices. Both locks were restored after reconciliation, preserving external blocks.
The new deployment freezes these reviewed current selections with the intended
supplier updates, without a dependency update. Exact source, untracked files and
both locks are preserved at `.git/release-state/preserve/deploy-20261113-minors.XUnm1U/`.
Both locked all-feature metadata checks and the Foundation adapter/selection
fixtures pass; evidence is `target/deploy-minors-20261113.4mmynn/`. The roster is
Foundation 0.77.0, Build 0.84.0, Text 0.75.0, Tools 0.92.0, Testkit 0.102.0.
No artifacts were retired.

First capture `batch-j5L1wRS7` stopped in archive tests after 22 passing stages:
a Build fixture read Cargo's private build-script stdout path, changed in 1.100.
It created no prepared version, tag or upload. Retain its checkout and durable
journal. The fixture now compares cached JSON exports with its own `OUT_DIR`
marker; the exact regression passes. A corrected source capture requires a full
gate because current receipts cannot admit stages across source/checkout changes.

## Testkit snapshot correction and open issue

[#263 — Cached snapshots and timer state](https://github.com/dragginzgame/ic-tooling/issues/263)
is corrected in published Testkit 0.101.0, release `08a193d8`, source `a54410f1`.
Frozen `batch-Aps5IIGp` completed and was reconciled. Its complete 32-stage Linux
gate passed. Retain `.git/release-state/frozen/batch-Aps5IIGp/`, its managed
checkout and `target/deploy263/`. The registry checksum and delivery verification
are recorded in those receipts and the Testkit ledger.

Native capture prepares metadata-upload snapshots through upstream PocketIC and
Platform-owned DTOs/codec, preserving timer/hook, globals, heap/stable, Wasm,
certified data and chunks. It requires two slots per target and transfer cycles;
stop/drain/start, exact senders, funding and partial-failure recovery remain
caller-owned. No client fork, second transport, timer reset or time rewind.
Installed regressions cover repeated mutated-baseline reuse and callback rearming.
Keep #263 open for Canic's exact two-case acceptance against the corrected release.
No Canic edits, dependency overrides or new tests are authorized by this delivery.
[Timers #87 — Snapshot regression qualification](https://github.com/dragginzgame/ic-timers/issues/87)
tracks its own coverage; the timer ownership guard remains intact.

[Push CI on 43ebe178](https://github.com/dragginzgame/ic-tooling/actions/runs/43180163629)
passes all hosts after its HTTP 403 retry (`target/ci43180163629/`); this does not
qualify later local source changes or Rust 1.100.0.

## Next independent fixes

1. Preserve exact paths in [Build tool probes](../../crates/ic-build/src/tools/probe/mod.rs)
   and check raw directory names before archive normalization.
2. Propagate filesystem errors from [Cargo configuration selection](../../crates/ic-build/src/cargo/environment/mod.rs).
3. Reject empty/malformed successful [process-state observations](../../crates/ic-host-process/src/testing/mod.rs).

These findings are not part of this release. Canic's separate Build migration is
[Canic #762 — Explicit maintenance and pruning](https://github.com/dragginzgame/canic/issues/762).

## Retained evidence and contracts

The [previous committed handoff](https://github.com/dragginzgame/ic-tooling/blob/43ebe17817da21f9a0c376d8e87d7638a76582f6/docs/status/current.md)
records earlier batches, exact publication identities, preservation refs and
recovery paths. Retain them all; failed attempts must not resume under newer
published versions or today's adapters. Existing `preserve/*` refs and
`.git/release-state/preserve/` hold deferred lock/reconciliation evidence.
The retired client archive remains `/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`;
original source is `0c17900090f0c2d2ebf3182598b49e651223845e`.

Keep two-pass generated-root discovery, Darwin `File::sync_all` durability and
discovery error downcast identity. Retain the snapshot cleanup callback's explicit
future type; attempted `AsyncFnMut` alternatives failed its Send lifetime contract.
Native macOS 15 Intel and Apple Silicon evidence is separate from Linux. Hosted
qualification after delivery is asynchronous; do not wait or infer passes.
