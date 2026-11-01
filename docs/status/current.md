# Current handoff — 2026-11-01

## Published Foundation 0.73.6 / Tools 0.77.0 / Testkit 0.78.0

| Group | Version | Release commit | Public packages |
| --- | --- | --- | --- |
| Foundation | 0.73.6 | `ffafc54b10f1de750fe92526c49aa0ea70e4e124` | Artifacts, FS, HTTP, Process, Text, Build, Wasm inspector |
| Host Tools | 0.77.0 | `d2b14582ab0b6dcb3ef6e72094b8f7da4b195c1a` | Host Tools |
| Testkit | 0.78.0 | `94ca12bfb5f83b45d615e6d2c51887db7881b659` | Testkit, Testkit Native |

Testkit 0.78.0 is released under `testkit-v0.78.0`. Both public versions are live
and non-yanked; downloaded registry archives match the upload checksums and
release commit. Private Dev and perf probe versions are 0.78.0; neither was
uploaded. Foundation and Host Tools remain unchanged. Release and publication
are complete; do not replay an increment or upload.

## Wire and Metrics adoption

[PR #310](https://github.com/dragginzgame/ic-tooling/pull/310) adopts Wire 0.14.
[PR #311](https://github.com/dragginzgame/ic-tooling/pull/311), merged as
`3bf5f486669c0eabc2f4b56fe3743e6bb0c1e88c`, completes Metrics 0.25 adoption and
corrects maintained dependency examples. Consumers passing limits to
`WireCallSpec` or matching codec errors must align their direct Wire dependency
to 0.14. Frame formats, caller-selected budgets and existing Testkit error
contracts remain unchanged. See the [Testkit ledger](../../releases/testkit/CHANGELOG.md).

Metrics remains arithmetic-only with default features disabled; its native Wire
metadata is not selected. Platform 0.17.0, Runtime 0.17.1 and Types 0.16.0 remain
selected. Native clients and client-owned DTOs come from
`ic_platform::pocket_ic`; the server remains pinned to PocketIC 16.1.0.
Wasm profiles exclude the native client and retain the executor singleton.
No application consumer checkout or server pin changed. Release groups,
extraction boundaries, Rust 2024 and MSRV 1.91 remain unchanged.

## Qualification

The complete local Linux CI and Rust 1.91 release gate passed on source
`3bf5f486` before version preparation. It qualified all ten normalized public
archives, native feature boundaries, installed PocketIC behavior and deployed
Wasm compilation. The selected two-package publication then passed its separate
dry run, clean-source/remote provenance admission, verification, upload and
independent index/archive readback.

Hosted evidence is separate from local Linux. On source PR head
`08c30083ca83d766fcdc9d6f64775da9b414e142`, thirteen of fifteen checks had passed
at delivery, including [native Linux](https://github.com/dragginzgame/ic-tooling/actions/runs/41332463631/job/124925284822),
MSRV, lint/security, browser, optimizer and PocketIC concurrency workflows.
Full [macOS 15 Intel](https://github.com/dragginzgame/ic-tooling/actions/runs/41332463631/job/124925285006)
and [macOS 15 Apple Silicon](https://github.com/dragginzgame/ic-tooling/actions/runs/41332463631/job/124925284834)
qualification were still running asynchronously; neither is a recorded pass.
Consult those jobs for later results. The PR head and merged source share tree
`a2832443e253447b52f5d560b8f69987c26e9e4b`. Release commit `94ca12bf` changes only
selected group metadata; its CI is separate from this source observation.

## Ordinary checkout and durable evidence

At synchronization, the ordinary checkout contained the released versions and
only Cargo.lock remained dirty. All 458 incoming external records, including the
unrelated Thiserror 2.0.21 selections, were preserved exactly. Only the four
Testkit-family local package versions changed. Full locked offline metadata
passed. The preservation stash is
`bc5e544bb9adfea1e0b0a0aac77945ca573321d2`; do not apply its complete old metadata
patch over the delivered versions. Subsequent independent local edits remain
outside this release's source and qualification.

Durable evidence is under `.git/release-state/deployment-testkit-0.78.0/`:
release plans/receipts, complete gate and publication logs, downloaded registry
archives, native CI observations, lock-preservation records and checksum-verified
`product-evidence.tar.gz` excluding compilation caches. `release-checkout-path`
identifies the retained isolated checkout reused from the previous deployment;
its build artifacts remain intact. No artifact cleanup was performed.

The [previous handoff](https://github.com/dragginzgame/ic-tooling/blob/9e53f990f3df638b1c9f51d0ed008fc61cf04991/docs/status/current.md)
preserves the completed 0.77 delivery, its full native qualification and older
recovery coordinates. Existing release ledgers and durable records remain intact.
The reviewed Shared snapshot remains 152 files at
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a` (VERSION/export 0.27.21).
