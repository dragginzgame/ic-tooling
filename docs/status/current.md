# Current handoff

## Platform 0.29.0 adoption for the pending Tools/Testkit deployment

Both product and qualification catalogs/locks now select published Platform
0.29.0 (registry checksum 96a33522b86e1c775fbe3ce124657a4ff8a63c9a4280f3bb851e49e1b4fc210b).
Only Platform changed during the scoped update; Types/Runtime/Wire are unchanged.
Existing codec entrypoints retain their work-derived skip budget. No new Tooling
policy or public controls are introduced. Codec/ledger/public-decoder tests,
management identity, installed management/snapshot behavior, public contracts,
native/Wasm graphs, snapshot/pin admission, formatting and docs checks pass on
Linux. Evidence: target/platform-adoption. Native macOS was not rerun.

Foundation/Build batch batch-aL68C1ft retains its original Platform 0.28 inputs.
Foundation 0.76.2 has published; Build qualification is in progress. Apply this
adoption after reconciling that batch, before Tools 0.89.0 / Testkit 0.97.0.
Unrelated newer lock selections are retained on preserve/platform029-lock-selections
and in .git/release-state/deferred-platform029-20261109 for final restoration.

## IC Wire 0.19.0 / Toko Tooling 0.38.2 — local adoption

Published registry selections and the authenticated 187-file tooling snapshot
are adopted. The earlier 0.38.0 placement blocker is superseded: 0.38.1 supplies
atomic qualification metadata/lock handling, retained by 0.38.2. Existing snapshot
entrypoints, earlier CI repairs and unrelated Cargo.lock selections are preserved.

Nine public products remain in crates/. Private ic-testkit-dev and
ic-testkit-perf-probe moved from crates/ into the independent qualification/
workspace, with its own catalog and Cargo.lock. Their private version remains
0.96.0 and they no longer belong to Testkit's release group. Product package
versions are unchanged. Root resolution no longer contains the probe-only
Runtime, macros, guard or slotmap. Qualification retains those dependencies.

The shared qualification_workspace release setting owns both manifests/locks
in one transaction. Product release updates preserve private versions and
unrelated suppliers. Full lint/tests/docs and formatting cover both roots;
source policy, graph checks, package driver, browser fixture paths, release
selection and post-publication contract callers now use the correct root.
Tooling admission compares both workspaces against the same installed snapshot.

Focused Linux evidence under target/wire019-toko0382 includes:
- Both locked graphs fetched; snapshot/member admission and dependency pins pass.
- Native/Wasm graph policy passes; an incompatible qualification-only requirement
  refuses qualification resolution without blocking the independent product graph.
- Six package-driver tests, four public-contract tests, nine Wire adapter tests
  and the installed PocketIC 16.1 Wasm Wire probe pass.
- Testkit release adapter passes with setup/full qualification substituted,
  including rollback and preservation of private versions across the two locks.
- Release selection and publication-roster fixtures pass with external effects
  substituted; CI routing retains its 32-stage gate. Both-workspace formatting
  and actual hook-preservation checks pass.

No commit, push, release or publication ran for this adoption. Native macOS
qualification and the full release gate were not rerun; prior macOS evidence
below belongs to the earlier source. Pending Wire adoption is recorded under
Testkit 0.97.0; no package version has been prepared.

## CI run 42639956856 — local repairs

Run 42639956856 tested 4005928c. Native macOS 15 Intel and Apple Silicon both
passed CI/publication qualification. Linux stopped before qualification when
PocketIC's curl download returned exit 22 (HTTP error). Its saved download.stderr
was omitted from both artifact selections, so the HTTP status/cause cannot be
recovered from the retained evidence. The dependent msrv job failed as a result.

The separate history scan found two generated Mermaid package-edge identifiers
in ff1b681f:docs/stack.md, lines 398 and 406. Exact historical fingerprints are
now ignored, following the existing narrow ignore convention; the complete
redacted history scan passes with zero findings. Product evidence now selects
failed server download.stderr/failure.txt, excluding binaries/partial archives.
The evidence fixture, ShellCheck and diff checks pass. Evidence is under
target/ci-42639956856. These fixes are local; no push or rerun occurred. The HTTP
failure itself remains unclassified, not proven fixed by this diagnostic change.

## Toko Tooling 0.37.5 — local adoption

The published installer refreshed all 185 recorded files with the existing
entrypoints preserved. Root pin, policy exception, Cargo.lock and receipt select
0.37.5; the scoped lock update changed only Toko and preserved the existing
patch selections. This release adds a private Wire benchmark Candid oracle
exception; Tooling retains its own narrower dependency policy.

