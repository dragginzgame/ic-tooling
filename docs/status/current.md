# Current handoff — 2026-11-13

## Deployment delivered

The Toko 0.42.5 [frozen workflow](../publishing.md) delivered all seven selected
packages to GitHub and crates.io. Downloaded registry archives match index
checksums, non-yanked versions and exact release commits; remote annotated tags
and main were verified. The ordinary checkout reconciled cleanly to `64fb53c2`;
this handoff is subsequent Git-only documentation. No sibling changes or cleanup.

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Foundation (five packages) | 0.77.2 | `9d938d2814b637ea40d4ddbae110e87ab402c881` | `foundation-v0.77.2` |
| Host Tools | 0.92.1 | `1c2eeb4614256108d247bd0bc90a9c2e3b53b582` | `tools-v0.92.1` |
| Testkit | 0.103.0 | `64fb53c25121c3c54f9e2102551637bd92e9d48c` | `testkit-v0.103.0` |

Patch `batch-iZ1FgHB1` captured corrected `45d7f88b`; after reconciliation,
minor `batch-iZa8DNVj` captured `1c2eeb46`. Both completed and remain retained.
Build 0.84.1 and Text 0.75.0 stay current: archive comparison proves Git-only
development tooling/lint maintenance. Private qualification versions stay 0.96.0.

Preserve entry source/locks, supplier indexes, archive comparisons and logs under
`.git/release-state/preserve/deploy0425-20261113.clk428/`.
Current locks select Platform 0.32.2, Types 0.20.2, Runtime/macros/guard 0.22.2,
Wire 0.22.1 and Mio 1.2.5. Both final locks differ from entry only in the selected
owned package versions. Do not reapply older saved locks over these selections.
Declared minimums deliberately retain the reviewed compatible requirements.
Tools changes its Platform/Types minimums; Testkit exposes Wire 0.22 identities.
No application-state migration or PocketIC client fork is involved.

The first frozen-hook fixture correctly refused competing synthetic/real pending
headings; fixture copies now use independent synthetic patch ledgers. Real
history is untouched. The corrected fixture passed five group queries, order
refusal, deferred locks, interrupted/uncertain recovery, both archive identities,
completed resume and remote conflict. Evidence: `frozen-hooks-corrected.log` in
the durable deployment directory and `target/tmp/frozen-release.ydSER8/`.
Its local bare-remote Git effects and Cargo dry runs were not public releases.

Failed public batch `batch-iJtCSlO_` stopped before preparation or delivery in the
detached adapter fixture. Duplicate `VALIDATION_REPOSITORY_ROOT` selected the
helper instead of consumer checkout. `45d7f88b` retains the consumer `$PWD`;
the focused Foundation/shared recovery check passed before recapture. The
corrected batch reran the complete gate. Preserve the failed journal, checkout,
`patch-batch.log` and gate logs; never resume it as validation of changed source.

## Adopted tooling and validation

Both workspaces select Toko 0.42.5. The authenticated exporter moved executables,
catalogs, actions and hooks under `vendor/toko-tooling/`; canonical root files
and the root receipt remain. All 116 installed files are admitted. Consumer
Make/script/CI/fixture callers are migrated without compatibility wrappers.
Both locks preserve non-tooling package blocks during each tooling adoption.
The private binary's pin uses inherited `target.'cfg(any())'.dev-dependencies`;
libraries retain ordinary dev-dependencies. Rust/edition remain 1.100.0/2024.

Snapshot/tool/pin admission, formatting, 665 document references, native script
lint and relevant Make/CI routing, retention, evidence and reuse checks pass.
Focused supplier qualification covers native codec/Agent/Registry adapters,
installed PocketIC stable-memory restore, Wire state/typed-failure canister,
portable/Wasm graphs and strict affected-library/probe Clippy. Complete release
gates passed 32 stages each for Foundation, Tools and Testkit. Rounded stage sums
were 610/502/605 seconds; tooling accounted for 257/250/251. Whole-command wall
times were 1,624.82 seconds for the patch batch and 866.71 for the minor, excluding
planning, fixture qualification, failures, reconciliation and readback. Durable
`deployment-records/` contains receipts and gate logs; `registry/` holds archives.
See [cost observations](../design/release-validation.md#deployment-cost-observations).

Explicit asynchronous [CI](https://github.com/dragginzgame/ic-tooling/actions/runs/43489328225),
[optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/43489334454),
[runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/43489342928) and
[browser](https://github.com/dragginzgame/ic-tooling/actions/runs/43489351593) runs
all returned main at `64fb53c2`, in progress at handoff. Their independent run
identities survive the documentation push. Fresh native macOS 15 Intel and
Apple Silicon passes are not yet claimed; do not infer them from Linux evidence.

Toko 0.42.6 is available; all 116 installed payloads match 0.42.5 in bytes/mode.
Defer that producer-only fixture change to the next refresh. Retain reviewed
native pins and Build 0.73.0 metadata editor. Optional cargo-sort-derives 0.15.0,
yq 4.54.3, cloc 2.12 and editor 0.84.1 belong to tool maintenance. The selected
cloc flow counts source without archive expansion. Other reviewed native tools
are current. Never hand-edit managed catalogs or snapshot receipts.

## Outstanding acceptance and independent work

- [#353](https://github.com/dragginzgame/ic-tooling/issues/353): deployment phase
  timing and collector recovery remain incomplete; no whole-gate reuse is added.
- [#354](https://github.com/dragginzgame/ic-tooling/issues/354) and
  [#355](https://github.com/dragginzgame/ic-tooling/issues/355): keep open pending
  native macOS acceptance; Linux checks alone do not close them.
- [#263](https://github.com/dragginzgame/ic-tooling/issues/263): Canic still owns
  exact two-case snapshot/timer acceptance. Testkit 0.101.0 shipped the correction;
  Linux installed tests pass. Keep timer hooks, two slots and transfer cycles;
  sender/funding/stop/drain/start and partial-failure recovery stay caller-owned.
  [Timers #87](https://github.com/dragginzgame/ic-timers/issues/87) is separate.
- Independent follow-ups: preserve exact Build probe paths and reject raw archive
  directory names before normalization; propagate Cargo-config selection FS
  errors; reject malformed successful process observations. No sibling fixes.
- Build 0.84.0 compiler override facts require consumer vector/match migration:
  [Canic #247](https://github.com/dragginzgame/canic/issues/247). Canic maintenance
  adoption is [#762](https://github.com/dragginzgame/canic/issues/762).

## Preserved history and invariants

The [preceding handoff](https://github.com/dragginzgame/ic-tooling/blob/130fd2754b36e853c8bf59027e98760f79ae7f10/docs/status/current.md)
records earlier supplier/tooling proofs, batches and exact release commits.
Foundation 0.77.1/Build 0.84.1 were verified from registry archives in
`.git/release-state/preserve/deploy041-20261113.jBRdpw/`; retain `batch-4szjd3du`.
Earlier nine-package delivery is `batch-_HKWam5i`. Preserve all failed journals,
original adapters, `preserve/*` refs and retained `.git` evidence; never relabel
old receipts. Earlier ordinary `target/` losses were external, cause unknown;
only surviving raw logs or durable copies are evidence. No cleanup is authorized.
Retired client archive: `/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`,
original source `0c17900090f0c2d2ebf3182598b49e651223845e`.
Keep two-pass generated-root discovery, Darwin `File::sync_all`, discovery error
downcast identity and the snapshot cleanup callback's Send-safe future type.
