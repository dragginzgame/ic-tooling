# Current handoff — 2026-11-01

## Published Foundation 0.73.6 / Tools 0.77.0 / Testkit 0.77.0

| Group | Version | Release commit | Public packages |
| --- | --- | --- | --- |
| Foundation | 0.73.6 | `ffafc54b10f1de750fe92526c49aa0ea70e4e124` | Artifacts, FS, HTTP, Process, Text, Build, Wasm inspector |
| Host Tools | 0.77.0 | `d2b14582ab0b6dcb3ef6e72094b8f7da4b195c1a` | Host Tools |
| Testkit | 0.77.0 | `9269e63aebcf6a85b2ae9848b5bf2f505ea9b6f7` | Testkit, Testkit Native |

Tags are `foundation-v0.73.6`, `tools-v0.77.0` and `testkit-v0.77.0`.
The three new 0.77.0 registry versions are live and non-yanked. Downloaded
archives match the upload checksums and each selected release commit. Private
Dev and perf probe versions are 0.77.0; neither was uploaded. Foundation remains
unchanged. These delivery effects are complete; do not replay an increment or upload.

## Platform adoption and consumer contract

[PR #305](https://github.com/dragginzgame/ic-tooling/pull/305), merged as
`454a9d95d52da5d54ae3b45cea48f939663d59bb`, adopts published Platform 0.17.0,
Runtime 0.17.1 and Types 0.16.0. Native Testkit now selects Platform's maintained
PocketIC client through the existing `ic_platform::pocket_ic` imports. Consumers
passing clients or client-owned DTOs must align on Platform 0.17. Host Tools'
exposed Platform principals/certificates require the same dependency alignment.
See the [Tools](../../releases/tools/CHANGELOG.md) and
[Testkit](../../releases/testkit/CHANGELOG.md) migration notes.

The PocketIC server remains pinned to 16.1.0. Wasm graphs exclude the native
client; system calls and the private probe use the independent Runtime owner,
with the existing executor singleton. Host/Tools graphs exclude the runtime and
native client implementation. No consumer checkout or server pin was changed.
The [three release groups](../design/release-groups.md), extraction boundaries,
Rust 2024 and MSRV 1.91 remain unchanged.

## Qualification

Both selected groups ran complete local Linux CI and Rust 1.91 qualification
before version preparation: Tools on source `454a9d95`, Testkit on `d2b14582`.
Each gate includes all ten normalized public archives, feature boundaries,
installed PocketIC behavior and deployed Wasm compilation. Each publication
then passed its separate dry run, clean-source/remote provenance admission,
verification, upload and independent registry/archive readback.

Native hosted evidence is separate from local Linux. All fifteen source checks
passed on PR head `71d9bb7de9bcf77a893a76114ace27a1e09210c9`, including
[Linux](https://github.com/dragginzgame/ic-tooling/actions/runs/41323510659/job/124898559843),
[macOS 15 Intel](https://github.com/dragginzgame/ic-tooling/actions/runs/41323510659/job/124898559840)
and [macOS 15 Apple Silicon](https://github.com/dragginzgame/ic-tooling/actions/runs/41323510659/job/124898559836).
That head and merged source `454a9d95` share tree
`9380b4d024972784be572c2c0c811ecb256209f0`. Later release commits change only
selected group metadata; their CI is separate from this source qualification.

## Ordinary checkout and durable evidence

The ordinary checkout contains the released versions. Its only working-tree
change is Cargo.lock: incoming Metrics 0.24.5, Wire/Derive 0.13.5 and Thiserror
2.0.21 selections remain outside the released source batch. All 458 external
lock records were preserved exactly; only five owned package versions changed.
Full locked offline metadata passes. The preservation stash is
`19c01f315b83cc5943cf477c2eb3c68104241f9c`; do not apply its complete old metadata
patch over the delivered versions.

Durable evidence is under
`.git/release-state/deployment-tools-0.77.0-testkit-0.77.0/`:
release plans/receipts, complete gates and publication logs, downloaded registry
archives, source CI results, lock-preservation records and checksum-verified
`product-evidence.tar.gz` excluding compilation caches. The isolated release
checkout and its build artifacts remain there too. No cleanup was performed.
An initial `/tmp` checkout lost Git metadata/logs during qualification for an
unconfirmed reason. Remote/index checks established no release or publication
before fresh recovery in this durable location; its surviving fragments are not
the recovery authority.

The [previous handoff](https://github.com/dragginzgame/ic-tooling/blob/580b0306b8ecee1e735496c49b031cf09548412a/docs/status/current.md)
retains the completed Foundation 0.73.6 / Tools-Testkit 0.76.2 delivery and older
recovery coordinates. Existing release ledgers and durable records remain untouched.
The reviewed Shared snapshot remains 152 files at
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a` (VERSION/export 0.27.21).
