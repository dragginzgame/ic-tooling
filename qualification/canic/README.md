# Canic retained-record admission

## Current Foundation qualification

Run `make test-canic-consumer` for retained records, `make test-canic-filesystem`
for filesystem callers, or `make test-canic-runtime` to include locks, parent
preparation and the CLI report/viewer tests. Each run captures the current Canic
working tree and uses its own source, manifests and tests. Cargo command-line
overrides select FS and Artifacts from this checkout or from an explicit
`IC_TOOLING_CANDIDATE_ROOT`. Metadata must show both normal, unconditional
edges from Canic Host to the selected local manifests. Every named test must
report a pass. The capture, lock, graph, supplier hashes and test output remain
under `target/canic-consumer.*/`.

On Linux the runtime selection passed 54 Host and three CLI tests, published a
Wasm artifact with its verified digest and permissions, and refused a symlinked
parent without creating the destination. Canic HEAD was
`dd44ddcf2ffcc3f8ba693237ea176c3ad14eff18` including local edits, with source
archive SHA-256 `c88a401210dd1354ac387773be06ec0ddafef48005e9972723a8c49dc28fc6bd`.
See `target/canic-consumer.JlzUNrQn/`. The records selection passed 19 cases and
the retained-read entrypoint passed 15 on the same source. The filesystem
selection passed 39 against the extracted FS and Artifacts 0.80.5 manifests
in `target/build-release.kNWOlSyD/candidate/` (`target/canic-consumer.JmCCLKyF/`).
A missing exact name alongside a passing case was rejected.

The dated migration results below describe their original source captures.
Canic owns Store, filesystem publication and the corresponding regression tests.
The separate Build migration experiment requires an actual Build consumer.

## Module layout maintenance — 2026-11-17

Qualification runners place modules with fixture children in `foo/mod.rs`,
including Store, app-build, signer capture and staging. Lock and retained-read
fixtures use ordinary child-module discovery without path overrides. Store/app-build
patch paths follow these export-only moves; migration logic and stale-context
checks are unchanged. This layout cleanup does not renew the historical consumer
acceptance below or make obsolete migration patches applicable to the rewritten
consumer.

## Current source review — 2026-11-16

The current Canic working tree has already adopted the published host foundation.
A read-only capture at HEAD `37270cb30f840d45592781a4fd957e823c39c895`, including
working-tree changes, is retained at `target/canic-next-review.iDdmNsEk` with
source-tar SHA-256
`ec5e50fcaa60fa7218db1d56ee079d6ccd1e68ca6150c12e9a387b2c98eb0503`.
Its product lock selects registry Artifacts/FS 0.80.2 and Process 0.80.2; Text
remains 0.76.0. Its current root catalog has no `ic-build` dependency.

The CLI calls `canic_host::save_fleet_report`. Host's `fleet/report.rs` prepares
the report parent, uses `staging_directory::allocate` to obtain a held directory,
then publishes `index.html` through `output_publication::publish_in`. Naming,
collision budget and report policy remain Canic-owned; FS owns custody and
publication. The old automatic-report-allocation migration obligation is removed:
this path already uses the replacement API without a new Tooling mechanism.

`git apply --check` of `report-publication.patch` refuses the captured CLI's
changed `fleet/mod.rs`, as required. Do not replay this historical patch or repair
it to recreate an already-adopted implementation. Reassess each old runner against
the current workflow before selecting it; further product changes belong in Canic.
This is source and lock inspection plus stale-patch refusal, not new runtime,
archive or native-host qualification. Canic's own
`docs/qualification/history.md` records its adoption acceptance separately.

## Historical migration recipes

The recipes below selected local Build 0.87.3 / Text 0.77.2 where used. Build is
now published; Text remains unpublished. Their runtime evidence remains bound to
the recorded captures, and does not qualify the current Canic working tree.

## CLI/runtime composition

`make test-canic-runtime` extends the composed Host recipe on the same captured
source. It migrates CLI viewer handoff to local Process, compiles the real CLI,
runs all-target CLI Clippy, and checks the four local viewer/launch cases without
opening a browser. It then runs the real owned-Wasm probe through artifact output,
parent preparation, alias replacement, compilation/publication refusal and cleanup.
App checks retain pre-lock duplicate refusal, config-relative recipes, lock
failure/preservation and serial partial output. Metadata checks explicitly select
JSON output and refuse invalid metadata/tools before compilation or publication.

The Host native graph remains free of legacy FS/Process; CLI's direct Process
edge selects local 0.80.2. Automatic fleet-report directory allocation still requires legacy FS, so
this recipe does not retire its normal dependency or claim whole-workspace
filesystem isolation. Text stays at the current consumer selection. Foreground
profile/PTY and registry Store-build runtime checks are outside this selection;
their earlier independent results do not qualify the current composition.
No deployment, live signing, browser launch or sibling edit is involved. Local
loopback access and the explicitly prepared Rust/Wasm toolchain are required.

Passing Linux run: `target/canic-build.zeOv61KJ`, logged in
`target/canic-runtime-composed.log`. Canic 0.216.5 HEAD
`2ee4add03126aa2f5259c609fa05eae8a14bcd14` plus working-tree changes; source-tar
SHA-256 `2dab00357b0dda539467006d736fb2a8b6d2a0ed852c19638ef4b611cd9f7db7`.
The identical 52-case inventories, two custody cases, nine Host Process cases and
four viewer cases pass on this capture. Library/CLI compilation, Host/CLI Clippy,
graph/lock admission, artifact/app runtime checks and all 20 metadata/tool refusals
pass. Viewer evidence includes Linux child reaping after observer drop. These
results select tools 0.1.3 and do not refresh the unselected profile/Store flows.

### Report file publication

The runtime recipe now installs the same two report publication fixtures before
and after migration. The CLI export reuses the private artifact-output adapter:
held parent preparation, a sealed 0600 stage and create-only publication replace
the released pathname writer. Parent creation requests 0777 reduced by umask;
existing modes remain unchanged. Exact rendered bytes, existing-file preservation,
occupied directories, final links and ancestor-link refusal are compared.
No new public Tooling API or product implementation is introduced.

This adapter deliberately narrows report paths: parent traversal is rejected
before effects. The released writer accepts `not-created/../escape.html`; that
is not a preserved refusal. An explicitly selected replacement-only test checks
the new policy separately from the paired inventory. Automatic snapshot-directory
allocation remains on the released helper, and serving/rendering policy remains
owned by Canic. Production adoption must account for the narrower path policy.

The first attempted capture, `target/canic-build.nZAqfbPY`, failed in unchanged
Canic `plan/mod.rs` because its predecessor value was `()` rather than a collection.
Its source archive SHA-256 was
`78ff293deb4752bc0961839c6dd99f055b031b615f17c46dba7d5ae1d8dd7840`.
A fresh capture after the sibling changed passed that compilation. The next
baseline (`target/canic-build.GugNcuwJ`) exposed the traversal-policy difference;
the fixture now distinguishes preserved behavior from the new policy. Neither
failure was repaired in the sibling or hidden by falling back to older source.

Passing Linux export: `target/canic-build.VIaAaG1B`; log
`target/canic-report-publication.log`. Canic 0.216.5 HEAD
`2ee4add03126aa2f5259c609fa05eae8a14bcd14` plus working-tree changes, archive
SHA-256 `bd5088aa6b141eeef12911d879a36063799b38cd584446ec5631e47ab4fd3c8e`.
All 54 paired cases (52 Host, two report), the replacement-only report policy
case, two custody, nine Host Process and four viewer cases pass. Host/CLI Clippy,
exact local graph and lock admission, real artifact/app builds and 20 metadata/tool
refusals also pass. The report test inventories match; the policy test is ignored
in the paired run and explicitly required with `--ignored --exact` afterward.

## Composed Host Process and filesystem graph

