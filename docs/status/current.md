# Current handoff — 2026-11-01

## Published Integration 0.76.0

The authorized Platform adoption is released and published:
`integration-v0.76.0`, commit `2973946c6f7dc13552af78f887d5e2a8dbf37a61`.
`ic-host-tools`, `ic-testkit` and `ic-testkit-native` are independently confirmed
live and non-yanked in the official Cargo index. Uploaded and downloaded registry
archive checksums match. Each published manifest requires Platform `^0.16.0` with
its defaults disabled. Private Dev/probe versions are 0.76.0 and remain unpublished.
Foundation stays at published **0.73.4**; none of its seven packages was republished.
No further release increment or upload is pending. Do not replay delivery effects.

[PR #295](https://github.com/dragginzgame/ic-tooling/pull/295) adopted Platform
**0.16.0**, including matching macros/runtime guard, and arithmetic-only Metrics
**0.24.4**. Wire/Derive **0.13.2** and canonical Types **0.15.1** remain unchanged.
Edition 2024, Rust 1.91 and existing capability/lifecycle boundaries are preserved.

This is an integration minor because Tools' public certificate/hash-tree return
identities now belong to Platform. Consumers align Platform dependencies; native
Agent callers enable `agent` and use Platform's explicit `From`/`Into` conversions.
Strict CBOR admission, encoded bytes and caller-owned authentication are unchanged.
Certificate-only profiles exclude registry `ic-certification`; native Agent keeps
its separate external dependency. Metrics remains arithmetic-only, without native
metadata or Wire entering Testkit's production graph.

## Authorized delivery — Foundation 0.73.5 / Integration 0.76.1

[PR #296](https://github.com/dragginzgame/ic-tooling/pull/296) is merged as
`7c0f9956`, adding caller-selected checks for already-produced archives while
preserving the convenience API's packaging and test roster. Its 15 checks passed,
including Linux, macOS 15 Intel/Apple Silicon and MSRV. Foundation's compatible
patch is 0.73.5. Integration's compatible Wire adoption follows as 0.76.1.
Release and publication of both groups are authorized, Foundation first, through
the standard separate commands and complete gates. Preserve failed evidence and
leave hosted CI asynchronous; no publication has occurred for these candidates.

The local root catalog now selects published Wire/Derive **0.13.3**. The incoming
lock already selected both versions and is preserved byte-for-byte; its backup
and check logs are under `/tmp/ic-tooling-wire0133.QzBFuP/`. Existing public types,
byte-array representations, frames, budgets and feature boundaries are unchanged,
so notes select a compatible patch. Own versions await normal release preparation;
the Wire source is being delivered by PR. Platform/Types and Metrics are unchanged.
Focused Linux Clippy, nine Wire transport tests and one probe contract test on
Rust 1.91, the Rust 1.91 probe Wasm build and Testkit graph policies pass.
Native macOS qualification has not been rerun for this local Wire adoption.

## Qualification and asynchronous CI

The standard complete local Linux CI and Rust 1.91 release gates passed on merged
source `cf75aa44b5d5d3f4d96505a648b41e616c47de6b`, before normal group metadata
preparation. All ten public archives qualified, including tests and documentation;
installed stock PocketIC 16.1 checks passed. The separate Integration dry run,
clean-source admission, remote provenance fetch, package verification and uploads
then passed at release commit `2973946c`. Git release and publication were separate.

Native macOS evidence is separate from Linux execution. At 0.76.0 delivery, the
[source PR matrix](https://github.com/dragginzgame/ic-tooling/actions/runs/41296300109)
had passed native Linux and macOS 15 Apple Silicon; **macOS 15 Intel was still in
progress**. Source MSRV and lint/security passed. Separate browser-session,
PocketIC concurrency and optimizer workflows passed on all three hosts. Those
separate jobs do not replace complete native qualification. Hosted CI continues
asynchronously; pending or cancelled jobs are never counted as passes.

PR head `8cbf71d7` and merged source `cf75aa44` have identical tree
`4256e6207e21ff1fd0238268a4dfbce2186d5e75`. The release changes only selected
Integration version/catalog/lock/changelog-date metadata. Later release-head or
handoff CI is distinct evidence, not another completed source qualification.

## Checkout, evidence and recovery

The ordinary checkout retains Foundation 0.73.4 / Integration 0.76.0 package
versions, with the pending Wire changes above. This delivery's incoming work and
original lock are backed up under `/tmp/ic-tooling-deploy0735-0761.ZATSzj/`.
At 0.76.0 delivery, locked offline
metadata passed and the tree was clean; incoming source remains in a recorded
stash, and unrelated lock records were preserved. Earlier Metrics/Platform lock
selections were incorporated into the authorized adoption. Do not apply a complete
saved patch over the released metadata.

The canonical completed record is
`.git/release-state/deployment-integration-0.76.0/delivery-result.json`.
It retains release receipts, source metadata, publication/provenance evidence,
registry confirmations, upload/readback archives, source qualification and work
backups. `product-evidence.tar.gz` has a verified adjacent SHA-256 and excludes
build caches. Original logs, artifacts and the pinned release checkout remain
under `/tmp/ic-tooling-integration0760.iALuHh/` (`release/`). Preserve this context.

The prior Foundation record remains
`.git/release-state/deployment-foundation-0.73.4/delivery-result.json`; its original
checkout and artifacts remain under `/tmp/ic-tooling-foundation0734.md3B6O/`.
[Earlier handoff](https://github.com/dragginzgame/ic-tooling/blob/9022ac3c912daa846dc34c86dab51dc2e889155c/docs/status/current.md)
preserves Foundation qualification, prior stashes and older recovery references.
YAML stays on maintained `yaml_serde` / `libyaml-rs`, not a memory-safety rewrite.
Tooling issues #285, #286 and #287 are closed; [Canic #719](https://github.com/dragginzgame/canic/issues/719)
owns distinct downstream YAML adoption. The PocketIC client takeover in Platform
is separate from this release.

The Shared snapshot remains 152 files at
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a`: upstream's commit label is 0.27.22,
while VERSION/export declare 0.27.21. Preserve exact payloads. Independent
[release groups](../design/release-groups.md) retain the [extraction contract](../extraction.md).
Do not reinterpret old journals under new policy or clean artifacts implicitly.
Plain `cargo clean` retains its normal whole-workspace target scope.
