# Current handoff — 2026-11-03

## Platform 0.20 adoption — local

The working tree selects published Platform and independent PocketIC client
0.20.0. Runtime remains 0.19.0, canonical Types 0.18.0 and Wire 0.16.0;
the installed PocketIC server remains 16.1.0. Only the two requested package
records changed relative to the incoming lockfile; unrelated selections remain.
Tools 0.80.0 and Testkit 0.83.0 have pending minor notes for exposed dependency
identities and the additional client-error variant. Package versions are unchanged.

Platform's bounded snapshot API is available, but this adoption preserves the
existing Testkit metadata/data transfer behavior and caller-selected limits.
Focused adoption evidence and incoming backups are retained under
`/tmp/ic-tooling-platform020-adoption.jRLpyl/`. Native/Wasm graphs, affected-package
all-target Clippy, Tools' Wasm adapter Clippy, declaration policy and 20 certificate,
public-contract and live PocketIC snapshot/timer/call tests passed on Linux.
Native macOS qualification and release/publication of these local changes have
not been performed.

## Wire delivery

[PR #319](https://github.com/dragginzgame/ic-tooling/pull/319) merged Wire/Core
and derive 0.16.0 at `400335ce3a43e43945027bb4e16d2ccd77f010aa`.
Testkit **0.82.0** is published from `e644afe09f0fa9ef248db6f98b3efa461de9f890`.
Direct consumer dependencies must select Wire 0.16 when supplying Testkit's
exposed `Limits` or matching Wire error causes. Private Dev/probe versions
follow 0.82.0 and remain excluded from registry uploads.

Tooling retains primitive/local probe DTOs, method IDs, limits and feature
boundaries without selecting Wire's optional Platform schema adapter. The
checksum-verified Core archive changes only that adapter's Rust source from
0.15.1; derive's Rust source is unchanged. Account byte/contract migration remains
with Account consumers; this is not an old-Account reader or automatic migration.
That delivery selected Platform, PocketIC and Runtime 0.19 with Types 0.18.

The complete 35-obligation local Linux release gate passed on the scoped locked
source, including installed archives and live PocketIC/Wasm consumers. Git
release and registry publication used separate standard commands. Independent
registry checksum and clean VCS-source readback passed. Native macOS 15
Intel/Apple Silicon qualification of this delivery remains asynchronous.

Incoming registry lock selections remain preserved; only owned workspace package
versions advance in the ordinary checkout. Source and lock backups, release and
publication logs, readbacks and the incoming stash reference are retained under
`/tmp/ic-tooling-testkit082-deploy.DsxSsd/`; durable copies live in
`.git/release-state/deployment-testkit082-20261103/`. The reused isolated
checkout and its original gate evidence remain under
`/tmp/ic-tooling-platform-deploy.BGZn8d/release/`. Focused adoption checks
remain under `/tmp/ic-tooling-wire-adoption.RCFp41/`.

## Platform delivery

[PR #317](https://github.com/dragginzgame/ic-tooling/pull/317) merged Platform,
PocketIC and Runtime 0.19.0, with Types 0.18.0, at
`259e22288e039fcfa758b7d2cb66a3c720e5141a`. Tools 0.79.0 and Testkit 0.81.0
shipped that adoption. Their minor releases expose the new canonical types and upstream
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
| Testkit | 0.82.0 | `e644afe09f0fa9ef248db6f98b3efa461de9f890` |

Native remains 0.80.0. Build's independent 0.74.2 notes remain pending; this
Platform deployment selected Tools and Testkit. Private Dev/probe versions follow
Testkit 0.82.0 and are excluded from registry uploads. Shared remains 0.27.27 at
`651f5f7acb2e3055abd51ba5f835a2112801666a`, with 159 managed files.

## Qualification and custody

The earlier Tools 0.79.0/Testkit 0.81.0 groups passed complete local Linux gates,
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
executed/excluded stages and recovery evidence. Routine main pushes repeatedly
cancelled the slower native comparisons. Explicit qualification dispatches now
finish independently while routine CI retains newest-revision cancellation.
Qualification must bind one source across all four workflows and retain actual
full/focused native steps and artifacts; later main pushes do not erase completed
ancestor evidence. See the [qualification procedure](../design/release-validation.md).

[Platform #134](https://github.com/dragginzgame/ic-platform/issues/134) shipped
typed failures and is closed. [#136](https://github.com/dragginzgame/ic-platform/issues/136)
shipped bounded snapshot transfer in the 0.20 client; Testkit's existing helpers
still use their metadata/data transfer implementation. The
private arithmetic exception remains in AGENTS.md, coordinated through
[Shared #347](https://github.com/dragginzgame/shared-tooling/issues/347) and
[Metrics #127](https://github.com/dragginzgame/ic-observability/issues/127).
