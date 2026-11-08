# Current handoff

## Shared 0.31.11 and Platform 0.26 adoption — 2026-11-08

Implemented locally on main at `54ed82d2`; nothing staged, committed or published
by this adoption. Shared was refreshed through its canonical exporter to
`6c529e6940a97226cca826726491e03173b08cd2` (0.31.11), with independent snapshot
verification. Platform's root requirement and lock now select published 0.26.0.
The Platform update preserved unrelated lock records; the later Native cut
pruned only its own entry. All retained records remain identical, including
Runtime/macros/guard 0.20.1, Wire 0.17.13 and the user's unrelated patch selections.
Management DTOs remain exactly 0.11.0; Rust remains 1.99.1 / edition 2024.

Authenticated download caching now surrounds direct tool setup in CI's Linux
lint/security job, optimizer jobs and runtime jobs. Native CI inherits caching
through Shared's common-tools action. Cache saves precede tests and retain the
Shared default-branch-push guard. Tool pins are unchanged. Gitleaks retains one
repository CI owner, including documentation changes, outside release gates.
New member-scoped formatting, hashing admission and crate/module LOC selectors
are imported; no Shared payload was hand-edited and no source owner was moved.

The subsequent 0.31.11 refresh adopts Shared's
[issue closure policy](../../rules/agent-maintenance.md#issue-review-and-repair).
Snapshot integrity and documentation checks cover this governance-only update;
the earlier code qualification remains applicable. Refresh logs remain under
`target/adoption-shared03111/`.

## Native allocator retirement — 2026-11-08

Removed `ic-testkit-native`, its tests, workspace/catalog membership, active
Native release selector and qualification/publication branches. Release-fixture
proof controls now use existing Text/Build packages. Five active groups publish
nine packages; two private qualification crates remain. No API moved into
PocketIC Testkit. The historical Native ledger, published versions, tags and
Git/release artifacts remain intact. Shared's immutable stack inventory update
is tracked in [Shared #365 — Retire Native from the active stack inventory](https://github.com/dragginzgame/shared-tooling/issues/365);
its snapshot-owned files were not hand-edited.

Metadata, retained graphs, both archive-selector tests, formatting, docs and shell
lint passed. The publication and Make fixtures passed; the latter now supplies
real owned-member metadata for Shared's explicit formatter selection. The
validation fixture passed all five full fallbacks, focused Text/Tools/Build,
failed stages, source drift and proof/receipt custody using substituted product
effects. Frozen-hook qualification also passed five group queries, interruption/
uncertain-publication recovery, exact Text/Testkit archive identities and stale-
remote refusal, with real Cargo dry runs and substituted product gates. No real
registry upload or repository delivery occurred. Logs and entry backups remain
in `target/retire-native-20261108/`.

## Pending impact and publication

| Group | Pending version | Reason |
| --- | --- | --- |
| Foundation | 0.75.1 | Merged #343 bounded process/lock observations; Shared maintenance notes. |
| Build | 0.79.1 | Merged #344 Canic machinery extraction. |
| Host Tools | 0.87.0 | Platform 0.26 changes the owned ledger `Value` generation exposed by ledger evidence. |
| Testkit | 0.93.2 | Compatible native Platform adoption; upstream Candid/management/subnet DTOs retain identity. |

All package versions remain at their existing released values. These are pending
notes, not release execution. Tools consumers exchanging ledger values must adopt
Platform 0.26 together. The new private public-contract case verifies that exact
exchange. Testkit's Candid contract error is an upstream re-export; boxed codec
causes retain their documented optional supplier-downcast coupling. No compatibility
bridge, error/API cleanup or new package was added.

[PR #343](https://github.com/dragginzgame/ic-tooling/pull/343) merged normally at
`f5901b38`, including the per-thread child-budget correction and its regression.
[PR #344](https://github.com/dragginzgame/ic-tooling/pull/344) merged normally at
`54ed82d2`, with pending Build notes corrected after published 0.79.0. The
Canic worktree remains clean at its original `d61f9b9b`; no sibling source was
changed. Those supplier additions are not published yet. Consumer adoption stays
with Canic, including its existing
[#742](https://github.com/dragginzgame/canic/issues/742) Build migration.

## Focused qualification

Linux x86-64 / Rust 1.99.1 passed: complete locked all-feature metadata, declared
IC/hash boundaries, native/Wasm feature graph policies, affected-package Clippy
for Tools/Testkit/Dev, formatting, formatter/hook preservation, workflow lint and
103 CI-routing/refusal assertions. The public ledger-value, Principal/Wire,
subnet-metrics and exact management DTO contracts passed. Runtime coverage totals
21 focused cases: ten Candid/ledger, two codec/no-replay, one management identity,
four public contracts and four explicitly admitted PocketIC 16.1 installed cases.
The latter exercise management/snapshot state, builder/gateway custody and logging.

Native macOS Intel/Apple Silicon and a full release/workspace gate were not run
for this local adoption. Previous release qualification does not establish this
new source. Logs and the entry lock remain under
`target/adoption-shared03110-platform026/`. The prior PR-focused 18-case/lint
proof remains in `target/pr343-pr344-merge-20261108/`.

## Last published versions and retained evidence

Build 0.79.0 (`e3e514f0`), Text 0.74.3 (`439ead57`) and Testkit 0.93.1
(`f494ea2d`) were delivered through completed frozen batches `batch-b1n6Stqb`
and `batch-XbEtnv9Z`. All three complete 34-stage Linux gates passed, and official
registry checksums/clean VCS identities matched. Foundation remains 0.75.0,
Host Tools 0.86.0 and Native 0.80.0; Dev/probe are private. Query can adopt Text's
published exact decimal-quantity and ratio-percentage helpers separately.

Batch Git, plans and publication receipts remain under `.git/release-state/frozen/`;
candidate outputs and sealed proofs remain under `target/release-frozen/`.
Registry readbacks/reconciliation logs remain in
`target/deployment-cleanup-20261108/`. Superseded `batch-SeyigorP` retains two
HTTP-504 preflight failures without tag/upload effects; do not resume it as a new
release. No artifacts, tools, batches or sibling directories were retired.

Deferred lock custody includes `preservation/pr343-pr344-lock-20261108`
(`658375dd`), `preservation/cleanup-release-lock-20261108` (`7021230f`) and
`preservation/cleanup-release-lock-20261108-updated` (`7fee0a54`). Never overwrite
newer user changes with older backups. Other preserved refs, completed batches,
native CI run links and original recovery facts remain in the
[handoff before this adoption](https://github.com/dragginzgame/ic-tooling/blob/54ed82d2f3dfdcb8e82472aedd6d4f5e39dc1fb6/docs/status/current.md).
Normal frozen releases already publish; do not run a second `make publish`.

[#345 — Admit unchanged release lint](https://github.com/dragginzgame/ic-tooling/issues/345)
retains the observed saved-context mismatch: the publisher safely ran fresh
Clippy. This adoption does not claim to fix it. Publication and artifact retirement remain separate authorized tasks; see
[publishing](../publishing.md).