The composed preparation recipe now also applies the existing lock, signer,
metadata and Store-source Process adapters before making legacy FS test-only.
Host uses local Process 0.80.2 directly, sharing the owner already selected by
Build. Native `cargo tree --edges normal,build` must contain neither released
Process 0.79.0 nor FS 0.79.0. Metadata separately admits the exact local Process
path and test-only direct FS edge; library-only compilation checks direct access.
This graph boundary is Host-specific. CLI/viewer/profile-client dependencies and
production manifests remain unchanged by this recipe.

Store-source lookup now uses current Canic's registry input contract, including
the locked protocol/runtime version check. Its Process patch was refreshed against
that implementation; the old Git-source selection is not restored. The nine
additional Process checks cover lock evidence/diagnostics, compiler diagnostics,
registry-pair admission, signer version/principal refusals and bounded capture
(stream overflow, deadline and spawn failure). The signer is a harmless local
fixture; it uses no keys or network calls. These checks complement the paired
filesystem cases without claiming live submission or a refreshed full CLI build.

Reused lock/signer fixtures now use ordinary Rust child modules. This removes
their `#[path]` wiring in the touched runners without importing fixtures into
product crates or changing their assertions.

Passing Linux run: `target/canic-build.BCSYpwy3`, logged in
`target/canic-host-composed.log`; additional checks are in
`host-process-tests.log` within that export. Canic 0.216.5 HEAD
`2ee4add03126aa2f5259c609fa05eae8a14bcd14` plus working-tree changes; source-tar
SHA-256 `979811db02103018f91770bd3007b3969e164d0aafe46955f1b94240b733cb06`.
All 52 paired cases, two custody cases and nine focused Process cases pass,
alongside library compilation, Clippy and graph/lock admission. The saved native
normal/build tree contains local FS 0.80.1 and Process 0.80.2, with neither legacy
package. Their released test oracles and unrelated workspace consumers remain.

Earlier composition attempts stopped safely at patch context checks: the path
fixture registration conflicted with a lock-adapter EOF insertion, then metadata
context assumed adjacent module declarations. The registration now precedes the
type declaration and the metadata patch retains focused unchanged context.
Failure logs are `target/canic-host-composition-patch-refusal.log` and
`target/canic-host-metadata-patch-refusal.log`; no consumer repair or fallback was
used. The refreshed full CLI/runtime recipe has not been rerun by this slice.

## Host's direct legacy filesystem dependency

After the filesystem caller adapters, `filesystem-test-only.patch` moves Host's
released `ic-host-fs` declaration to unconditional dev-dependencies. The registry
package remains available to original fixture allocation, preparation and lock
interoperability tests. Production library code must compile without that direct
dependency; `cargo check -p canic-host --lib` tests this boundary independently of
the all-target Clippy and paired behavior checks.

Resolved metadata must contain exactly one Host-to-released-FS edge, with exactly
one unconditional `dev` kind. Local FS/Bytes identity and external-lock checks
remain required. The runner saves `host-runtime-tree.txt` rather than treating
the direct-edge change as full runtime removal: released Process 0.79.0 still
depends on released FS 0.79.0. Composing the existing Process migration with this
filesystem export is a separate qualification step. No public legacy export is
removed, and the production sibling manifest is unchanged.

Passing Linux run: `target/canic-build.W5YqoIT7`, logged in
`target/canic-filesystem-test-only.log`. Current Canic 0.216.5 HEAD
`2ee4add03126aa2f5259c609fa05eae8a14bcd14` plus working-tree changes; source-tar
SHA-256 `31f64eef8b10043f1c80075b037b4fde4f0dfb3398670a4c26b23db7d3b877f0`.
The library-only compile, exact direct-edge assertion, all-target Clippy and
dependency/lock admission pass, as do all 52 paired cases and two custody tests.
The saved normal/build tree confirms the remaining Process-to-legacy-FS edge.

## App-output path comparison

`output-path.patch` moves the app recipe's destination comparison to the private
export module `output_path`. It preserves the original path for the independent
artifact writer and performs no creation, removal or locking. Existing paths use
native canonicalization; missing suffixes are resolved component by component,
including dangling aliases and traversal back into existing ancestors. Raw native
components retain `file/.`, `file/..` and trailing-separator refusal. The fixed
Canic allowance of 40 active missing-target expansions remains local policy;
completed independent aliases do not consume that allowance.

This is a newly implemented consumer adapter, derived from required comparison
outcomes and the released resolver's documented behavior. The production reference
was read-only; its general path API, implementation and test tree were not imported.
No path normalization API is added to Tooling. This comparison is not a held-path
capability, a confinement boundary or proof against concurrent namespace writers.

Three paired fixtures select the released resolver before migration and the
private resolver afterward, retaining identical assertions. They cover missing
suffixes, absolute/relative/dangling aliases, parent traversal, native filenames,
file traversal refusal, cycles and active-depth limits. Existing app tests also
require output-alias refusal before lock/output creation, even with one canister
selected. No historical source fallback or production sibling modification occurs.

Passing Linux run: `target/canic-build.IMzTNGo3`, logged in
`target/canic-output-path.log`. Current Canic 0.216.5 HEAD
`2ee4add03126aa2f5259c609fa05eae8a14bcd14` plus working-tree changes; source-tar
SHA-256 `65b6805f6cff5f9097e185b83e1a59e62ef8e78aad38d457f0c8c2404bddf401`.
Both suppliers pass the same 52 cases, both replacement custody tests pass, and
Host all-target warning-denied Clippy plus local dependency/lock admission pass.
An earlier run (`target/canic-build.GINe379G`) passed baseline but refused the
test adapter's context-free patch; adding function context fixed the patch, not
consumer source. That failure log is `target/canic-output-path-patch-refusal.log`.
The legacy dependency remains selected for independent test oracles; normal graph
isolation, packaged artifacts and native macOS still need separate qualification.

## Artifact inspection

The composed preparation recipe adds `artifact-inspection.patch` and two identical
inspection fixtures to both suppliers. Canic resolves its selected input aliases;
held-directory admission then opens a regular file with no additional link
following. Existing permission/hard-link policy is unrestricted. Descriptor size
admission precedes bounded complete-byte reading, which independently refuses
growth beyond `MAX_ARTIFACT_BYTES`. The existing Wasm header and identity policy
remains in Canic. No generic path-following API enters the filesystem owner.

Fixtures check the known minimal-Wasm digest and length through direct, parent
alias, final alias and hard-link paths. They also check non-Wasm and oversized
inputs, directory/FIFO refusal, missing-file cause and dangling-link preservation.
The FIFO fixture uses the prepared host's `mkfifo` utility. This adds inspection
to the existing composition without changing retained records' final-link refusal.
Namespace/content writer exclusion remains required; canonicalization does not
freeze a path or grant hostile-writer confinement. No performance improvement is
claimed; the retained consumer admission still computes its own digest.

Passing Linux run: `target/canic-build.u6G01qsU`, logged in
`target/canic-artifact-inspection.log`. Canic 0.216.4 HEAD
`f0cbb3e78592b869a464cc46b80289700e9d4cd7` plus working-tree changes; source-tar
SHA-256 `4b9f26340ea6507b7b94e7a95be7945c51ab65c2dc0b0cb97457226e47ace360`.
Both suppliers pass identical 49-case inventories, plus both replacement custody
tests, Host all-target warning-denied Clippy and dependency/lock admission.
App-output path resolution remains on the released filesystem supplier; test
helpers remain independent oracles. Native macOS and production migration remain
unqualified.

## Recovery and release preparation

`recovery-preparation.patch` retains prepared directory custody for signed
authority/attempt/reply/rejection writes, publication dispatch deadlines, and
fleet release module/plan writes. A private `store::put_json_in` preserves the
existing pretty-JSON encoding and record bound through `put_in`; no schema or
filesystem public API changes. Existing evidence reads, signing scratch cleanup,
authority admission, expiry checks and network dispatch ordering remain unchanged.
Read paths and signing scratch still require caller namespace/content exclusion.

The composed recipe selects six additional original publication tests alongside
the existing signed recovery and filesystem cases. These cover retained deadlines,
conflicting authority, receipt/manifest binding and unpublished-release refusal.
The resulting 47 paired cases do not exercise live update replies or certified
rejections: those migrated branches receive compile/Clippy validation here,
with their file publication primitive covered separately. Full fleet release
execution and actual signing/submission remain outside this evidence.

