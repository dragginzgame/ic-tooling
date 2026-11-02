# Current handoff — 2026-11-02

## Authorized corrective delivery

This Tooling session is authorized to qualify, release and publish Foundation,
Build, Text, Tools and Testkit sequentially. No Platform, Blob, simulator, npm or
application release is included. The current user additionally requested Shared
Tooling adoption before continuation; that adoption is complete.

Integration source is `f6f6e7d9`; Shared Tooling adoption is `99ed3437`.
Foundation 0.74.0 passed its complete Linux/native/Wasm/installed/package gate,
was committed as `6e446a61f08fc8f01ad36d653c6a051b45f401f7`, and was atomically
pushed with `foundation-v0.74.0`. No Foundation crate has been uploaded.
The publication dry run exposed FS's dev dependency on Process, whose normal
FS dependency makes Cargo's publication order cyclic. Do not repeat that upload.

A repair is prepared in the isolated delivery checkout: run the three
FD-sensitive FS cases in separate libtest executables and remove FS's Process
dev dependency. No production API or dependency is added. The three tests,
strict workspace Clippy and complete Foundation publication dry run pass.
The proposed next Foundation ledger entry is 0.74.1; package versions remain
0.74.0 in that checkout, and the existing Git tag is unchanged.

The maintainer approved publishing corrective Foundation 0.74.1, then continuing
Build 0.74.0, Text 0.74.0, Tools 0.78.0 and Testkit 0.79.0. Preserve the existing
0.74.0 Git tag; run the standard patch release from the committed repair.
Those four remaining group releases have not started. Update the stopped
controller to Foundation 0.74.1 before publication; retain earlier failed logs.
The maintainer also requested new Types 0.17. Its matching Runtime/facade
releases are not published yet; keep the compatible Types 0.16 selection while
independent Foundation/Build/Text delivery proceeds. Qualify the new singleton
graph before Tools/Testkit release. Never release another repository here.

## Checkout and evidence custody

The ordinary checkout remains on integration commit `99ed3437`, with unrelated
Cargo.lock selections preserved outside both integration commits. This handoff
is an additional local documentation edit. Final synchronization must preserve
the lock selections and any new user work; do not restore its whole pre-release
lock over updated workspace versions. No artifact cleanup is authorized.

`.git/release-state/deployment-five-groups-20261102/release-checkout-path` names
the isolated checkout under `/tmp/ic-tooling-five-groups.iOEuyP/checkout`.
Its `.git/release-state` contains the exact completed Foundation plan/receipts;
keep it for recovery. The ordinary Git directory's delivery evidence retains
setup and gate logs, failed dry-run evidence, the original input patch, scoped
lock delta, source identities and registry observations. Its `delivery-repair/`
contains the reviewable fix patch and passing focused/dry-run evidence.

The original Foundation validation was intentionally interrupted before metadata
preparation to adopt Shared Tooling. Its retained log is not a completed pass.
The later `foundation-release-02727.log` is the successful complete release gate.
Publication attempt `check.7a4XAG` failed on the cycle without uploading. Repair
attempt `check.jdkzZs` stopped only on sandbox DNS; the subsequent network-enabled
repair dry run passed for all five Foundation packages. All attempts are retained.

## Adopted source contracts

Shared Tooling 0.27.27 at `651f5f7acb2e3055abd51ba5f835a2112801666a` is
exported canonically; all 162 managed payloads verify. Make routing, actual
formatting hooks and all documentation links pass. The consumer explicitly
includes the linked validation-runner guide. Tool pins and lock selections are
unchanged by the snapshot refresh. Group validation plans are opt-in: Tooling
has no qualified narrow-stage mapping, so this cross-cutting batch keeps full
gates. Repository-wide hosted CI stays unchanged.

All twelve packages use Rust 1.99.1, matching the pinned compiler. Metrics 0.26
is native/arithmetic-only and absent from Testkit Wasm. Testkit selects Types
0.16 and Runtime 0.17.1 directly; native PocketIC remains on Platform 0.17 and
server 16.1.0. Build and Text have independent groups; Foundation owns the five
lower-level packages, with Artifacts as primary. Old release histories and
plans retain their original scope. The publishing guide's retired Rust 1.91 and
msrv-qualification instructions are corrected.

The completed local release gate is Linux evidence. Hosted CI started for the
Foundation commit; native macOS 15 Intel/Apple Silicon results must be observed
and reported separately. No macOS pass is established by local Linux evidence.

## Older published baseline and supplier boundary

Before this delivery the registry baseline remains Foundation/Build/Text 0.73.6,
Tools 0.77.0 and Testkit 0.78.0. The ten public packages exist; the two private
Testkit helpers are never uploaded. Completed Testkit 0.78 receipts remain in
`.git/release-state/deployment-testkit-0.78.0/`; do not replay its increment/upload.
The earlier preservation stash is `bc5e544bb9adfea1e0b0a0aac77945ca573321d2`.
Do not apply its complete old metadata patch over delivered versions.

[Platform #134](https://github.com/dragginzgame/ic-platform/issues/134) owns typed
PocketIC transport errors and remains outside this delivery. No supplier or
consumer repository is released by this Tooling request.
