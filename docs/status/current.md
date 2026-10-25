# Current handoff

## Source and ownership

The checkout is based on `c9c5ef7c44e3d0c3b23c8531a5d43c1800656c0a`, the
0.68.1 release. Local complexity cleanup is prepared for the compatible
[0.68.2 draft](../changelog/0.68.md#0682---unreleased). Own package versions,
Git refs and the index are unchanged. The incoming Cargo.lock selects Wire/derive
0.7.5 and remains byte-for-byte preserved; manifest minimums remain 0.7.2.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
The reviewed Shared Tooling snapshot is 0.26.14 at
`e14050bcc0c627f9c8a9064051699cac44fd7ab3`, recorded in
[the snapshot manifest](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and Linux/native macOS 15 Intel and Apple Silicon remain required.

## Current local work

Build uses one shared-target acquisition path after locking, preserving ordinary
miss re-resolution, prepared leases, cache rechecks and scheduled maintenance.
Release rendering admits saved/live context before selecting dependency edits;
process qualification shares its existing two-worker handshake.

Testkit projects spawn settings once and shares startup-deadline admission.
Reply fixtures share catalog budgets, pools share timed recipe-stage attribution,
and benchmark validation/summaries share named phase values. Public APIs,
feature boundaries, report formats and lifecycle/recovery policy are unchanged.

Graph profiles select parser exclusions explicitly. The earlier
[#250 — Host/Build layering](https://github.com/dragginzgame/ic-tooling/issues/250)
guards are already in the base commit; this cleanup retains their coverage.
The accepted [#249 tradeoff](https://github.com/dragginzgame/ic-tooling/issues/249)
remains: keep the existing native graph instead of adding a published crate.

## Qualification and evidence

Current focused evidence is under `/tmp/ic-tooling-complexity-20261025/`.
Build's 75 cache cases, real two-worker coordination and saved/live release
context regressions pass on Linux. Prepared PocketIC 16.1.0 qualifies startup,
async cancellation, baseline recovery and immediate/held/cycle reply fixtures.
Initial sandbox listener refusals remain in the logs; native permission reruns pass.

Dependency profiles pass for Linux and both macOS target selections, and the
six-case layering rejection fixture passes. Changed-owner Clippy and Rust 1.91
checks pass for all targets/features, as do report/error regressions, formatting
and shell checks.
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
