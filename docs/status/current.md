# Current handoff

## Source and ownership

This checkout starts at `e0ce26f933187eea1c45d8cb3dfb4173704105b5`: version
0.68.8 plus merged [PR #254 — Changelog emoji sections](https://github.com/dragginzgame/ic-tooling/pull/254).
Shared Tooling and dependency adoption are prepared locally in the
[0.68.9 draft](../changelog/0.68.md#0689---unreleased). Own package versions
remain 0.68.8 and the index is unchanged. Platform/macros 0.6.19, Wire/derive
0.7.17 and openssl-probe 0.2.2 are selected in the updated lock.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
Shared Tooling is now 0.26.21 at `66c91c4412856baa147482e648fc80428db8b179`,
recorded in [the snapshot](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and native Linux/macOS 15 Intel and Apple Silicon remain required.

## Current local changes

The canonical exporter refreshed the existing snapshot selection to the reviewed
0.26.21 commit. Only the compact baseline and changelog rules changed; the
shared executable payloads retain their prior qualification. Root notes use the
new emoji sections; older release bullets, links, versions and dates are preserved.
Existing executable-tool pins and workflow selections remain unchanged.

Platform and Wire minimums and native examples match the selected releases.
Their four runtime/macro source directories are unchanged from the previous
releases. The openssl-probe patch adds a Solaris certificate-file location;
Linux/macOS probing is unchanged. Cargo selected the compatible updates under
Rust 1.91; upstream exact requirements such as PocketIC 16.1.0 on thiserror
2.0.18 remain intact. No override or new feature selection was added.

Normal increments and exact-attempt recovery retain the delivered
[release contract](../releases.md). No release command or state cleanup was
executed during this adoption.

## Qualification and evidence

Current evidence is under `/tmp/ic-tooling-adopt-02621.fYXFtb/`: starting source,
registry observations, resolver previews, selected lock and source review.
PR #254 was reviewed and merged normally, and the clean local checkout was
fast-forwarded before adoption edits. Removing its sole added section restores
the earlier changelog byte-for-byte. On actual Rust 1.91, nine Wire tests, five
Agent tests, deployed Wasm compilation and the live Linux PocketIC probe pass.
Native/Wasm graph policies, snapshot integrity, dependency declarations,
manifest ordering and documentation-link checks also pass. Native macOS
execution of the new selections remains outstanding. No full workspace/release
gate or release/publication command was run; adoption edits remain uncommitted.

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
