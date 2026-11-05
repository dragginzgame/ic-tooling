# Current handoff

## Shared adoption and async-cut deployment — 2026-11-05

The maintainer authorized deployment of the reviewed batch based on `1401a36f`.
The selected order is Foundation 0.74.4, Build 0.74.4, then Testkit 0.87.0 from
ordinary main, through the standard release and separate publication adapters.
Source integration and qualification are in progress; no new publication is
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
included. Native macOS Intel/Apple Silicon qualification has not run for either
local batch. Full release qualification remains a separately requested workflow.

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

Cargo.lock remains intentionally dirty with unrelated maintainer selections.
The async cut changes only Dev's direct Tokio edge and the now-unused Tokio
signal-hook-registry edge/package; every other incoming package record was
verified unchanged. The original async incoming lock remains in its evidence
directory. This adoption's incoming lock/diff/handoff are saved under
`target/shared-adoption-20261105/incoming/`; Cargo.lock is byte-identical to that
saved lock after the adoption. Do not reset it to HEAD or include unrelated
selections implicitly in a release.

Last published Testkit is 0.86.0: release commit
`71de6abfc21d8a26d33b3ac485499890654513db`, tag `testkit-v0.86.0`, preceded by
Wire adoption `0a70714040588e2367d62ef4af1859f191e6a0de`. Foundation/Build 0.74.3,
Text 0.74.2, Tools 0.82.0 and Native 0.80.0 remain published. Build's additive
archive API is Git-delivered but pending publication as 0.74.4.

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
