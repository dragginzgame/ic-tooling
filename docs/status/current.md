# Current handoff — 2026-11-02

## Authorized five-group delivery

This Tooling session has explicit authorization to qualify, release and publish
Foundation 0.74.0, Build 0.74.0, Text 0.74.0, Tools 0.78.0 and Testkit 0.79.0,
sequentially. No Platform, Blob, simulator, npm or application release is included.
Source integration is committed as `f6f6e7d9`; unrelated Cargo.lock selections
remain dirty in the ordinary checkout. Delivery uses the isolated checkout named
by `.git/release-state/deployment-five-groups-20261102/release-checkout-path`.
That directory retains source/lock preservation records, setup and gate logs.
The formatting fixture now keeps Foundation's labelled first entry fixed.

The first Foundation validation was interrupted on request before preparation
to adopt Shared Tooling 0.27.27. It changed no versions and uploaded nothing.
Group-scoped validation remains opt-in until Tooling qualifies its stage mapping;
this cross-cutting compiler/release batch retains the full gate.

## Local Shared Tooling and compiler adoption

The working tree adopts Shared Tooling 0.27.27 at
`651f5f7acb2e3055abd51ba5f835a2112801666a` through the canonical exporter;
all 162 managed payloads verify. Common tool pins are unchanged. The temporary
revision-bound policy override in AGENTS.md is superseded by the installed core.
No sibling working checkout was edited.

[Tooling #313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313)
is implemented locally. All twelve members inherit Rust 1.99.1; the existing
development pin is also 1.99.1. Compiler admission reads those local declarations
and requires matching actual Cargo/rustc identities. Qualification records retain
the selected compiler policy. The obsolete atomic compatibility suppression is
gone; the replacement's concurrent/saturating reader test passes.

Native CI and release verification now own one compiler's native, isolated-feature,
Testkit Wasm and unsupported-filesystem checks. The old minimum-compiler build/cache
lane and Make targets are retired. GitHub's `msrv` status name remains as a small
gate on classification and all native results; it performs no duplicate build.
The main branch had no applicable rules or classic required-check protection at
inspection; no repository settings were changed. Hosted integration remains async.

The earlier cleanup remains present. Foundation, Build and Text 0.74, Host Tools
0.78 and Testkit 0.79 have pending breaking-change notes; package versions remain
unchanged. Integration is committed; no release, upload, tag or push was performed.
Deliver changed suppliers before their consumers when releasing.

Metrics 0.26.0 is adopted locally from the published registry release, matching
Rust 1.99.1. Only its version/checksum changed in the incoming lock. Native Testkit
keeps arithmetic-only Metrics; Wasm excludes it. Strict affected Clippy and all
23 benchmark tests pass; graph/policy and preservation evidence is retained in
`target/metrics-0.26-adoption/`. No native macOS or release qualification ran.

## Independent Build and Text releases

The local release policy now has five groups. Foundation's five lower-level
packages use ic-host-artifacts as primary; ic-build and ic-host-text each own
an independent group and ledger. Tools and Testkit membership is unchanged.
Package versions, dependency requirements and Cargo.lock are unchanged by this
split. The original seven-package Foundation history through 0.73.6 stays in
its ledger; the new Build/Text ledgers link to it. Their pending 0.74 notes
reflect the compiler-floor change above, not mandatory synchronized releases.
Historical release plans and receipts retain their original source and scope.

Focused split evidence is in `target/release-group-split/`: five-group real-editor
release/rollback and receipt-isolation fixtures, publication selection/refusal
fixtures, Make routing, unchanged metadata/lock/history checks and documentation
and policy checks. These fixtures substitute qualification and delivery effects;
no release ran and native macOS qualification was not performed for this split.
The initial preservation comparisons passed. Separate Platform dependency edits
then changed the catalog, Testkit and lockfile; these remain intact. Current-tree
locked metadata, package identities, Testkit admission, pins and sorting pass too.

## Focused evidence and preservation

Evidence is in `target/shared-tooling-0.27.26-adoption/`: exporter impact/refresh,
workspace metadata and locked compilation of all targets/features, native/Wasm
and isolated-feature checks, Build Clippy, the atomic reader test, compiler/source
record refusal fixtures, actual CI-selection/Make-routing fixtures, formatting
hooks, release-adapter fixtures, workflow lint, snapshot/pin/sort/doc checks and
branch-rule observations. The real release editor was used by the fixture;
installation, full release qualification and delivery effects were substituted.
No full repository test suite or release gate was run for this adoption.

Offline explicit setup reselected cached cargo-sort 2.1.4, cargo-sort-derives
0.14.1 and IC Build's release editor 0.73.0 under the new admission policy.
It downloaded nothing. Workflow lint used the already-installed, catalog-matching
Actionlint 1.7.12 through the supported Make override. Native macOS qualification
was not run for this batch; Linux and Wasm evidence do not establish it.

All 458 incoming external Cargo.lock records and the incoming root-manifest
spacing remain intact. The earlier cleanup evidence is retained under
`/tmp/ic-tooling-cleanup-20261102/`, including its incoming patch, final source
patch and the failed test's misplaced cache, moved there without deletion.
This adoption's incoming source patch and lock copy are in its evidence directory.

## Remaining supplier work

[Platform #134 — Return typed transport errors from the maintained PocketIC Rust client](https://github.com/dragginzgame/ic-platform/issues/134)
remains unimplemented. We maintain the Rust frontend in `ic-platform-pocket-ic`,
exposed through `ic_platform::pocket_ic`; its first transfer preserved signatures.
Testkit keeps its panic adapter until the fallible supplier is implemented,
published and adopted. The server stays at PocketIC 16.1.0; no server fork or
Platform publication is part of this work.

## Completed published baseline

| Group | Published version | Release commit |
| --- | --- | --- |
| Foundation | 0.73.6 | `ffafc54b10f1de750fe92526c49aa0ea70e4e124` |
| Host Tools | 0.77.0 | `d2b14582ab0b6dcb3ef6e72094b8f7da4b195c1a` |
| Testkit | 0.78.0 | `94ca12bfb5f83b45d615e6d2c51887db7881b659` |

All ten public packages are live; the two private testing packages were not
uploaded. Testkit 0.78 publication and independent archive/index readback are
complete: do not replay its increment or upload. Its Wire 0.14 and arithmetic-only
Metrics 0.25 adoption retain Platform 0.17.0, Runtime 0.17.1 and Types 0.16.0.
The new compiler floor above describes pending source, not rewritten old archives.

Durable delivery evidence remains in `.git/release-state/deployment-testkit-0.78.0/`:
plans/receipts, gate/publication logs, registry archives, native CI observations,
lock-preservation records and checksum-verified compact evidence. Its
`release-checkout-path` identifies the retained isolated delivery checkout.
The original preservation stash is `bc5e544bb9adfea1e0b0a0aac77945ca573321d2`;
do not apply its complete old metadata patch over delivered versions.
The [prior delivery handoff](https://github.com/dragginzgame/ic-tooling/blob/405b27ad13d95962d729b5e756634ee866c760a8/docs/status/current.md)
preserves the source/CI coordinates and older recovery details. These completed
delivery records do not qualify the current uncommitted changes.
