# Current handoff

## Source and ownership

This checkout starts clean at `12d8acb7d61ae9aea83bdd855113ce513273428d`, release
0.68.10. The earlier cleanup and Wire/derive 0.7.18/rustix 1.1.6 lock selections
are committed there. Compatible issue work is prepared in the
[0.68.11 draft](../changelog/0.68.md#06811---unreleased); own package versions
remain 0.68.10. Cargo.lock, the index and Git refs remain unchanged.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
Shared Tooling is now 0.26.21 at `66c91c4412856baa147482e648fc80428db8b179`,
recorded in [the snapshot](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and native Linux/macOS 15 Intel and Apple Silicon remain required.

## Current local changes

`WasmBuildEvidence::record()` borrows the existing retained record for artifact
rosters, tool facts and recipe comparison. The public example checks exact
cardinality and consumes retained bytes before final freshness verification.
Source custody and final acceptance remain consumer-owned. This is the local
implementation of [#255 — Expose retained Wasm build evidence](https://github.com/dragginzgame/ic-tooling/issues/255),
not a published supplier or completed Timers adoption.

The [#256 — Use existing thiserror derives for native errors](https://github.com/dragginzgame/ic-tooling/issues/256)
cleanup replaces the eleven confirmed mechanical implementations. Existing
dependency selections suffice, including the private benchmark's existing
thiserror dependency. Conditional recovery errors and deliberate source-chain
projections outside that roster remain manual. No new Wasm dependency is added.

The [private cache hash evaluation](../design/wasm-cache-hashing.md) records the
measured no-go decision for [#252 — Evaluate Platform BLAKE3 for private Wasm cache verification](https://github.com/dragginzgame/ic-tooling/issues/252).
Keep SHA-256 and current cache formats. Its temporary instrumentation and probe
remain isolated evaluation artifacts; no production dependency was added.

Normal increments and exact-attempt recovery retain the delivered
[release contract](../releases.md). No release command or state cleanup was
executed during this work.

## Qualification and evidence

Current baseline, lock, focused logs and evaluation source/data are under
`/tmp/ic-tooling-open-issues.fOTL8a/`. On actual Rust 1.91 the two evidence tests
pass, covering retention during pruning, release after the last owner drops,
source/configuration freshness refusal, typed retention recovery and exact
exported bytes. The public example runs both cold and warm with real Rust 1.91
Wasm compilation. Build's all-target/all-feature Clippy passes on the declared
development compiler. An additional Rust 1.91 Clippy attempt refused an existing
unfulfilled `result_large_err` expectation in `wasm_batch.rs`; its log is retained
and it is not reported as a pass. Native macOS execution of this addition remains
outstanding. No full workspace/release gate, commit, push, release or publication
was run.

The error-derive batch passes affected-owner all-target Clippy with the changed
Tools profiles (`candid`, `encoding`, `icp-project`), plus four Rust 1.91 contract
tests covering unbounded generic formatting, boxed-cause identity and the local
parser wrapper. The affected staging, package, retained-write, encoding, project,
service, statistics and install-message module tests pass on Rust 1.91, as does
the changed Tools encoding Wasm library check. The evidence tests also pass
after the error-owner changes. Initial mechanical-edit
and test-module placement failures are retained alongside corrected passing
evidence. This remains local implementation, with delivery and native macOS
qualification separate.

Earlier cleanup evidence remains under `/tmp/ic-tooling-cleanup-06810.l2W3fI/`:
88 cache/batch tests, 12 package-admission tests, 29 private benchmark tests and
CLI stream/exit projections on Rust 1.91, plus affected-owner Clippy. These
observations describe that earlier source and are not new qualification of the
evidence accessor.

The delivered Shared/Platform/Wire adoption evidence remains under
`/tmp/ic-tooling-adopt-02621.fYXFtb/`, tied to its own source and dependency
selections. Its prior Linux and Rust 1.91 results do not qualify this addition.

Earlier Shared 0.26.20 adoption evidence remains under
`/tmp/ic-tooling-shared-02620.1xOqGH/`, including the consumer Make, release and
retention checks. Preserve its original failed archived-Draft fixture at
`/tmp/host-fixture-retention.8AIbwe/`.

The earlier cleanup and Platform/Wire adoption evidence remains under
`/tmp/ic-tooling-cleanup-0687-20261026/` and `/tmp/ic-tooling-adopt-20261026/`.
Those observations retain their own source identities and host limits; they are
not new native-host qualification of this tooling update. The previous detailed
handoff remains [in Git history](https://github.com/dragginzgame/ic-tooling/blob/ae80ee173250848087b8da3322b68a1ed6e2feba/docs/status/current.md).

## Dependency blocker and retained history

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
remains separate, with the supplying correction at
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50). The selected
graph still contains crypto-bigint 0.5.5. Actual ECDSA signing prevents an
unused-code exemption; no local crypto fork, override or exemption was added.
The 2026-10-27 primary registry recheck still finds Agent 0.49.3 and
ic-secp256k1 0.3.0 constrained to the older curve families. Current consumer
inverse graphs and frozen registry records are in the current evidence
directory's `crypto/` subdirectory. This is a dependency-constraint review,
not a fresh advisory audit or constant-time execution proof.

Earlier .6 evidence remains under `/tmp/ic-tooling-improvements-20261026/`,
`/tmp/ic-tooling-0686-followup-20261026/` and `/tmp/ic-tooling-251-20261026/`.
Earlier cleanup evidence remains under `/tmp/ic-tooling-complexity-5-20261025/`,
`/tmp/ic-tooling-complexity-4-20261025/`, `/tmp/ic-tooling-complexity-3-20261025/`,
`/tmp/ic-tooling-complexity-2-20261025/` and `/tmp/ic-tooling-complexity-20261025/`.
Dependency-adoption evidence remains at `/tmp/ic-tooling-platform-wire-20261025/`
and `/tmp/ic-tooling-adopt-20261024/` for its own source identities.

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
remains at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`. Retained recovery
plans, failed attempts, original artifacts and backups are not disposable.