Passing Linux run: `target/canic-build.UbqRprbi`, logged in
`target/canic-recovery-preparation.log`. Canic 0.216.4 HEAD
`f0cbb3e78592b869a464cc46b80289700e9d4cd7` plus working-tree changes; source-tar
SHA-256 `8c3ac40011aadd1d779a970a546cac866684bd9831dfc9642a73ae8300538488`.
Both suppliers pass identical 47-case inventories, plus both replacement custody
tests, Host all-target warning-denied Clippy and dependency/lock admission.
Remaining released filesystem calls in product source are artifact inspection
and app-output path resolution; test helpers remain independent baseline oracles.
This inspection does not establish full filesystem migration or archive acceptance.

## Held staging allocation

`fleet-members.patch` keeps the prepared root and child directories while writing
the fleet's staged records. Preparation remains root then application children;
publication remains root module/plan, optional Store module, application modules,
then fleet JSON. Child handles follow the slot order already admitted by
`FleetPlan::digest`; no slot policy or persisted representation moves to Tooling.
The optional Store directory is prepared at its original point in that sequence.
All writes use `store::put_in`, retaining the record bound and create-only policy.
Content validation still calls the original `load` before directory publication.
This removes fleet staging member pathname reopening, with no new public API.

Fleet member migration passes all 41 paired cases and both replacement custody
tests in `target/canic-build.dOpCLZbT`, logged in `target/canic-fleet-members.log`.
Host all-target warning-denied Clippy, local dependency admission and external-lock
checks pass. The capture has the same HEAD and source-tar hash recorded below.
The ten original fleet cases retain one/two-child slot mapping, original digest,
Store binding, retained approvals, cancellation, expiry and artifact refusals.
This is Linux export evidence; it does not migrate the production consumer or
qualify native macOS.

The recipe's `starter-members.patch` then uses the held staging directory for
starter member creation. Each generated relative member parent is decomposed into
normal names, prepared with 0700, and passed directly to `store::put_in`. This
removes the starter's released private-parent helper and pathname-based member
publication. Existing record bounds, 0600 file mode, create-only policy, sorted
member order, partial staging retention and exact content verification remain.
Verification still uses the retained staging path; namespace/content exclusion
remains required. Signed recovery and fleet release preparation are separate migrations.

Starter member migration passes the same 41 paired cases and two replacement
custody tests, plus Host all-target Clippy and dependency admission, in
`target/canic-build.nHJiFWDy` (`target/canic-starter-members.log`). This capture
has the identical HEAD and source-tar hash recorded below for staging allocation.
The six unchanged starter tests include complete project creation, pre-effect
refusals, alias policy and concurrent initializers; no sibling source was edited.

The parent-preparation recipe additionally applies `staging-directory.patch` to
starter and fleet publication. Their private allocator keeps the two Canic
prefixes, process/sequence names, 32 collision attempts and 0700 request reduced
by umask. It uses `Directory::create_directory` and retains that returned handle
through content verification and no-replace publication. No new public allocator
or consumer policy enters the Tooling crates. Content verification still uses paths;
callers must exclude namespace/content mutation throughout the operation.

Only occupied candidates retry; other creation, parent-sync or admission failures
stop and identify the candidate, retaining any created directory. This adds parent
synchronization at allocation; failures are not silently treated as collisions.
The original released allocator is inspected solely for naming, collision and
retention obligations; its generic options and implementation are not imported.

The same 41 consumer cases compare released and replacement suppliers. Two
replacement-only tests separately check occupied files/directories/symlinks,
budget exhaustion, private mode, held-parent rename and refusal of substituted
staging without removing either original or foreign entries. These are custody
acceptance tests, not additional paired baseline cases.

Passing Linux run on 2026-11-16: `target/canic-build.PuxsEEij`, logged in
`target/canic-staging-allocation.log`. Current Canic 0.216.4 HEAD
`f0cbb3e78592b869a464cc46b80289700e9d4cd7` plus working-tree changes; source-tar
SHA-256 `bfa61b57c0b751c65ff727bf49a3b659a170954284f2fa1d75166b961e40d5e3`.
Both suppliers pass the identical 41-case inventory; both new custody tests,
Host all-target warning-denied Clippy, local dependency admission and external-lock
checks pass. Native macOS and production migration remain open.

## Store and plan directory preparation

`make test-canic-parent-preparation` adds `parent-preparation.patch` to the
composed read/publication/lock comparison. Store uses the prepared held directory
directly to acquire its sidecar. Single-plan publication retains the prepared
directory while publishing both module and serialized plan through `store::put_in`.
This removes the prepare-path/reopen-path sequence at these two boundaries.

The shared private output adapter owns Canic's path decomposition; Tooling still
accepts explicit child names. Existing directory modes remain unchanged, missing
private children request 0700, and parent traversal or symlink components refuse.
Record size limits, create-only destinations, serialization order, partial plan
publication and original lock policy remain unchanged. Signed/publication/fleet
workflow preparation and unique staging allocation are not migrated by this slice.

Two additional tests, installed identically for both suppliers, check private
creation with existing-mode preservation and traversal/alias refusal before
effects. The full comparison requires 41 exact cases, including the original
plan-byte, recovery and lock assertions. No original consumer assertion changes.

Passing run on 2026-11-16: Canic 0.216.4, HEAD
`f0cbb3e78592b869a464cc46b80289700e9d4cd7` plus working-tree changes, source-tar
SHA-256 `a1fe4e786b9a4d2c16a0834680c6916020644e0ddc835b1a8be1bb9d278591fa`.
Both suppliers pass the same 41 tests, Host all-target warning-denied Clippy,
local byte/filesystem admission and retained external-lock checks. Export:
`target/canic-build.mSmzY4Jw`; log: `target/canic-parent-preparation.log`.

The runner refreshes only exported file timestamps after capture. A preceding
artifact attempt reported missing `Slot::ALL` without rebuilding the protocol;
the untouched identical capture compiled and contained that constant, indicating
stale shared-target reuse rather than a missing source definition.
The failed attempt is retained at `target/canic-build.EJrLEf36` with
`target/canic-publication-refresh-failure.log`; its untouched-source check is
`target/canic-publication-baseline-check.log` (passed). Source tar receipts and
bytes are unchanged by timestamp refresh.
The final artifact CLI rerun passes in `target/canic-build.rp7Ie6Qz`, logged in
`target/canic-publication-run.log`, on identical source bytes to the final 41-case
comparison. No source repair or historical fallback was used.

## Store and configuration sidecar locks

`make test-canic-sidecar-lock` composes the filesystem comparison below with
`sidecar-lock.patch`. Store holds the new `Lock` owner; configuration retains a
local owner across reload, update, encoding and publication. Scope exit releases
the lock as before. The existing configuration contention message remains intact.
The private adapter resolves the caller-selected parent once, preserves ancestor
alias policy, requests 0600 for new files and allows existing modes/hard links.
It never creates parents, replaces a sidecar, or adds synchronization.

The two additional qualification tests run unchanged against both suppliers:
released-helper/Store contention in both directions, release on Store drop,
existing inode/bytes/mode preservation with a hard link, and rejection of sidecar
symlinks/directories without foreign-entry changes. Together with the composed
read/publication cases, 39 exact selections are required. Original Canic tests,
sidecar naming, reload-under-lock and configuration validation remain unchanged.

Explicit private-parent preparation, unique staging allocation, artifact/path
helpers and test-only released lock fixtures remain selected from the old
filesystem supplier. This is interoperability qualification, not full dependency
removal, native macOS acceptance or production migration.

Passing run on 2026-11-16: Canic 0.216.3, local HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes, source-tar
SHA-256 `b040680021326157080ce90bc66d0bc87ce58db2d0e7be6dfd45976e4df8d714`.
Both suppliers pass the same 39 tests, Host all-target warning-denied Clippy,
exact byte/filesystem selection and retained external-lock admission. Export:
`target/canic-build.DDa6GUbq`; log: `target/canic-sidecar-lock.log`.

