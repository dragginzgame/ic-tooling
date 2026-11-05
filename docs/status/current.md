# Current handoff

## Platform codec deployment in progress — 2026-11-05

The maintainer authorized Tools 0.83.0 followed by Testkit 0.87.0, using ordinary
`/home/adam/projects/ic-tooling` main, direct origin/main plus group tags and
separate crates.io publication. Evidence is under
`target/deploy-platform023-20261105/`; durable incoming lock/source backups are in
`.git/release-state/deployment-platform023-20261105/`. Hosted CI is asynchronous.

The release lock is derived from committed main with only Platform 0.23.0,
required Types 0.18.1 and removed workspace dependency edges. Runtime and unrelated
patch selections retain the committed release graph. The newer incoming lock is
kept separately and will be restored with only released workspace versions
aligned after publication. Do not overwrite those snapshots or mix their
unrelated selections into the release. Standard release gates remain required.

## Published Platform 0.23 adoption — 2026-11-05

The maintainer requested the published supplier after its codec release. The root
catalog now selects `ic-platform = "0.23.0"` with defaults off. Types and Runtime
requirements remain independently selected. The incoming lock already contained
Platform 0.22.3, Types 0.18.1 and Runtime 0.19.2; the targeted update changes only
the Platform package record. Every non-Platform record is byte-identical to that
incoming lock, retained under `target/platform-adoption-20261105/` alongside the
incoming source diff. Do not replace it with the older deployment snapshots.

