# Native execution: next owner admission

Status: unpublished `ic-host-process` 0.80.2 starts the [handoff design](process-handoff.md).
Synchronous implementation is unchanged; the following 0.80.1 evidence covers adoption
by Build, with current Canic artifact/metadata, foreground-client and signer
version/principal capture qualification. Native macOS acceptance remains open.
The existing [design](DESIGN.md#process-session) names the execution ownership
boundary. This review checks it against current local callers rather than copying
`ic-host-process` or exposing Build's private implementation unchanged.

## Demonstrated callers

Read-only observations on 2026-11-15 use current `../canic` at HEAD
`724744bab2a5bf6972f3021431e6da0e66b55b26` plus working-tree edits. These are inspected
requirements, not a stable exported runtime proof; implementation qualification
must capture the then-current working tree using the shared helper.

| Caller | Required mechanism | Policy retained by caller |
| --- | --- | --- |
| `crates/canic-host/src/signed/mod.rs`, `admit_signer` and `signing_principal` | Concurrent bounded stdout/stderr capture, elapsed deadline and owned-group cleanup | Executable choice, expected ICP version bytes, identity argument, Principal decoding and no replay |
| `apps/profile/client/src/main.rs`, ICP invocation | Direct-child ownership, inherited terminal stdin/stderr, bounded piped stdout and elapsed deadline | Foreground password interaction, arguments, network/identity choice, hex decoding and uncertain-effect reporting |
| `crates/ic-build/src/process/mod.rs` here | Owned-group lifetime, fair pipe draining, bounded diagnostics, cancellation, incremental stdout delivery and explicit cleanup errors | Cargo command/environment admission, protocol decoding, artifact selection and scratch custody |

Canic's profile demo is an application profile, not evidence that the missing
`ic-profiler` process-observation consumer moved there. Do not infer process-sampling
requirements from its name. A borrowed PID supplies no termination authority.

## Owner and proposed first slice

One native execution owner should remove the caller obligation to combine spawn,
pipe custody, draining, completion and cleanup manually. It must be usable without
Cargo, IC identity, signing, networking or simulator dependencies. Product Canic
code and its data types do not belong here. Registry naming/version selection is
separate from this requirements review; no empty package is introduced.

The first implementation should support the demonstrated owned-group capture and
foreground direct-child communication with explicit input/stream policy. Keep
these ownership choices distinct: signalling an owned group is not valid for an
interactive child sharing the caller's foreground group. Null input and inherited
input are different requests; inheriting stderr must not be misreported as a
captured, byte-bounded diagnostic stream. Do not add arbitrary borrowed-PID handles,
process observation, environment discovery, executable installation or automatic
retry. Additional modes such as piped-input writing require current caller evidence
before expanding this slice.

Return bounded retained bytes together with original exit status, communication
refusal and cleanup evidence. Preserve nonzero exit, overflow, timeout,
cancellation, sink failure and cleanup failure independently; partial output must
not look complete. A completed direct child does not prove every descendant is
gone. Cleanup operates only within the selected ownership scope and assumes
cooperative process groups and exclusive reaping. Keep the group leader unreaped
until signalling finishes so its identifier cannot be reused during cleanup.

At review start Build hardcoded null stdin, two piped streams and a new process group,
and couples its report to Build's limits/failure types. Those are real Build
choices, not a suitable general public contract. Evaluate the existing private
mechanism as a narrow reuse candidate only after defining the new state model;
do not duplicate it into a second permanent owner. Move Build onto the admitted
shared core before claiming execution ownership is consolidated. Preserve its
streaming Cargo accumulator and existing failure evidence.

## Acceptance before consumer replacement

Use independent local child fixtures, not ICP credentials or live network calls.
Require concurrent stdout/stderr pressure, exact/over output bounds, nonzero exit,
deadline/cancellation, inherited versus piped stream behavior, direct-child versus
owned-group lifetime and observable cleanup failures. A foreground/PTY case must
prove that terminal interaction remains possible without group-wide termination.
Preserve original process status when communication also fails. Never execute
signing/submission merely to qualify the command runner.

Then use a read-only current Canic export with prepared harmless tool fixtures to
check argument custody, expected version bytes, output admission and refusal
without changing signing or retry policy. Requalify the affected Build failure
selection after shared-core adoption. Linux checks do not establish native macOS
acceptance. No performance improvement, publication or sibling migration follows
from this review.

## Other source-availability findings

The previously recorded `../ic-profiler` directory is absent. Profiler ownership,
required measurements and native acceptance remain unresolved until its current
checkout or retirement/on-hold status is identified. Do not reconstruct its API
from the historical production supplier.

`../ic-platform-new` at observed HEAD
`46bba06a9a2523de443c7a4ce0a812dbb7de7092` has no current Tooling dependency in its
inspected product/qualification manifests or Rust imports. Its native heap profiler
is private qualification, not evidence for the missing process-observation
consumer. Retain earlier Platform fixture results as source-bound evidence; do
not expand old Platform migrations to satisfy the rewrite's future requirements.
Current Query inspection found only already-admitted Text operations, not a new
native execution or simulator requirement.

## Admitted implementation and reuse review

The crates.io sparse index was queried on 2026-11-15 and selected the non-yanked
0.79.0 line (`target/ic-host-process-registry.ndjson`). The replacement uses 0.80.2
for the incompatible contract and remains unpublished. Its only normal suppliers
are the existing Rustix and thiserror selections; no external lock tuples changed.
The independent Process release group owns this package.

`run(command, Policy, cancellation, observer)` executes communication and explicit
close together. `Policy` distinguishes Group versus Direct ownership, null versus
inherited stdin, captured-bounded versus inherited stderr, stdout byte budget and
elapsed deadline. Only owned Child handles are retained. `Report` exposes reaped
status, primary failure and cleanup failures separately. The caller decides whether
nonzero status, incomplete communication or cleanup evidence makes its operation
acceptable. Observer delivery has a per-stream cumulative byte ceiling; the owner
retains no output buffers or decoding policy. Observer blocking and kernel calls
are not subject to a hard wall-clock guarantee.

The private Session transitions from spawned to draining, then attempts shutdown
and reaps once; taking its child makes close locally idempotent. Drop is an unwind
fallback without cleanup reporting. Direct children join the caller's group and
only the owned child is signalled. Owned groups retain the unreaped leader until
group signalling finishes and preserve group failure even when direct fallback
succeeds. Neither mode grants signal authority over borrowed PIDs.

Reuse decision: the replacement Build process module at GitHub checkpoint
`03efc892248a3d354cf0d5a757b1f3a2d88a2fee` supplied the reviewed coherent
unreaped-leader close and fair nonblocking-drain mechanism. This is locally authored
replacement code, not the historical `ic-host-process` crate. The mechanism now
lives once in Process, with explicit policies and stream-labelled failures.
Build's old engine was removed; its short adapter retains streaming Cargo facts,
fallible diagnostic allocation, diagnostic sink writes/flush and its existing
public failure categories. Flush now occurs after explicit execution close; a
flush refusal still retains process status and cleanup evidence. No legacy test
tree or generic compatibility facade was copied.

Linux evidence: `target/process-tests.log` records five new process tests and all
19 retained Build tests. Checks cover concurrent independent exact stream budgets,
overflow before delivery, observer failure with preserved natural nonzero status,
spawn failure, close idempotence, direct-group membership, inherited stderr,
readiness-triggered cancellation and an exited leader with inherited pipes held
open until the deadline. `target/process-pty.log` records a separately selected
real controlling-terminal test: inherited terminal stdin/stderr, fixture input and
successful direct-child completion. Use `make test-process` and, on Linux,
`make test-process-terminal`. Clippy/Rustdoc evidence is in `target/process-clippy.log`
and `target/process-doc.log`. No performance gain is claimed.

This initial evidence did not complete native acceptance. The follow-up below
adds cleanup failures, descendant lifetime and fresh Canic Build proof. Native
macOS/PTY evidence and Canic signer group-capture migration remain open; historical
Canic results do not retroactively qualify the changed supplier graph.

## Cleanup and consumer follow-up

Eight process tests now pass alongside the 19 retained Build tests. A private
close-operation seam injects group, direct-signal and wait failures without
changing credentials or exposing a public backend interface. Real fixture children
are always reaped; synthetic wait refusal is reported as unknown status, not a
fabricated success. Tests preserve ordered errors and natural nonzero status,
retain group refusal after successful direct fallback, and assert that repeated
close invokes no further signal/wait operation. These are fault-injection checks,
not evidence that an OS permission failure was reproduced.

An independent Linux fixture records descendant PID plus procfs start identity
before cancellation. Owned-group close is followed by observation that the same
process is gone or non-running (including zombie state); this does not claim
reaping grandchildren. Under Direct ownership, a descendant waits for a file
released only after close and then writes its completion witness. It has bounded
self-expiry; no borrowed PID is signalled. Logs: `target/process-cleanup-tests.log`,
`target/process-cleanup-clippy.log`, `target/process-cleanup-pty.log` and
`target/process-cleanup-doc.log`. The foreground terminal case passes again with
the revised close path.

Current Canic qualification initially refused the stale lock/artifact patch
(`target/process-canic-build.log`). Its current Cargo failure formatting is a real
requirement: compiler text is bounded, escaped and retained with the primary error.
The refreshed qualification adapter uses Build's existing bounded stderr evidence
from `json-render-diagnostics`; it keeps formatting in Canic, preserves missing
package identification and needs no new Build API. The current artifact and
metadata recipes pass, and three unchanged consumer diagnostic tests pass.
See [Canic evidence](../qualification/canic/README.md). Foreground signer/client
migration remains separate; legacy Process is still selected for those callers.

## Foreground consumer acceptance

Current Canic's profile client now passes an export-only migration to this run
contract. It removes the caller's separate child/communication/success/completeness
assembly while preserving Direct authority, inherited terminal descriptors,
16 KiB stdout, the 300-second deadline and no retry. Caller-owned retained output
and complete Report remain available on refusal. The product Process API did not
need to grow. The fixture exercises argument custody and unchanged decoding,
exact/over bounds, nonzero/invalid replies, single invocation and real PTY input.
Source identity and Linux evidence are recorded in the
[Canic qualification](../qualification/canic/README.md#foreground-client-migration).
This does not migrate Canic's group-capture signer checks or qualify macOS.

## Process 0.80.1: signer capture

After GitHub checkpoint `f086547`, the public API and core remain unchanged.
Current Canic's `admit_signer` and `signing_principal` use a private export-only
adapter over `run(Group, Null, Capture)`. It owns fallible stdout/stderr retention
and rejects any communication failure, cleanup error or unsuccessful/missing
status before decoding. It retains all report fields and captured bytes on error;
formatting reports byte counts without dumping signer output. This removes the
legacy capture/success/completeness chain without moving ICP version selection,
principal decoding, identity arguments or retry policy into Tooling.

Two focused consumer tests cover exact version admission, version refusal before
identity invocation, literal argument handling, malformed UTF-8/principal replies,
nonzero status, stdout/stderr overflow, exact capture boundaries and group deadline.
Each caller attempt has exactly the expected invocations. Fixtures are prepared
before concurrent tests start; preparing executables inside tests produced an
`ETXTBSY` fixture failure and was corrected without adding invocation retries.
Consumer Clippy and external-lock subset checks pass; no new external selection.
See the source identity and command in [Canic qualification](../qualification/canic/README.md).
The actual signing/submission path, other legacy Process callers and native macOS
are outside this slice. Prior 0.80.0 runtime records remain version-bound.

### Follow-up: exact signer failure evidence and remaining callers

The current Canic 0.213.0 working-tree export passes four focused tests. Replace
broad `is_err()` capture checks with independent observations of `Limit(Stdout)`,
`Limit(Stderr)`, `Deadline` and `Io(NotFound)`. Check original exit code 7 with both
retained streams, known status after cleanup and absent status on spawn refusal.
These assertions prevent an unrelated spawn or decoding failure from falsely
qualifying a timeout/overflow scenario. They live inside the exported private
adapter's test module; no public test hooks or new product API are added.

Two remaining callers need separate admission before further migration:

- `app_build::build_app` initializes a missing Cargo lock with 1 MiB per stream
  and a 120-second deadline. It formats bounded escaped Cargo diagnostics through
  `artifact::cargo_failure`. A migration must preserve that diagnostic contract,
  existing-lock custody and serial partial-build behavior; signer error formatting
  deliberately does not expose captured bytes and is not a drop-in substitute.
- Actual `signed::sign` uses `Command::status`, inherited stdin/stderr and stdout
  redirected to stderr, then admits a retained signing file. It does not currently
  use the legacy Process supplier. The new owner always pipes stdout, so routing
  this caller through it would change terminal behavior and impose new deadline
  policy. Leave it unchanged until those requirements are explicitly admitted.

These findings do not require a public API expansion or migration merely to remove
remaining lexical references. Current source identity and results are recorded in
[Canic qualification](../qualification/canic/README.md).

### Process 0.80.1: Cargo lock initialization

`make test-canic-lock` now qualifies `app_build::build_app` lock initialization in
an independent current Canic export. A private caller adapter replaces the legacy
capture/success/completeness chain with one `Group` run, null stdin, 1 MiB stream
budgets and a 120-second deadline. It retains both buffers, status, primary failure
and cleanup evidence on refusal; no Process API or execution core change is needed.
Unlike signer presentation, this caller must render Cargo diagnostics. Extracting
Canic's existing diagnostic formatter into a byte-slice helper lets both its
retained legacy artifact caller and the new lock caller use identical escaping,
4096-character bounds and truncation markers. No Canic formatter enters Tooling.

Current Canic 0.213.0 passes two focused status/byte/diagnostic tests and all-target
Host Clippy. Real app builds check duplicate-destination preflight, an injected
single-attempt lock failure with escaped diagnostics and unchanged destinations,
initial creation, preservation of an existing lock (with a refusing initializer
still on PATH), exact Wasm identity and serial partial failure. Only the local
Process edge is added; the external lock subset remains unchanged.

This export retains Canic's other Build/Text suppliers, so it does not claim the
combined Build + Text + Process migration. That composition, with current-source
patch admission, is the next qualification step. Actual interactive signing is
unchanged. Source identity is in [Canic qualification](../qualification/canic/README.md).

### Combined caller follow-up

Current Canic 0.213.0 passes `make test-canic-app-build` with Build 0.87.3,
Text 0.77.2 and Process 0.80.1 selected together. The runner checks both Process
edges (Build execution and Host lock initialization) and Text selection, admits
external tuples against the reviewed Build lock, and runs Host/CLI Clippy, two
lock diagnostic tests, app build/refusal fixtures and 34 display tests. Stale
patches were refreshed against captured current source, including new Text
callers, without importing consumer code into product crates or expanding APIs.
See [combined source evidence](../qualification/canic/README.md).

### Same-capture signer integration

The combined app target now qualifies signer version/principal capture alongside
Build, Text and lock initialization on one current Canic export. It shares the
standalone signer's fixture setup and invocation helper, removing duplicated test
wiring. Four signer tests pass together with two lock tests, app-build behavior,
34 display tests, all-target Host/CLI Clippy and dependency admission. The local
Process alias is already selected by the lock patch; the signer migration applies
only its implementation hunks and relies on the combined graph's exact supplier
check. No product source or public API changed. Actual interactive signing and the
separate foreground-client PTY qualification remain distinct boundaries.

### Same-capture foreground integration

Linux `make test-canic-process` adds the direct-child profile client to the combined
Build/Text/lock/signer export. It reuses the selected Process alias and checks the
client's exact local edge and absence of its old direct dependency. A shared
foreground fixture runner keeps standalone and combined test wiring consistent.
The current package-directory rename is reflected only in qualification paths;
no new Process API or consumer policy is introduced. Forty-four tests pass,
including inherited terminal input/stderr through a real PTY, together with real
app builds/refusals and Host/CLI/client Clippy. Ordinary app qualification keeps
its existing platform scope. See the current source evidence in
[Canic qualification](../qualification/canic/README.md).

### Remaining metadata capture boundary

The combined export still uses the retained Process supplier for prepared Cargo
metadata execution. Its next migration must retain null input, owned-group
cleanup, 8 MiB stdout, 1 MiB stderr, a 120-second deadline, Cargo diagnostic
formatting, selection refusal before compilation and destination preservation.
The current combined recipe now proves nonzero exit after valid JSON, both stream
overflows and invalid UTF-8 cannot proceed to compilation. Together with ten
selection and six prepared-tool/wrapper cases, this gives 20 refusal checks before
changing that caller. Existing group/direct-child and display acceptance still
passes on the same captured tree; see [source evidence](../qualification/canic/README.md).

### Metadata execution migration completed in the combined export

The combined Process recipe now migrates the remaining Host metadata capture to
Process 0.80.1. Reuse is limited to the reviewed private lock capture adapter,
relocated into Canic's `cargo_process` module and parameterized by stdout budget.
Lock setup and metadata share capture/refusal handling; Cargo diagnostics remain
consumer-owned. The old ToolError/evidence bridge and synthetic diagnostic evidence
are removed, and Host no longer directly depends on legacy Process in this export.
The existing artifact diagnostic tests retain their assertions with byte inputs.

Forty-seven selected tests and all 20 metadata/tool refusal cases pass on current
Canic, including real app builds, terminal interaction and Clippy. No public Process
API or implementation change was needed. Source identity and exact boundaries are
in [Canic qualification](../qualification/canic/README.md). Ordinary app/standalone
recipes have not been silently promoted to this broader migration; remaining
workspace callers, native macOS and actual signing/submission stay separate.

## Desktop-opener boundary before Process 0.80.2

This section records the retained baseline and the requirement that led to the
implemented [handoff contract](process-handoff.md). The completed replacement
qualification follows below; desktop handoff is no longer an open design task.

Current Canic CLI `view::launch` is the remaining direct legacy Process caller
outside the migrated Host/profile-client slices. It appends one URL argument,
uses null stdin/stdout and inherited stderr, spawns a direct child, then relinquishes
cleanup. The viewer continues serving while the opener may run. It reports a
nonzero exit only when already observed; launch return is neither readiness nor
successful browser startup. Actual signing's direct `Command::status` is separate.

`make test-canic-viewer` now qualifies this retained caller against current source.
A harmless shell fixture waits for a gate opened only after `launch` returns,
then records the exact literal URL. It has bounded self-expiry and performs no
browser/network action. A missing executable retains `NotFound`; the two original
loopback-viewer tests verify report-only serving and polling updates. CLI Clippy
and all four tests pass. Socket denial in the sandbox was resolved by running the
local-loopback qualification with the required access, not by skipping tests.

The replacement's synchronous `run` always closes its child, so it cannot be
substituted here. The read-only production reference documents `relinquish` as a
single nonblocking observation followed by loss of signal/wait custody; it explicitly
leaves subsequent lifetime/reaping to the caller. This fixture proves continued
execution, not eventual reaping. Before adding a handoff mode, define who retains
reaping custody, failures before/after transfer, caller shutdown behavior and the
limits of any completion observation. Do not import the old child/session API or
infer readiness/retry authority merely to remove a dependency. Native macOS and
actual desktop-opener integration remain separate qualification.

### Process 0.80.2 complete execution composition

The current Canic 0.214.0 combined recipe now includes viewer handoff/reaping and
selects only local Process in its resolved workspace graph. Fifty-two tests, all
20 metadata/tool refusals, real app builds, PTY and loopback fixtures, and Clippy
pass. The newly introduced locked Store-source resolver uses the same private
Cargo capture adapter; its source admission and consumer-owned output policy are
preserved. A tiny borrowed-source Wasm fixture and whole-tree comparison check
that publication does not modify the supplier. No Store product implementation
was imported or qualified. See [current evidence](../qualification/canic/README.md).
