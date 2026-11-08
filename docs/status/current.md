# Current handoff

## Published deployment — 2026-11-08

Frozen batch `batch-0sSkZugM`, captured from `4c3bb1f4`, delivered and published
all four affected groups. Normal main was fast-forwarded to `66c718ad`; this
documentation update follows that release commit.

| Group | Published version | Release commit |
| --- | --- | --- |
| Foundation | 0.76.0, all five packages | `68e975d9` |
| Build | 0.80.0 | `eaef2491` |
| Host Tools | 0.88.0 | `d2db2799` |
| Testkit | 0.94.0 | `66c718ad` |

All four group tags and main were pushed normally. Official registry checksums
and clean archive VCS identities matched all eight package releases. Text remains
at 0.74.3; Dev and the Wasm probe remain private. Five active groups publish nine
packages. The frozen command already published; do not run a second `make publish`.

## Delivered changes

Toko Tooling 0.34.0 is selected through root workspace metadata and
`toko-tooling.lock`, with archive SHA-256
`5ca8ad09ad9359bc4945a4afdaafce301dd5a4b4955cd5c9f5058d7a54ffcf10`.
The authenticated installer preserved recorded entrypoints and local edits;
all Cargo dependency declarations for Toko are removed. The two private members
contain real qualification code and remain. Use `make shared-tooling-update` for
updates and `make shared-tooling-install` to reinstall the locked selection.
Release and direct-publication adapters check tooling before effects.

The architectural reduction removes the generic Artifacts Wasm-encoding surface,
orphan scripts and duplicate fixtures. Testkit encodes its raw-reply fixture
directly, retaining an independent exact-byte contract. Installation, snapshots
and diagnostics are inherent async PocketIC methods; the three extension traits
and `pic::prelude` are removed. Build's domain reports expose shared outcomes
through `report.batch`. Startup keeps instance admission and failure custody on
one owner. See the group ledgers for caller changes.

Host Tools splits binary `candid` decoding from the optional textual
`candid-service` parser. DID/Wasm comparisons select `candid-service,ic-interface`.
Platform's minimum is 0.27.0. Build's HMAC implementation adopts HMAC 0.13 and its
matching SHA2 0.11 dependency with exact-byte regression coverage; other SHA2
consumers retain their existing identity. Management DTOs remain exactly 0.11.0.
Imported client protocol bytes, async/no-replay semantics and caller custody are
unchanged. Rust remains 1.99.1 / edition 2024.

Publication now reuses equivalent qualified lint evidence through the nested
Make entrypoints ([#345](https://github.com/dragginzgame/ic-tooling/issues/345)).
Only repeated adjacent leading checkout tool-directory pairs are normalized;
source, flags and effective precedence changes still invalidate reuse.
All four actual publications reported qualified-lint reuse.

## Qualification

All four complete Linux x86-64 release gates passed. Coverage includes workspace
Clippy, formatting/hooks, docs, release/recovery and routing fixtures, native/Wasm
dependency graphs, package archives and doctests, installed PocketIC 16.1
management/snapshot/gateway contracts, and the complete workspace tests.
Publication retained fresh archive/provenance admission, followed by independent
official registry checksum and VCS readbacks for all eight packages.

Native macOS qualification remains separate and asynchronous. These explicit
runs select final published commit `66c718adb5abe65d09d0c29cbf8235d93aeee620`:

- [CI](https://github.com/dragginzgame/ic-tooling/actions/runs/42509005472).
- [Build optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/42509007987).
- [Testkit runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/42509010662).
- [Browser fixtures](https://github.com/dragginzgame/ic-tooling/actions/runs/42509013392).

All four were running at dispatch readback. No new native macOS 15 Intel or
Apple Silicon pass is claimed. Their explicit concurrency identities survive
later routine pushes; do not wait solely for hosted qualification during delivery.

## Preserved work and recovery

The ordinary checkout contains all delivered versions. Unrelated lock selections
remain uncommitted and outside the released source: Wire 0.17.14,
Runtime/macros/guard 0.20.2, arc-swap 1.9.4, crypto-common 0.1.6 and generic-array
0.14.9. Only their external-selection patch was reapplied after reconciliation,
preserving the new workspace versions. Locked offline metadata confirms all
11 members. The reconciled lock is preserved on
`preservation/audit-toko034-lock-deployed-20261108` (`2138838f`); original selections
remain on `preservation/audit-toko034-lock-20261108` and older preservation refs.
Never overwrite newer work with an older backup.

Logs, deferred locks, registry archives/readbacks and dispatch evidence remain in
`target/deployment-audit-toko034/`. Durable Git, plans, receipts and batch intents
remain under `.git/release-state/frozen/`; captured checkouts, tools and builds
remain under `target/release-frozen/`. No cleanup or batch retirement was run.
Failed batches `batch-98dqLbbV` and `batch-H2b76S8n` stopped before tag/push/upload;
their test-caller and routing-fixture defects were fixed before the successful
capture. Do not resume them against already-published versions.

Earlier deployment identities and retained recovery evidence remain in the
[preceding handoff](https://github.com/dragginzgame/ic-tooling/blob/4c3bb1f44a63ed7425e916fd569a12b49a7f85df/docs/status/current.md).
The Native crate remains retired, with its historical ledger and refs preserved.

Consumer adoption remains owned by
[Canic #744](https://github.com/dragginzgame/canic/issues/744) and
[Query #119](https://github.com/dragginzgame/ic-query/issues/119).
The supplier versions are now published; consumer qualification is separate.
