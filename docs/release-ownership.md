# Replacement release ownership

The maintainer approved group ledgers on 2026-11-15. The executable ownership map
is [ci/cargo-release-groups.json](../ci/cargo-release-groups.json); package manifests
remain the version authority. [CHANGELOG.md](../CHANGELOG.md) is navigation only.
The initial map did not change package versions or publication settings; Testkit
now has its own group as its requirements-driven implementation begins.

| Group | Contract and reason for separate release |
| --- | --- |
| foundation | Byte admission and held filesystem custody share the existing 0.80 development line and capture/publication contracts. Build and the qualification executable consume them; they remain coordinated in this slice. |
| build | Cargo execution and owned Wasm capture have distinct consumer migrations from storage/byte operations. A presentation or HTTP change must not force a Build version change. |
| cargo-metadata | Installable read-only release metadata rendering serves Toko transactions; its command protocol and publication are independent of Build and the private external-lock checker. |
| http | Complete response-body admission depends on Reqwest and has its own native consumer requirements. It stays outside Build and portable byte graphs. |
| text | Pure escaping and table layout serve Query and DB independently of HTTP, process execution and filesystem authority. |
| process | Native child/group execution and bounded communication serve Build and foreground tools without Cargo, IC or observation policy. |
| testkit | Shared PocketIC client identity, bounded generated fixtures and exclusive held-reply control serve test drivers independently of Build and product runtimes. |
| tools | The external-lock executable has its own command contract. Its ledger also owns maintained repository qualification/setup workflows; these scripts are not executable library features or runtime dependencies. |

All nine product packages belong to exactly one group. `qualification/owned-wasm-probe`
is a private fixture in the independent qualification workspace, not a product group.
The server CLI belongs to Testkit's package and group; the private prototype is removed.
The Testkit group owns an actual admitted package, not a future simulator wrapper.
Frozen Wire work and absent packages receive no group. Root dependency aliases list only actual member usage; the policy's
alias rosters reflect those declarations without adding display-only dependencies.

Pending headings describe unpublished development history or future candidates;
all seven libraries now have dated release entries. Breaking
consumer migrations are explicit in the affected ledgers. Registry availability,
version reconciliation and release admission remain required before publication.
Existing development version increments were preserved by this organizational work.

Earlier Stage 1–2 notes are retained verbatim in
[history](../releases/history.md), explicitly labelled unpublished. They are not
retroactively assigned to individual groups. Later pending notes now live with
their owners; detailed test results remain in the existing qualification records.

`make deps-list` reports ownership across both workspaces. No release/publication
entrypoint is introduced by this change; future release tooling must require an
explicit group selection. Seven library manifests permit only `crates-io`;
the private qualification executable retains `publish = false`. All seven
libraries are published; publication permission in a manifest is not delivery.

## Independent library publication

### Testkit 0.109.0 delivery

