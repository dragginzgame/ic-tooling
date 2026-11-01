# Current handoff — 2026-11-01

## Pending Shared retention adoption

The isolated adoption selects Shared 0.27.26 source
`a2b3c22fe20d4913803244c8e754824ebfc401bf` from
[Shared PR #341](https://github.com/dragginzgame/shared-tooling/pull/341).
Common tools and the existing release-editor adapter keep their current pins.
The new snapshot trims successful tool-build scratch and supplies explicit
[store maintenance](../tool-store-maintenance.md). No live store or sibling
working checkout was cleaned. Package versions and lock selections are unchanged.

Focused snapshot/link/dependency checks and the three-group release-adapter fixture
pass. Actual Linux editor builds at 0.73.0 and 0.73.1 pass offline admission with
successful compiler scratch absent. An isolated two-consumer refresh/reclamation
case retires the obsolete generation and handles while preserving the peer's
selection. Full product gates and native macOS qualification remain separate.

## Published Foundation 0.73.6 / Tools 0.76.2 / Testkit 0.76.2

All ten public crates are released and published in supplier-first order:

| Group | Version | Release commit | Public packages |
| --- | --- | --- | --- |
| Foundation | 0.73.6 | `ffafc54b10f1de750fe92526c49aa0ea70e4e124` | Artifacts, FS, HTTP, Process, Text, Build, Wasm inspector |
| Host Tools | 0.76.2 | `924a7eca13daf67682157b0b45bd2f213625657f` | Host Tools |
| Testkit | 0.76.2 | `09f7b3d199b5fc7036dc2ea255ba2bac97e93a62` | Testkit, Testkit Native |

Tags are `foundation-v0.73.6`, `tools-v0.76.2` and `testkit-v0.76.2`.
Private Dev and perf probe versions are 0.76.2; neither was uploaded. Independent
Cargo-index checks confirm every public version live and non-yanked. Downloaded
registry archives match the actual upload checksums. There is no pending upload
or increment; do not replay these completed delivery effects.

## Delivered contracts

[PR #302](https://github.com/dragginzgame/ic-tooling/pull/302), merged as
`4047979dea917e22c9b5f48f943f25ce0f038490`, implements
[#299](https://github.com/dragginzgame/ic-tooling/issues/299) and
[#300](https://github.com/dragginzgame/ic-tooling/issues/300).

Host Tools and the Testkit family now release independently. Foundation retains
its seven-package boundary; Tools owns only Host Tools; Testkit owns Testkit,
Native and the private Dev/probe packages. See [release groups](../design/release-groups.md).
The original joint integration ledger remains frozen through 0.76.1, with its
original scope, dates, tags, receipts and recovery context. Never reinterpret old
integration attempts under the new policy.

Build 0.73.6 adds opt-in qualified target cfg recipes and the
`ic-build-rustc-wrapper` CLI. It preserves Cargo's effective flags, refuses
conflicts unless explicitly qualified, and retains bounded compiler identities
and arguments in exact artifact caches. This opt-in requires isolated compiler
state and refuses ambiguous wrappers/custody overrides; ordinary recipes retain
their selected behavior. Post-link work reuses the existing admitted-tool and
transactional cache boundary. [Effective recipes](../build/effective-recipes.md)
records concrete Canic/Auth adoption, constraints and measured final-Wasm results.
No consumer checkout, deployment profile or tool pin was changed. Consumers can
now select the published Foundation library and wrapper under their own policy.

## Qualification

Each group ran the complete local Linux CI and Rust 1.91 gate before version
preparation: Foundation on source `4047979d`, Tools on `ffafc54b`, Testkit on
`924a7eca`. Each gate qualified all ten normalized public archives, feature
boundaries, deployed Wasm and installed stock PocketIC 16.1 behavior. Each separate
publication dry run passed, followed by clean-source/remote provenance admission,
verification, upload and independent index/download readback. Release and registry
publication remained separate commands.

Focused implementation evidence also covers three-group rollback/receipts and
unselected metadata preservation; independent Tools/Testkit source and normalized
archive consumers on Rust 1.99.1/1.91; actual old certificate identity refusal;
real Cargo config/ordinary/encoded flags, conflicts, explicit overrides, native
isolation, profile settings and exact cache reuse. Fresh Platform-maintained
ECDSA vectors passed eight cases with three samples per backend in stock
PocketIC. Compiler/optimizer identities, ordered arguments, raw/final Wasm,
uncompressed module/code bytes and IC instruction counts remain under
`/tmp/ic-tooling-300-qualification-final/` and in the delivery evidence bundle.

Native hosted evidence is separate from local Linux. Source PR head
`ea1e5645d16611a83d1668c96925007b1aa84cb3` and merged source `4047979d` have
identical tree `d402f89883cf9e9ab4a87dc24f67528e4566b6b4`. All fifteen source checks passed, including native
Linux, MSRV, [macOS 15 Apple Silicon](https://github.com/dragginzgame/ic-tooling/actions/runs/41309896915/job/124857910951) and
[macOS 15 Intel](https://github.com/dragginzgame/ic-tooling/actions/runs/41309896915/job/124857910970).
Native macOS execution is separate from local Linux evidence. Later release
commits change only selected group metadata; their CI is separate from this
source proof. #299 and #300 are implemented, qualified and published.

## Ordinary checkout, evidence and recovery

The ordinary checkout contains Foundation 0.73.6 / Tools 0.76.2 / Testkit 0.76.2.
Full locked offline metadata passes. Its only remaining working-tree change is
Cargo.lock: Metrics 0.24.5 and Wire/Derive 0.13.4 were preserved outside this
source batch. Every unrelated lock record matches the incoming local selection;
only the twelve owned package versions changed. Published source retains its
prior Metrics 0.24.4 / Wire 0.13.3 requirements and lock selections.

The original source/index/lock recovery stash is
`762c5098a5c73353616b23d97dd1399e12e7dbf5`. Complete incoming/source patches and working-file archives remain
under `/tmp/ic-tooling-publish-299-300.LIasr2/`. Do not apply the whole saved patch over released metadata.
The isolated release checkout remains at `/tmp/ic-tooling-publish-299-300.LIasr2/release/`; build artifacts
and failed/successful evidence were preserved without cleanup.

The durable completion record is
`.git/release-state/deployment-foundation-0.73.6-tools-0.76.2-testkit-0.76.2/delivery-result.json`.
It retains all three plans/receipts, full gate/publication logs, upload/readback
archives, source qualification, work-preservation records and a checksum-verified
product evidence bundle excluding compilation caches. No publication, cleanup
or consumer deployment should be inferred from that record beyond its receipts.

[Previous handoff](https://github.com/dragginzgame/ic-tooling/blob/09f7b3d199b5fc7036dc2ea255ba2bac97e93a62/docs/status/current.md)
preserves the completed Foundation 0.73.5 / joint Integration 0.76.1 and earlier
delivery/recovery coordinates. Their existing records remain untouched.
The released baseline used Shared `143a9abb9ae81feae9400ff3844b03b1ddfdd75a`
(VERSION/export 0.27.21); the pending adoption above supersedes its tooling files.
Rust 2024, MSRV 1.91 and the [extraction contract](../extraction.md) remain unchanged.
