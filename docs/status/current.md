# Current handoff

## Source and ownership

This checkout starts clean at `fdf6ef8466cff741435e78324edadbe7b18dc260`, release
0.68.11. The retained evidence accessor and mechanical error derives are
committed in that release; ic-build 0.68.11 is published. Compatible internal
cleanup is prepared in the [0.68.12 draft](../changelog/0.68.md#06812---unreleased).
Own package versions remain 0.68.11; Cargo.lock, the index and Git refs are
unchanged by this work.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
Shared Tooling remains 0.26.21 at `66c91c4412856baa147482e648fc80428db8b179`,
recorded in [the snapshot](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and native Linux/macOS 15 Intel and Apple Silicon remain required.

## Current local changes

The ten accepted internal cleanups bind verified transaction outputs to their
specifications, keep process-stream capture state together and make cooperative
benchmark-worker completion explicit through normal returns and unwinding.
Batch resolution uses named intermediate facts, depfile parsing retains its own
rule state, and pool acquisitions select final dispositions directly. Existing
phase timing handles publication work. Marker diagnostics retain one source
context; directory sizing and benchmark case construction use simpler loops.

Public APIs, cache/report formats, source-custody checks, independent destination
verification, feature boundaries and dependency selections are preserved. Worker
sampling panics now request a cooperative stop and wait before resuming the
original panic. No group kill, cleanup retry or teardown deadline is introduced.
Normal increments and exact-attempt recovery retain the delivered
[release contract](../releases.md). No release command or state cleanup ran.

## Qualification and evidence

Baseline archive, exact lock and focused logs are retained under
`/tmp/ic-tooling-cleanup-06812.vBhDNi/`. On actual Rust 1.91, 36 transaction,
20 input/depfile, 88 Wasm cache/batch, 77 process, two directory-size, 21 private
driver, 21 benchmark-marker and four pool unit tests pass. The driver test covers
both a sampling error and a sampling panic, preserving the original cause and
waiting for the worker's completion marker.

Three live Linux PocketIC 16.1 pool cases pass on Rust 1.91: normal build/restore
and explicit rebuild, failed restore/recovery, and unwind-driven invalidation.
The existing server bundle passed offline archive/executable/version admission;
no download occurred. The initial sandbox attempt could not bind loopback and
remains retained separately from the passing native runs.

Affected-owner all-target/all-feature Clippy passes on the declared development
compiler. The Testkit Wasm library compiles on Rust 1.91. Formatting, documentation
links and diff checks pass. Native macOS execution of this cleanup remains
outstanding. No full workspace/release gate, commit, push, release or publication
was run for this batch.

The preceding 0.68.11 implementation and evaluation evidence remains under
`/tmp/ic-tooling-open-issues.fOTL8a/`, including the private cache hashing no-go
report and primary crypto constraint observations. The supplying accessor is now
delivered; Timers' separately prepared adoption is consumer-owned. Earlier local
qualification and failed attempts retain their original source identities and
host limits. The previous detailed handoff remains in
[release history](https://github.com/dragginzgame/ic-tooling/blob/fdf6ef8466cff741435e78324edadbe7b18dc260/docs/status/current.md).

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