The maintainer authorized deployment on 2026-11-17 from this checkout's `main`.
[IC Tooling #11 — Support bounded clock progression with held Testkit replies](https://github.com/dragginzgame/ic-tooling/issues/11)
adds `Fixture::advance_time` at the existing exclusive controller, preserving
immutable replies, original HTTP expiry and consumed-before-dispatch release.
Interrupted clock transitions refuse further control. The new public error
variants require a minor release for exhaustive matches. Other public packages
and dependency requirements are unchanged; private Tools notes remain pending.

The complete Linux Testkit archive gate passes at `target/library-release.ZLBFiAoX`
with committed dependencies: lint, default/optional graph isolation, unit/docs,
CLI setup/lifetime, three installed fixture cases and two installed server cases.
Independent inventory observes all five fixture instances deleted. The clock case
qualifies release across 31 seconds and at original HTTP dispatch + 60 seconds,
then refusal one nanosecond afterward without application bytes. All fixture
source bytes are compared again at versioned publication admission. No new
Canic execution or native macOS acceptance is claimed; hosted CI is asynchronous.

Unrelated dependency lock updates are saved under `target/deploy-clock/` and
excluded from the release selection. Restore them with only Testkit's member
version advanced. Canic owns its same-receiver adoption and application assertions.

Published from `1fdd03705377c1a74211ec1f4ca9a15440adc0a0`. `main` and annotated
`testkit-v0.109.0` were pushed atomically and remote identities verified.
Clean-source Cargo publication preflight and upload passed with all features.
Versioned archive sources match the qualified candidate exactly, the normalized
manifest differs only in package version, and provenance identifies the clean
release commit and package path.

The non-yanked registry archive SHA-256 is
`6405078e5332b34ab898d4c32c41f39b358655bb3d4aae508d3ff17f781cf66c`.
Downloaded bytes match both approved preflight and upload archives. Evidence and
recovery locks remain under `target/deploy-clock/`; unrelated lock updates are
restored with Testkit 0.109.0 and remain uncommitted. Post-push review of the
controller, uncertainty boundary and installed callers found no further defect.

### Process 0.80.5 and Testkit 0.108.1 delivery

The maintainer authorized deployment on 2026-11-17 from this checkout's `main`.
Moving inline tests and the private fixture constructor changes both package
payloads, so their compatible patch releases are selected. Runtime behavior,
public APIs and dependency requirements are unchanged. Scanner setup diagnostics
ship through Git; the private Tools ledger stays pending.

Release gates use the committed dependency selection. Concurrent Protocol,
Clap/Nix and Zeroize updates are preserved under `target/deploy-scanner/` and
excluded from publication; restore them with the two released member versions.
The native Build/Process archive gate passes at `target/build-release.XWtmn8e5`;
Testkit's complete archive/CLI/installed-simulator gate passes at
`target/library-release.UJu3W7Jy`. Both macOS Process targets pass all-target
Clippy for the relocated tests, without claiming native execution. Scanner setup
has 57 passing failure/redaction/reporting checks and actionlint acceptance.
Hosted native CI remains asynchronous; successful setup runs do not prove a
hosted failure-artifact upload. Final publication admission verifies the versioned
archives against these qualified source trees before upload.

Published from `02ac139f175069e4809c93aab428ddaee30eca17`; `main` and annotated
`process-v0.80.5` / `testkit-v0.108.1` tags were pushed atomically and their remote
identities verified. Clean-source Cargo publication preflight and upload passed
with all features. Extracted source/licence bytes match the qualified candidates;
normalized manifests differ only in the selected package versions. Provenance
identifies the clean release commit and exact package directories.

| Published package | Verified registry/archive SHA-256 |
| --- | --- |
| `ic-host-process` 0.80.5 | `73930a7f8d56062e7fa5c8821b5259b355ea841e9137842644619bfbbe8927b2` |
| `ic-testkit` 0.108.1 | `dae330c65abdc613e8bfdabe5d45bbfcc613c48a1d9f0f250194608f6b1291e9` |

Both registry records are non-yanked; downloaded archives match the verified
preflight/upload bytes. Publication logs, source comparisons, registry records
and recovery locks remain under `target/deploy-scanner/`. The concurrent lock
edits are restored with released member versions and remain uncommitted. The
post-push review of the release and its affected callers found no further defect;
hosted native CI runs asynchronously.

Toko installed the registry Testkit 0.108.1 CLI in release profile. Its version,
offline prepared-tool receipt and prepared-server admission checks pass; the
selected executable and verification output are retained in the same evidence root.

### Foundation 0.80.3, Process 0.80.4 and Testkit 0.108.0 delivery

The maintainer authorized deployment on 2026-11-17 from this checkout's `main`
to `origin/main` and crates.io. Foundation's module layout changes affect its
package payloads, so both coordinated members advance. Process relaxes its
libproc requirement; Testkit adopts Protocol 0.3 and compatible WAT/validator
requirements, with a minor release for its exposed encoder-error type identity.
Existing compatible foundation/Process dependency minimums remain unchanged;
Build and the other library contracts need no release. Private repository fixes
ship with this commit; the private Tools ledger remains pending.

The module-layout cleanup moved six Rust files into their existing directories
as `mod.rs`, preserving source bytes and module paths. Canic qualification
generators use the same layout and ordinary child discovery; this does not renew
historical consumer acceptance. Focused Clippy/formatting checks and 28 module
tests passed; evidence remains under `target/module-layout-review/`. Qualification
path-resolution acceptance remains with the [source-capture checks](qualification/local-sources.md).

The Build/Foundation/Process archive gate passes at `target/build-release.Q6xVhMK1/`.
Testkit's archive gate at `target/library-release.Ss7XdgVq/` passes package/source,
feature graphs, Clippy, unit/docs, CLI and all four installed fixture/server cases.
Its first CLI attempt was refused by sandbox loopback restrictions; remaining
stages passed with local-port access against the same extracted source and tools.
Versioned Scheduling replacement admission passes at
`target/scheduling-new.6tFpSuVA/`. Version preparation changes package metadata,
not qualified Rust implementation. Native CI remains asynchronous by maintainer
direction; prior native evidence is not relabelled as a new pass.

Cargo selected non-yanked Protocol 0.3.3 during preparation. Its archive SHA-256
is `4676f2fabfff6a9aab9d6cccdf2a5870059e88b21345bc1913818be621dc1f34`;
the HTTP encoder is byte-identical to reviewed 0.3.2. The compatible minimum stays
0.3.2. Concurrent lock selections are retained under `target/deploy-ranges/` for
restoration after publication.

Published from `78f1d2fbfbda6cac3b39f49dfe8aaa3c26472135`. `main` and annotated
tags `foundation-v0.80.3`, `process-v0.80.4` and `testkit-v0.108.0` were pushed
atomically and their remote identities verified. Clean-source Cargo publication
preflight and upload passed with all features. All four non-yanked registry
records and downloaded archives match the preflight/upload bytes:

| Package | Archive SHA-256 |
| --- | --- |
| `ic-host-artifacts` 0.80.3 | `54cd6b9013ecab7db68af690e5c77157289ec007faec965e1d8b1ad1d5f6a43f` |
| `ic-host-fs` 0.80.3 | `90a8189bf22f96b959a03c16c11db97c69032825be4dae8e5078249a17034d41` |
| `ic-host-process` 0.80.4 | `a88492b7a656509c3664f4f4ea68d5cf5303c3ec1c22bc64d6f4c41ded6128a3` |
| `ic-testkit` 0.108.0 | `6fa2e6157d27d15ba1bc93923adc3221e1c09f9f28ec2f8bdde09b04b528cf80` |

Archive provenance identifies the exact clean release commit and package paths.
Evidence is retained under `target/deploy-ranges/`. Post-push review of the
published package boundaries and affected qualification callers found no new
actionable defect. Hosted CI remains asynchronous; sibling delivery is separate.

Toko installed the registry Testkit 0.108.0 CLI in release profile. Its reported
version, offline Toko receipt admission and offline prepared-server check pass.
Pre-existing Clap/Nix lock updates are restored as uncommitted working changes
after delivery, with the released member versions preserved; they are not part
of the release commit or its qualification claim.

### Process 0.80.3 and Cargo metadata 0.1.0 delivery

The maintainer authorized libproc adoption and deployment with Tooling #8's fix.
The selected publishable groups are Process 0.80.3 and Cargo metadata 0.1.0,
delivered from this checkout's `main` to `origin/main` with annotated tags
`process-v0.80.3` and `cargo-metadata-v0.1.0`, then crates.io publication.
Build/Testkit need no new release: their existing compatible Process requirements
already admit the patch. The inherited Protocol 0.2.8 lock selection is retained;
it is outside these packages' graphs and does not claim new Testkit qualification.

Published from `de86fe21dc746eda79e662008f0d670f20970ac7`; `main` and both annotated
tags were pushed atomically and their remote identities verified. Clean-source
Cargo publication admission passed; source/tests, manifests, licence and clean
VCS provenance match the release. Registry readback is non-yanked and byte-identical
to preflight and uploaded archives:

| Package | Archive SHA-256 |
| --- | --- |
| `ic-host-process` 0.80.3 | `2b473c407b5d0e39597559bb9b506637849c6e7ad540ef45ab3b566b97244f82` |
| `ic-cargo-metadata` 0.1.0 | `5430d9278184df2c90d096712bd44b5dffa5c619dba550a646ee979c13400d0d` |

Toko installed the released metadata command in release profile (17-second
compilation); offline tool admission passed. The installed command passed all
Toko coordinated/multi-lock and Scheduling Timers/Jobs transaction checks under
`target/cargo-metadata-consumer.vDBN9VkA/`. Tooling #8 is closed; adoption and
delivery in receiving repositories remain with their owners.
Post-push review of the delivered package boundaries and directly affected
archive/consumer callers found no actionable defect. No new native runtime CI
pass is claimed. Publication admission was followed by root-lock resolution drift;
both variants are retained in the release evidence. Rechecking the committed
inputs left the root lock unchanged and produced byte-identical archives before
upload. The final checkout retains the committed selection.

The libproc archive and source review are recorded in
[Process execution](process-execution.md). Both macOS targets pass all-target
Process Clippy with libproc 0.15.0. This is cross-compilation evidence, not native
runtime acceptance. Linux packaged Build-stack unit/PTY tests, Clippy and docs
pass at `target/build-release.u9sAKBc9/`; only Process is selected for publication.
The metadata editor's selected Linux archive gate passes at
`target/library-release.xHftYEED/`. Existing actual Toko/Scheduling transaction
acceptance remains bound to its captured sources; registry installation and its
fresh consumer checks are verified above. Logs are under `target/deploy-metadata-libproc/`.
The existing CI matrix owns native runtime acceptance asynchronously, under the
maintainer's explicit direction to push without waiting for CI.

Process qualification adapters select 0.80.3 while retaining their source-drift
refusals. Updating a supplier selection does not claim a new Canic migration;
its earlier source-bound evidence remains historical. The archive gate now runs
Clippy before its expensive runtime checks.

### Testkit 0.107.4 publication

Published on 2026-11-16 from `67ac61f89a7311e278027bee3078de7e24a0a241`.
`main` and annotated tag `testkit-v0.107.4` were pushed atomically and verified.
Cargo's clean-source, all-feature publication admission passed before upload.
The non-yanked registry archive is byte-identical to both dry-run and uploaded
archives, SHA-256 `8629db43fa7a3e3655184c6173a5ccdecf67a87d29f0c929228bec7f8a1c392b`.
Its clean VCS provenance, source, original/normalized manifests and licence match
the release and qualified candidate. A registry-only consumer resolves exact
Testkit 0.107.4 with exact Protocol 0.2.7 and held-reply fixtures without overrides;
[Tooling #9](https://github.com/dragginzgame/ic-tooling/issues/9) is closed.
Post-push review of the delivered package and directly affected callers found no
actionable defect. No new native macOS pass is claimed.

The maintainer authorized Protocol adoption and deployment on 2026-11-16.
The selected public group is Testkit 0.107.4: the optional setup/check/run CLI and
a compatible Protocol requirement from 0.2.7. Delivery uses this checkout's
`main`, `origin/main`, annotated `testkit-v0.107.4`, and crates.io. Other public
package source/requirements are unchanged; the root's added optional CLI
dependencies are used only by Testkit. Qualification/documentation maintenance
delivers through Git, with the private tools ledger remaining pending.

Protocol archive identity and the unchanged HTTP encoder are recorded in the
[adoption review](qualification/platform-adoption.md#protocol-027-deployment).
The selected complete local gate installs and exercises the extracted Testkit
archive, including feature isolation, lint/docs, managed-server and held replies.
Hosted native CI runs asynchronously; earlier macOS passes are not new CLI or
Protocol acceptance. Registry installation is verified separately from archive
qualification; its receipt is recorded below.

The complete Linux gate passed at `target/library-release.KR6eIDms/`: default
and enabled-feature lint/tests/docs, isolated dependency graphs, installed
archived CLI setup/IO/failure/cleanup, both held-reply checks with three observed
deletions, and both managed-server installed checks. Source/tool hashes remained
stable. Its precommit archive SHA-256 is
`f730d5125d08ff0e006e71f6a87695510323551eb4948ec82f37306c79bc7625`.
The current Scheduling pressure/held-upgrade pass and exact Protocol admission
are retained at `target/scheduling-new.wJUWIZWv/`. Logs, independent lock-drift
refusal and registry observations are under `target/deploy-01074/`.

The existing Toko installer successfully installed the published `cli` binary in
release profile, then admitted that exact selection with offline `--check`.
`--version` reports `ic-testkit-server 0.107.4`; offline prepared-server admission,
piped input/output, preservation of child exit 23 and independent endpoint closure
pass through the registry-installed executable. Installer compilation took 106
seconds; its phase log, selected path and runtime evidence are retained under
`target/deploy-01074/`. Tooling #10's remaining native-host acceptance stays with
asynchronous CI; registry delivery and supported installation are complete.

Subsequent [CI run 43968510609](https://github.com/dragginzgame/ic-tooling/actions/runs/43968510609)
at `a00f69718564c323ac96812c6986a82c8a872533` passed on Linux and both native macOS
architectures. Every library lane passed actual setup, CLI cleanup and packaged
library acceptance; the Build-stack lanes also passed. Tooling #10 is closed.

### Testkit 0.107.3 publication

Published on 2026-11-16 from `3c3fd63a6c16f4f1c38eef7984d211791022ba37`.
`main` and annotated tag `testkit-v0.107.3` were pushed atomically to the canonical
GitHub repository. Cargo verified the clean committed package before upload.
Registry readback is non-yanked and byte-identical to the uploaded archive, with
SHA-256 `f1abdecdc13e767e8f5f7f83f2c19e48a414d6a7eea2e2ffdbfe95222c56d6c2`.
The index checksum, clean Cargo VCS identity, source/examples, original/normalized
manifests and licence match the release source. Protocol is 0.2.6, Types remains
compatible with 0.21.2, and the optional Host dependencies resolve from crates.io.
Toko 0.44.11 is installed and selected in both maintained workspaces and the
reviewed cache actions. Hosted CI is asynchronous; no new native macOS result is
claimed. Post-push review of the delivered package and its archive/CI callers
found no actionable defect. Canic adoption remains with
[Canic #15](https://github.com/dragginzgame/canic/issues/15).

The maintainer authorized Toko adoption and deployment on 2026-11-16. The selected
public group is Testkit: compatible opt-in managed-server APIs and Protocol 0.2.6.
Other libraries have no changed public source or runtime requirements; the
inactive Toko marker does not require their republication. The private tools
ledger stays pending and its qualification maintenance delivers through Git.
Delivery uses this checkout's `main`, `origin/main` and `testkit-v0.107.3`, with
crates.io publication after local archive qualification and package admission.
Hosted native CI remains asynchronous under the existing patch-release policy.

The Testkit archive gate now selects both optional features and qualifies the
managed-server refusal tests and two installed lifecycle checks from the extracted
archive, alongside the existing held-fixture tests and independent deletion
observation. It additionally requires absolute `TESTKIT_SERVER_ARCHIVE` and
`TESTKIT_GZIP` inputs, whose hashes are retained with the launcher/server hashes.
Hosted setup selects `archive.gz` from the launcher's verified bundle and
`/usr/bin/gzip`; neither ordinary tests nor the archive gate download assets.
The existing bootstrap remains separate from the newly published library API.

Pre-version archive gate `target/library-release.VQtbHJuf` passes ten unit cases,
both installed held-fixture cases with all three deletions observed, and both
managed-server installed cases. Default and feature-enabled lint/docs, graph
isolation and source/tool stability checks pass. The versioned 0.107.3 Cargo
publication dry run also passes. Its source and examples match that qualified
archive exactly; the normalized manifest differs only in package version.
Afterward the runner's comma-containing feature argument was quoted without
changing its value; ShellCheck and Actionlint pass on final wiring. Snapshot,
metadata and formatting admission pass with Toko 0.44.11. Evidence is retained
under `target/deploy-testkit/`; registry publication and readback are recorded above.

### Initial independent library publication

The maintainer authorized HTTP 0.80.0, Text 0.77.2 and Testkit 0.107.1 publication
after preparation and the Platform/Toko adoption. All three were published on
2026-11-16 from `18198d6cfe6c097ef007a1a6a8b312c0a394e211`. Annotated tags
`http-v0.80.0`, `text-v0.77.2` and `testkit-v0.107.1` were pushed atomically with
`main`. The private tools package and consumer migrations are outside this delivery;
Backup adoption remains maintainer-owned.

Cargo verified each package before upload. Registry readback confirms non-yanked
versions, matching index checksums and byte-identical preflight/upload/download
archives. Their source, original manifests, licence and clean Cargo VCS provenance
match the tagged release. No local dependency override is present in the normalized
manifests. HTTP/Text sources match their qualified archives; their normalized
manifest change is only the inactive Toko marker from 0.44.5 to 0.44.6. Testkit's
source and normalized manifest match its final Protocol-qualified archive; the
commit hook only reformatted the original manifest's feature array.

| Published archive | SHA-256 |
| --- | --- |
| HTTP 0.80.0 | `81c55b5294ff4c4bcab841ca6b72140796abd5c095651bf7e44e77ac2f5f843a` |
| Text 0.77.2 | `421d01724ce7a61567333c6fd1a18ccf7878958cff441729581f69959a2389bb` |
| Testkit 0.107.1 | `a4fe4ea7882118e731ef1ebe8c668fc5dfa5252c1c972b1de68bc444868fae42` |

The [library readiness matrix](https://github.com/dragginzgame/ic-tooling/actions/runs/43899230778)
passed on Linux, Apple Silicon macOS and Intel macOS at `0329953`, before the
Protocol migration. Migrated Testkit's final local archive acceptance is recorded
below. Hosted checks for the newer adoption/release source remain asynchronous
under the maintainer's direction; the earlier matrix is not represented as new
Protocol qualification. Publication logs, registry records, archives and provenance
checks remain under `target/publication-libraries-2026-11-16/`.

## Independent library readiness

The initial preparation request selected HTTP, Text and Testkit independently;
the later publication authorization and receipts are recorded above. Preparation
did not include the private tools package. The Canic source-review documentation
is recorded in a separate documentation commit. No Backup or other sibling source is changed.

| Group | Candidate | Latest registry release at review |
| --- | --- | --- |
| http | `ic-host-http` 0.80.0 | 0.79.0 |
| text | `ic-host-text` 0.77.2 | 0.76.0 |
| testkit | `ic-testkit` 0.107.1 | 0.106.0 |

Pre-publication registry review on 2026-11-16 confirmed all three candidate versions
were absent; their new minor lines carry the incompatible replacement APIs. At the initial
review, first-party selections were current stable, non-yanked releases: Platform
0.33.7, Platform Types 0.20.4 and Toko Tooling 0.44.5. Subsequent maintenance adopts
Toko 0.44.6. The [Platform migration](testkit-held-replies.md#platform-034-migration-review)
now adopts Protocol 0.2.3's published HTTP encoder and Types 0.21.1, removing
Platform 0.33.7 and its internal Types 0.20.4 edge. The certificate-only Platform
0.34.3 package has no role in Testkit's fixture graph.
[Platform #5](https://github.com/dragginzgame/ic-platform/issues/5) supplied the
missing capability. Final publication admission must use this current selection.
The original installed-test bootstrap used released `ic-testkit-server` 0.106.0
with PocketIC 16.1.0. Current Tooling qualification replaces that external tool
with the optional [Testkit CLI](simulator-ownership.md). The gate installs it from
the extracted candidate and tests that exact executable. Historical receipts retain their inputs.

`make test-library-release-candidate RELEASE_GROUPS='http text testkit'` selects
only these groups; either an individual group or a subset is supported. Testkit
requires absolute `TESTKIT_SERVER_CLI`, `TESTKIT_SERVER_DIRECTORY`,
`TESTKIT_SERVER_ARCHIVE` and `TESTKIT_GZIP` paths. The runner admits prepared assets
offline, never downloads tools implicitly, and records hashes before and after use.
Set `LIBRARY_RELEASE_ONLINE=true` only when preparing uncached dependencies.

The gate runs warning-denied Clippy before packaging, compares packaged source
and licence bytes, then tests the extracted archives and builds warning-denied
Rustdoc. Testkit's default graph must exclude fixture-only WAT/Platform edges;
default and feature-enabled configurations are checked separately. Installed
module and controller tests run against the candidate on an externally owned
server, followed by independent observation of all three deleted instances.
Logs, archives, checksums, source capture and the candidate lock remain under
`target/library-release.*`; changed source refuses completion. No sibling migration
or older consumer test run substitutes for this archive gate.

Both archive runners retain fresh evidence while reusing disposable dependency
compilation output. See the [cache contract, invalidation regression and local
measurements](qualification/archive-cache.md); use
`ARCHIVE_QUALIFICATION_CACHE=false` to request a fresh compilation directory.

The existing GitHub workflow now has a separate `libraries` matrix for Linux,
Apple Silicon macOS and Intel macOS, sharing the single repository secret scan.
The initial incompatible HTTP, Text and Testkit publications retain this specific
new-host-matrix qualification requirement. It applies to these first replacement
releases, not subsequent routine patches or unrelated groups. Routine patches
proceed after their required local archive gates and publication admission; hosted
CI is asynchronous unless an enforced protection or a maintainer-selected, named
qualification requires waiting. Known relevant failures still need resolution or
explicit disposition. Report the requirement and its source before waiting.
Readiness alone does not authorize tags or uploads; release notes remain undated
until authorized delivery.

Linux archive run `target/library-release.vBZQpJd2` passes two HTTP tests, three
Text tests, six Testkit unit tests and both explicitly selected installed tests.
All three server instances are independently observed as deleted. Both Testkit
feature configurations pass Clippy/Rustdoc and default graph isolation. Archive
source matches the linted source; source/tool hashes remain unchanged throughout.
Formatting, ShellCheck, Actionlint and missing-group/tool refusals pass. Registry
ownership confirms `borovan` for all three names. The library native matrix later
passed at `0329953`; prior four-crate results do not qualify these libraries.
All three `cargo publish --dry-run --allow-dirty --locked` checks pass, with
verification enabled and no uploads. Their generated archives exactly match
the qualified archives. Cargo's registry preflight requires network access even
for a dry run; the refused offline attempt is retained beside the passing logs.
The final Protocol 0.2.3 / Types 0.21.1 adoption requalifies only Testkit in
`target/library-release.sCTEKGgV`: six unit tests, both installed tests, deleted
instance observation, lint/docs and default-graph isolation pass. The feature
graph excludes the old Platform/Types packages. Earlier Testkit archive/dry-run
receipts retain their older selections and are superseded for this candidate.
The refreshed Testkit publication dry run passes with verification enabled;
its archive exactly matches the final qualified archive. These readiness checks
performed no uploads; subsequent authorized publication is recorded above.

## First production cutover scope

### Foundation 0.80.2 follow-up

The maintainer authorized publication of foundation 0.80.2 on 2026-11-16,
excluding Backup migration. Artifacts and FS remain one coordinated group;
Artifacts was published first, then FS verified against that registry supplier. Artifacts
has no runtime change. FS adds original retained-file admission for create-only
publication; no existing byte or persistence format changes. Other package
versions and all external dependency selections remain unchanged.

Product source is unchanged from `0ace03b62684b876c3dc43cdb7280bbc980fa58c`, whose
[hosted acceptance](https://github.com/dragginzgame/ic-tooling/actions/runs/43858852879)
passes repository admission, the single history secret scan and packaged native
tests/Clippy/Rustdoc on Linux and both macOS architectures. Early foundation
Clippy also passes locally. Final versioned archives pass
[run 43862099971](https://github.com/dragginzgame/ic-tooling/actions/runs/43862099971)
at release commit `828c2692523a7a906188c111c9b58f30d0f52468` on all three native
hosts, including repository admission and the single secret scan.
The required first-party product supplier is Artifacts in this same cohort;
Toko 0.44.4 was the reviewed qualification supplier. Registry review found
stable non-yanked Toko 0.44.5; its refresh was deferred during publication and
subsequently adopted in the next tooling-maintenance increment. No freshness
claim is made for unrelated product or qualification graphs.

Both 0.80.2 packages were published on 2026-11-16 from release commit `828c269`;
the annotated `foundation-v0.80.2` tag and `main` were pushed atomically.
Downloaded registry archives are non-yanked, match uploaded bytes and index
checksums, and contain the release commit's exact source and licence. Cargo
verified FS against registry Artifacts 0.80.2 without local overrides.
The final normalized manifests match the CI candidates. Qualification-only
unused patch records are absent from the publication locks, and FS's publication
lock identifies Artifacts by its registry source/checksum.

| Published archive | SHA-256 |
| --- | --- |
| Artifacts 0.80.2 | `f8645c876c9f26469c46e0ec764b639b192d2f2336d71815a2fd11d4c86b6f1e` |
| FS 0.80.2 | `81c73a53b929355c2056a2783c0f61676251c0ddba2d6e0109c3ac53841b396b` |

Publication logs, downloaded archives, source/provenance checks and Linux CI
evidence remain under `target/publication-foundation-0.80.2/`. Backup was not
modified or migrated; the maintainer owns that next step.

### Initial four-crate publication

The maintainer selected first production adoption as the next milestone, ahead
of completing every legacy workload. The initial publication on 2026-11-16 consists
of the dependency-complete owned-build stack:

| Package | Published version |
| --- | --- |
| `ic-host-artifacts` | 0.80.1 |
| `ic-host-fs` | 0.80.1 |
| `ic-host-process` | 0.80.2 |
| `ic-build` | 0.87.3 |

Artifacts, FS, Process and Build were published in that order from commit
`ca0bef0cd29ab272c3e3419d28292ca83ecff875`. The public canonical repository is
[dragginzgame/ic-tooling](https://github.com/dragginzgame/ic-tooling); the legacy
repository had already moved to `dragginzgame/ic-tooling-legacy`. Release tags
are `foundation-v0.80.1`, `process-v0.80.2` and `build-v0.87.3`.
All four versions were absent before upload and are now available, not yanked.
Cargo package verification ran for every upload; Build resolved its three
suppliers from crates.io without local overrides. Downloaded archives match
the uploaded bytes, index checksums and committed product source. Receipts are
under `target/publication-2026-11-16/`.

| Published archive | SHA-256 |
| --- | --- |
| Artifacts 0.80.1 | `b9083e3927eb9649c429a1c8dfeb5b7be844cebf1596e93b077f9908d6289dcc` |
| FS 0.80.1 | `6dfb8ed3f645487c12ec0f7a8384bc1750e67649bdf7062bb12f136b3384e461` |
| Process 0.80.2 | `d4189c2a9262d34336c59ca7b22851cab1c629cd9afa98cd28a878e124d2e6d1` |
| Build 0.87.3 | `dfaf7d08e0033e1a245fa84f87098bb1fd7229947183e5a47f6e0f1b957ca433` |

Current Canic owned-build qualification is the starting consumer evidence;
production adoption must use its then-current source and account for its ongoing
rewrite. Native macOS acceptance is required by the adopted baseline; the first
four-crate candidate now satisfies that gate on both supported architectures.

Text 0.77.2, HTTP 0.80.0 and Testkit 0.107.1 are independent later adoption batches;
they are not dependencies of this first stack. The private qualification executable
0.1.3 remains unpublished. Backup restore failures and frozen/on-hold consumers
do not block this scoped build release. Publication is complete; consumer
cutover remains separate. No sibling source or deployment was changed.

## Candidate qualification and delivery boundary

The first four-crate stack passed readiness before publication. [Run 43841772100](https://github.com/dragginzgame/ic-tooling/actions/runs/43841772100)
passed at `6541ba0d53bb4d99fc358fbed5850f0f5cc2df8b` on
`release/build-readiness`: Ubuntu 24.04, macOS 15 Apple Silicon, macOS 15 Intel
and the full-history secret scan. Extracted archives pass 61 Linux unit cases
or 60 macOS unit cases, plus the foreground-terminal case on each host,
warning-denied Clippy and Rustdoc. The run retains each host's archives, hashes,
source identity and logs. The publication commit changes repository metadata,
release notes and the CI branch trigger; product source and Cargo.lock are
unchanged from this qualified commit. Runtime evidence is retained, with fresh
Cargo package verification and registry archive admission at publication.

The latest local complete candidate run is `target/build-release.1XQOqMeC` at
`076d33dfab15ceb14282be0a0efed54ff6e4f8f2`, before the terminal fixture/driver
correction in the passing CI commit; product runtime code is unchanged between
these commits. Its current Canic export `target/canic-build.IANsOjKU` has HEAD
`2ee4add03126aa2f5259c609fa05eae8a14bcd14` plus working edits, source archive
SHA-256 `7547a001603207db3b2a2ab87c051e1c07f04478784a7a07274a7c3b80987056`.
The real Wasm build and 20 metadata/tool refusals pass with all four extracted
candidate paths admitted. This is Linux consumer evidence; hosted native jobs
qualify the four-crate core, not a macOS Canic migration.

`make test-build-release-candidate` creates four Cargo archives under a fresh
`target/build-release.*`, records hashes, extracts them, and qualifies only those
four packages. Normalized manifests retain registry dependency requirements.
Local registry overrides supply the unpublished candidate dependencies during
qualification; they are not a claim that crates.io can already resolve them.
Tests run against extracted source, not product checkout paths. Build's separate
owned-Wasm probe is copied as qualification input; it is not shipped in Build.

The command runs native unit and foreground-terminal tests, warning-denied Clippy
and Rustdoc, then captures current Canic read-only for its real Wasm build and
20 metadata/tool refusal cases. Dependency admission requires the extracted
candidate paths. It does not expand this first cutover to the whole CLI migration.
Existing deployment, signing and application data remain untouched.

Linux archive evidence: `target/build-release.xrogPDbD` passed 61 unit cases,
Clippy, Rustdoc and the current Canic capture `target/canic-build.QveKsBGz`.
That consumer capture has HEAD `2ee4add03126aa2f5259c609fa05eae8a14bcd14`
plus local edits, source archive SHA-256
`fc0d640b0caa0a5a9e0122bca15810968b8ab09123d19d9a07fa8754ecd03ef6`.
The later native-only run `target/build-release.asOphwy1` also passes the explicit
foreground-terminal case (62 cases total). The first terminal evidence parser
refused colored output; selecting libtest `--color never` corrected the parser,
without weakening the test. Archive hashes and per-command logs live with each run.

The GitHub `Build release candidate` workflow checks Ubuntu 24.04, macOS 15 Apple
Silicon and macOS 15 Intel after one repository job runs `make check` and
`make fmt-check`. That job explicitly prepares Rustfmt, Clippy and the common tools using
the shared installers and pinned download cache. Pull requests have no path
filter, so documentation and tooling changes receive repository admission and
the single history secret scan too. [Run 43858083308](https://github.com/dragginzgame/ic-tooling/actions/runs/43858083308)
at `a66eae8` passes repository admission, formatting, the secret scan and all three
native jobs. Its setup log exposed implicit Clippy preparation during admission;
the follow-up selects both repository-required Rust components in explicit setup.

The native matrix checks Ubuntu 24.04, macOS 15 Apple
Silicon and macOS 15 Intel. It prepares Rust 1.100.0, Clippy and the Wasm target;
`BUILD_RELEASE_ONLINE=true` permits dependency preparation before locked offline
tests. `BUILD_RELEASE_CONSUMER=none` explicitly excludes the unavailable sibling
on hosted runners; the Linux current-consumer evidence is separate. It uploads
archives, source identity, logs and completion evidence even after failure.
The first hosted run exposed four Linux-sized mode assumptions in FS creation;
the shared permission validator now returns Rustix's native mode after rejecting
bits outside 0777. It also exposed missing ripgrep on the Linux runner; the single
native log assertion uses standard grep. The next native run found that Rustix
does not expose `mkfifoat` on macOS; the lock refusal fixture now creates its FIFO
using native `mkfifo`, retaining the same product assertion. The passing run
above includes these fixes.
An executing macOS run also reproduced concurrent sidecar creation returning
`ENOENT` from `openat(O_CREAT)`. Sidecars now use exclusive creation, opening an
existing entry only after `EEXIST`; no missing-name retry or repair is introduced.
The first executing macOS tests also found Darwin's zombie-only group `EPERM`
behavior. Process now proves exited-leader/no-other-member state before treating
that specific refusal as complete; uncertain or live groups retain cleanup errors.
See the [native execution contract](process-execution.md) for the dependency and
custody boundaries. Xcode/Command Line Tools with libclang are required on macOS.
The required Gitleaks history scan runs once outside the host matrix using the
installed pinned shared installer. Native jobs need Bash, Git, jq, grep, Perl,
tar, shasum and mkfifo. Linux uses util-linux script for its PTY; macOS uses native
Expect to wait for the child's readiness prompt before sending input. BSD script
injected EOF ahead of piped input in the first terminal run. The fixture still
requires terminal stdin/stderr and the exact answer; it does not skip the contract.

Before publishing: require the selected commit's native jobs to pass, review the
archive inventory and candidate hashes, and recheck registry versions and owner
credentials. Publish in dependency order, verifying each supplier's registry
checksum before resolving and publishing its dependent. A publication request
remains separate from this readiness work. Current source is intentionally
incompatible with legacy APIs; consumers need the explicit Build adapter before
changing versions. After publication, rerun the selected consumer against registry
packages without local overrides before production adoption.
