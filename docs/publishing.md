# Releasing and publishing the workspace

## Git releases

The maintainer-operated `make release-patch`, `make release-minor` and
`make release-major` commands use the [shared release contract](releases.md).
Host's [delivery policy](../ci/release-delivery.txt) selects `RELEASE_DELIVERY=direct`.
Normal increment commands use Shared's [frozen release batches](release-batches.md):
capture committed source, qualify, prepare, commit/tag, atomically push, then
publish each selected group to crates.io. **Do not follow a successful release
with a second `make publish`.** Other delivery selections are rejected;
the shared runner's optional PR flow is not qualified by this adapter.

Select `RELEASE_GROUP=foundation`, `build`, `text`, `tools` or `testkit` explicitly
for a single-group release. For a batch, select `RELEASE_GROUPS` once as a
comma-separated supplier-first roster. Unknown, duplicate and reversed workspace
dependency selections refuse before capture; every group uses the same increment.
Shared defaults the roster to the selected group. Clearing the roster cannot
return a selected group to live-checkout delivery.
The
[group policy](../ci/cargo-release-groups.json) retains one qualification workspace
with independent version sources and ledgers; see
[release ownership](design/release-groups.md). For example:

```sh
make release-minor RELEASE_GROUP=tools
make release-patch RELEASE_GROUPS=foundation,build
make release-batch-status BATCH=batch-XXXXXXXX
make release-batch-resume BATCH=batch-XXXXXXXX
```

The default `RELEASE_PUBLISH_TARGET=publish` keeps Git and registry
delivery in the same captured batch while preserving their separate receipts.
Select an empty value explicitly for Git-only delivery. `release-resume
RELEASE_GROUP=... VERSION=...` is for historical normal-checkout attempts; batch
recovery uses the recorded batch ID and resumes the captured single-group plan
internally. Uncertain publication refuses redispatch. Verify registry/receipts,
then use `PUBLICATION_RESULT=published` or `not-published` on batch resume.

Shared 0.31 uses schema-2 batch journals and a recorded publication commit.
Historical schema-1 batches require their original reviewed tooling for recovery;
do not rewrite their journals or infer delivery from the current branch.

The runner prints `target/release-frozen/BATCH` and durable state under
`.git/release-state/frozen/BATCH`, with a source pin under `refs/releases/frozen/`.
Start from clean committed source. Later normal-checkout lock/source edits stay
deferred and cannot change active validation or uploads. The runner does not
update the normal branch automatically. After preserving deferred work, use
Shared's `release-batch-reconcile BATCH=...` on a clean checkout to import and
fast-forward the delivered refs, then deliberately reapply the deferred changes.
Reconciliation never stashes, resets or resolves conflicts. Retain candidate Git,
plans, publication receipts and unfinished work; normal `cargo clean` removes the
managed checkout too, so it cannot run during delivery. Git/Cargo context
overrides and mutable ancestor configuration retain Shared's explicit limits.
`release-batch-retire BATCH=...` explicitly deletes that completed batch's
checkout, ignored build/tool outputs and recovery records after integration and
custody checks. Preserve needed evidence separately before selecting it; adoption
or successful publication does not authorize retirement.

Shared invokes `release-candidate-setup` before fresh version lookup and before
resumed release/publication hooks. Its `prepare` mode selects host parsers; `check`
admits the saved selection offline without installation. The standard release
preflight retains editor/server setup and full product qualification. Read-only
version queries still use Cargo metadata without installing tools. Setup uses existing owners and verified shared
stores; no normal-checkout tools, locks or compilation artifacts are copied.
`bash scripts/release/test-frozen.sh` qualifies five fresh group queries, real
metadata/Git hooks, two selected publication dry runs, deferred lock bytes,
interrupted/uncertain recovery, completed resume and remote conflicts. Its product
gate bodies are substituted; the installed release gate and actual registry
receipts remain independent required proof.

