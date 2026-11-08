# Current handoff

## Published deployment — 2026-11-08

The requested deployment is complete through frozen release batches. Normal main
was reconciled to `bdb4ac62`; the handoff update follows that release commit.

| Group | Published version | Release commit | Completed batch |
| --- | --- | --- | --- |
| Foundation | 0.75.1, all five packages | `42fad7e2` | `batch-Ddc4uVSy` |
| Build | 0.79.1 | `f0a42ee5` | `batch-Ddc4uVSy` |
| Host Tools | 0.87.0 | `1e485306` | `batch-gh19Zrhj` |
| Testkit | 0.93.2 | `bdb4ac62` | `batch-wp1Vct7C` |

All four group tags and main were pushed normally. Official registry checksums
and clean archive VCS identities matched all eight package releases. Text remains
at its already-published 0.74.3. Dev and the Wasm probe remain private. Normal
frozen commands already published; do not run a second `make publish`.

## Delivered changes

Shared Tooling 0.31.11 is pinned at
`6c529e6940a97226cca826726491e03173b08cd2` through the canonical exporter and
independent snapshot verification. Its authenticated download caches surround
direct CI setup; member-scoped formatting, hashing admission, scoped LOC reports
and focused-check issue closure are adopted. Gitleaks retains one CI owner,
outside release/publication gates. No Shared payload was hand-edited.

Platform's minimum requirement is 0.26.0, confirmed in the published Tools and
Testkit manifests. Tools consumers exchanging ledger `Value` objects must align
their direct Platform selection to 0.26. Testkit's upstream Candid, management
and subnet DTO identities remain compatible; management stays exactly 0.11.0.
Protocol bytes, caller custody and Wasm exclusions are unchanged. Rust remains
1.99.1 / edition 2024.

Foundation includes the merged bounded process/lock observations from
[PR #343](https://github.com/dragginzgame/ic-tooling/pull/343). Its non-Linux
kernel-holder stub is now const, correcting the observed macOS Clippy failure.
Build includes the reusable Canic machinery from
[PR #344](https://github.com/dragginzgame/ic-tooling/pull/344). Consumer adoption
remains with Canic, including its
[#742](https://github.com/dragginzgame/canic/issues/742) migration.

`ic-testkit-native`, its tests, workspace membership and active release/CI/
publication wiring are removed. Five active groups publish nine packages, with
two private qualification crates. No API moved into PocketIC Testkit. The
[Native ledger](../../releases/native/CHANGELOG.md), final published 0.80.0, tags
and original recovery records remain. Shared's immutable central inventory
update remains owned by
[Shared #365](https://github.com/dragginzgame/shared-tooling/issues/365).

## Qualification

All four complete 34-stage Linux x86-64 release gates passed with Rust 1.99.1.
Coverage includes workspace Clippy, formatting/hooks, docs, release/recovery
fixtures, native/Wasm graphs, package archives, installed PocketIC 16.1 management/
snapshot/gateway contracts and the complete workspace tests. Publication retained
fresh archive/provenance admission; registry readbacks verified the final payloads.

GitHub's official PocketIC asset API repeatedly returned HTTP 504 locally.
The user explicitly approved copying only the existing pinned 16.1 Linux bundle
into this deployment's frozen candidates. Each copy passed archive, executable
and version admission before the unchanged full gates ran. No source, lockfiles
or build outputs were copied. Independent fresh server downloads were not
qualified locally by that exception.

Native macOS qualification is separate and asynchronous. These explicit runs all
select the final published commit
`bdb4ac625b7dc443559184a8c50ce1570b1d9f07`:

- [CI](https://github.com/dragginzgame/ic-tooling/actions/runs/42480613652).
- [Build optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/42480616062).
- [Testkit runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/42480618543).
- [Browser fixtures](https://github.com/dragginzgame/ic-tooling/actions/runs/42480621231).

At dispatch readback, CI was running and the other three were queued. No new
native macOS Intel or Apple Silicon pass is claimed. Explicit qualification runs
retain their independent concurrency identities; routine later pushes do not
cancel them. Do not poll solely to keep a delivery turn open.

## Preserved work and recovery

The ordinary checkout includes every delivered crate/version and the Native cut.
Unrelated lock selections remain uncommitted and outside the released source:
Wire 0.17.14, Runtime/macros/guard 0.20.1, arc-swap 1.9.4, crypto-common 0.1.6 and
generic-array 0.14.9. Reconciliation merged these onto the new workspace versions
rather than restoring an old whole lockfile. The reconciled lock is preserved on
`preservation/shared03111-platform026-lock-deployed-20261108`; its commit is
recorded in the deployment evidence. Original selections also remain on
`preservation/shared03111-platform026-lock-retry-20261108` (`0d248820`), alongside
older preservation refs. Never overwrite newer work with an older backup.

Deployment logs, lock snapshots, bundle admission and official registry archives/
readbacks remain in `target/deployment-shared03111-platform026/`. Durable journals
and Git remain under `.git/release-state/frozen/`; captured source, tools and
builds remain under `target/release-frozen/`. No artifact cleanup or batch
retirement was run.
The superseded `batch-93c6DTSU` retains its preflight download failures and had no
version/tag/upload effects. Do not resume it against the now-published versions.

Earlier completed batches, superseded attempts, preservation refs and historical
recovery evidence remain described in the
[handoff before this deployment](https://github.com/dragginzgame/ic-tooling/blob/8fbf3a9428a7432c49eaabd626aca40eda167c7a/docs/status/current.md).
Recover historical batches only through their recorded source and adapter.

The safe fresh-Clippy fallback observed during publication is still tracked in
[#345](https://github.com/dragginzgame/ic-tooling/issues/345); this deployment
makes no claim to fix its environment/context mismatch.