Snapshot/member admission, actual Host/Build dependency-boundary and feature
graph callers, dependency pins, documentation links and diff checks pass on
Linux. Evidence: target/toko0375-adoption. No executable tool setup, native
macOS qualification, commit or deployment ran for this adoption.

## Testkit 0.96.0 delivered

Frozen batch batch-Uw3itG2U qualified b1ad4500 and delivered release commit
34a2582b7aa7dea13df7b210e9a16dc3854b9a3e to origin/main with testkit-v0.96.0.
The complete Linux gate and selected registry package verification passed.
crates.io confirms ic-testkit 0.96.0, not yanked, checksum
0b919e80e03d7bd2ea5fa984a1a34982955facd7aa146d6430bf3cf083c9f6b3.
Only Testkit was published; Foundation 0.77, Build 0.81.1 and Tools 0.89 remain
pending. Hosted CI runs asynchronously; native macOS is not qualified by the
Linux evidence. At delivery, CI 42639704365, runtime 42639704393 and optimizer
42639704573 were running; browser fixture 42639704328 passed.

The ordinary checkout reconciled the release. Its deferred ic-logging 0.24.13,
itoa 1.0.19 and syn 3.0.10 selections were restored through Cargo and remain
uncommitted. The original lock backup is under
.git/release-state/testkit-0.96.0-deferred. Successful and failed batch evidence
remains under target/release-frozen and .git/release-state/frozen; no cleanup ran.

## Testkit 0.96 deployment qualification

Frozen batch batch-MXrdzwWs captured ff1b681f and stopped before version
preparation or delivery. Its tooling gate exposed stale graph-diagnostic and
nonempty-Candid-allowance fixture assumptions. Follow-up qualification also
found an implicit apps directory in the relocation fixture. All three are fixed;
graph/Candid/retention/routing, five-group release adapter, validation plan,
publication and release-command fixtures pass. A fresh capture owns release
qualification; retain the failed batch.

## Toko 0.37.4 and SHA-2 0.11.1 — local

The authenticated released installer refreshed the recorded snapshot to Toko
Tooling 0.37.4 (185 managed files). Root exact pin, inherited member selections,
Cargo.lock and installed snapshot pass admission. Existing entrypoints and local
changes are preserved; required engineering and Candid guides remain explicit
entrypoints. No executable tool setup was requested by this refresh.
Direct SHA-2 consumers now select 0.11.1 once; the separate sha2_11 alias is gone.
Older upstream cryptography still selects 0.10 transitively. Algorithms and
digest bytes are unchanged; the public serialization fixture now uses the
existing Host HashingWriter rather than the retired supplier Write impl.

Affected-package all-target/all-feature compilation, artifact hashing/consumer
contracts, Build artifact/HMAC/cache tests, eight snapshot failure tests, source
provenance and consumer CI routing pass on Linux. Evidence is under
target/tooling0374-sha2. No macOS or full release qualification ran.

Testkit's delivered 0.96 release lets consumers adopt the owned Platform types.
The complete local batch also has Foundation 0.77, Build 0.81.1
and Tools 0.89 pending. The maintainer subsequently selected Testkit 0.96.0 for
frozen deployment; the other package groups remain unpublished. Unrelated lock
patch selections are retained in .git/release-state/testkit-0.96.0-deferred for
restoration after reconciliation.

## Owned Platform adoption — published cohort resolved

Cargo.lock now selects registry Platform 0.28.0, Types 0.19.0 and
Runtime/macros/guard 0.21.0. The scoped Cargo update and locked fetch passed;
unrelated version selections are preserved. Registry management and the textual
Candid parser are absent from the lock. Only the five Platform package versions
and new num-bigint 0.5.1 were added; retired parser dependencies were removed.
No temporary path/Git selection remains.

`make dependency-adoption-check` passed against that published cohort on Linux:
locked source inventory, native/Wasm feature graphs, Host Tools adapters,
management identity, transport/snapshot refusals, optional Wire, installed
PocketIC 16.1 management/snapshots/certified metrics and installed Wasm Wire
state/typed-failure checks. Toko snapshot admission also passed. Evidence is
`target/platform-owned-adoption/registry-qualification.log`. No native macOS
qualification or full release gate ran; the work remains uncommitted and
unpublished in Tooling.

