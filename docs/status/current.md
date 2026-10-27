# Current handoff

## Source and ownership

This checkout starts at `42a9ec79b10f7bf9e3a5658980ca73817c2a771d`, release
0.68.12. The ten internal cleanups are committed in that release. Shared Tooling
and Platform adoption are prepared in the [0.69.0 draft](../changelog/0.69.md#0690---unreleased).
Own package versions remain 0.68.12. The incoming Platform/macros 0.6.21 lock edit
is archived; only those two records advance to 0.7.0. All other incoming lock
bytes are preserved. No Git index or ref was changed by this work.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
Shared Tooling is now 0.26.22 at `5fc904a935d82ad31f0fba14434b71ccb8c10345`,
recorded in [the snapshot](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and native Linux/macOS 15 Intel and Apple Silicon remain required.

## Current local changes

The canonical exporter updates five policy documents and the exact snapshot.
The local agent overlay records the new baseline. The adopted guidance clarifies
owned-type Candid exclusions, explicit external/retained-data boundaries, use of
existing native error derives and strict inspection/output admission. Tool pins,
executable scripts and CI wiring do not change.
Policy adoption alone does not install a new source scanner or runtime gate.

The root catalog now requires published IC Platform 0.7.0, with matching macros
and unchanged feature selections. Published archive checksums match the primary
registry records, with source revision `60c43108ca7f2a99bfac62ec21075afff5ed7a7e`.
Tooling does not call Platform's retired Candid compiler exports. Required
external parsing/decoding, SDK identities and wire/storage formats remain intact.

Registry shard APIs expose Platform's owned `Principal`, so advancing its crate
identity is a public compatibility change. Consumers exchanging those values
must update their direct Platform dependency to 0.7 alongside Tooling 0.69.
The pending release therefore requires a minor increment; package metadata is
left for the user's normal release preparation. No compatibility facade or
parallel production Platform version is added.

Normal increments and exact-attempt recovery retain the delivered
[release contract](../releases.md). No release command or state cleanup ran.

## Adoption qualification and evidence

The source archive, incoming lock/patch, exporter impact/refresh logs and
qualification records are under `/tmp/ic-tooling-adopt-02622.o8o6Gf/`.
Snapshot verification covers all 141 selected files at the exact source revision.
Tracked Rust source review finds Candid only in the external decoder boundary
and the standard ledger block projection; owned application payloads are not
given a Candid derive. The prior Shared-only checks remain policy/snapshot
evidence, separate from dependency qualification.

Platform evidence is under `/tmp/ic-tooling-platform-07.xNqWCh/`: incoming
source/lock/patch, primary index records, exact published archives/source,
resolver delta, scoped graphs and focused logs. On actual Rust 1.91, 59 Host
Tools tests, four native memory tests and 90 selected Testkit tests pass; six
deliberately ignored Testkit cases remain unexecuted. Testkit and its CDK/Wire
probe compile for Wasm. A standalone Registry consumer runs with Platform 0.7;
the 0.6 Principal control refuses with compiler code E0308, confirming the public
type-identity migration. That fixture's optional legacy dependency is not in
the production workspace graph.

Affected-owner all-target/all-feature Clippy passes. Five scoped native/Wasm
graph checks pass, retaining deployed Host/PocketIC/Candid exclusions. One live
PocketIC 16.1 Wire-call case passes on Linux after offline admission of the
existing server bundle. No server/tool download occurred. Documentation and
format checks remain distinct from these runtime results. Native macOS execution
is outstanding; no full workspace/release gate, commit, push or publication ran.

## Earlier cleanup qualification

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
The Platform 0.7 native Tools graph still selects Agent 0.49.3 and elliptic-curve
0.13.8 through that affected family, as retained in the current Platform evidence's
`tools-native-tree.txt`. This dependency adoption does not resolve the advisory.
The 2026-10-27 primary registry recheck still finds Agent 0.49.3 and
ic-secp256k1 0.3.0 constrained to the older curve families. Current consumer
inverse graphs and frozen registry records are in
`/tmp/ic-tooling-open-issues.fOTL8a/crypto/`. This is a dependency-constraint review,
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
