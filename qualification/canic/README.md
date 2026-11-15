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
