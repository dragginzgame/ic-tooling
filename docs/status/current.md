# Current handoff — 2026-11-13

## Deployment delivered — Toko 0.42.8 and independent Wire adapter

The authorized current-repository batch is published to GitHub and crates.io.
Frozen `batch-aVh51g1i` captured corrected source `a4818cd8`, then delivered:

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Testkit | 0.104.0 | `79804eb4f304e1ff41b1a90198d81117f3297831` | `testkit-v0.104.0` |
| Testkit Wire | 0.1.0 | `fbbbde7c9fac8b4b4f7b44aafea6b1eae795a57a` | `testkit-wire-v0.1.0` |

Both non-yanked registry archives match index checksums and exact clean VCS
commits. Remote annotated tags/main were verified; the ordinary checkout
reconciled cleanly to `fbbbde7c`. Git-only qualification corrections followed
in `edfedc6c`; this handoff is subsequent documentation. No sibling source,
release, cleanup or artifact retirement was included.

Foundation 0.77.2, Build 0.84.1, Text 0.75.0 and Tools 0.92.1 remain current;
private qualification packages retain 0.96.0. Source selection and release
ownership cover ten products, two private members and six release groups.

## Qualified selection and tooling

Both roots select published Toko 0.42.8. Authenticated export retains all 116
managed files and entrypoints, changing only the hook and hook installer for
CDPATH-independent resolution. The baseline, native pins, release/publication
code and required gates are unchanged. Snapshot/tool/server admission and actual
consumer formatting/preservation/hook installation pass with `CDPATH=/tmp`.

The Toko refresh preserved all non-tooling package blocks byte-for-byte. Final
locks differ from entry only in Toko and the two owned product versions. Entry
already selected Bumpalo 3.20.4, whose published multithreaded ZST fix was reviewed.
Platform 0.32.2, Types 0.20.2, Runtime/macros/guard 0.22.2, Wire 0.22.1 and Mio
1.2.5 remain selected. Do not reapply older preservation locks over these inputs.
Optional native-catalog/editor updates remain separate maintenance; selected
cloc source counting does not expand archives. Never hand-edit managed catalogs.

## #357 — shipped ownership cut and remaining acceptance

Testkit has no Wire/Serde adapter dependency, `wire` feature or compatibility
exports. Lifecycle and raw transport retain the selected upstream client.
`ic-testkit-wire` alone owns typed native calls, independently aligned to Wire
0.22. Wire remains sole framing/validation owner. Exact method/caller identity,
all limits, typed causes, panic behavior and one-shot updates are preserved.
See [migration and publication ownership](../testkit/wire-adapter.md).

Keep [#357](https://github.com/dragginzgame/ic-tooling/issues/357) open for Auth/Blob
migrations and actual matching-adapter/multiple-Wire-generation acceptance.
Those sibling edits/publications are excluded. Package publication here does
not complete that consumer proof. Future Wire-only adoption selects the
`testkit-wire` group without a lifecycle release when its client API is unchanged.

## Validation and asynchronous hosts

Both complete Linux gates passed all 32 stages: strict Clippy/Rustdoc, feature
and native/Wasm boundaries, installed PocketIC, extracted archive tests and
installed workspace tests. Rounded stage sums were 583/429 seconds, including
245/239 seconds of tooling fixtures. Only four admitted unchanged fixture proofs
were reused across groups. Both publications freshly admitted source/archives
and reused only their qualified lint receipts. Whole batch wall time was
1,498.37 seconds, excluding planning, focused qualification, the failed attempt,
reconciliation and registry readback. Individual delivery phase timings remain
unavailable; these are observations, not controlled speedups.

The registry-only caller proof passed all three public-contract tests against
exact published packages, with no source overrides or normal-lock edits.
`edfedc6c` corrects its selected-client import and explicit ledger features.
The adapter ledger's pending 0.1.1 note describes this private-only maintenance;
it requires no product version increment or additional upload. Focused caller
Clippy, shell checks, formatting and document links pass too.

Explicit [native CI](https://github.com/dragginzgame/ic-tooling/actions/runs/43502371156),
[optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/43502379157) and
[runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/43502385864) runs
were in progress; [browser qualification](https://github.com/dragginzgame/ic-tooling/actions/runs/43502393911)
succeeded at readback. All four dispatches returned main at exact `edfedc6c`;
their identities survive this documentation push. Native macOS 15 Intel and
Apple Silicon passes are not yet claimed. Do not infer them from Linux evidence.

## Retained evidence and failures

Preserve `.git/release-state/preserve/deploy0428-20261113.au1Irv/`: entry patch,
locks, handoff, authenticated supplier archives/indexes, logs, `registry/`,
`registry-cohort/` and `deployment-records/` (receipts and gate logs). Current raw
consumer evidence is `target/published-integration.a5ZH8P/`; #357's focused
checks remain under `target/evidence/issue357/`. Retain the completed journal,
candidate Git and managed checkout for `batch-aVh51g1i`; cleanup is not authorized.

First capture `batch-g7FKs6lB` stopped after eight passing stages before version
preparation/delivery: the Make fixture omitted the new adapter Wasm check from
its expected command roster. The corrected fixture passed before recapture;
the new source ran both complete gates. Keep the failed journal, checkout and
logs. No old receipts were relabelled or claimed as collector-only recovery.

## Outstanding work and preserved contracts

- [#353](https://github.com/dragginzgame/ic-tooling/issues/353): phase timing and
  collector recovery remain incomplete; no whole-gate reuse was added.
- [#354](https://github.com/dragginzgame/ic-tooling/issues/354) and
  [#355](https://github.com/dragginzgame/ic-tooling/issues/355): native macOS acceptance remains required.
- [#263](https://github.com/dragginzgame/ic-tooling/issues/263): Canic owns exact
  two-case snapshot/timer acceptance. Keep two slots and transfer cycles; sender,
  funding, stop/drain/start and effect reconciliation stay caller-owned.
  [Timers #87](https://github.com/dragginzgame/ic-timers/issues/87) is separate.
- Independent fixes: exact Build probe/archive paths, Cargo-config FS errors,
  malformed successful process observations. Consumer work remains in
  [Canic #247](https://github.com/dragginzgame/canic/issues/247) and
  [#762](https://github.com/dragginzgame/canic/issues/762).

The [preceding handoff](https://github.com/dragginzgame/ic-tooling/blob/5c6961f4120b6c500efe767dd37b3730291b059a/docs/status/current.md)
records prior verified releases and recovery paths. Preserve all original
journals/adapters, `preserve/*` refs and durable records, including
`deploy0425-20261113.clk428/`, `batch-4szjd3du` and `batch-_HKWam5i`.
Earlier target losses were external, cause unknown; only surviving logs/copies
are evidence. Retired client archive remains
`/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`, source
`0c17900090f0c2d2ebf3182598b49e651223845e`. Keep two-pass generated-root discovery,
Darwin sync durability, error downcasts and the snapshot callback's Send-safe future.