## Combined retained reads and publication

`make test-canic-filesystem` compares the released suppliers with the composed
current read, configuration, retained-record and starter/fleet publication adapters
on one source capture. Its 37 exact selections include 33 unchanged Canic tests
and four qualification cases installed identically before both branches. Missing
tests, a failing baseline, stale patches or differing passing inventories refuse.

`record-read.patch` refreshes only the existing retained-reader migration using
the already-selected local aliases. It resolves an allowed ancestor alias once,
holds the parent, admits a regular final file without following a final link,
then performs bounded byte admission. Existing permission/hardlink policy remains.
Trailing separators and directory inputs refuse; empty/over-limit reads and
missing-file `NotFound` behavior retain their checks. The focused fixture now
looks through the error source chain instead of requiring one supplier's wrapper
depth. These are existing qualification assertions adapted for both suppliers,
not edits to Canic's original tests or a new product API.

Locks, explicit private-directory preparation, staging-name allocation and
unrelated artifact-inspection/path helpers still select the released filesystem
supplier. This closes a composed retained-read/publication check, not full
dependency removal, live network qualification or production migration.

Passing run on 2026-11-16: Canic 0.216.3, HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes; source-tar
SHA-256 `195a96a91e989f58bab2b7f51d72aafed030c5724e168742d2e760e5f36a29b4`.
Both suppliers passed the same 37 tests, Host all-target warning-denied Clippy,
local byte/filesystem selection and retained external-lock admission. Export:
`target/canic-build.XMZIv7rU`; log: `target/canic-filesystem.log`. Earlier artifact
CLI builds remain separate source-bound evidence, not part of this comparison.

## Starter and fleet directory publication

`make test-canic-starter-publication` compares six unchanged starter cases.
`make test-canic-directory-publication` adds ten exact fleet-plan cases on the
same captured source. Baseline failure or a missing named test stops migration;
both branches must pass the same inventory. The export applies
`starter-publication.patch` and `fleet-publication.patch` to the namespace
transition only. Unique staging allocation, durable member writes, application
validation, journal locks and retained recovery policy stay with Canic.

Each caller holds the parent and staging directory before its existing content
verification, then publishes through `Directory::publish_directory`. Starter
still compares each generated file; fleet still loads and validates the exact
plan identity. Files and child directories were synchronized by the original
member writers; the new operation syncs the staging root and parent transition.
Failure never deletes staging. Starter retains both paths in its existing error;
post-rename synchronization errors retain the new typed publication receipt.

These workflows allocate fresh staging before committing, so they do not use the
old helper's implicit recovery from a missing staging name. Existing-destination
recovery and validation remain consumer decisions. No tree traversal, Canic schema
or public preparation callback is added to Tooling. Other old filesystem imports
remain; this is not complete consumer cutover or native macOS qualification.

The starter cases cover generated declarations, validation before staging, build
preflight/aliases, physical source aliases, occupied destination symlinks and
concurrent initializers. The fleet cases preserve manifest limits, Store binding,
upgrade completion, retained approval/expiry, cancellation, tampered-artifact
refusal and original two-child slot/digest mapping. These are local fixture tests,
not live signing or IC deployment.

Passing combined comparison on 2026-11-16: Canic 0.216.3, local HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes, source-tar
SHA-256 `9ac12b11844b3e428e5c56fb10e449d8bb41554649e6cc059eb0521a28884e22`.
Both suppliers pass the same 16 exact tests, Host all-target warning-denied Clippy,
exact local filesystem admission and retained external-lock checks. Export:
`target/canic-build.SwPA21MC`; log: `target/canic-directory-publication.log`.
The earlier standalone six-case starter comparison passed in
`target/canic-build.kwGUQ0gW`; it is superseded for this slice by the combined run.

## Held-parent artifact publication

`make test-canic-publication` composes the owned-Build adapter with the new
Filesystem 0.80.1 directory-preparation and file-publication operations in a
current source export. Only artifact output is migrated. Config, journals,
signed requests, starter trees and directory commits retain their suppliers.

The private `artifact-publication.rs` adapter keeps Canic path policy: select `/`
or the current directory once, reject parent traversal, admit normal components,
and prepare missing directories relative to held custody. Existing modes remain;
new directories request 0700 and files 0600. Publication uses the held leaf without
reopening its pathname. A private stage is created exclusively; collision refuses,
not retries. Failed publication explicitly aborts staging and retains any cleanup
failure; visible-but-unconfirmed errors retain the receipt and failed phase.
No Canic path, naming, serialization or recovery policy enters a Tooling product.

The runner checks default and explicit real Wasm builds, exact identities, nested
parent creation, existing modes, symlink-parent and traversal refusal, final
symlink/hardlink/dangling replacement, failed-build preservation, occupied-directory
refusal and stage/build scratch cleanup. Host all-target Clippy denies warnings;
metadata admits the exact local filesystem supplier and retained external locks.
Current CLI diagnostics explicitly select `--json`. The owned-Build patch context
was refreshed for Canic's registry-source working-directory correction; prepared
Cargo/Rustc selection remains explicit. This run does not requalify Store builds.

This is focused Linux artifact-output evidence. Native macOS, other writers and
production cutover remain separate gates. No sibling files are modified.

Passing run on 2026-11-16: Canic 0.216.3, HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes; source-tar
SHA-256 `801a061a6799194cadfd0980cd09aa4104990ae6a198dd3c02f5569e464c8144`.
Export: `target/canic-build.DV2V4iDF`; log: `target/canic-publication-run.log`.

## Configuration publication

`make test-canic-config-publication` runs six unchanged current configuration
tests before migration, then applies `config-publication.patch` and repeats the
same exact inventory. Missing selections refuse. Initial configuration selects
CreateNew; updates select Replace while retaining the original sidecar lock,
reload-under-lock, validation and serialization protocol. Retained-record reads
and locking still use the released supplier; this is not complete filesystem
dependency removal. Both output adapters share one private exported
`output_publication` module; no new Tooling API or Canic product code is added.

All six cases pass with both suppliers: malformed/unknown-field refusal,
app/fleet editing and lock contention, concurrent first initialization, resolved
fleet bounds, symlink refusal, and local bounded initialization with existing-file
preservation. Host all-target warning-denied Clippy, exact local supplier selection
and retained external-lock admission pass. No consumer assertions were changed.

Evidence on 2026-11-16: Canic 0.216.3, HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes;
source-tar SHA-256 `14155c641b281d56836695f88c4f42769dbafa8c19c1dc3f05d91d01373015ee`.
Final export: `target/canic-build.BL64f00o`; log: `target/canic-config-publication.log`.
Journal/signed writers, starter directory publication, native macOS and production
migration remain open. The shared helper's artifact checks also pass on identical
source bytes in `target/canic-build.S4elvkoV` (`target/canic-publication-run.log`).
The earlier artifact capture above has different source bytes.

## Retained-record publication

`make test-canic-record-publication` composes the configuration adapter with
`record-publication.patch`. `store::put` keeps its 12 MiB bound before effects and
selects CreateNew for retained records, including signed envelopes, authority,
replies and rejections. It never replaces existing evidence. Missing parents
request 0777 reduced by the umask, matching the retained writer; artifact/config
callers explicitly select 0700. Existing directory modes remain unchanged.

The runner compares 19 exact tests on one capture: six configuration cases, two
original plan/evidence cases, all nine current signed-authority/recovery cases,
and two qualification tests installed identically before both branches. The new
tests check oversize refusal before parent creation, parent-mode equivalence to
ordinary directory creation, create-only byte preservation and stage cleanup.
Every named test must exist. Original Canic assertions remain unchanged.

Only completed record publication is migrated. Store locking, explicit private
directory preparation in signed workflows, retained reads, signing scratch,
dispatch/retry policy and directory publication still belong to their existing
owners. This does not qualify live signing, IC dispatch or a complete filesystem
dependency removal. Partial and visible-but-unconfirmed publication errors retain
their typed causes and cleanup/receipt evidence through the shared private adapter.

