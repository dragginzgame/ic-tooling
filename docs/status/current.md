# Current handoff

## Source and ownership

The checkout is based on `4357e0ab8b9678e21ee5bb3b396ba1e81549b34e`, the
0.68.2 release. Further local complexity cleanup is prepared for the compatible
[0.68.3 draft](../changelog/0.68.md#0683---unreleased). Own package versions,
Git refs and the index are unchanged. The incoming Cargo.lock selects Wire/derive
0.7.5 and remains byte-for-byte preserved; manifest minimums remain 0.7.2.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
The reviewed Shared Tooling snapshot is 0.26.14 at
`e14050bcc0c627f9c8a9064051699cac44fd7ab3`, recorded in
[the snapshot manifest](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and Linux/native macOS 15 Intel and Apple Silicon remain required.

## Current local work

Build shares metadata arguments, lexical Cargo-name admission and the single/batch
tool/metadata probe sequence. Shared-target locks use one path with silent or
observed progress. Boundary caches and batch keys borrow their input keys;
lock-package decoding and input-evidence errors retain their failure contracts.
Discovery keeps its manual error source projection so callers still downcast the
boxed TOML cause directly. File adapters reuse durable stream/byte helpers.

Testkit pool acquisition matches its existing slot state directly, and metered
memory shares backing storage and counters through one allocation. Fresh and
pooled benchmarks share body/release timing boundaries. Public APIs, feature
boundaries, report formats and lifecycle/recovery policy are unchanged.
The benchmark CLI uses Clap derive while retaining help, repeated selections and
native paths. Cases retain their measurements; report maps are serialization
projections. Private statistics produce the existing count/mean/p50/p95 fields.

Publication reuses admitted package paths; the combined Tools graph profile
selects all features through Cargo. Development/MSRV Wasm checks share one
package/feature selection with explicit compiler selection. The earlier
[#250 — Host/Build layering](https://github.com/dragginzgame/ic-tooling/issues/250)
guards are already in the base commit; this cleanup retains their coverage.
The accepted [#249 tradeoff](https://github.com/dragginzgame/ic-tooling/issues/249)
remains: keep the existing native graph instead of adding a published crate.

## Qualification and evidence

Current focused evidence is under `/tmp/ic-tooling-complexity-3-20261025/`, including
the incoming dirty source archive and patch. Build's cache, batch, transaction,
input-evidence and discovery tests pass on Linux, along with real shared-target
coordination/maintenance and all 17 Wasm acquisition cases. Benchmark contract
tests and exact CLI help comparison pass. Prepared PocketIC 16.1.0 completes fresh
and pooled smoke runs with two repeats and four samples per run. Rotation, report
shape, p50/p95 and phase/body/release accounting are checked; this supplies
correctness evidence, not a performance comparison.

Changed-owner Clippy and Rust 1.91 checks pass for all targets/features. The actual
development/MSRV Wasm profiles pass. Make routing is qualified with substituted
Cargo, tool and release effects; no real release runs in that fixture.
Formatting, shell and documentation-link checks pass. Earlier pool/memory/file,
publication-fixture and Linux/macOS-target graph evidence remains under
`/tmp/ic-tooling-complexity-2-20261025/` for its own source identity; earlier 0.68.2
evidence remains at `/tmp/ic-tooling-complexity-20261025/`.
Native macOS 15 Intel and Apple Silicon execution remains outstanding; named-target
graph observations do not qualify native hosts. No full release gate, release,
publication or new setup/download was run. Earlier adoption evidence remains
at `/tmp/ic-tooling-adopt-20261024/` for its own source identity.

## Dependency blocker and retained history

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
remains separate, with the supplying correction at
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50). The selected
graph still contains crypto-bigint 0.5.5. Actual ECDSA signing prevents an
unused-code exemption; no local crypto fork, override or exemption was added.
Platform 0.6.15 and Wire adoption do not resolve that dependency risk.

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
