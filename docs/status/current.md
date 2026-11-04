# Current handoff

## Delivered Platform 0.22 adoption — 2026-11-04

Platform 0.22.0 is adopted through source `2d744d04`, with subsequent release
fixture/portability repairs through `6ebce02c`. The ordinary checkout remains
on main; no additional implementation or release checkout was created.

| Group | Published version | Release commit |
|---|---|---|
| Tools | 0.82.0 | `1163ee6614f809c17fec1dc0398ffa8c5d69c7ec` |
| Testkit | 0.85.1 | `3d371d7e948dd4e34307a81f7a19ff85e1c5bc45` |

Both commits and annotated group tags are pushed. Both public packages are live
on crates.io. Private Dev/probe members follow Testkit 0.85.1 without publication.
Each group passed its complete 35-stage Linux release gate, followed by separate
publication dry run/upload and independent index/archive checksum, upload-byte,
exact VCS and license verification. Testkit's imported provenance files also match
byte-for-byte. A registry-only consumer ran with both new packages and Platform
0.22, proving direct certificate/tree and management DTO exchanges.

Tools' certificate/tree values now share upstream `ic-certification` identity
with native Agent. Siblings adopting Platform 0.22 select Host Tools 0.82;
`^0.81` does not include it, and 0.81.1 was never published. Testkit's native
Candid/management adapter update preserves its client/DTO/error identities.
Canonical Types remains 0.18 and Runtime remains 0.19. The exact management DTO
pin remains 0.11.0. The [group ledgers](../../CHANGELOG.md) own upgrade notes.

## Validation and native evidence

Focused Linux adoption evidence covers 30 certificate/Agent tests, a management
DTO round trip and an installed PocketIC 16.1 management/snapshot test, alongside
certificate-profile and Testkit native/Wasm graph checks. Both complete release
gates additionally passed archive, runtime, feature, documentation and lint checks.

Native macOS evidence is separate: optimizer, PocketIC runtime and browser
qualification passed on macOS 15 Intel and Apple Silicon, alongside Linux, at
adoption source `fc9940b8e0da66e62039abec4cb5685eeba16327`:
[optimizer 41832289531](https://github.com/dragginzgame/ic-tooling/actions/runs/41832289531),
[runtime 41832294502](https://github.com/dragginzgame/ic-tooling/actions/runs/41832294502),
and [browser 41832298416](https://github.com/dragginzgame/ic-tooling/actions/runs/41832298416).
Full native CI at that source exposed a Bash 3.2 empty-array failure in the
validation fixture. The corrected wrapper is delivered at `6ebce02c`; final-source
full CI is dispatched asynchronously after the handoff push. Record its actual
run SHA/URL in deployment evidence; do not relabel earlier runs as final-source CI.

The consumer release fixtures now isolate inherited full-CI/release admission
settings, parse admissions with portable extended expressions and retain useful
failure diagnostics. The stage wrapper always supplies a nonempty argument array
for Bash 3.2. Focused routing, strict-POSIX-sed reproduction, source drift and
receipt custody checks passed. Product effects in adapter fixtures remain
substituted; the independent complete release gates establish Linux execution.

## Lockfile and release evidence custody

Incoming unrelated lock selections were excluded from release commits/uploads.
Unexpected later lock updates were preserved as `post-qualification-Cargo.lock`
and `post-testkit-Cargo.lock` under
`.git/release-state/deployment-platform022-20261104/`, with patches and earlier
incoming bytes. The latest selection is `post-testkit-Cargo.lock`; restore it with
only the three Testkit group versions aligned after source-sensitive delivery.

The user confirmed no concurrent writer. Traced locked/unlocked metadata and the
complete focused Testkit adapter made no writes to the real source lock. A passive
process/hash monitor observed only expected group version preparation during the
successful retry and was stopped afterward. The earlier writer remains
unidentified; no editor process or configuration was changed. Compact observations,
receipts, registry readbacks and publication state are in that Git-side directory.
Bulk release logs, archived packages, registry consumer and diagnostic traces are
under `target/deploy-platform022-20261104/`. Failed attempts remain preserved.

## Earlier cleanup and package delivery

The test-layout and complexity cleanup is integrated at `c53f0595`. Shared Tooling
0.29.4 is adopted from `628ccc75d813bfd9bae777bfbaef55838d25b1b2`; all 164 managed
files match. Foundation 0.74.3, Build 0.74.3 and Text 0.74.2 remain published;
Native remains 0.80.0. Testkit 0.85.0 supplied the preceding client cleanup.

Inline tests moved into ordinary test modules without widened production
visibility. Runtime forwarding and cache failure finalization are simpler. The
maintained client requires an explicit server URL: `new(url)` and
`list_instances(url)` replace implicit discovery. `pic` retains provisioning and
managed startup. A separately checksummed maintenance patch reverses before the
original donor/upstream reconstruction, retaining licenses, protocol/crypto
identities and fallible/no-replay/snapshot contracts.

Releases select one adapter group; full CI retains all six and the 35-obligation
roster. Shared's runner reuses exact retention-prefix input matches with fresh
source/group admission and original proof/log custody. Rust and archive stages
still execute. See [release validation](../design/release-validation.md).
The pending Build ledger records Git-delivered pipeline changes without another
crate release. Earlier evidence and named lock stashes remain under
`.git/release-state/deployment-cleanup-20261104/` and `target/deploy-20261104/`.

The obsolete nested release checkout was reviewed and removed with explicit user
approval. Its 760 KB of completed plans/receipts remain byte-for-byte under
`.git/release-state/deployment-tools-0.77.0-testkit-0.77.0/historical-release-records/`,
alongside ref/status inventories and the review record. No other artifact cleanup
was performed.