Passing comparison on 2026-11-16: Canic 0.216.3, HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes, source-tar
SHA-256 `14155c641b281d56836695f88c4f42769dbafa8c19c1dc3f05d91d01373015ee`.
Export: `target/canic-build.kSQ7MHPU`; log: `target/canic-record-publication.log`.
Both inventories contain the same 19 passing tests; all-target Host Clippy,
exact local supplier admission and retained external-lock checks pass.
Artifact publication also passes with the final shared helper on identical source
bytes in `target/canic-build.0uvmI8co` (`target/canic-publication-run.log`).


`make test-canic-consumer` snapshots the current `CANIC_SOURCE` working tree
(default `../canic`), including local edits and non-ignored additions, without
modifying it. [Source selection](../../docs/qualification/local-sources.md) records
the capture and refusal contract. The [historical migration patch](https://github.com/dragginzgame/ic-tooling/blob/c41ea2ed9624d3d9a702e46b6f798f87b0452031/qualification/canic/record-admission.patch)
contained the private caller change, tests and dependency delta for that comparison. Exports remain under `target/`.

## Selected contract and ownership

Canic's `store::read` admits retained plan, authority, signed-request and reply
bytes. It must return complete bounded regular-file bytes; reject a final symlink,
non-file, oversized input or missing file; and preserve supplier causes. Existing
ancestor symlinks remain supported. The consumer resolves the parent once, holds
that directory, opens its final component through `ic-host-fs` 0.80.1 and admits
bytes through `ic-host-artifacts` 0.80.1. Permissions and hard links retain the old
read policy; trusted path selection and exclusion of concurrent writers remain
caller obligations. Parent resolution is not a hostile namespace confinement claim.

This removes the old `read_file_no_follow` supplier from the record-read path.
No new product API, path-helper family or compatibility facade was introduced.
The private adapter is newly implemented against the existing owner contracts;
no legacy supplier source or test tree was copied into the replacement. Existing
Canic tests stay in its exported source and serve as consumer acceptance evidence.

Persisted JSON, hash inputs, signed bytes and reply bytes do not change. Failure
nesting can change; original I/O causes remain available. Missing final files carry
`NotFound`; the new bounded stream admits bytes without the old early metadata-size
shortcut. A rejected trailing slash cannot become a successful file read.

## Focused qualification

The runner checks both replacement aliases in Cargo's resolved `canic-host` node,
requires the two new tests to exist, then executes:

- Two new record-read tests: exact/empty/oversized bytes, original missing cause,
  non-file and final-link refusal, ancestor-link acceptance and trailing slash.
- Original plan identity/artifact custody and retained evidence/write-lock tests.
- Original signed recovery after effect expiry and uncertain expired attempt
  retention tests, including the exact retained reply and no fresh signature.

Latest current-source Linux run: all six tests passed in
`target/canic-consumer.aL9hWNgO`, from HEAD
`df9321fc17c99c93e07d631173200a99c2a890bc` plus local edits (including the
Wire 0.22.4 lock selection). Snapshot SHA-256:
`de4951e14d5efb3bf58671754715b44b65e5980fe46287ed6d38441b70ed9190`.
`target/stage3-canic/current-runner.log` retains execution evidence.
The previous `0086d49` run and its Clippy result are historical only.
These are local host tests, not network or installed canister qualification.
Prepared Rust 1.100.0 and cached dependencies are required; the runner is offline
and installs nothing. Metadata is retained alongside each export for inspection.

## Remaining boundaries

Legacy filesystem publication, parent preparation, directory commit and locking
remain explicit Canic dependencies. The existing writer implicitly creates missing
parents, so replacing it requires a separate parent-authority contract and its
actual callers. Artifact builds still use legacy process execution and path capture;
CLI presentation also remains unmigrated. Their presence is not a fallback for the
new record reader. No full Canic migration, archive/native macOS qualification or
performance improvement is claimed. Auth is on hold; Storage, Observability and
Wire remain frozen and are not migration targets for this slice.

## Current CLI Text

`make test-canic-text` captures Canic’s current tree and selects the local Text
crate through Cargo. The runner verifies the CLI’s direct dependency edge, lists
the CLI tests and runs the selected names with exact matching. Five display
suites and three configuration cases cover human output and retained JSON.

On Linux, Canic 0.232.0 at `548700eb6b9402aae6c1bdbf8a103c518bfa9c3e`
passed 40 CLI tests with local Text 0.77.3 and 54 Host tests with local
Foundation 0.80.5. All 11 grouped names in
[Tooling #14](https://github.com/dragginzgame/ic-tooling/issues/14) appeared in
Cargo’s listings and passed. An old CLI selector paired with a valid one was
refused before execution. Listings and results are in
`target/canic-text.uqd73yLf/` and `target/canic-consumer.YIViC9dS/`.

The earlier Text migration compared patched source. Its original evidence follows.

Linux evidence on 2026-11-15: local HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus edits, source tar SHA-256
`67f2b4b6cfb2b4ec0df7b0cf37f62e2482fe7a7c197ed55ea6bc227a580b5bb5`.
Export `target/canic-text.XBgmkNQb`, log `target/canic-text-run.log`: all 19 selected
cases passed. Equivalent source export `target/canic-text.JhcjASOF` supplies
warning-denied CLI/test Clippy (`target/canic-text-clippy.log`) and retained
external-lock identity admission. Three Text contract tests and product Clippy
pass; existing bordered-table/escaping tests remain unchanged. No performance or
production migration claim is made.

## Current owned build

`make test-canic-build` applies `owned-build.patch` independently to a current
local Canic snapshot. Canic keeps recipe/lockfile setup, manifest and package
selection, Wasm admission, default/explicit destination choice and publication.
The new Build owner replaces full Cargo-output retention, manual artifact-path
selection and the later mutable-file read with private execution and owned bytes.
No Canic implementation is copied into product crates and no new Build API is added.

The exported builder requires prepared `IC_OWNED_CARGO` and `IC_OWNED_RUSTC` and
cached dependencies. The runner selects Rust 1.100.0 explicitly. A bounded offline
metadata probe selects one workspace package by name and one cdylib target;
ambiguous or absent selections refuse. Compiler/manifest metadata runs from the
manifest directory. The default destination remains the selected Cargo target's
`wasm32-unknown-unknown/release/<library>.wasm`, but it receives separately captured
bytes after private build cleanup. Explicit destinations preserve their existing
publication behavior. This is an ordinary build, not a source-frozen release proof.

The runner uses this repository's maintained Wasm probe in a private export. It
seeds stale default output, runs the real Canic CLI for default and explicit
publication, checks returned SHA-256 against the resulting bytes and verifies
the probe marker. Compilation refusal preserves existing output; publication
onto an occupied directory refuses without removing its sentinel. All invocations
leave the selected private scratch empty. The CLI's actual Build dependency edge
is checked. No deployments, shared-target performance claim or automatic tool
installation is involved.

Linux evidence on 2026-11-15: source HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus current edits; final source tar SHA-256
`7950b7391ef6d8ded6d2258319b91853c3af0f68dea22c83b30f855127f1544b`,
export `target/canic-build.tnvbBh2y`, log `target/canic-build-final.log`.
The focused build/publication/refusal checks and final-snapshot host Clippy passed
(`target/canic-build-final-clippy.log`). The dependency diff adds Build and its
local byte/filesystem suppliers, plus `tempfile` 3.27.0 already selected by Build.
The strict external-subset checker correctly refuses that addition; existing
external selections were retained. Older Host FS/Process remain for consumer-owned
metadata, lock setup and publication. The separate Text/record recipes are not
implicitly combined or requalified by this run.

### App recipes on the next Build version

`make test-canic-app-build` selects the app workflow without rerunning the
artifact-command roster. It uses a fresh current-source export and Build 0.87.2.
The maintained Wasm probe supplies two app recipes; generated JSON is fixture data,
not copied Canic implementation. The commands make no deployment or network effects.

The check refuses duplicate outputs before initializing a missing lock or replacing
existing bytes. Valid recipes resolve relative to their config from a different
working directory, initialize the absent lock offline, and report exact artifacts
for both canisters. A repeated human-output build preserves lock bytes and says
nothing was deployed. A missing package in the second recipe then refuses after
the first has published: first output survives, second output is retained, and
private scratch is empty. This deliberately establishes serial partial effects,
not atomic app-wide publication or rollback. Existing lock generation/publication
remain Canic-owned operations using its retained Host suppliers.

The earlier lock delta refused current source after Canic added dependencies; the
refreshed patch preserves those selections. Linux evidence on 2026-11-15: HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus edits, source tar SHA-256
`911ee3ab140c43d447003a2678fd078f845be1c7f5925aa8e5c198b380e8249d`,
export `target/canic-build.nPFSGE6H`. All app checks passed
(`target/canic-app-build.log`); host/CLI warning-denied Clippy passed
(`target/canic-app-build-clippy.log`). Other qualification recipes keep their own
source boundaries and may refuse newer siblings rather than falling back.

### Metadata admission

`make test-canic-build-metadata` selects the adapter's metadata boundary. A
controlled Cargo executable returns captured probe metadata and delegates builds
to the prepared real Cargo. Valid metadata builds the probe with exact reported
SHA-256. Ten negative cases cover missing/foreign/malformed workspace membership,
absent/ambiguous packages and cdylibs, absent/relative target directories and invalid
JSON. Each refuses before invoking compilation, preserves a sentinel destination
and leaves private scratch empty. Structured errors distinguish invalid selection
from JSON parsing failure. The adapter now explicitly verifies workspace membership
and absolute Cargo target directories; no new product API or consumer code enters
product crates. This removes reliance on an unchecked workspace-selection assumption.

Linux evidence on 2026-11-15: current Canic HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus edits, source tar SHA-256
`6209e570067f188d2cb98a2de2d5d2530331b6cba09a5a712a1a92b3b5dc80e5`,
export `target/canic-build.3Y1dMnXE`. Metadata checks passed
(`target/canic-build-metadata.log`, structured-error assertions additionally in
`target/canic-build-metadata-errors.log`); Host/CLI warning-denied Clippy passed
(`target/canic-build-metadata-clippy.log`). Existing app/artifact evidence remains
scoped to the snapshots above; this check does not rerun those rosters.

### Prepared-tool admission on Build 0.87.3

The metadata selection also checks missing and unavailable `IC_OWNED_CARGO` and
`IC_OWNED_RUSTC`, and executable `RUSTC_WRAPPER` / `RUSTC_WORKSPACE_WRAPPER`
settings. All six refuse before compilation and retain destination bytes and empty
scratch; marker files prove neither the build nor either wrapper ran. The wrapper
cases retain valid controlled metadata, isolating Build's environment admission
from Cargo's own metadata behavior. This qualifies the adapter's forwarding of
wrapper settings to the existing Build refusal; it adds no product API or tool
installer and does not promise a hostile-tool sandbox.

Linux evidence on 2026-11-15: current Canic 0.210.0 source, HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus edits; source tar SHA-256
`686a1722cc1f95a253b531bea1bcb60a811efec24491b0e50ab986c1aca2301a`,
export `target/canic-build.XkJGhV6C`. The real-build control, ten metadata refusals
and six tool-setting refusals passed (`target/canic-tool-admission.log`). Host/CLI
warning-denied Clippy passed (`target/canic-tool-admission-clippy.log`). No sibling
source was edited; earlier app/Text/record results retain their own scope.

### Output aliases

`make test-canic-build` also selects output names that are a symlink, a hard
link to a foreign sentinel file, and a dangling symlink. An intentional compile
failure first preserves each link (including the hard-link identity), leaves the
foreign bytes unchanged and creates no missing referent. Private scratch is empty
after each failure. Successful publication replaces each selected
entry with a regular file containing the exact built Wasm and matching reported
SHA-256. Neither result aliases the foreign file, whose bytes remain unchanged.
This captures a required publication outcome for a future filesystem migration;
publication still uses Canic's retained Host FS supplier. It does not establish
parent confinement or safety against concurrent namespace writers.

Linux evidence on 2026-11-15: Canic 0.210.0 source HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus edits; source tar SHA-256
`5b0865deb906589c5d2d94c8cbabdae8349733af69d49a47c98408c46fc63c29`,
export `target/canic-build.hdUc3Qm5`, log `target/canic-alias-failure.log`.
Default/explicit output, all three aliases before/after compilation failure, exact identity,
occupied-directory refusal and empty private scratch all passed with Build 0.87.3.
This script-only extension changes no Rust source or product API; other slices
retain their separately recorded evidence.

### Combined Build and Text app workflow

`make test-canic-app-build` now applies both `owned-build.patch` and `text.patch`
to one current-source snapshot. Metadata must prove the Host -> local Build 0.87.3
and CLI -> local Text 0.77.2 edges, with exactly one Text supplier. Artifact and
metadata-only targets retain their independent Build selection; record admission
is not implicitly combined.

Current source added placements presentation and Store observation strings. The
refreshed Text patch migrates those callers to existing `escape`/`bordered_table`
operations; no new API or consumer implementation enters product crates. The
standalone Text runner also selects placements tests. The app target checks its
existing preflight, config-relative paths, lock preservation and serial partial
failure, then selects fleet-terminal and placements tests in the same graph.

Linux evidence on 2026-11-15: HEAD
`5b8d5e91ac601da13ec9d50e8d7a4f4f5efa16bb` plus local edits, source tar SHA-256
`400621e522327fde8c78380e21b2f8f701f92bc5c44ead845052b6997ebf1381`, export
`target/canic-build.ITw236vM`. App checks passed (`target/canic-combined-app.log`),
as did nine fleet-terminal and four placements tests (`target/canic-combined-display.log`)
and Host/CLI/test warning-denied Clippy (`target/canic-combined-clippy.log`). The
initial stale Build lock delta and newly unmigrated Text callers refused before
this refresh. Existing external selections are retained. No sibling was edited;
this does not refresh other independent slices or their native-host evidence.

## Shared Process adoption in Build

Build 0.87.3 now delegates execution to local Process 0.80.0. The current export's
metadata must show that exact edge, while Canic's unmigrated Process callers keep
their released supplier. The original patch refused changed lock and artifact
contexts; it was refreshed against current source, without changing the sibling.

Current Canic retains escaped, bounded Cargo diagnostics. The qualification
adapter now formats the new Build failure's existing stderr evidence through the
consumer's own formatter and retains the typed Build failure. Missing package
errors retain their requested name and supplier category. The real failing Wasm
fixture must expose `intentional owned-build diagnostic oracle`; no compiler
record facade or Canic formatting policy was added to Tooling. Retained external
lock tuples are unchanged; the only added external tuple is tempfile 3.27.0,
required by the existing owned-build implementation.

On 2026-11-15, current source HEAD
`724744bab2a5bf6972f3021431e6da0e66b55b26` plus captured edits, tar SHA-256
`25b912dfd7202651ceb7bf15c5abb48d008d31ae111207ac56d52ec45b991af0`, passes:

- Artifact build/publication, exact bytes/hash, compiler diagnostic preservation,
  symlink/hardlink/dangling failure preservation, publication refusal and cleanup:
  `target/canic-build.GlHwPHbc`, log `target/process-canic-build-refreshed.log`.
- Valid metadata plus ten malformed metadata refusals and six prepared-tool/wrapper
  refusals before compilation: `target/canic-build.pcowhaCE`, log
  `target/process-canic-metadata-run.log`.

The earlier source capture `target/canic-build.Y2RSU7Yn` at the same HEAD, tar
SHA-256 `b2089e7e5d73b608a845ec1a94132d4de54245f2cdf23816dc78238a16f828ab`, supplies
Host Clippy and the three unchanged artifact diagnostic tests in
`target/process-canic-clippy.log` and `target/process-canic-diagnostics-tests.log`.
These are separate captured identities. App/combined Text proof is not refreshed
by these runs, and native macOS and direct foreground-tool migration remain open.

## Foreground client migration

`make test-canic-foreground` captures current Canic and applies
`foreground-process.patch` only in the export. The profile client selects local
Process 0.80.0 through an explicit workspace alias while unrelated callers retain
the released supplier. Its previous manual spawn/communicate/success/completeness
sequence becomes one run with Direct ownership, inherited stdin/stderr, 16 KiB
stdout and the unchanged 300-second communication deadline. The consumer retains
bounded output with fallible allocation, error/status/cleanup evidence, argument
construction, hex decoding and its no-retry policy. No profile or signing code is
added to Tooling product crates.

The export-only tests call the actual private invocation functions using
`foreground-tool.sh`, a harmless ICP-shaped local fixture. They verify exact argv
including leading-dash/shell-looking values, binary argument-file selection, exact
reply bytes, exact/over output limit, nonzero status with retained bytes, invalid
UTF-8/odd/non-hex refusal and exactly one invocation for each attempt. A separately
selected Linux PTY test requires inherited terminal stdin/stderr and reads only
`fixture-answer`; no real credentials, ICP executable or network are used. Timeout
and cancellation mechanisms remain covered by the Process tests, not by waiting
for this consumer's 300-second deadline. The fixture cases do not qualify a real
signer's password or network behavior.

On 2026-11-15 this passed against HEAD
`724744bab2a5bf6972f3021431e6da0e66b55b26` plus captured edits, tar SHA-256
`5389f73680d83261d77b3407d5b691132e32f101584484a28f5b18d78726dde6`.
Export: `target/canic-foreground.RQhgFUv2`; log: `target/canic-foreground.log`.
Three ordinary tests and one PTY test pass. Client all-target Clippy, exact local
supplier-edge admission and external-lock subset checks pass; no new external
supplier tuple is required. The sibling and other Process callers remain unchanged.
Native macOS and the signer group-capture migration remain separate gates.

## Process 0.80.1 signer version/principal capture

`make test-canic-signer` captures current local Canic and applies
`signer-process.patch` only inside the export. The new private capture adapter
selects Process 0.80.1; other Host callers still use released Process. It preserves
exact ICP version admission, null stdin, 1024-byte version streams, 1024/4096-byte
principal streams, 5/30-second deadlines, principal decoding and no retries.
No real signer, keys or network calls are used.

Passing evidence: `target/canic-signer-qualification.log`, export
`target/canic-signer.UpV2YAjX`, source HEAD
`724744bab2a5bf6972f3021431e6da0e66b55b26`, archive SHA-256
`4b6dff0c5916c65dd8449faa4d70930ba1395c11d2d4a3d51963065cef566655`.
Two focused tests cover version/parse/status/limit refusals, exact argument and
invocation counts, exact capture budgets and a shortened adapter deadline.
All-target Host Clippy, local supplier-edge and external-lock subset checks pass.
Executable fixtures are prepared before tests start to avoid concurrent spawn
and executable-write interference; failures are never retried.

This qualifies version/principal capture only, not signing/submission, complete
legacy Process removal, native macOS or production migration. Process 0.80.1 adds
no public API; its eight core tests and the 19 Build tests also pass in
`target/process-0.80.1-tests.log`.

### Signer failure-evidence follow-up

The strengthened runner passes against current Canic 0.213.0, source HEAD
`724744bab2a5bf6972f3021431e6da0e66b55b26` with working-tree archive SHA-256
`249288c4364890dc52fa20ce8b4ac316c223110d33e9228cd3600748c668bac5`.
Export: `target/canic-signer.cdeIMGmh`; log:
`target/canic-signer-evidence-tests.log`.

One caller test and three private adapter tests pass, along with all-target Host
Clippy and external-lock subset/local supplier checks. The new tests assert exact
stream-limit, deadline and missing-executable causes; original nonzero status and
retained stdout/stderr; and cleanup/status distinctions. This supersedes the broad
capture-refusal test without broadening production behavior. Source now selects
Wire 0.24.1; the export preserves that selection and does not modify the frozen
Wire repository. Earlier source-specific results remain historical evidence.

## Process 0.80.1 Cargo lock initialization

Run `make test-canic-lock`. The runner applies `lock-process.patch` to a stable
current local export, checks the Host-to-local-Process edge and external lock
subset, then runs all-target Host Clippy, two private adapter tests and real CLI
app builds. Other Build/Text suppliers stay selected; this is not combined
replacement qualification.

Passed on current Canic 0.213.0 at HEAD
`724744bab2a5bf6972f3021431e6da0e66b55b26`, working-tree archive SHA-256
`8749880f0ca791d29f56ba305142799cdd48fc098795de797829042e9dcba308`.
Export: `target/canic-lock.IxMU8KUs`; log: `target/canic-lock-qualification.log`.
Tests preserve exit code 7, captured stdout/stderr, terminal escaping, the 4096
character diagnostic bound and partial-output marker. The CLI fixture verifies
one lock-init failure without retry, absent lock and unchanged destinations after
refusal; successful initialization; an existing lock bypassing the refusing
initializer; exact Wasm output; and retained first output after a later recipe
failure. No sibling changes or deployment occur. Native macOS remains unqualified.

## Combined Build/Text/Process qualification

`make test-canic-app-build` now composes current-source Build, Text and Process
lock-initialization patches in one export. The first attempt correctly refused
stale Build lockfile and Text caller hunks. Refreshes preserve current status and
progress semantics and migrate newly present subnet-comparison/artifact-listing
presentation and Cargo diagnostic escaping to the existing Text API. Patches
still require admission; no historical source or permissive fallback is used.

Passing export: `target/canic-build.TETX46K0`, Canic 0.213.0 at source HEAD
`6c05a8d7c351f80d7ed93128c41bc3d7e7aa206c`, archive SHA-256
`ff6fb0444917eb272f3bd64b37a0f924b9ec90963185ccc88429e9411a1a6526`.
Log: `target/canic-combined-qualification.log`.

The graph checks prove Host -> local Build 0.87.3 -> local Process 0.80.1,
Host -> local Process for lock initialization, and CLI -> local Text 0.77.2.
Text/Process composition adds no external tuples beyond the admitted Build lock;
Build retains its previously reviewed tempfile 3.27.0 addition. Host/CLI all-target
Clippy, two lock diagnostic tests, real app preflight/lock refusal/preservation,
exact output and serial partial-failure checks pass. Selected display tests pass:
nine fleet-terminal, four placements, fifteen status, four subnet comparison and
two artifact listing. New caller semantics and original tests remain consumer-owned.

The lock patch now places its dependency away from the Build insertion and uses
two context lines so the diagnostic helper can compose with either selected Text
API. Its behavior is unchanged. Native macOS, complete legacy Process removal,
signing/submission and production migration remain outside this evidence.

### Signer capture in the combined graph

`make test-canic-app-build` also applies the signer implementation migration after
the lock migration selects the identical local Process alias. Only the redundant
signer manifest hunks are excluded; implementation hunks still require strict
patch admission, and the combined metadata checks prove the supplier selection.
The standalone and combined recipes now share `canic-signer-fixture.sh` for fixture
preparation and the four signer tests, rather than maintaining parallel setup.

Combined evidence: `target/canic-combined-signer.log`, export
`target/canic-build.OreUxuYf`, source HEAD
`6c05a8d7c351f80d7ed93128c41bc3d7e7aa206c`, archive SHA-256
`482f7c89bc614c3da05fa599b6ee1da658d4aeee915939492859e724307eb6b3`.
Host/CLI Clippy, external-lock admission, two lock diagnostic tests, four signer
tests, app preflight/build/lock-failure/preservation cases and 34 display tests pass
on this single capture. This does not combine the separate foreground-client PTY
recipe or qualify actual signing/submission, native macOS or production cutover.

The refactored standalone command also passes: log
`target/canic-signer-shared-fixture.log`, export `target/canic-signer.XIJwsExz`.
It independently performs source/patch/graph admission and Host Clippy before
running the same four tests; it does not depend on a prior combined export.

## Combined group and foreground execution

On Linux, `make test-canic-process` extends the combined app recipe with the
foreground client's dependency and implementation migration. The existing local
Process alias is reused; metadata proves the client selects that exact local
package and has no remaining direct legacy Process edge. The client now lives at
`apps/profile/canic-profile-client`; the old path correctly refused before its
qualification patch was refreshed. Source logic is unchanged by that path update.
Standalone and combined foreground checks share `canic-foreground-fixture.sh`.
The ordinary `test-canic-app-build` target does not acquire a Linux PTY requirement.

Combined passing evidence: `target/canic-process-combined.log`, export
`target/canic-build.s9UhibXV`, current Canic 0.213.0 at source HEAD
`6c05a8d7c351f80d7ed93128c41bc3d7e7aa206c`, archive SHA-256
`482f7c89bc614c3da05fa599b6ee1da658d4aeee915939492859e724307eb6b3`.
The same capture passes 44 tests: two lock, four signer, three foreground, one
separately selected real PTY case and 34 displays. Real app builds, lock refusal
and preservation, serial partial failure, external-lock admission and all-target
Host/CLI/client Clippy also pass. Fixtures use no keys or live IC calls.

The refactored standalone foreground command independently passes against the
same source identity: `target/foreground-current-qualification.log`, export
`target/canic-foreground.pkxDT3gO`. This does not qualify actual interactive signing,
all remaining Host Process callers, native macOS or production cutover.

### Metadata subprocess migration requirements

The combined Process target now also runs the metadata fixture on its own prepared
command environment. Four added cases refuse before compilation and preserve an
existing destination: nonzero exit after otherwise valid metadata, stdout above
8 MiB, stderr above 1 MiB and invalid UTF-8. The nonzero case preserves the escaped
diagnostic. All ten metadata-selection and six prepared-tool/wrapper refusals also
pass. Cargo metadata capture itself still selects the retained Process supplier;
these are acceptance requirements for its next migration, not proof of cutover.

Passing current export: `target/canic-build.ktTS2nFP`, source HEAD
`6c05a8d7c351f80d7ed93128c41bc3d7e7aa206c`, archive SHA-256
`14dda462e1fc9058cd3a8ce5289f86303e32ba7d5c847cbddaec86d1c73f40a8`.
Log: `target/canic-metadata-process-refusals.log`. All 44 previously selected
fixture/display tests, real app builds and Host/CLI/client Clippy pass as well.

Current Canic moved artifact/progress/placements/artifacts files to module
directories and introduced typed status presentation. Patch admission refused
those changes first. Qualification paths and Text adapters were refreshed against
the captured source, preserving its typed status and continuation logic. No
historical source fallback or sibling modification was used.

## Metadata execution on replacement Process

`make test-canic-process` now applies `metadata-process.patch` after the admitted
Build/Text/lock/signer migrations. The patch moves the reviewed lock capture adapter
into private Canic `cargo_process`, sharing it with metadata execution. Metadata
retains its 8 MiB stdout budget; lock setup retains 1 MiB. Both retain null stdin,
1 MiB stderr, a 120-second deadline, fallible capture, original status/failure and
cleanup evidence, and the existing bounded escaped Cargo diagnostics.

The export removes Host's direct legacy Process dependency, the ToolError wrapper
and synthetic ExecutionEvidence conversions. Original diagnostic tests use byte
inputs while retaining their compiler-error selection, Unicode/control escaping,
bounded/truncated output and destination-preservation assertions. Graph admission
checks absence of the old direct dependency for both Host and profile client.
Other workspace callers may still select the legacy package; no global removal
or production migration is claimed.

Passed: `target/canic-metadata-owner-qualification.log`, export
`target/canic-build.j69lrrpF`, current source HEAD
`6c05a8d7c351f80d7ed93128c41bc3d7e7aa206c`, archive SHA-256
`3fc33c6e96e9fd6684f543ec489a0157ca0c3b29a77ed8ec409bd0583e5e9839`.
All 47 selected tests pass (the previous 44 plus three original artifact tests),
as do all 20 metadata/tool refusals, real app-build/lock-preservation/partial-failure
checks, Linux PTY interaction, Host/CLI/client Clippy and external-lock admission.
The ordinary app and standalone lock/signer recipes remain independent variants;
only the combined Process recipe admits this additional metadata migration.

## Retained desktop-opener requirements

`make test-canic-viewer` captures current local Canic without a Process migration.
It admits the CLI's registry Process 0.79.0 edge, runs all-target CLI Clippy and
four `view::` tests. The two new fixtures verify running-child relinquishment with
a parent-opened completion gate, exact literal URL arguments and original missing
executable failure. The existing tests verify loopback-only report serving and
polling updates. No real browser, credentials or external network calls are used.

Passing export: `target/canic-viewer.YreL6ltH`; log:
`target/canic-viewer-boundary.log`. Source HEAD
`6c05a8d7c351f80d7ed93128c41bc3d7e7aa206c`, archive SHA-256
`3fc33c6e96e9fd6684f543ec489a0157ca0c3b29a77ed8ec409bd0583e5e9839`.
The initial restricted run passed opener fixtures but denied loopback binding;
the complete run passed with local socket access. These results do not prove
reaping after relinquishment, replacement Process handoff, macOS acceptance or
browser readiness. The explicit handoff/reaping design remains the next boundary.

## Replacement viewer handoff and reaping

`make test-canic-viewer-replacement` passes on current Canic 0.214.0 after its
workspace dependency rename settled. Four tests preserve opener liveness, literal
URL/spawn errors and loopback serving/polling. In replacement mode the opener test
also observes PID/start identity before release and eventual disappearance after
Canic drops its completion observer; a zombie is not accepted as reaped.

Log: `target/process-handoff-viewer-reaping.log`; export
`target/canic-viewer.PG2w964W`; source HEAD
`c39aee1e34dae82be6da89bf6242c3a0e22ade84`; tar SHA-256
`58a69ad66ad5fa2c7631a2094f543a39bc5831c6bfcc85d3e318c7b7434486c0`.
CLI Clippy, exact local Process 0.80.2 selection, absence of the CLI's old direct
Process edge and external-lock subset admission pass. No browser, external calls,
sibling mutation or native macOS acceptance is implied.

## Complete current execution composition with handoff

The current Canic 0.214.0 combined Process export now includes the CLI viewer
handoff and reaping. `make test-canic-process` applies the reviewed viewer patch,
retires the unused root legacy Process declaration, and admits exactly one Process
package: local 0.80.2. Host, foreground client and CLI all use that owner. Loopback
access is required; no actual browser, external requests or deployments are used.

Source drift was refused before refresh: Canic now resolves `ic-wasm-store` from
its exact locked protocol Git source and places default output in the consumer's
`target/store-build`. The Build adapter preserves that source-selection branch
and destination policy. The resolver's capture uses the already admitted private
Cargo adapter; its exact-source refusal test is retained. The tiny Store-shaped
fixture uses the existing probe's Wasm payload, not production Store source. It
checks one lookup, consumer-owned default output, no supplier target directory and
unchanged supplier files. A focused follow-up compares complete supplier tar bytes
before/after; no supplier writes are accepted. No frozen sibling is modified.

Passing combined evidence: `target/process-all-callers.log`, export
`target/canic-build.1BER0nYl`, source HEAD
`c39aee1e34dae82be6da89bf6242c3a0e22ade84`, archive SHA-256
`05809f807e58e70b7155b98a09f1c37f2adebd658b24aa73cb5f0578081ca550`.
Fifty-two tests pass (47 prior cases, one Store source-selection test and four
viewer tests), plus all 20 metadata/tool refusals, real app build/refusal cases,
PTY interaction, supplier-output placement, and Host/CLI/client Clippy. The exact
single-Process graph assertion passes against this export's recorded metadata.
Whole supplier-tree preservation passes in `target/store-source-preservation.log`,
using this compiled CLI and fixture `target/store-source-preservation.sGdoV5HZ`.

This replaces earlier claims of pending viewer composition for this source. Native
macOS, real desktop-browser integration, the production Store runtime and actual
signing/submission remain outside the evidence. Other standalone recipes retain
explicit source/version boundaries and may refuse newer sibling edits.