Production client/fixtures/snapshots now use canonical Types Principal and
Platform-owned management DTOs. Direct registry Candid and management declarations
and the obsolete exact-pin exception are removed. Snapshot and Host decode
budgets explicitly use owned work/allocation units; typed encode/decode refusals
retain accepted ingress and no-replay custody. The native read-state transport
crossing remains a private byte conversion. Host binary decoding and ledger
evidence use the owned codec. The upstream transport still brings Candid
transitively; do not claim the entire native graph is Candid-free.

The maintainer selected removal of textual service inspection: its module,
feature, oneway projection, parser tests and combined CI/release target are gone.
Wasm export facts remain. Canic Host still has the old feature and parser-based
artifact comparison and needs its separately scoped hard-cut migration. No Canic
source was changed. Pending ledgers now select Tools 0.89 and Testkit 0.96;
package versions remain unchanged.

The separately authorized Platform contribution adds four native-only DTOs to
its existing owner, with independent source guards and registry schema/value/Serde
oracles. Platform's owner committed it as 4c17f3a for the corrected deployment.
No Platform release was run by this session. Original snapshot, donor and
upstream hashes/licenses are unchanged; Tooling's cumulative adaptation patch
and its checksum include this adoption and pass reverse reconstruction/mutations.

Focused Linux source evidence: warning-denied Clippy for the affected Tooling
packages and Platform adapter; native all-target/all-feature compilation; 23
client failure cases, two management identity/frozen-byte tests, nine Wire
checks, public decoding/allocation/ledger/Agent tests, installed 16.1 management
snapshots/certified metrics, installed Wire/Runtime probe and held HTTP replies.
Native and Wasm graphs, actual Testkit Wasm compilation, Candid source policy,
formatting, source guards and 32-stage CI routing also pass. Native macOS and
full release gates were not run. Evidence is under target/platform-owned-adoption.

Those earlier source checks used explicit temporary Cargo patches selecting the new local
Platform source while its manifests still had old versions. That proves the
implementation, not new registry package identities. The source lock and metadata
are retained with the evidence; ordinary Cargo.lock contains no temporary source
selection. Wire 0.18's Types ^0.18 dependency belongs to its optional platform-types
feature, which Tooling does not enable; actual locked metadata selects no Wire
features. Wire requires no release for the profiles selected here.

## Local issue fixes — 2026-11-09

