# Native execution: next owner admission

Status: partial implementation in unpublished `ic-host-process` 0.80.0, adopted
by Build, with current Canic artifact/metadata and foreground-client qualification.
Signer group-capture migration and native macOS acceptance remain open.
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
0.79.0 line (`target/ic-host-process-registry.ndjson`). The replacement uses 0.80.0
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