Every package has an explicit version owned by its group primary; the root
Cargo metadata has no version. Root `CHANGELOG.md` indexes the
[Foundation](../releases/foundation/CHANGELOG.md),
[Build](../releases/build/CHANGELOG.md), [Text](../releases/text/CHANGELOG.md),
[Host Tools](../releases/tools/CHANGELOG.md),
[Testkit](../releases/testkit/CHANGELOG.md) ledgers and the retired
[Native history](../releases/native/CHANGELOG.md). Original joint integration
notes remain in the [frozen ledger](../releases/integration/CHANGELOG.md), with
earlier coordinated notes retained in [shared history](../releases/history.md).
Group tags and saved plans are prefixed with the group.
One queue per normal checkout serializes frozen batches; the captured checkout
retains its standard single-group release lock.

Prepare the development toolchain selected in [rust-toolchain.toml](../rust-toolchain.toml),
then run `make install-tools` and `make tools-check`. Shared Tooling owns
[common setup and executable storage](local-setup.md). Release preparation uses
the released IC Build `ic-build-release-metadata` editor, pinned separately in
`ci/release-editor.env` and selected through its verified tool-store receipt.
Shared admits explicit group membership and reads member identities from the
selected source; IC Build projects only the group's owned versions and supplier
requirements. Committed checks use the selected commit independently of today's
working files. Preparation validates candidate TOML and the prepared
locked Cargo graph, restoring original bytes/modes after failure or catchable
interruption and retaining recovery evidence. Shared retains the preparation
transaction, receipts and rollback. `make install-tools` includes the release
editor; `make tools-check`/`make release-editor-check` admit it offline. Preflight
can prepare a missing editor after source admission without per-shell exports;
standalone verification never installs. Workspace libraries and release
qualification require the pinned Rust 1.99.1 compiler. No Cargo-edit command renders release metadata.

The read-only `make release-version RELEASE_GROUP=<group>` query needs Cargo,
jq and the locked offline workspace metadata; it installs no tools. Prepare
tools with `make install-tools` / `make tools-check` and missing dependencies
explicitly with `cargo fetch --locked`. Explicit server setup fetches locked dependencies before
building its setup CLI offline. Selected release preflight admits its
[validation plan](design/release-validation.md), prepares the applicable tools and
fetches workspace dependencies with `cargo fetch --locked` before offline
validation. Cargo honours explicit offline settings: missing inputs stop the
release without an online retry. Standalone `make release-verify` checks prepared
inputs offline and requires the runner's selected release environment; it is not
a standalone release command. Neither cache preparation nor validation changes
the selected dependency versions.

The complete gate runs `make clippy-workspace` after prerequisite admission and
before expensive tests or builds, with the existing workspace/all-target/all-feature
scope and warnings denied. Failure stops later stages under parallel Make too.
Native hosted CI combines this gate with read-only workspace qualification through
`make ci-publication`: `workspace.sh qualify` selects the complete public roster,
runs one lint gate and Cargo dry run, then the remaining offline CI checks.
Both full CI entrypoints derive their ordered checks from the release-validation
policy through Shared roster admission; there is no separate Make check list.
This mode cannot upload or select a subset. Standalone `make ci` and
`make publish-check` each retain their full lint gate. This combined command
does not upload packages or authorize publication.
Full release verification and CI use the same Rust 1.99.1 qualification: native
workspace checks, isolated feature profiles, Testkit Wasm and unsupported-filesystem
compilation. `make compiler-check` admits the actual compiler; `make ci` retains
the complete gate without the retired `msrv-qualification` target.
Release verification uses Tooling's [source-bound group plans](design/release-validation.md)
through Shared's [plan runner](release-group-validation.md). Foundation and Testkit
retain the complete gate. Text, Tools and Build may narrow only from
an admitted complete three-host baseline with unchanged shared inputs and bounded
consumer relationships. Tools/Build need a fresh comparison that executes their
new focused contracts on all three hosts; older Text qualification is
insufficient. Missing evidence, shared changes or uncertain scope select the
complete gate; selecting a group alone never omits qualification. Ordinary CI
and publication qualification keep their complete workspace gates.
Both numbered pending-heading spellings, `## [X.Y.Z]` and
`## [X.Y.Z] - Unreleased`, are finalized to the saved UTC date during preparation.

