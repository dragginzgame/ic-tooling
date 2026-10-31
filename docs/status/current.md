# Current handoff — 2026-10-31

## Delivered Foundation 0.73.3 and integration 0.75.2

The authorized cleanup and Wire adoption are released and published:

- Foundation **0.73.3**: `foundation-v0.73.3`,
  `3048df1d03baaebfa11e30462892d336b2361ca8`; seven public packages.
- Integration **0.75.2**: `integration-v0.75.2`,
  `005419ae728f08f66599c87e3288cccbb683e326`; three public packages.
  The two private members advanced without publication.

All ten versions are live and non-yanked in the official Cargo index; checksums
match the local upload archives. Source delivery was
[PR #290 — Qualification cleanup and Wire 0.13.2](https://github.com/dragginzgame/ic-tooling/pull/290),
merged normally at `6a0291d8c1d64fd36944349dc7f89f0bed76a775`.
No further release increment or upload is pending. Do not replay completed effects.

The batch shares package-command capture and independent log retention, couples
acquired cache targets with their shared evidence, reuses bounded startup reads,
and uses safe direct-child signaling in the private worker. Public benchmark APIs,
cache configuration, feature boundaries, edition 2024 and MSRV 1.91 remain unchanged.
One workspace/catalog/lock serves the independent
[release groups](../design/release-groups.md), under the
[extraction contract](../extraction.md).

## Canonical delivery and qualification evidence

The completed delivery record is
`.git/release-state/deployment-foundation-0.73.3-integration-0.75.2/delivery-result.json`.
That directory retains independent registry confirmations, preserved-work patches,
and the verified `product-evidence.tar.gz` with its SHA-256. The archive excludes
Cargo build caches. Original release/publication logs, source/lock backups and
the clean release checkout remain under
`/var/tmp/ic-tooling-deploy0733-0752.orDI1k/`; its checkout is `release/`.
Both group journals and validation receipts remain in that checkout's
`.git/release-state/`. Preserve these records on success, failure and retry.

Both groups passed the standard complete isolated Linux CI and Rust 1.91 gates,
then their separate publication dry runs, source-provenance admission and uploads.
The release graph selects Wire/Derive **0.13.2**, Metrics **0.24.0** and Platform
**0.15.2**, with canonical Types **0.15.1**. Wire's patch preserves public type
identity, codec behavior and budgets; no consumer code migration is required.

Native macOS qualification remains separate from Linux evidence. Hosted CI is
asynchronous; consult the delivery record and latest retained main run before
claiming full qualification of this release. Source-PR browser, optimizer and
PocketIC-concurrency checks passed on both macOS hosts; those passes do not prove
the full native workspace matrix. Prior source `a7369dc2` has a complete successful
[Linux/macOS run](https://github.com/dragginzgame/ic-tooling/actions/runs/41228142298).
GitHub issues remain the sole follow-up tracker.

## Preserved ordinary checkout and baseline

The ordinary checkout contains both released groups. Its only local change is
`Cargo.lock`: Metrics **0.24.1** and Platform/macros/runtime guard **0.15.3**.
Those unrelated selections were restored after fast-forwarding main and were
excluded from this deployment. Locked offline metadata passes for that checkout;
this does not qualify all its supplier changes for release.
The complete incoming work is retained in stash
`bcfc98c4c7b7412237d4475c407e1097fa8752ca` and the deployment's incoming patch/lock
backups. The earlier stash `9eadfae0d0593d35183b82bd951c9d1a4bf35848` remains.
Do not reapply the complete stash over main: its cleanup is already delivered.

The adopted Shared snapshot remains 151 files at
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a`. Upstream calls the commit 0.27.22;
VERSION/export declare 0.27.21. Preserve its exact managed payloads.

## Historical recovery evidence

Superseded handoffs remain in Git history at `a7369dc2` and `6a0291d8`.
Previous deployments retain canonical records at
`.git/release-state/deployment-foundation-0.73.2-integration-0.75.1/` and
`.git/release-state/deployment-integration-0.75.0/`.
Earlier focused evidence remains under `/var/tmp/ic-tooling-286.eTpfLb/`,
`/var/tmp/ic-tooling-shared-refresh.M3bsiQ/` and
`/var/tmp/ic-tooling-shared020-metrics024.vmORyU/`.

Earlier `/tmp` supplier, cohort, deployment and migration directories disappeared
externally during the preceding deployment. Historical paths are not proof of
availability. Preserve remaining artifacts, installations, archived refs and
migration evidence. Old grouped/ungrouped plans retain their recorded source,
adapter and policy; recovery uses that source in isolation with its own authority
and destination checks. No artifact cleanup ran. Plain `cargo clean` retains
whole-workspace target semantics; durable receipts remain outside target.
