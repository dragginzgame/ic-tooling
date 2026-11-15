# Canic retained-record admission

Current recipes select the next unpublished Build 0.87.3 / Text 0.77.2 where
used. Their patches and locked graphs admit current local sources; historical
runtime evidence below remains bound to its recorded checkpoint inputs.


`make test-canic-consumer` snapshots the current `CANIC_SOURCE` working tree
(default `../canic`), including local edits and non-ignored additions, without
modifying it. [Source selection](../../docs/qualification/local-sources.md) records
the capture and refusal contract. The [migration patch](record-admission.patch)
contains the private caller change, tests and dependency delta; it must apply to
the current source. Exports remain under `target/`.

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

`make test-canic-text` snapshots the current local Canic working tree and applies
`text.patch` independently of the record-admission recipe. The initial run selected Text 0.77.0,
migrates escapes and plain tables, and removes the repeated two-space/header-rule
options from the callers. Canic retains its own report selection, configuration,
colours, topology, shell quoting, status meanings and raw JSON. No Canic
implementation is admitted into product crates. The patch does not migrate other
Host/Build dependencies or import old Canic source.

The focused runner checks the CLI's actual Text dependency edge and requires
nonempty named test selections: eight status tests, eight fleet terminal tests and
three local declaration/output cases. These cover unknown/uncertain effects,
retained evidence, terminal controls, roles, partial inventories and raw JSON.
They do not deploy canisters, start a browser or contact a live IC network.

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
