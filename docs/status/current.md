# Current handoff

## Source and ownership

The checkout is based on `600fbd0db3ad761a92cf8418daac3517539b1da2`, the
0.68.3 release, which includes the earlier complexity cleanup. Local dependency
adoption and the next complexity cleanup are prepared for the compatible
[0.68.4 draft](../changelog/0.68.md#0684---unreleased). Own package versions,
Git refs and the index are unchanged. The incoming Cargo.lock selects Platform/
macros 0.6.16 and Wire/derive 0.7.10 and remains byte-for-byte preserved.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
The reviewed Shared Tooling snapshot is 0.26.14 at
`e14050bcc0c627f9c8a9064051699cac44fd7ab3`, recorded in
[the snapshot manifest](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and Linux/native macOS 15 Intel and Apple Silicon remain required.

## Current local adoption

The root catalog now requires Platform 0.6.16 and Wire 0.7.10, matching the
published registry versions already selected in the incoming lockfile. Testkit
README/overview examples use those minimums. Registry metadata, checksums and
published source revisions were checked; the sibling checkouts were read-only.

Platform's entrypoint refactor and Wire's derive/host-schema changes preserve the
current integration. Tooling keeps its public APIs, feature selections, endpoint
identities, schemas, frame bytes and caller-owned budgets. No Tooling client
regeneration, stored-byte migration or reset is required. Rust 1.91 remains the
supported floor; no new setup, release or publication runs as part of adoption.

The earlier
[#250 — Host/Build layering](https://github.com/dragginzgame/ic-tooling/issues/250)
guards are already in the base commit; this cleanup retains their coverage.
The accepted [#249 tradeoff](https://github.com/dragginzgame/ic-tooling/issues/249)
remains: keep the existing native graph instead of adding a published crate.

The current cleanup uses typed CLI/context admission, one observed/silent Build
invocation, separate precomputed release-reference lookups, shared filesystem
component checks and provisioning evidence writes, and existing error derives.
Benchmark scheduling receives its fresh/pooled operation once. Stable server
version admission avoids allocation, and CI honours the selected server CLI.
Public APIs, independent release validation, feature boundaries and caller-owned
lifecycle policy remain intact. Discovery's boxed TOML cause stays manually
unboxed; it must not acquire an extra box through an error derive.

## Qualification and evidence

Cleanup evidence is under `/tmp/ic-tooling-complexity-4-20261025/`, with the
incoming working-tree archive, patch and lockfile. Focused CLI, Cargo release,
batch, cache/digest, filesystem and provisioning cases pass, including real Cargo
Wasm builds with observed and silent batches. CLI help matches the captured
baseline. Make's substituted CI recipe honours a server CLI path with spaces.
Changed-owner Clippy and Rust 1.91 checks pass. The documented non-Unix FS Wasm
compile profile passes on both compilers; an over-broad all-target attempt included
Unix-only unit fixtures and is retained separately. CLI graph profiles pass for
Linux and both macOS target selections; base libraries exclude Clap.

Prepared PocketIC 16.1.0 completes fresh/pooled benchmark cohorts with 16 checked
samples and the existing report/timing contracts. The separate CLI child-process
test also passes with that installed server. Prior dependency-adoption evidence
remains at `/tmp/ic-tooling-platform-wire-20261025/`: real native Wire
calls, rebuilt Wasm entrypoints, frame/budget refusals, caller forwarding and
traps, affected tests, MSRV/Clippy and deployed Wasm profiles passed.
Native macOS 15 Intel and Apple Silicon execution remains outstanding; named-target
graph observations do not qualify native hosts. No full release gate, release,
publication or new setup/download was run. Earlier cleanup evidence remains at
`/tmp/ic-tooling-complexity-3-20261025/`, `/tmp/ic-tooling-complexity-2-20261025/`
and `/tmp/ic-tooling-complexity-20261025/` for its own source identities. Earlier
adoption evidence remains at `/tmp/ic-tooling-adopt-20261024/`.

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
