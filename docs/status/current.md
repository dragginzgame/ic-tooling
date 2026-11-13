# Current handoff — 2026-11-13

## Authorized deployment in progress

Publish Foundation 0.77.2 (five packages) and Tools 0.92.1 in a patch batch,
reconcile it, then publish Testkit 0.103.0 in a minor batch. Use the installed
Toko 0.42.5 frozen workflow in [publishing](../publishing.md); a successful
release already publishes. No sibling changes, delivery, retirement or cleanup.
Build 0.84.1 and Text 0.75.0 remain current: archive comparison proves Git-only
development tooling/lint maintenance. Private qualification versions stay 0.96.0.
Main is unprotected; direct integration pushes, tags and uploads are authorized.
Hosted qualification is asynchronous; local required release gates remain intact.

Preserve entry source/locks, supplier indexes, archive comparisons and logs under
`.git/release-state/preserve/deploy0425-20261113.clk428/`.
Current locks select Platform 0.32.2, Types 0.20.2, Runtime/macros/guard 0.22.2,
Wire 0.22.1 and Mio 1.2.5. Their compatible patches were present at entry;
do not refresh dependencies during deployment or reapply older saved locks.
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

Public batch `batch-iJtCSlO_` captured `130fd275` and stopped before preparation,
Git delivery or upload in the detached release-adapter fixture. A duplicate
`VALIDATION_REPOSITORY_ROOT` assignment selected the helper rather than consumer
checkout. The normal-source correction retains the existing consumer `$PWD`;
its Foundation/shared recovery cases pass. Commit and recapture. Do not alter
or resume the failed capture with changed source. Preserve its journal, checkout,
`patch-batch.log` and Foundation gate logs. Its first eight stages passed,
including strict Clippy, formatting and docs; changed runner input invalidates
whole-gate reuse. New capture needs the complete gate, not a collector-only retry.

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
gates qualify the frozen current patches independently. Fresh native macOS 15
Intel and Apple Silicon qualification is not yet claimed.

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
