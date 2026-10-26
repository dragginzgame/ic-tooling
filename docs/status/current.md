# Current handoff

## Source and ownership

This checkout is based on `dede95dd1d6ea1cb79367ecd3957233fcf3a2c81`, release
0.68.6. Compatible local cleanup is prepared in the
[0.68.7 draft](../changelog/0.68.md#0687---unreleased). Own package versions remain
0.68.6; the index and Git refs are unchanged. Workspace minimums now select
Platform 0.6.18 and Wire 0.7.16. The incoming lock already selected these releases
and matching macros/derive packages; this adoption preserves it byte-for-byte.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
Shared Tooling remains 0.26.14 at `e14050bcc0c627f9c8a9064051699cac44fd7ab3`,
recorded in [the snapshot](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and native Linux/macOS 15 Intel and Apple Silicon remain required.

## Current local cleanup

One local tooling roster drives command execution and the selected Make and
retention fixtures; behavioral assertions remain independent. Graph exclusions
share local vocabulary while preserving every rendered graph-check argument.

Sync and async instance construction share final deadline/server admission.
Sync construction now refuses a completion that arrives after its deadline.
Their wait, cancellation and ownership mechanisms remain separate. Cargo
completion tracking and valid synthetic artifact records have common private
owners. Error derives preserve messages and typed causes; custom recovery-source
projection stays explicit. Benchmark duration checks share one predicate, and
exact-cache preparation is inlined without changing retention or lock scopes.

Platform/Wire adoption keeps existing capabilities and frame schemas. Platform's
optional BLAKE3 feature remains unselected. Wire transport covers the changed
16-bit decoder with signed/unsigned limits and malformed frames. Tooling has no
caller of Wire's changed host-generator diagnostic interface.

The publication Clippy guard remains independent: the delivered Linux log shows
its repeated pass took 0.61 seconds. Removing it would add bypass state or another
CI mode for little demonstrated benefit. No publication contract was weakened.

## Qualification and evidence

Current evidence and starting source are under
`/tmp/ic-tooling-cleanup-0687-20261026/`. Build's 272 library tests, four baseline
contract tests and 29 driver tests pass. Linux startup passes 28 ordinary tests,
six real PocketIC 16.1.0 lifetime tests and one failed-recovery integration.
Changed-owner Clippy and all-target/all-feature checks on actual Rust 1.91 pass.
Make routing/order/failure, fixture retention, shell lint, formatting, snapshot,
dependency declarations, tracked-path and documentation-link checks also pass.
Linux and both macOS target graph selections and layering refusals pass; those
named-target observations do not qualify native macOS execution of this patch.
Adoption evidence is under `/tmp/ic-tooling-adopt-20261026/`, including the incoming
dirty tree, registry records and release identities. Nine Wire tests pass on both
compilers; affected-package Clippy/MSRV, native/Wasm feature graphs and both Wasm
compiler selections pass. The live Linux PocketIC probe and all five focused
Agent tests pass. Formatting, dependency-pin and documentation-link checks pass;
the incoming lock remains unchanged.

The approved pinned release editor 0.18.0 was prepared through the shared
installer after retention qualification exposed a missing selection. Earlier
failed attempts, the external lock selection and all artifacts are retained.
Native macOS qualification of the new cleanup remains outstanding. No full
workspace/release gate, commit, push, release or publication was run.

The delivered 0.68.6 [CI matrix](https://github.com/dragginzgame/ic-tooling/actions/runs/40295204797)
passes Linux, native macOS 15 Intel/Apple Silicon, MSRV and lint/security.
That evidence closes [#251 — Native cancellation readiness on macOS Intel](https://github.com/dragginzgame/ic-tooling/issues/251);
it does not qualify the current dirty cleanup.

Earlier .6 evidence remains under `/tmp/ic-tooling-improvements-20261026/`,
`/tmp/ic-tooling-0686-followup-20261026/` and `/tmp/ic-tooling-251-20261026/`.
Earlier cleanup evidence remains under `/tmp/ic-tooling-complexity-5-20261025/`,
`/tmp/ic-tooling-complexity-4-20261025/`, `/tmp/ic-tooling-complexity-3-20261025/`,
`/tmp/ic-tooling-complexity-2-20261025/` and `/tmp/ic-tooling-complexity-20261025/`.
Dependency-adoption evidence remains at `/tmp/ic-tooling-platform-wire-20261025/`
and `/tmp/ic-tooling-adopt-20261024/` for its own source identities.

## Dependency blocker and retained history

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
remains separate, with the supplying correction at
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50). The selected
graph still contains crypto-bigint 0.5.5. Actual ECDSA signing prevents an
unused-code exemption; no local crypto fork, override or exemption was added.
The selected Platform/Wire updates do not remove that dependency risk.

Keep `/tmp/ic-tooling-249-250/`, `/tmp/ic-tooling-wire-07/` and
`/tmp/ic-tooling-wire-071/` as earlier graph/codec evidence. Original audits remain
under `/tmp/ic-tooling-audit-fixes/`, `/tmp/ic-tooling-audit-2/`,
`/tmp/ic-tooling-audit-2-fixes/` and `/tmp/ic-tooling-shared-02610/`.
Preserve the browser reproduction at `/tmp/ic-tooling-audit-browser-RwZzM9/`.

The nested-workspace target fix is delivered. Preserve its original failed
`target/package-qualification-3180841-0/`, `/tmp/ic-tooling-239/` and
`/tmp/ic-tooling-package-phase/`. Earlier detailed qualification is retained in
[the released handoff](https://github.com/dragginzgame/ic-tooling/blob/248df96b27b44366095d9d8e195cca8a1023bde1/docs/status/current.md).

Migration and local-repository retirement archives remain under
`/tmp/ic-tooling-migration/`, including `retirement/`; original refs were
preserved in the former Host repository. Stale release-lock recovery evidence
remains at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`. Retained plans,
failed attempts, original artifacts and backups are not disposable.
