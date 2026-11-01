# Current handoff — 2026-11-01

## Source implementation of #299 and #300

This source batch implements
[#299](https://github.com/dragginzgame/ic-tooling/issues/299) and
[#300](https://github.com/dragginzgame/ic-tooling/issues/300). Implementation preserves existing package versions, tags, journals and consumer
deployment profiles. Source integration and each group's release/publication
remain separate delivery steps.

Release selection now has three groups: `foundation` (unchanged seven public
packages), `tools` (Host Tools), and `testkit` (Testkit/Native plus private Dev and
perf probe). Primaries are Build, Host Tools and Testkit respectively. Current
versions remain 0.73.5 / 0.76.1 / 0.76.1. New Tools/Testkit ledgers link the frozen
joint integration history through 0.76.1; no published history was rewritten.
Old integration attempts must retain their original source/adapter for recovery.
See [release groups](../design/release-groups.md) for concrete dependency/type
ownership, publication ordering and explicit selection.

Build adds an opt-in qualified rustc recipe and CLI wrapper. It preserves Cargo's
effective flags, applies requirements only to the selected Wasm target, refuses
conflicts unless explicitly qualified, and retains bounded compiler arguments
and identities with exact artifacts. Existing wrappers, compiler-custody config
overrides and shared compiler state are refused for this opt-in. Default recipes
retain their existing behavior. Post-link work reuses the existing admitted-tool
and transactional cache boundary. [Effective recipes](../build/effective-recipes.md)
records concrete Auth/Canic adoption and fresh final-Wasm measurements. Their
active checkouts and tool pins remain unchanged; persistent consumer adoption
follows a published Foundation version.

Focused local Linux checks passed: all-three-group preparation/rollback/receipt
fixtures, publication selection and Make routing; source and real normalized
Tools/Testkit archives with published peers on Rust 1.99.1 and 1.91.0; deliberate
old certificate identity refusal; Build Clippy (all targets/features, warnings
denied); four real-Cargo compiler recipe fixtures on current/MSRV; 78 Cargo and
77 Wasm-cache regression tests. Archive evidence is retained in
`/tmp/integration-cohort.2NUZza/` (Tools) and
`/tmp/integration-cohort.RPVDn3/` (Testkit).

The complete installed qualification is under
`/tmp/ic-tooling-300-qualification-final/`: Platform-maintained vectors, published
Platform 0.16/K256 0.14/crypto-bigint 0.7.10, Rust 1.91, Binaryen 132 and prepared
stock PocketIC 16.1.0. All eight cases passed three samples per backend. Compiler
recipes, ordered optimizer arguments, raw/final Wasm, uncompressed module/code
bytes, IC instruction counts, locks and checked source/tool/result hashes remain
there. Earlier failed attempts remain under `/tmp/ic-tooling-300-qualification-*`;
the final run completed without substituting their evidence.

Native macOS evidence for these local changes is still pending; the published
source qualification below does not qualify this new source. Full release gates run under the separate delivery workflow; the focused
checks above do not substitute for them. The ordinary checkout's separate
Metrics 0.24.5 and Wire/Derive 0.13.4 lock selections are preserved outside this
source batch. This delivery checkout retains the committed dependency selections.

## Published Foundation 0.73.5 / Integration 0.76.1

Both authorized patches are released and published, Foundation first:

- `foundation-v0.73.5`, commit `bc1f4f392757d43639505d99fdca90711c6bf5dd`:
  Artifacts, FS, HTTP, Process, Text, Build and the Wasm inspector.
- `integration-v0.76.1`, commit `9a46c075f27efac3ee97f42c1bb4ef2b411bb5ba`:
  Host Tools, Testkit and Testkit Native. Private Dev/probe versions are 0.76.1
  and remain unpublished.

All ten public versions are independently confirmed live and non-yanked in the
Cargo index. Downloaded registry archives match the actual upload checksums.
Those completed release effects must not be replayed; the source batch above
has its own subsequent group delivery.

## Delivered contracts

[PR #296](https://github.com/dragginzgame/ic-tooling/pull/296), merged as
`7c0f9956`, adds Unix `qualify_existing_package_archives` for already-produced
archives and caller-selected checks. It preserves frozen external selections and
final archive/extracted-source/manifest/lock custody. Consumers own native/MSRV/Wasm
coverage, tools, budgets and release policy. The existing convenience API retains
packaging and its exact all-target/doctest roster. The new API is in Build 0.73.5.

[PR #298](https://github.com/dragginzgame/ic-tooling/pull/298) selects published
Wire/Derive **0.13.3**. Existing public types, byte-array representations, frames,
budgets and feature boundaries are unchanged. The incoming Wire lock selections
were incorporated without unrelated dependency updates. Integration 0.76.1 inherits
published Foundation 0.73.5 requirements through the normal group renderer.

Platform **0.16.0**, canonical Types **0.15.1** and arithmetic-only Metrics
**0.24.4** remain selected. Edition 2024, Rust 1.91 and extraction boundaries stay
unchanged. The earlier Integration 0.76.0 certificate migration still applies:
Tools returns Platform-owned certificate/tree types; native Agent callers align
Platform and use its explicit conversions. CBOR admission, encoded bytes and
caller-owned authentication are unchanged. Certificate-only profiles exclude
registry certification; native Agent retains its separate external dependency.

## Qualification

Each group ran its standard complete local Linux CI and Rust 1.91 gate before
version preparation: Foundation on source `f9c4d4a4`, Integration on `bc1f4f39`.
Both runs qualified all ten extracted public archives, feature graphs, deployed
Wasm profiles, documentation and installed stock PocketIC 16.1 behavior.
Each separate publication dry run then passed, followed by clean-source/remote
provenance admission, package verification, uploads and independent readback.
Git release and registry publication were separate commands for each group.

Native macOS evidence is separate from local Linux execution. All 15 checks on
[source PR #298](https://github.com/dragginzgame/ic-tooling/actions/runs/41302928032)
passed, including native Linux, macOS 15 Intel, macOS 15 Apple Silicon, MSRV and
lint/security. Separate browser-session, PocketIC concurrency and optimizer jobs
also passed on all three hosts. No pending or cancelled check is counted as a pass.

PR head `90e14eb2` and merged source `f9c4d4a4` have identical tree
`df13b00de2ab2c6247e8f44022ce4051ee498a6e`. Both release commits change only
selected group version/catalog/lock/changelog-date metadata. Later release-head
and handoff CI continues asynchronously and is distinct from that source proof.

## Ordinary checkout, evidence and recovery

Before the local issue work above, the ordinary checkout contained Foundation
0.73.5 / Integration 0.76.1 with a clean tree and passing full locked offline
metadata. All unrelated lock records from that completed delivery
match the incoming graph exactly; only the twelve owned package versions changed.
Incoming source is retained in the recorded recovery stash and backups. Do not
apply a complete saved patch over the released metadata.

The canonical completed record is
`.git/release-state/deployment-foundation-0.73.5-integration-0.76.1/delivery-result.json`.
It retains both journals/receipts, exact pre-preparation source metadata,
publication/provenance records, all ten upload/readback archives, registry facts,
native qualification and work-preservation evidence. `product-evidence.tar.gz`
has a verified adjacent SHA-256 and excludes build caches. Original logs, artifacts
and the pinned release checkout remain under `/tmp/ic-tooling-deploy0735-0761.ZATSzj/`
(`release/`). Preserve both the durable record and original execution context.

Prior Foundation 0.73.4 and Integration 0.76.0 records remain under their existing
`.git/release-state/deployment-*` directories. Their checkouts remain under
`/tmp/ic-tooling-foundation0734.md3B6O/` and `/tmp/ic-tooling-integration0760.iALuHh/`.
[Earlier handoff](https://github.com/dragginzgame/ic-tooling/blob/7c0f995612cdbeb7515d8bac1bf8111046b5c0d6/docs/status/current.md)
preserves older qualification, stashes and recovery coordinates. YAML remains on
maintained `yaml_serde` / `libyaml-rs`, not a memory-safety rewrite; [Canic #719](https://github.com/dragginzgame/canic/issues/719)
owns its distinct consumer adoption. The Platform PocketIC takeover is separate.

The Shared snapshot remains 152 files at
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a`: upstream labels the commit 0.27.22,
while VERSION/export declare 0.27.21. Preserve exact payloads and the independent
[release groups](../design/release-groups.md) / [extraction contract](../extraction.md).
Do not reinterpret old journals under new policy or clean artifacts implicitly.
Plain `cargo clean` retains its normal whole-workspace target scope.
