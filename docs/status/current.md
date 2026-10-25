# Current handoff

## Source and ownership

This checkout is based on `41f5d1688bbf4321be89d352974f17957a0ef0bc`, release
0.68.4. The preceding complexity cleanup and Platform/Wire adoption are in that
base. The next local cleanup is prepared for the compatible
[0.68.5 draft](../changelog/0.68.md#0685---unreleased). Own package versions remain
0.68.4; Git refs and the index are unchanged. Platform/macros 0.6.16 remain
selected. A separate lockfile update to Wire/derive 0.7.12 appeared during checks
and is preserved; this cleanup does not change dependency requirements.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
The reviewed Shared Tooling snapshot is 0.26.14 at
`e14050bcc0c627f9c8a9064051699cac44fd7ab3`, recorded in
[the snapshot manifest](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and Linux/native macOS 15 Intel and Apple Silicon remain required.

## Current local cleanup

Native startup carries its admitted deadline through readiness and construction.
Baseline validation derives the stage from its preparation, and simple native
error derives preserve the existing diagnostics and direct causes. Discovery's
boxed TOML cause remains manually unboxed; custom redaction/cleanup projection
also remains with its existing owners.

Ordinary Wasm hits and misses share completion within their existing lock scope.
Scheduled maintenance retains separate completion because its guard lifetime
ends elsewhere. Scheduled checks carry a definite elapsed duration internally;
only the disabled-policy boundary omits it. No admission or source revalidation
was removed, and no additional lifecycle owner or guard state was introduced.

Benchmark phase accumulation uses its fixed known shape. Named RSS measurements
flow into the existing flat report, retaining null phases and statistics. Cargo
fixtures now generate a tiny isolated two-package graph; they do not inspect the
repository graph. Shared private filesystem test plumbing retains process
isolation. Hex decoding consumes admitted digit pairs after full validation.
Public APIs, feature selections, endpoint schemas and frame bytes are preserved.
No consumer regeneration, stored-byte migration or reset is required.

## Qualification and evidence

Current evidence is under `/tmp/ic-tooling-complexity-5-20261025/`, including the
starting tracked-source archive and both observed lockfile identities. Focused cache,
transaction, filesystem, response, driver, startup, transport and pool-policy
cases pass. Prepared PocketIC 16.1.0 passes actual sync/async startup and pool
recovery; fresh/pooled benchmark cohorts produce 16 samples satisfying the prior
report-shape, timing and percentile contracts. Changed-owner Clippy and actual
Rust 1.91 checks pass, as do deployed Testkit and non-Unix FS Wasm profiles on
both compilers. Host/Tools graphs pass for Linux and both macOS target selections;
Testkit native/Wasm graph guards also pass. Formatting, dependency pins and local
documentation links are checked independently.

Native macOS 15 Intel and Apple Silicon execution remains outstanding; named-target
graph checks do not qualify native hosts. No full CI/release gate, release,
publication or new setup/download was run. Earlier cleanup evidence remains at
`/tmp/ic-tooling-complexity-4-20261025/`, `/tmp/ic-tooling-complexity-3-20261025/`,
`/tmp/ic-tooling-complexity-2-20261025/` and `/tmp/ic-tooling-complexity-20261025/`
for its own source identities. Dependency-adoption evidence remains at
`/tmp/ic-tooling-platform-wire-20261025/` and `/tmp/ic-tooling-adopt-20261024/`.

## Dependency blocker and retained history

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
remains separate, with the supplying correction at
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50). The selected
graph still contains crypto-bigint 0.5.5. Actual ECDSA signing prevents an
unused-code exemption; no local crypto fork, override or exemption was added.
Platform 0.6.16 and Wire adoption do not resolve that dependency risk.

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
