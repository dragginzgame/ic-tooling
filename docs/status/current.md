# Current handoff

## Source and ownership

This checkout starts at `a748463715f77d83029c22654ffad8c75a585187`, release
0.68.9. Compatible internal cleanup is prepared in the
[0.68.10 draft](../changelog/0.68.md#06810---unreleased); own package versions
remain 0.68.9. The incoming Cargo.lock edit selects Wire/derive 0.7.18 and
rustix 1.1.6. It is preserved byte-for-byte, separate from this cleanup.
The index and Git refs remain unchanged.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
Shared Tooling is now 0.26.21 at `66c91c4412856baa147482e648fc80428db8b179`,
recorded in [the snapshot](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and native Linux/macOS 15 Intel and Apple Silicon remain required.

## Current local changes

Build keeps each batch specification, group and pending result in one entry.
Prepared admission, error/timing attribution and source-lease invalidation keep
their existing behavior. Cache records derive target selection from resolved
inputs. Contextual I/O errors retain operation, path and original typed causes.
Package naming has one owner, and extracted-workspace commands share directory
setup while retaining their own offline/lock/feature selections.

The private benchmark driver validates against the exact worker arguments it
rendered. Its flat JSON schema, unknown-field refusals, positional arguments and
workload checks are unchanged. Valid report fixtures share one test-only owner;
malformed cases retain independently chosen perturbations and assertions.
Summary phase lookups and Clap exit projection stay with their existing owners.
Public APIs, cache/report formats, crate boundaries and tool pins are unchanged.

Normal increments and exact-attempt recovery retain the delivered
[release contract](../releases.md). No release command or state cleanup was
executed during this work.

## Qualification and evidence

Current source and incoming-lock evidence are under
`/tmp/ic-tooling-cleanup-06810.l2W3fI/`. On actual Rust 1.91, 88 cache/batch
tests, 12 package-admission tests and 29 private benchmark tests pass. Both
benchmark binaries preserve help on stdout with exit 0 and invalid arguments on
stderr with exit 2. Affected-owner all-target/all-feature Clippy passes, as do
formatting and documentation links. Earlier lint failures remain in the evidence
directory. The incoming lock is unchanged. Native macOS execution of this
cleanup remains outstanding. No full workspace/release gate, commit, push,
release or publication was run.

The delivered Shared/Platform/Wire adoption evidence remains under
`/tmp/ic-tooling-adopt-02621.fYXFtb/`, tied to its own source and dependency
selections. Its prior Linux and Rust 1.91 results do not qualify this cleanup.

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