Follow the [shared recovery procedure](releases.md#source-changes-after-saved-preparation-intent)
for interrupted preparation and exact-version resume. Host preserves prior
validation receipts, notes, gate directories and logs before fresh verification. Retain unfinished
plans, earlier tool installations and failed artifacts; a retry must not relabel
old evidence as validation of new source. Native qualification belongs to the
[host guide](hosts.md), independently of release and publication effects.
Old ungrouped and joint `integration` attempts remain under their original identities. Recover them
using their recorded pre-group source/adapter in an isolated checkout; never
rename their plans or receipts into a group namespace.
Earlier `tooling-release-validation-2`, `tooling-release-validation-3` and
`tooling-release-validation-4` receipts retain their original adapters. Version 5
also seals the original proof and logs for admitted unchanged-stage reuse before
version preparation, then checks the frozen inventory during payload/recovery
admission. See [stage reuse and custody](design/release-validation.md#selected-adapter-cases-and-unchanged-stage-reuse).
Never reinterpret an older receipt as version 5.

## Registry publication

The consumer-owned [public package list](../ci/public-packages.txt) selects both
publication and archive qualification. Publication checks it against
Cargo metadata before running lint or packaging; private test packages stay out.

Publication requires explicit `RELEASE_GROUP=foundation` for its five public
packages, `build` for IC Build, `text` for Host Text, `tools` for Host Tools, or
`testkit` for Testkit.
Private Testkit harnesses are never uploaded. For a Host Tools release:

```sh
make publish-check RELEASE_GROUP=tools
make publish RELEASE_GROUP=tools
```

An explicit command-line
`PACKAGE='name ...'` selects a nonempty batch from it for either publication mode:

```bash
make publish-check RELEASE_GROUP=foundation PACKAGE='ic-host-artifacts'
make publish RELEASE_GROUP=foundation PACKAGE='ic-host-artifacts'
```

Unknown/private names, duplicates and an explicitly empty selection are refused.
The default `PACKAGE` used by development commands, or an environment-only value,
does not narrow publication. Missing, empty or unknown groups are refused before
validation or effects, including when `PACKAGE` is supplied. A batch must stay
inside its selected group; cross-group batches are refused. All public workspace metadata must
still satisfy the complete roster, stable-version, registry and repository policy.
Each selected archive
receives the same license and provenance admission, exact version selectors,
source-drift checks and retained-attempt handling as a complete upload. Workspace
archive qualification still selects the complete roster independently of a batch.

`make publish-check` runs Cargo's publication checks with all features
and `--locked --registry crates-io --dry-run`. It packages and verifies the selected
workspace packages through exact `--package NAME@VERSION` selectors without
uploading. The default selection is the chosen group's public members. Across
all five groups, the roster remains six Host libraries, `ic-build`, `ic-testkit`,
and the `ic-host-wasm-inspect` CLI. The development driver and
Wasm probe remain private. It admits working edits so packaging can be reviewed
before committing. It can access the registry; set `CARGO_NET_OFFLINE=true` for
an offline attempt against already prepared caches. Registry lookup can require
network access even with cached dependencies. Offline failure never triggers an
online retry.

Both publication modes require prepared dependencies. After fresh source and
metadata admission, they reuse full workspace lint only through the selected
committed release's preflight selection context and version-5 receipt. Tooling
carries that context when saving preflight selection; it never parses Shared's
private release journal. The context grants reuse only after receipt re-admission.
Older selections lacking context fall back to fresh lint. Admission checks the exact
prepared payload, unchanged source, gate/log inventory, environment, compiler and
host, and requires a completed full workspace lint stage. Missing, stale, corrupt
or focused-only evidence falls back to offline `make clippy-workspace` before
packaging or registry effects. The admission attempt retains `lint-evidence.log`.
Whole-workspace `qualify` always runs fresh lint. Archive, license, provenance
and source-drift checks stay fresh in every publication. The wrapper never
installs tools or fetches missing lint dependencies.

`make publish` uploads each selected package at its own current committed
version to crates.io.
It requires a clean working tree, eligible selected publishable packages and
prepared Cargo 1.99, Git, tar and the pinned local jq. Cargo owns dependency ordering,
package verification and index waiting. Package policy restricts each package to
crates.io; the wrapper always selects that registry explicitly. Credentials stay
with Cargo's normal login or credential-provider configuration. Neither command
creates commits, changes versions, creates tags or pushes Git refs. Normal frozen
release commands invoke this upload target after Git delivery; standalone
publication remains available for separately reconciled Git-only releases.

Before any upload, the wrapper prepares all selected archives with `cargo package
--package NAME@VERSION` for every selected public package, with
`--all-features --locked --no-verify`. This step resolves package
dependencies but does not compile or upload; the final Cargo publication still
verifies packages. Each archive's `.cargo_vcs_info.json` must name the selected
clean `HEAD` and the package directory recorded by admitted Cargo metadata.
Package paths must be physical directories beneath the checkout, without parent
traversal or CR/LF. License notices are checked in those same directories after
metadata admission and before package preparation or upload. The explicit package
allowlist remains separate from workspace membership. All package metadata must
declare `https://github.com/dragginzgame/ic-tooling` as the repository.

Shared's [canonical provenance helper](../scripts/ci/check-cargo-package-provenance.sh)
checks the prepared archives and fetches that exact commit from the declared HTTPS repository
into a fresh bare object store beneath the publication attempt. Failed fetches,
mismatched commit identities, missing/dirty package provenance and source changes
stop before upload. Git prompts are disabled; ordinary trusted Git transport
configuration still applies. The source request, archive provenance, fetch log
and retrieved objects are retained in the admission attempt's `provenance/`
directory. This guard is a remote-read effect of the
explicit upload command; it never pushes a ref or creates a commit.

The [reviewed Shared snapshot](../.shared-tooling.snapshot) supplies this helper and the
[selected-source reader](../scripts/ci/read-cargo-source-context.sh), alongside
the existing release metadata/receipt custody. Host selects packages, manifests,
versions and recovery; Shared owns exact commit retrieval, archive provenance
and selected-source member observation. There are no local source-evidence copies.
Both Git object formats, ambiguous parser-stream refusal and retained failure
evidence remain part of admission; replacing the helper owner removes no check.

`publish-check` retains its dirty-worktree dry-run contract and does not claim
remote provenance verification. The upload guard does not establish CI success
or native host qualification, and callers must keep source stable throughout
packaging/publication. See [#7](https://github.com/dragginzgame/ic-tooling/issues/7).

Most packages use the standard MIT SPDX identifier. Testkit uses
`MIT AND Apache-2.0` for its maintained PocketIC client and includes the imported
Apache licenses, notices and reconstruction evidence under
`pocket-ic-provenance/`. Its source guards preserve those original bytes.
Each selected package's root `LICENSE` remains a regular copy of the canonical
MIT notice for Tooling-owned code, compatible with the formatting hook and
included in the package archive without redundant `license-file` metadata.
Both publication commands reject missing, symlinked or differing
notices before invoking Cargo. The CI publication fixture preserves real package
notices, so the complete release gate observes source drift instead of synthesizing
matching notices. Update every selected package's copy when the root notice changes.

Before upload, review the dry run and confirm that the committed version is the
intended registry release. Inspect current registry state for that exact version;
never republish an existing version with changed source or metadata. Pending
changelog entries do not change package versions. A Git tag does not prove
registry publication or host qualification.

Host retains its selection, metadata and provenance in
`target/publish/admission/check.*` or `target/publish/admission/publish.*`.
The `publication-attempt` file links to the corresponding fresh command directory
under `target/publish/check.*` or `target/publish/publish.*`. The selected
[Shared attempt helper](cargo-publication.md) records exact package/version
selectors, source, arguments and working status before its dispatch marker. It
retains Cargo output plus separate `cargo-status`, `log-status` and combined
`status` files. Cargo failure takes priority over a logger failure; a logger failure
after successful Cargo execution also refuses success.
New admission intent records use `publication-2` with the mode, source, registry
and exact `NAME@VERSION` selections; `release-group` records the selected group.
Read-only all-group qualification records an empty group and `purpose=qualify`.
Older attempt records remain unchanged.

Build/package artifacts and prior attempt evidence remain in `target`; there is
no artifact cleanup. Host's `target/publish/lock` covers admission through command
completion. Shared's stable `target/publish/.cargo-publication.lock` also blocks
another command attempt and remains after interruption or incomplete evidence.
A dispatch marker without completed statuses means the upload result is uncertain.
Before removing only a stale lock, inspect its recorded owner and reconcile the
exact source/packages against registry state. Neither command removes stale locks,
retries an upload or overwrites earlier evidence. Directory paths containing CR/LF
are refused before Cargo admission or dispatch; ordinary spaces remain supported.

### Foundation-first bootstrap

Platform 0.13 has no Host Artifacts dependency. The current workspace therefore
needs no Platform-to-Tooling bootstrap exception: adopt published Platform/Wire
suppliers, then qualify and publish the intended Tooling cohort normally.

The procedure below remains available for older supplier selections that still
depend on an unpublished Host change, including Platform's retired gzip facade.
Use it only after confirming that dependency in the actual selected graph:

1. Prepare the intended foundation release and commit the foundational
   changes while upper-layer dependencies still resolve to published releases.
   Batch selection limits package preparation/upload; full workspace metadata and
   the existing lint gate still apply.
   If later adoption will change this same planned release, use a preparation
   commit and finalize its Git release/tag after the last source change. If the
   version is already tagged, finish its remaining packages from that unchanged
   source and put subsequent adoption in the next normal release.
2. Check and publish the required Host batch with `RELEASE_GROUP=foundation`
   and an explicit `PACKAGE` selection,
   as in the example above. Include any other unpublished local dependencies the
   selected packages need. Cargo owns resolution, verification and ordering.
3. Publish the required Platform and Wire updates through their release flows,
   then adopt their published versions in Tooling and commit the remaining changes.
   Keep the already uploaded packages' version and source intact. Each batch
   records its own committed source; no release tags or versions are changed by
   the publisher.
4. Check and publish only the remaining packages within each intended group.
   For an Artifacts-only first batch, the remaining Foundation batch is:

   ```bash
   remaining=(ic-host-fs ic-host-http ic-host-process ic-host-wasm-inspect)
   make publish-check RELEASE_GROUP=foundation PACKAGE="${remaining[*]}"
   make publish RELEASE_GROUP=foundation PACKAGE="${remaining[*]}"
   ```

   Build, Text, Tools and Testkit require their own explicit group selection;
   release only groups with pending changes, after their actual suppliers.

Use an already published compatible Host version when possible; that needs no
bootstrap batch. After any partial publication, use explicit remaining selections:
the default within a selected group still requests that group's complete public
roster. The wrapper never
looks up registry state, automatically skips versions or retries an upload.

### Partial-upload recovery

Workspace uploads are not atomic. A failure, timeout or lost reply can leave some
packages published. The wrapper stops and does not retry, skip existing versions,
bump versions or interpret failure as absence. Retain the intent and artifacts,
inspect the exact name/version entries on crates.io, and compare their published
source with the recorded source before continuing. Stop if registry access or
payload identity is uncertain. After confirming which packages are present,
publish only confirmed missing packages using `make publish PACKAGE='name ...'`
after reconciling any retained attempt locks. This preserves the wrapper's
admission checks; Cargo owns dependency ordering.
Do not rerun the full workspace upload blindly or change source under the same version.

The [Cargo publish documentation](https://doc.rust-lang.org/cargo/commands/cargo-publish.html)
defines dry-run, registry, packaging and index-waiting behavior. The offline
`make publish-command-check` fixtures verify the consumer wrapper and shared
attempt owner using substituted Git, Cargo and logger commands, including source
drift, original status preservation, interruption and refused replay. They do not
contact a registry or publish.
