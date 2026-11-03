# Current handoff — 2026-11-03

## Wire adoption prepared

Local source selects published Wire/Core and derive 0.16.0. The registry archive
checksum is verified; Core's only runtime source change from 0.15.1 is its optional
Platform schema adapter. Tooling retains its existing primitive/local probe DTOs,
method IDs, limits and feature boundaries without selecting that adapter.
Testkit exposes Wire `Limits`/`Error` identities, so pending notes select a
Testkit 0.82.0 minor and require matching direct Wire 0.16 dependencies. Package
versions and registry publication remain at the delivered state below.

All 466 non-Wire package records in the incoming lock remain unchanged. Native
and Wasm affected-package Clippy, nine Wire tests, the protocol-bound test, live
PocketIC calls and the rebuilt Wasm benchmark probe pass. Native/Wasm graph,
manifest-formatting and document-link checks pass. Evidence is retained under
`/tmp/ic-tooling-wire-adoption.RCFp41/`. Native macOS qualification of this local
Wire update has not run.

## Platform delivery

[PR #317](https://github.com/dragginzgame/ic-tooling/pull/317) merged Platform,
PocketIC and Runtime 0.19.0, with Types 0.18.0, at
`259e22288e039fcfa758b7d2cb66a3c720e5141a`. Tools 0.79.0 and Testkit 0.81.0
are published. Their minor releases expose the new canonical types and upstream
`ClientError`; direct consumer dependencies must follow the selected family.

Testkit removed panic-string adapters and returns typed failures through calls,
installation, diagnostics, snapshots and bounded startup. Simulated progression
and install cooldowns are fallible; only explicit rate-limit rejections permit
another installation attempt. Unrelated application panics propagate. The
PocketIC server remains 16.1.0. That release lock retained Wire 0.15.0 without its
optional `platform-types` feature; exercised graphs contain one Types 0.18 owner.
The ordinary checkout's incoming Wire 0.15.1 selection is advanced above while
other unrelated lockfile selections remain preserved. No old Account schema
compatibility is implied by that feature choice.

## Published groups

| Group | Version | Release source |
|---|---|---|
| Foundation | 0.74.2 | `aa62709bff680ac7c8a050c88cff769c818eebf2` |
| Build | 0.74.1 | `aeff79aa1b6d3e6466b7f990b679db9379215466` |
| Text | 0.74.1 | `df9d9cdffe8d56fb82c3132a4860d2567d6f01a7` |
| Tools | 0.79.0 | `c4b6b9730afa1747bc6f59b7018785b57da6eec1` |
| Testkit | 0.81.0 | `4b023d5e3b2cdac0424f17d37b2bacc3f186c44a` |

Native remains 0.80.0. Build's independent 0.74.2 notes remain pending; this
Platform deployment selected Tools and Testkit. Private Dev/probe versions follow
Testkit 0.81.0 and are excluded from registry uploads. Shared remains 0.27.27 at
`651f5f7acb2e3055abd51ba5f835a2112801666a`, with 159 managed files.

## Qualification and custody

Both groups passed their complete 35-obligation local Linux release gates,
including workspace Clippy, isolated profiles, native tests, real PocketIC,
deployed Wasm builds, installed archives, docs and release/rollback fixtures.
Git release and registry upload used separate standard commands. Each uploaded
archive's registry checksum and clean VCS source were independently read back.
Earlier focused adoption evidence includes real snapshots/held replies, the
benchmark probe and corrected fallible startup doctests.

Native macOS 15 Intel/Apple Silicon qualification of this delivery remains with
asynchronous hosted CI; local Linux results do not establish it. Historical
three-host Rust 1.99.1 proof for closed #313 belongs to source
`fc66f67b241af549034ac993c1254b46ab378934` and
[CI 41521867036](https://github.com/dragginzgame/ic-tooling/actions/runs/41521867036).

Deployment logs, incoming backups, registry archives and exact readbacks remain
under `/tmp/ic-tooling-platform-deploy.BGZn8d/`; durable receipt copies are retained in the ordinary
checkout's `.git/release-state/deployment-platform-20261103/`. Original adoption
checks remain under `/tmp/ic-tooling-platform-adoption.WgLoB2/`. The refused
external-target preflight is retained; successful releases used standard target
paths. Incoming source/lock changes were also preserved in a named Git stash.

## Issue evidence

[PR #316](https://github.com/dragginzgame/ic-tooling/pull/316) removed #314's
hard-coded Text fixture version. Group plans retain all obligations for Tools,
Testkit, Foundation, Build and uncertain/shared changes; Native/Text narrow only
from a complete admitted three-host baseline. Receipts bind source, environment,
executed/excluded stages and recovery evidence. The #314 closure checker follows
main and requires actual full/focused native steps and retained artifacts before
closing; its status is under the adoption evidence's `issue-314-closure/`.

[Platform #134](https://github.com/dragginzgame/ic-platform/issues/134) shipped
typed failures and is closed. [#136](https://github.com/dragginzgame/ic-platform/issues/136)
tracks bounded snapshot transfer; this deployment retains the metadata/data
transfer implementation required by the selected 0.19 client. The
private arithmetic exception remains in AGENTS.md, coordinated through
[Shared #347](https://github.com/dragginzgame/shared-tooling/issues/347) and
[Metrics #127](https://github.com/dragginzgame/ic-observability/issues/127).
