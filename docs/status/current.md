# Current handoff — 2026-11-01

## Published Foundation 0.73.4

The authorized Foundation patch is released and published:
`foundation-v0.73.4`, commit `d897fd38649c9cc7609f6cceeb4350f7b58592b9`.
All seven public Foundation packages are independently confirmed live and
non-yanked in the official Cargo index; their checksums match the upload archives.
Integration stays at published **0.75.2**. No private or integration package was
republished, and no further release increment or upload is pending.

[PR #292](https://github.com/dragginzgame/ic-tooling/pull/292) delivered the
maintained `yaml_serde 0.10.7` / `libyaml-rs 0.3.0` backend behind Host Text's
unchanged optional YAML API. Preserve byte-first admission, duplicate/tag/merge
policy and caller-owned schemas/redaction. This is a maintained low-level Rust
port with unsafe internals, not a memory-safety rewrite. The same PR adopted
Shared's exact prose classifier without changing tool pins or release gates.

[PR #293](https://github.com/dragginzgame/ic-tooling/pull/293) corrected the
Bash 3.2 classification guard: failure exits explicitly, with refusal tested
without relying on `errexit`. The prior eight native fixture failures remain
failure evidence. Owning Tooling issues #285, #286 and #287 are closed.

## Qualification and publication evidence

The canonical completed delivery record is
`.git/release-state/deployment-foundation-0.73.4/delivery-result.json`.
It retains registry confirmations, source qualification, upload archives,
publication/provenance evidence, release receipts and preserved-work recovery.
`product-evidence.tar.gz` is verified by its adjacent SHA-256 and excludes
Cargo build caches. The original release checkout, logs, journals and all build
artifacts remain under `/tmp/ic-tooling-foundation0734.md3B6O/`; its checkout is
`release/`. Preserve both the durable record and original execution context.

The standard complete local Linux CI and Rust 1.91 release gates passed on
source `035fbff291ce4882195756a965014ddbdddccd53`, before normal metadata preparation.
The separate Foundation dry run, clean-source admission, remote provenance fetch,
package verification and seven uploads then passed at release commit `d897fd38`.
Git release and crates.io publication were separate commands; do not replay them.

Native macOS evidence is separate from local Linux execution. The complete
[source PR matrix](https://github.com/dragginzgame/ic-tooling/actions/runs/41290850648)
passed Linux, macOS 15 Intel and Apple Silicon, plus MSRV and lint/security.
Its head `587f89fe` and merged source `035fbff2` have the identical tree
`4629cf9cf01cdd09eb269b110c883db5e81b02e7`. The release changes only the standard
version/catalog/lock/changelog-date metadata. Pending or superseded release-head
and later documentation CI is not counted as another native pass.

The release graph retains Wire/Derive **0.13.2**, Metrics **0.24.0**, Platform
**0.15.2** and canonical Types **0.15.1**. Edition 2024, Rust 1.91, package names,
public APIs and extraction boundaries are unchanged. The PocketIC client takeover
in Platform is separate from this Foundation publication.

## Ordinary checkout, baseline and recovery

The ordinary checkout contains Foundation 0.73.4 with Integration 0.75.2.
Its unrelated incoming lock selections remain Metrics **0.24.3** and
Platform/macros/runtime guard **0.15.3**, outside this release. Every non-Foundation
lock record is preserved exactly, and locked offline metadata passes; that alone
does not qualify these supplier changes for publication. The delivery record owns
the incoming lock/patch backups and stash identity. Do not apply a complete saved
working patch over the integrated source.

The Shared snapshot is 152 files at
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a`, including the added classifier.
Upstream labels the commit 0.27.22 while VERSION/export declare 0.27.21; preserve
its exact managed payloads. One workspace/catalog/lock retains independent
[Foundation and integration groups](../design/release-groups.md), under the
[extraction contract](../extraction.md). Root CHANGELOG.md remains their index.

[Canic #719](https://github.com/dragginzgame/canic/issues/719) owns downstream
YAML adoption. Compatible consumers can select Text 0.73.4 through their normal
lock update; producer publication does not close distinct consumer work.

Prior Foundation 0.73.3 / integration 0.75.2 receipts, older saved attempts,
remaining stashes and historical artifact coordinates are retained in the
[previous handoff](https://github.com/dragginzgame/ic-tooling/blob/035fbff291ce4882195756a965014ddbdddccd53/docs/status/current.md).
Do not reinterpret old journals under new source or policy. Exact recovery uses
its recorded checkout, adapter, destination and receipt context. Normal builds
and releases do not clean artifacts; plain `cargo clean` retains its ordinary
whole-workspace target scope. Earlier externally removed `/tmp` locations are
not proof that artifacts still exist.