[#348](https://github.com/dragginzgame/ic-tooling/issues/348) moves the inspector
to `crates/ic-host-wasm-inspect`. All nine package files compare byte-for-byte;
package/binary/version, Foundation membership and publication eligibility remain.
Workspace/catalog and active consumer docs use the new path; Candid admission
covers an apps tree only when present. The old tree and aliases are absent.
The immutable Toko fleet inventory retains its original observed remote paths.
Inspector Clippy, eight tests and real archive verification passed on Linux.

[#349](https://github.com/dragginzgame/ic-tooling/issues/349) adds a Wire-only
native graph beside the default graph and tightens generic Host/Build exclusion
of the entire Platform family. `make dependency-adoption-check` owns the focused
current-source run: locked version/source inventory, isolated profiles, existing
Host Tools adapters, management DTO identity, transport/snapshot refusals, Wire
admission and installed PocketIC 16.1 certified metrics/snapshot/Wire calls.
These passed with Platform 0.27.1, Types 0.18.3, Runtime 0.20.3 and Wire 0.18.0.
Release-plan fallback, formatting, links, source-root controls and ShellCheck
passed. Existing full gates, singleton checks and native hosts remain required;
suppliers acquire no reciprocal wait or release prerequisite. Testkit's pending
ledger selects 0.95.1 for qualification changes; package versions are unchanged.

Both issues are closed with explicit local-completion notes. The fixes remain
uncommitted/unpushed/unpublished with the Toko adoption below.
Cargo.lock is byte-identical to the start of these issue fixes. Evidence remains
under `target/issues-348-349/`; no native macOS execution or cleanup is claimed.

## Local Toko Tooling 0.37.1 adoption — 2026-11-09

Published Toko Tooling 0.37.1 is adopted locally and remains uncommitted.
Root Cargo.toml selects `=0.37.1`; all 11 real packages retain their inherited
dev-dependency and cargo-machete exemption. Cargo.lock selects the official
archive SHA-256
`47d50f4e65b643cd42c88f0fafe91f071044a94d1cfd0aa0998469b97b68f68b`.
The scoped Cargo update changed only Toko; the pre-existing unrelated lock
selections are preserved.

The authenticated new crate installer previewed and applied the complete
121-entrypoint selection. Its exporter retired unchanged old managed files;
186 installed payloads are now recorded in `.toko-tooling.snapshot`.
`TOKO-TOOLING.md` replaces `DRAGGINZGAME.md`; the renamed receipt and baseline
were not recreated as compatibility aliases. The previous receipt remains in
Git history and adoption evidence.

Consumer Make/CI/release/publication callers use `toko-tooling-*` commands,
`TOKO_TOOLING_*` settings and the new receipt path. Both private qualification
members use `package.metadata.toko-tooling.loc`, preserving their existing
benchmark/build-tooling/test classification. Historical source identities and
release prose retain their original names; four archived local links now target
the maintained baseline/receipt. Snapshot-owned files changed only through the
published installer.

Future updates use `make toko-tooling-update` after deliberately changing the
root exact pin, its policy exception and Cargo.lock. Existing selections install
through `make toko-tooling-install`; `make toko-tooling-preview` is read-only.
`TOKO_TOOLING_INSTALL_TOOLS=0` selects snapshot-only installation. The normal
pinned tool/server setup and offline admission passed after the naming migration.

Pending Foundation notes select **0.77.0** because the maintained command,
settings and receipt names are a hard cut. Product package versions are unchanged.
No commit, push, tag, publication, sibling contribution or release gate was run
for this adoption. Existing product release and native-host requirements remain.

## Focused qualification

Linux checks passed: installed snapshot/member/pin admission, tools-check,
dependency policy, formatting and real hook preservation, documentation links,
ShellCheck with the repository's source-following options, and LOC reporting.
Both private members retain explicit categories with no unclassified source.

The actual Make routing fixture qualifies the renamed commands, preview dispatch,
checkout-local path selection and pipe jobserver. Custom CI, publication and
explicit installers now attach execution admission before their prerequisites;
unsafe Make modes refuse before effects, while an ordinary help dry run works.
Both CI entrypoints retain the same 33-stage order, one lint and fail-fast behavior.
Direct publication refuses tooling drift before effects. Validation selection and
the Foundation adapter passed with product/delivery effects substituted.

The frozen-caller fixture passed all five group queries, supplier order refusal,
deferred lock preservation, interrupted/uncertain recovery, two archive identities,
completed resume and remote conflict. Its product gates and delivery effects are
substituted; this is consumer-hook evidence, not a new complete product gate or
native macOS qualification. Evidence is under `target/toko-latest-adoption/` and
the retained `target/tmp/frozen-release.5jrxCG/` fixture. No artifact cleanup ran.

## Published baseline and consumer adoption

The preceding deployment remains Foundation 0.76.1 (all five packages), Build
0.81.0 and Testkit 0.95.0, with Tools 0.88.0 and Text 0.74.3 unchanged.
The two private Testkit members remain 0.95.0. Release commits are `58a1e041`,
`9a8c3a32` and `9e66ba0f`; all seven registry archives and clean VCS identities
were verified. Ordinary main was reconciled to that deployment, then its handoff
was committed as `e296ceed`. The
[published handoff](https://github.com/dragginzgame/ic-tooling/blob/e296ceed/docs/status/current.md)
records complete Linux gates, asynchronous native CI links, package identities
and original custody. No new native macOS result is claimed by this adoption.

Canic's maintenance-builder caller and Auth's concrete instance annotation still
require their separately scoped minor-version adoption; sibling source was not
changed or qualified. Preserve the completed frozen batches `batch-jNmWYunP`
and `batch-4F91gJ1U`, all unfinished older batches, source pins, receipts and
retained builds. Do not replay earlier attempts against already-published versions.

## Frozen deployment selection — 2026-11-09

Deployment is authorized for Foundation 0.76.2 and Build 0.81.1 (patch), then
Tools 0.89.0 and Testkit 0.97.0 (minor). Foundation tooling adoption does not
break its published library contract; its pending heading was corrected.
Unrelated lock patches are preserved on local branch
`preserve/wire019-lock-selections` and in `.git/release-state/deferred-wire019-20261109`.
The release captures the prior ic-logging/itoa/syn selections; deferred selections
will be reapplied after reconciliation. No sibling repository is included.

## Deployment dispatch correction

Batch batch-st9pUW60 stopped before version preparation or upload because the
redundant generic release-command fixture lacks qualification admission inputs.
Its captured source and failure evidence are retained. The existing consumer
Make fixture now owns complete dispatch coverage, including inner/frozen routes,
runner failures and conflicting commands; focused routing and scratch checks pass.
A new batch captures this correction without changing the failed batch.
