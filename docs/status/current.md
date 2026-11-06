# Current handoff

## Testkit 0.89.1 delivered — 2026-11-06

`ic-testkit 0.89.1` is published from clean release commit
`7d7ffbfcdd4a39061c0cd2c33809b661cd3c76e5`; `main` and `testkit-v0.89.1`
are pushed to `dragginzgame/ic-tooling`. Private Dev/probe follow 0.89.1 and
remain unpublished; other groups retain their versions. Canic can pin that tag
for the repository II driver, whose schema-1 record now adds `serverUrl` and
numeric `instanceId` for borrowed native attachment. The
[consumer guide](../../testing/browser/internet-identity/README.md#native-attachment-and-cleanup-ownership)
documents application setup, request/runtime ownership, session lifetime and
exclusive fixture-owned instance/gateway/server teardown. Updating the crate
alone does not supply this private repository fixture.

All 34 complete Linux release stages passed on Rust 1.99.1, with logs under
`.git/release-state/testkit-gate.IsJ5dt/logs/validation.FqfebM/`. The downloaded
registry archive matches uploaded bytes and exact clean Git provenance, with
registry checksum `dd2879282c81fc23e57d00e89dd8f8a87564c624110f0c7bf800e704aaffa944`.
The registry-only consumer passes all three public contract cases; evidence is
`target/published-integration.0JkRBU/`.

Final installed Chromium acceptance on the release dependency graph confirms a
live management instance, real II sign-in, matching signed-ingress caller,
wrong-root/altered-signature refusal and exit-zero teardown closing both gateway
and management server. `target/evidence/ii-testkit-0.89.1-001/` retains its proof,
trace and cleanup receipts. Earlier focused proof with incoming patch selections
remains at `target/evidence/ii-attachment-001/`.

The normal checkout contains the release. Its unrelated incoming lock selections
are restored and remain uncommitted; their exact original bytes/patch and a
restoration comparison are retained in
`.git/release-state/deployment-testkit-0.89.1/`. Release qualification/publication
used the committed graph; restoration changes only the three owned Testkit
versions relative to the saved original. No historical artifact cleanup occurred.
Native macOS is not claimed for this batch; the complete native matrix is
[running asynchronously](https://github.com/dragginzgame/ic-tooling/actions/runs/42210498533)
at the release commit.

## Testkit 0.89.0 delivered — 2026-11-06

`ic-testkit 0.89.0` is published to crates.io from clean release commit
`094d1447c07681c8ef973c0b0e5d9c1193e6c0b4`; `main` and `testkit-v0.89.0`
are pushed to `dragginzgame/ic-tooling`. Private Dev/probe follow 0.89.0 and
remain unpublished. Other groups retain their versions. The normal checkout
contains the release, including the reviewed cc 1.11.1 / flate2 1.1.11 selections.
Consumers must adopt [application-owned logging](../testkit/client-migration.md#application-owned-logging-in-089).

All 34 complete Linux release stages passed on Rust 1.99.1, including installed
PocketIC 16.1 contracts, native/Wasm graphs, normalized archive qualification,
workspace tests, Clippy and formatting. Initial routing fixtures incorrectly ran
the new Candid prerequisite without a source tree; `8b376d8d` corrects their
substitutes while retaining the separate real rejection control. The isolated
logging child also safely tolerates aggregate installed-test selection. The
failed attempt and successful gate remain under `.git/release-state/`; final
logs are `testkit-gate.ee7pAE/logs/validation.lc3NoV`.

The downloaded registry archive matches the uploaded bytes and registry checksum
`a8cc750166a6898536fd89ebfc8870195e822fe1c2fd1f04a2275e912f671517`,
with exact clean release provenance. The registry-only consumer passes all three
public contract cases; evidence is `target/published-integration.cMEai9/`.
Final Chromium 156.0.8078.4 acceptance passes real virtual-WebAuthn II sign-in,
`getIdentity`, matching signed-ingress caller identity, wrong-root refusal and
altered-signature refusal. `target/evidence/ii-testkit-0.89.0-001/` retains proof,
trace, input/tool hashes and exit-zero/closed-gateway teardown. This is virtual
authenticator integration evidence, not physical-authenticator assurance.

Issues #333, #334, #336 and #337 are closed after delivery and acceptance; #335
was already closed following IcyDB adoption. Native macOS remains unqualified
for this batch. The complete Linux/macOS 15 Intel/Apple Silicon matrix is
[dispatched asynchronously](https://github.com/dragginzgame/ic-tooling/actions/runs/42200448086)
at the release commit; its completion is not claimed here. No historical
artifact cleanup was performed. The preparation notes below retain their
original scope and are superseded by this delivery record.

## Local logging ownership and fixture scratch — 2026-11-06

The unused root Semver catalog entry is removed; transitive Semver remains.
Tooling #333 removes client-side log-file/global-subscriber setup and the retained
worker. Imported tracing events remain compatible with application-owned
projection into `ic-logging`. That canonical sink and a registry-only subscriber
are test dependencies; the production graph excludes both logging engines and
the sink. Nine unused logging-support packages leave the lock. The new
checksummed logging patch reverses before the existing source-provenance chain;
original donor/upstream fingerprints and licenses are unchanged.

Tooling #334 puts local shell/browser source fixtures under workspace `target/tmp`
by default and preserves explicit/aliased `TMPDIR`, refusal and retention behavior.
The product driver owns the final imported release-command probe too, so nested
probes share the same scratch selection. Old `/tmp` evidence, Git-side release
records, persistent state and tool stores are untouched. See the
[scratch contract](../design/fixture-scratch.md).

Focused Linux evidence passes installed multi-client/borrowed-handle cleanup with
early and late application logging, invalid legacy logging settings, strict
selected Clippy, source reconstruction/mutation rejection, native/Wasm normal
graphs, scratch/retention/Make routing and ten browser source-fixture checks.
Native macOS has not been rerun. An independent updater selected flate2 1.1.11
during final checks; it and the incoming cc 1.11.1 are preserved. Other retained
package versions/checksums are unchanged; focused Rust checks use this final lock.

Pending Testkit is now 0.89.0 because removing implicit logging/environment
behavior needs caller migration; the earlier II fixture notes move with it.
Package versions remain unchanged. GitHub Shared 0.30.6 is still the latest
committed supplier and is already adopted. Tooling #335 is closed after verifying
IcyDB's default branch requires Tools 0.83.0 and uses both published decoders.
No staging, commit, push, release or publication was performed in this batch.

## Local Candid source admission — 2026-11-06

Tooling #337 adopts Shared 0.30.6 at
`45c0295e23d3d23225e31f47c018c30ff0c75581` through its canonical exporter;
all 173 snapshot files were verified against that commit. The unmodified Candid
checker and source masker scan all 435 maintained Rust files under `crates`,
`apps`, `scripts` and `docs`, including native tests and the independent recipe
driver. Three exact external declarations remain admitted: Tools' bounded
decoder and legacy ledger record, and native PocketIC's management-snapshot
helper. No owned application or codec-comparison allowances were added.

The common dependency-admission target runs this check once with the complete
policy, including during group-selected validation. Focused consumer controls
prove rejection through Platform re-exports and cfg-disabled fixture aliases,
stop metadata validation on failure, and check Make execution is not duplicated.
The source scan, consumer control, ShellCheck and local documentation links pass;
no Cargo compilation or native-host qualification was run for this tooling change.
Root release notes and the [coverage guide](../design/candid-boundary.md) describe
the maintained scope and lexical limitations. CBOR enforcement is separate.
Package versions and Rust public APIs are unchanged; the preceding #336 work is
preserved locally. No integration commit, push or publication is included.

## Local Internet Identity fixture — 2026-11-06

The repository fixture for Tooling #336 pins official II release
`release-2026-10-02` / `b9a8aaf43977adc1a2edf6ce98729826fc45133c`, verifies
the matched backend/frontend and original interfaces, and disables dummy auth.
The private driver uses existing Testkit startup/teardown owners and upstream
Candid definitions; no published API or Canic policy changes. The compatible
test-tooling addition is carried under pending Testkit 0.89.0; package versions,
release tags and the published baseline below are unchanged.

Linux Chromium 156.0.8078.4 completed real virtual-WebAuthn registration,
AuthClient 10.0.2 `signIn`/`getIdentity`, matching signed-ingress caller bytes,
typed wrong-root rejection and replica rejection of an altered signature.
`target/ii-336-accepted/` retains the provider connection, trace, exact tool
hashes, proof and successful closed-gateway teardown receipt. Input-rejection
tests, selected driver Clippy and formatting checks pass. Startup/ready signal
cancellation and pre-start tamper rejection pass with retained receipts under
`target/ii-336-lifecycle-final/`. Native macOS has not
been qualified for this fixture. See the
[consumer guide](../../testing/browser/internet-identity/README.md).

## Delivered Build/Testkit hard cut — 2026-11-06

| Group | Published package | Release commit / tag |
| --- | --- | --- |
| Build | ic-build 0.75.0 | `1239e9a8` / `build-v0.75.0` |
| Testkit | ic-testkit 0.88.0 | `0bc2cc4c` / `testkit-v0.88.0` |

Both complete 34-stage Linux release gates passed on Rust 1.99.1, including
fresh normalized archives, native/Wasm graphs, installed management, certified
metrics, snapshots, persistent-state reopening and workspace tests. Both release
commits and tags are pushed to origin/main; both packages are published to
crates.io. Private Dev/probe versions follow Testkit 0.88.0 and remain unpublished.
Foundation, Text, Tools and Native allocation versions were not advanced.

Downloaded registry archives match their index checksums, preserved publication
archives and exact clean release Git provenance. The registry-only consumer
passes all three public identity/contract cases with Build 0.75.0, Testkit 0.88.0,
Tools 0.83.0, Platform 0.23.2, Types 0.18.1 and Wire 0.17.3. Incoming Platform/Wire
patch selections were reviewed and included in fresh qualification; earlier
focused cleanup checks used Platform 0.23.0/Wire 0.17.2. No unrelated lock
selections were restored or discarded.

Deployment evidence is under `target/deploy-hard-cut-20261106/`; the registry
consumer evidence is `target/published-integration.ciGB2J/`. Standard plans and
full gate records remain under `.git/release-state/`; small deployment/readback
and asynchronous native-CI receipts belong under
`.git/release-state/deployment-hard-cut-20261106/`. Native macOS qualification
is asynchronous and separate from Linux success. No artifact cleanup is included.

Source integration is `7d3a761f`. The hard cut removes unused benchmark reporting,
PocketIC endpoint/configuration/trait surfaces and Build inspection/readers.
Testkit retains supported consumer configurations, explicit async teardown,
no-replay receipts, protocol/management identities and bounded snapshots. The
server owns configuration validation; persistent directories remain caller-owned.
Both active cache flows and Canic/IcyDB progress/timing consumers remain. See
[the migration](../testkit/client-migration.md) and owning group ledgers.

The source audit measured production Rust 33,228 -> 31,520 and all Rust
84,199 -> 81,278 with no unclassified code. Focused evidence remains under
`target/hard-cut-20261105/`, `target/pocket-builder-cut-20261105/`,
`target/build-surface-cut-20261105/` and `target/testkit-traits-cut-20261106/`.
The installed persistent-state fixture remains at
`target/tmp/pocket-builder-state-1383307-0/`. Original donor/upstream histories,
licenses and fingerprints are preserved through the independently checked
surface patch; no sibling source or existing persistent directory was modified.

## Delivered Platform codec integration — 2026-11-05

| Group | Published package | Release commit / tag |
| --- | --- | --- |
| Tools | ic-host-tools 0.83.0 | `094b7787` / `tools-v0.83.0` |
| Testkit | ic-testkit 0.87.0 | `e3b8a885` / `testkit-v0.87.0` |

Both complete 34-stage Linux release gates passed on Rust 1.99.1, including
native/Wasm graphs, installed metrics/management/snapshots, fresh archives and
workspace execution. Both releases are pushed to origin/main with their tags and
published to crates.io. Downloaded archives match their index checksums, local
publication archives and exact clean release Git provenance. Private Dev/probe
versions follow Testkit 0.87.0; Foundation/Build, Text and Native were not advanced.

The registry-only consumer passes all three public-contract cases with Tools
0.83.0, Testkit 0.87.0, Platform 0.23.0 and the exact locked Types/Wire versions.
Evidence: `target/published-integration.5dfo1w/`. Its verifier now obtains exact
versions from direct workspace resolve edges; catalog minimums such as Types
0.18.0 must not replace the qualified 0.18.1 selection. The earlier incompatible
minimum-pin attempt remains at `target/published-integration.qqDxMm/`.

The maintainer explicitly included the incoming lock selections after the first
gate refused their rewrite. Runtime 0.19.2, Candid 0.10.40 and compatible patches
are committed release inputs. Do not restore older snapshots over them. The failed
attempt is retained, not reused. Tools' first atomic push was rejected by GitHub
with `fatal error in commit_refs`; remote inspection showed neither ref delivered,
and exact `release-resume` completed the saved commit/tag push without a new bump.

Deployment evidence is under `target/deploy-platform023-20261105/`, with durable
backups/receipts in `.git/release-state/deployment-platform023-20261105/` and the
standard group plans. Source integration is `b0e267b9`; approved lock integration
is `3f559604`. Hosted native qualification remains asynchronous; its run identities
belong in this deployment's `native-ci.json`. Linux success does not establish a
new native macOS pass. No artifacts or prior release evidence were cleaned.

Tools consumers remove the retired certificate feature/module and import
Platform 0.23's strict certificate codecs directly. Testkit consumers await its
client/lifecycle calls and explicit teardown; see the
[client migration](../testkit/client-migration.md). Query verification policy and
PocketIC transport/orchestration retain their owners.

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
complete release/archive gates were separate from this initial focused evidence.
Subsequent full Linux release and registry evidence is recorded above.

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

## Earlier delivered groups and interrupted Testkit release — 2026-11-05

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

The previous Testkit 0.86.0 publication used release commit
`71de6abfc21d8a26d33b3ac485499890654513db`, tag `testkit-v0.86.0`, preceded by
Wire adoption `0a70714040588e2367d62ef4af1859f191e6a0de`. Foundation/Build 0.74.4,
Tools 0.82.1, Text 0.74.2 and Native 0.80.0 are published. Build's additive archive
API shipped in 0.74.4. The latest Tools 0.83.0 and Testkit 0.87.0 delivery is
recorded at the top of this handoff.

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
