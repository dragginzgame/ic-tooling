# Current handoff — 2026-10-27

## Source and ownership

Main is at `17e91a6e516719ef1c5874d18f9d04b8b1ff3daf` after normal merge
commits for [PR #258](https://github.com/dragginzgame/ic-tooling/pull/258),
[PR #259](https://github.com/dragginzgame/ic-tooling/pull/259) and
[PR #261](https://github.com/dragginzgame/ic-tooling/pull/261). The merges were
explicitly authorized. Pending 0.69.1 combines those changes and the local Shared
Tooling adoption. Package versions remain 0.69.0; no release or publication ran.
The adoption remains unstaged. Host, Build and Testkit retain their boundaries;
consumers own targets, budgets, credentials, installation and recovery policy.

Shared Tooling is 0.26.26 at `5683060a5d8ce79af2fcfedb53b3eb24037fd064`,
exported from clean committed source. The snapshot owns 144 exact files. Platform
0.7, Rust 1.91 MSRV, development compiler and product-tool pins remain unchanged.

## Gzip integration and preservation

PR #261 adds `artifact::with_gzip_reader`: a bounded decoded Read interface over
caller-owned buffered input. Successful callbacks require bounded completion,
CRC/length validation and single-member admission. Source IO, integrity, both
limits and typed consumer errors remain distinct. Existing slice APIs reuse the
completion engine. Read-ahead, deadlines, parser policy and provisional effects
remain caller-owned; no dependency or publication policy was added.

The PR records 22 passing gzip tests on actual Rust 1.91, warning-denied Clippy
and Rustdoc, and an explicit IC Crates file/tar integration probe. The probe is
not released consumer adoption or SDK/browser/provider qualification. At merge,
Linux native, native macOS Apple Silicon, MSRV and all other completed checks
passed; native macOS Intel was still running. No pending result is counted as a
pass. Follow-up hosted integration runs asynchronously on merged main.

The uncommitted adoption was archived and stashed before fast-forwarding, then
restored. Only the handoff conflicted; its current summary preserves both owners.
Backups, incoming index/patch/source, the retained stash identity and final checks
are under `/tmp/ic-tooling-merge-261.iblr21/`. The stash remains available. No
source or artifact was cleaned, and the initial empty index is restored.
Combined formatting, 144-file snapshot integrity, 668 documentation links and
diff checks pass. All restored adoption payloads match the incoming archive;
product source, manifests and lockfile match merged main. Issue #260 auto-closed,
and the final GitHub inventory has no open IC Tooling PRs.

## Shared adoption evidence

Formatting reports pass timings and admits prepared tools offline. Balanced Rust
attributes are inspected through the exported RustSource.pm companion, including
in the hook/release fixtures. Real module-path overrides and incomplete metadata
still select uncached execution; this workspace retains explicit test paths.
The Node guide completes governance links without enabling setup or a scanner.

The 0.26.26 source, incoming work, exporter logs and Linux results remain at
`/tmp/ic-tooling-shared-02626.oDCiTD/`: snapshot integrity, actual formatting/hook
preservation, Make routing/failure evidence, substituted release routing, syntax
and 668 documentation references pass. These retain their original inputs;
post-merge checks are recorded separately with the merge evidence. No native
macOS or full-workspace/release gate ran locally for this adoption.

Earlier evidence remains under `/tmp/ic-tooling-shared-02625.8thmUs/` (revision-only
refresh), `/tmp/ic-tooling-shared-02624.gEgnQv/` (hooks/docs), and
`/tmp/ic-tooling-shared-02623.LIFM9m/` (formatting/release adapters and PR #259 note
reconciliation). Preserve the initial failed Make fixture at
`/tmp/host-make-snapshot.VqPuwx/`. No tool download occurred. The earlier release
adapter proof used substituted setup, qualification and Git effects.

## Other delivery and retained obligations

PR #258 supplies mutable Testkit access without splitting managed-server custody;
[Auth #90 — Simplify client and qualification state](https://github.com/dragginzgame/ic-auth/issues/90)
owns consumer adoption after publication. Its focused Linux evidence remains in
`/tmp/ic-tooling-auth-wrapper-20261027/target/auth-instance-access/`.
PR #259 consolidates ordinary gzip fixtures at the artifact owner; its initial
Intel runtime job failed before tests on Rust download DNS resolution. That
failed attempt remains separate from later qualification and the reader PR.

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
and [Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) remain
unresolved: crypto-bigint 0.5.5 is selected through actual ECDSA signing. No fork,
override or exemption was added; this merge is not a new crypto qualification.
Issues #255/#256 closed after exact 0.68.11 Linux, native macOS Intel/Apple Silicon,
MSRV and lint/security passed in [run 40478736900](https://github.com/dragginzgame/ic-tooling/actions/runs/40478736900).

The [0.69.0 handoff](https://github.com/dragginzgame/ic-tooling/blob/f268aef5d205e51811e934648afcd13f026635f1/docs/status/current.md)
retains earlier Shared/Platform and cleanup evidence. Migration/retirement archives
at `/tmp/ic-tooling-migration/`, original Host refs and stale-lock evidence at
`/tmp/ic-tooling-stale-release-lock.qHQFTn/lock` remain preserved. Failed attempts,
release plans, artifacts and backups are not disposable. Normal releases start
fresh; exact-attempt recovery uses saved source rather than partial preparation.