Registry metadata and packaged source confirm the published `certification-cbor`
and `ic-protocol` features, including `Result<Option<SubnetMetrics>, CborError>`
with null absence. Platform [#145](https://github.com/dragginzgame/ic-platform/issues/145)
is supplied in 0.23.0. No sibling path or uncommitted supplier is selected.

Linux focused evidence is under `target/platform-adoption-20261105/`: strict
all-target/all-feature Clippy for Tools, Testkit and Dev; 107 Testkit and 58 Tools
library cases; three public-contract cases; management DTO identity; native/Wasm
feature graphs and Testkit Wasm compilation. The existing installed 16.1 client
test now also reads real certified subnet metrics; metrics, management and bounded
snapshot restoration passed together. Six library cases remain explicitly ignored;
the focused checks do not claim their execution. Native macOS qualification and
complete release/archive gates have not run for this adoption. Source changes
remain local; Tools 0.83.0 and Testkit 0.87.0 are pending releases.

## Certificate/protocol owner extraction — 2026-11-05

The maintainer confirmed the in-flight certificate/hash-tree and subnet-metrics
codec extraction as the next batch. Tools' pending minor is 0.83.0; Testkit's
async-only minor remains 0.87.0. Platform owns strict certificate CBOR and typed
read-state packet codecs; Query retains application verification and Testkit
retains HTTP/status handling, tree lookup, optional results and orchestration.
No compatibility certificate feature/module remains in Tools.

The retired certificate test target and stage are removed. The full gate now
has 34 stages; the separate source-bound focused plans still require a matching
three-host comparison. Generic decoder tests belong to Platform. Dev's public
contract checks Testkit's actual subnet-metrics DTO identity. The independent
protocol-codec patch is formatted and checksummed, and mutation coverage includes
that new provenance layer. Original donor/upstream fingerprints are unchanged.
Direct certificate, transport DTO and CBOR declarations are removed; local
dependency policy refuses their reintroduction outside Platform. Focused Make
routing, scope-selection refusal, provenance mutations, declaration boundaries,
formatting, shell lint and documentation links pass on Linux. Evidence is under
`target/platform-codec-preparation-20261105/`; these checks do not establish
runtime, graph, archive or native macOS qualification of the new dependency.

The original 0.22.2 supplier gap and draft's non-optional metrics mismatch were
resolved by Platform 0.23.0 and its adoption above. Platform retains the optional
CBOR leaf grammar; Testkit adds no local CBOR-byte handling or null-as-error policy.

## Delivered groups and interrupted Testkit release — 2026-11-05

| Group | Published version | Release commit / tag |
| --- | --- | --- |
| Foundation (five public packages) | 0.74.4 | `fd4bc9e2` / `foundation-v0.74.4` |
| Build | 0.74.4 | `9ed16c7a` / `build-v0.74.4` |
| Tools | 0.82.1 | `85d45e69` / `tools-v0.82.1` |

All three complete Linux release gates, Git delivery, crates.io publication and
exact archive/checksum/Git-provenance readback passed. Tools 0.82.1 includes #335's
bounded ICP inventory and cycle-balance decoders. Testkit 0.87.0 has no release
commit, tag or upload: the new codec edits changed manifests/Makefile during
archive qualification and the custody guard refused the attempt.

Current deployment evidence is in `target/deploy-ready-20261105/` and
`.git/release-state/deployment-ready-20261105/`. Both earlier lock snapshots are
preserved there; `refreshed-incoming-Cargo.lock` in Git-side state includes the
maintainer's Runtime 0.19.2 selections. That deployment used its committed release
graph. The current Platform adoption preserves a newer incoming lock instead, as
described above; retain both historical snapshots and avoid restoring them over
the current supplier selection.

Input monitoring identified `cargo update -v` from the checkout's VS Code
terminal as the earlier lock writer; the maintainer paused dependency updates.
A diagnostic archive trace recorded no checkout-lock writes, but distorted a pool
test and was stopped with its owned processes; it is not qualification evidence.
The optimizer workflow now retains setup pipeline failures. Two exact historical
Mermaid identifiers have scoped secret-scan fingerprints; the history scan passes.

At published Tools commit `85d45e69`, hosted optimizer, runtime and browser
workflows passed; full native CI remained asynchronous at observation. See
`target/deploy-ready-20261105/native-ci-status.json`. These results do not qualify
the new, uncommitted extraction. No new native macOS pass is claimed for that batch.

## Tools/Build focused qualification — 2026-11-05

The maintainer requested focused Tools and Build release plans. Their selected
owner/consumer commands, per-group omissions, archive dependency closure and
conservative fallback are implemented. Local evidence is under
`target/group-validation-20261105/`; matched hosted comparison remains required
before these scopes can narrow a release. The CI profile list comes from the
validated coverage policy. Foundation/Testkit, Shared refreshes and mixed-group
changes retain full validation. See [the qualification contract](../design/release-validation.md).
Host Tools #335 was subsequently integrated and published in 0.82.1. The current
codec extraction changes this mapping and requires fresh qualification.

## Earlier Shared adoption and async-cut attempts — 2026-11-05

The maintainer authorized deployment of the reviewed batch based on `1401a36f`.
The selected order is Foundation 0.74.4, Build 0.74.4, then Testkit 0.87.0 from
ordinary main, through the standard release and separate publication adapters.
Reviewed source and fixture fixes are pushed through `6e6475a8`. Release
qualification stopped on concurrent source edits; no new publication is
claimed here. Current attempt evidence is under `target/deploy-async087-20261105/`.
Unrelated incoming Cargo.lock selections are saved there and will be restored
after release; the release lock includes only the async cut's required changes.

Shared Tooling 0.30.0 at `891c73694974e54c920472f896a047647273ae44` is adopted
through the canonical exporter, matching GitHub main when selected. All 170
managed files match the selected commit. This refresh adds the LOC table's short
workspace label and removes the duplicate single-workspace total; its focused
evidence is under `target/shared-adoption-github-20261105/`.
The [completed rule audit](../reports/audits/2026/11/05/shared-tooling/01/report.md)
records inspected surfaces, fixes, focused evidence and retained exceptions.
Four private test-helper owners now use conventional test paths. Both remaining
Rust allow attributes became checked expectations. Private package roles make
LOC schema 2 distinguish tooling, benchmarks and tests; public feature-selected
`testing` APIs remain library code. Package names, features and APIs are preserved
by this adoption. Optional release isolation/reuse and schedules were not enabled.

Audit follow-up lives only in GitHub: [#333](https://github.com/dragginzgame/ic-tooling/issues/333)
owns inherited PocketIC global logging setup;
[#334](https://github.com/dragginzgame/ic-tooling/issues/334) owns target-directory
defaults for generated qualification fixture scratch. No historical cleanup is
included. The delivered-groups section above records subsequent release evidence;
new native macOS qualification of the pending extraction remains separate.

## Async-only Testkit contract

Testkit exports one async client at `pocket_ic::PocketIc`; the imported async
implementation retains its private donor file paths. Blocking facade, runtime
threads/channels, public `nonblocking` path and destructor network I/O are removed.
The caller owns the Tokio runtime. Builders, network/lifecycle/Wire helpers and
recipes are awaited; explicit instance/baseline/pool close retains server custody.
Tokio owns FIFO capacity; cancellation during pool preparation forces rebuild
before reuse. Pure fixture generation and command execution remain synchronous.
The private driver retains one multithread runtime across worker handles and
measures explicit close. Reused clients and pools must not outlive their runtime.

Protocol bytes, server 16.1, external DTO identity, typed failures, no-replay and
bounded snapshot transfer are unchanged. The exact management DTO pin is 0.11.0,
aligned with Platform 0.22; Types 0.18, Runtime 0.19 and Wire 0.17 remain selected.
Original donor/upstream hashes and Apache notices are unchanged; source guards
independently reconstruct the maintenance patch. The
[migration guide](../testkit/client-migration.md) owns consumer instructions.
Canic and IcyDB need async caller adoption before upgrading; no sibling source
was edited.

Linux async evidence is under `/tmp/ic-tooling-async-cut/`: 107 Testkit unit cases,
65 ordinary integration cases, explicit installed lifetime/management/snapshot
cases, isolated dead-server recovery and real CLI subprocess checks passed.
Private driver, canister probe and two-worker fresh/pooled smoke checks passed.
Strict Clippy, doctests, native/Wasm graphs, Wasm compilation, provenance mutation
checks, formatting and links passed. An earlier stalled live-pool attempt is
retained; its task-owned process was stopped. The caller-runtime ownership fix
passed the fresh integration run. These are focused checks, not a release gate.

Current adoption evidence is under `target/shared-adoption-20261105/`: snapshot
impact/refresh, LOC inventories, strict affected-owner Clippy with all features
and minimal Build/Testkit selections, 440 owner/subsystem tests and one installed
ownership/install-recovery test. The initial sandbox-denied socket attempt is
retained separately from the passing native-access run. See the audit for exact
commands and scope; no new native macOS evidence is claimed.

## Preserved lock state and release evidence

The async cut's original incoming lock and the Shared adoption's incoming
lock/diff/handoff remain in their original evidence directories. The completed
release batch used only its admitted dependency graph; unrelated maintainer
selections remain in the Git-side backups described above. Do not include them
implicitly in supplier adoption or overwrite them while updating release versions.

Last published Testkit is 0.86.0: release commit
`71de6abfc21d8a26d33b3ac485499890654513db`, tag `testkit-v0.86.0`, preceded by
Wire adoption `0a70714040588e2367d62ef4af1859f191e6a0de`. Foundation/Build 0.74.4,
Tools 0.82.1, Text 0.74.2 and Native 0.80.0 are published. Build's additive archive
API shipped in 0.74.4; Testkit 0.87.0 and Tools 0.83.0 remain pending.

The complete 35-stage Linux gate and separate publication/readback for Testkit
0.86.0 passed. Its archive SHA-256 is
`7f140a5aef804f8bc9ca838f42992fa6e63c6a76c9bf09ce97b1d1ee670d1288`.
Receipts and restored user-lock records remain under
`.git/release-state/deployment-wire017-20261104/`; larger evidence is under
`target/deploy-wire017-20261104/` and `target/published-integration.Uo0uMi/`.
The deployment's `native-ci.json` owns hosted-run identity; do not reuse older
macOS passes for the pending source cut. Earlier interrupted lock-rewrite attempts
and their failed logs remain preserved; diagnostic monitors were stopped.

Build's fresh-target archive API, owned compiler-cache policy and exact-input
fixture reuse retain their original guards and evidence under
`target/release-speed/`. Shared's parser optimization (#354) is included in the
new snapshot; adoption alone is not a release-speed claim. Previous deployment
receipts remain under `.git/release-state/deployment-platform022-20261104/` and
`deployment-cleanup-20261104/`. The explicitly retired historical checkout's
760 KB of plans/receipts remain under
`.git/release-state/deployment-tools-0.77.0-testkit-0.77.0/historical-release-records/`.
