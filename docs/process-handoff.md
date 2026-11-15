# Desktop-opener handoff — Process 0.80.2 design

Status: implemented independently in unpublished Process 0.80.2 after checkpoint
`607ff8a`. Fourteen Process tests (six handoff), 19 Build tests, Clippy and Rustdoc
pass on Linux. Current Canic replacement qualification is blocked by its incomplete
workspace dependency rename; native macOS remains unqualified. Caller evidence is in
[the execution contract](process-execution.md#remaining-desktop-opener-boundary).

## Required outcome and ownership

Canic's desktop opener must return while its child may still run, pass the URL as
one literal argument, retain original spawn errors and avoid killing a launched
application on return. Its retained supplier relinquishes waiting custody; a new
implementation must make subsequent reaping ownership explicit instead.

The selected model is a worker that owns exactly one child until wait completes,
plus a caller-held completion observer. Dropping the observer neither signals the
child nor cancels the worker. The caller may observe a terminal status or wait for
it; neither successful spawn nor absence of a status proves readiness. No borrowed
PID, signal handle, retry mechanism, process sampling or global supervisor is added.
This is a separate lifecycle from synchronous `run`, whose close-on-return
guarantee remains intact.

Desktop opening remains consumer policy: executable selection, URL construction,
manual-open fallback and disclosure. The first operation uses null stdin/stdout
and inherited stderr, without captured pipes or an elapsed kill deadline. The
child remains in the caller's process group; this is not daemonization or a promise
to survive parent/session shutdown. The worker exists only while the host process
is alive. Caller limits govern how many launches may run concurrently.

## Transfer and failure sequence

1. Prepare the worker and startup/completion channels before spawning the child.
   Worker creation failure must return its original error with no child effect.
2. Transfer the command to the worker. The worker attempts exactly one spawn and
   reports the original spawn failure or successful ownership acquisition through
   the startup channel. No failure authorizes a second spawn.
3. On success, return a completion observer. The worker retains the owned `Child`
   and performs the wait even if the observer disappears or startup delivery fails.
   Channel failure must not trigger teardown of a successfully launched child.
4. Deliver exit status or the original wait error to the observer. A wait error
   is not evidence of successful reaping; document the remaining uncertainty.
   Only interruption may be retried as part of the same wait, never as a new spawn.
5. After terminal observation, repeated observation must not wait again on a
   numeric PID or reacquire termination authority. No cleanup signal is issued.

The API is `launch(Command) -> io::Result<Completion>`;
`Completion::try_result(&mut self)` borrows a cached completion result, and
`Completion::wait(self)` consumes the observer and returns the original result.
The implementation adds a separate private worker module; it does not import
the old `OwnedChild` API or change synchronous `run`. Private worker-setup and
spawn/wait seams support failure tests without a public testing API.

## Acceptance before consumer migration

- A parent-opened gate proves return precedes child completion; dropping the
  observer still allows the fixture to finish. Use bounded self-expiry, not a
  timing benchmark or an unbounded background process.
- Independently prove eventual reaping on Linux, including after observer drop.
  A child completion file alone is insufficient; a zombie is not reaped. Native
  macOS needs its own applicable evidence, without borrowed-PID signalling.
- Preserve literal argv, original spawn refusal, nonzero exit and wait failure.
  Inject worker-setup and wait failures through a private seam; no public test API.
- Prove one spawn, no cleanup signal and no rewait after a terminal observation.
  Closed startup/completion receivers must not lose an already owned child.
- Qualify the current Canic viewer in a read-only export with harmless opener and
  loopback fixtures. Preserve viewer availability after opener refusal. Never
  launch a real browser or infer browser readiness from process status.
- Recheck synchronous Process and Build acceptance if shared mechanisms change.
  Keep current 0.80.1 consumer evidence version-bound until rerun on new behavior.

No Canic implementation belongs in Tooling product crates. The delivery remains
unpublished; implementation, native-host acceptance and production migration are
separate gates.

## Implementation evidence and current consumer blocker

`target/process-handoff-tests.log` records six new cases: original setup/spawn
errors without child effects; cached nonzero completion; wait interruption and
original terminal errors; reaping despite lost startup/completion receivers;
uncertain worker disconnection; and an independently observed Linux child that
finishes and disappears from procfs after observer drop. The last test records
PID/start identity and does not accept a still-present zombie as reaped. It sends
no signal to a borrowed PID. Core and Build tests, Clippy and Rustdoc also pass.

`make test-canic-viewer-replacement` applies a qualification-only adapter using
`launch` and optional completion observation. Returning drops only the observer;
the worker continues reaping. Its exact local supplier edge and unchanged external
lock are checked before the original loopback and opener fixtures.

The attempted current capture is `target/canic-viewer.VyNQtTn5`, source HEAD
`c39aee1e34dae82be6da89bf6242c3a0e22ade84`, tar SHA-256
`ccfa05cf0c864eb06201ffbe4d2e15ed7c5968b0b518d2c78eab9131bb1f4580`.
Cargo metadata refuses because members inherit `ic-wasm-store-protocol` while
the workspace still declares `canic-wasm-store-protocol`. No sibling edits or
historical fallback were used. `target/process-handoff-viewer.log` retains the
refusal. The replacement viewer patch is not yet consumer-qualified.
