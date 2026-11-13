# Current handoff — 2026-11-13

## Authorized deployment — Testkit and Wire adapter

Deploy the reviewed #357 cut and Toko 0.42.8 refresh in this repository only.
Select one supplier-first minor batch: `RELEASE_GROUPS=testkit,testkit-wire`.
Expected versions are Testkit 0.104.0 and first `ic-testkit-wire` 0.1.0.
Source versions remain 0.103.0 and unpublished 0.0.0 until preparation. Main is
current/unprotected; commits, normal main/tag pushes and crates.io uploads are
authorized. Use [frozen delivery](../publishing.md); release already publishes.
Reconcile the completed batch without discarding deferred normal-checkout edits.
No sibling edits/releases, cleanup, artifact retirement or force pushes.

Both groups select the full local Linux gate. Earlier comparable gates took
502–610 rounded stage seconds per group, plus setup/preparation/package/delivery;
these are observations, not a promised duration. Hosted qualification remains
asynchronous. New-group metadata/recovery and all six validation plans passed
before deployment with product/delivery effects substituted. The frozen runner
and publication code are unchanged; do not rerun producer-wide fixture matrices.

Entry patch, locks, handoff, published tool archives, registry indexes and logs
are preserved in `.git/release-state/preserve/deploy0428-20261113.au1Irv/`.
Entry locks already selected Bumpalo 3.20.4, the published ZST multithreading fix.
The targeted Toko refresh preserves every non-tooling package block byte-for-byte.
Freeze those reviewed selections, including Platform 0.32.2, Types 0.20.2,
Runtime/macros/guard 0.22.2, Wire 0.22.1 and Mio 1.2.5. No other supplier update
belongs in this deployment. Planned registry versions are unoccupied.

## Adopted Toko 0.42.8

Both workspaces and all twelve members select the exact published package.
Authenticated preview/update retains all 116 managed files and entrypoints,
changing only the pre-commit hook and hook installer. The baseline, native pins,
release/publication code and product qualification requirements are unchanged.
Standard tools and PocketIC admission pass; old artifacts/generations remain.
Actual consumer formatting/hook preservation passes with `CDPATH=/tmp`, along
with both-workspace snapshot admission, formatting and local document links.
No new deployment procedure or release exception was introduced by this refresh.

## #357 — independent adapter

Typed query/update calls now live only in `ic-testkit-wire`, with independent
`testkit-wire` release ownership. Testkit has no Wire/Serde adapter dependency,
Wire feature or compatibility exports. Lifecycle and raw transport retain the
selected upstream client; Wire remains sole framing/validation owner. Exact
method/caller identity, all limits, typed causes and one-shot effects are retained.
See [migration and publication ownership](../testkit/wire-adapter.md).

Focused Linux proof passes: strict product/caller Clippy, nine adapter unit cases,
two installed rejection/raw-reply cases, private state/caller/failure canister,
three public contracts, native/Wasm graph boundaries, empty adapter Wasm build,
strict Rustdoc and standalone archive verification against published Testkit
0.103.0. New group metadata/recovery and all six group plans pass with effects
substituted. Wrong-group probes now use the catalog. Evidence is retained under
`target/evidence/issue357/`, including failed fixture and missing-cache attempts.
Required full release gates and new native macOS results are separate.

Keep [#357](https://github.com/dragginzgame/ic-tooling/issues/357) open for Auth/Blob
migrations and actual matching-adapter/multiple-Wire-generation acceptance.
Those sibling edits are excluded; deployment here does not complete that proof.
Build 0.84.1, Text 0.75.0, Foundation 0.77.2 and Tools 0.92.1 need no new upload:
the remaining maintenance is development metadata, documentation or native tooling.
Private qualification packages retain 0.96.0.

## Outstanding acceptance and independent work

- [#353](https://github.com/dragginzgame/ic-tooling/issues/353): phase timing and
  collector recovery remain incomplete; no whole-gate reuse was added.
- [#354](https://github.com/dragginzgame/ic-tooling/issues/354) and
  [#355](https://github.com/dragginzgame/ic-tooling/issues/355): native macOS acceptance
  remains required; do not infer it from Linux or pending hosted checks.
- [#263](https://github.com/dragginzgame/ic-tooling/issues/263): Canic owns exact
  two-case snapshot/timer acceptance of the correction shipped in Testkit 0.101.0.
  Keep two slots and transfer cycles; sender/funding/stop/drain/start and effect
  reconciliation stay caller-owned. [Timers #87](https://github.com/dragginzgame/ic-timers/issues/87) is separate.
- Independent fixes: preserve exact Build probe/archive paths, propagate Cargo
  configuration FS errors, reject malformed successful process observations.
- Consumer Build override migration: [Canic #247](https://github.com/dragginzgame/canic/issues/247);
  maintenance/pruning adoption: [Canic #762](https://github.com/dragginzgame/canic/issues/762).

## Preserved releases and invariants

The [preceding handoff](https://github.com/dragginzgame/ic-tooling/blob/5c6961f4120b6c500efe767dd37b3730291b059a/docs/status/current.md)
records the verified Toko 0.42.5 delivery: `batch-iZ1FgHB1` (Foundation 0.77.2,
Tools 0.92.1), then `batch-iZa8DNVj` (Testkit 0.103.0, `64fb53c2`). Preserve
`.git/release-state/preserve/deploy0425-20261113.clk428/`, original journals and
all failed captures, including `batch-iJtCSlO_`. Do not relabel old receipts.
Earlier releases are `batch-4szjd3du` and `batch-_HKWam5i`; keep their records and
`preserve/*` refs. Earlier target losses were external, cause unknown; only
surviving logs/durable copies are evidence. No cleanup is authorized.
Retired client archive: `/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`,
source `0c17900090f0c2d2ebf3182598b49e651223845e`. Keep two-pass generated-root
discovery, Darwin sync durability, error downcasts and the snapshot callback's
Send-safe future type. Defer optional native-catalog/editor upgrades to maintenance;
selected cloc source counting does not expand archives. Never hand-edit catalogs.
