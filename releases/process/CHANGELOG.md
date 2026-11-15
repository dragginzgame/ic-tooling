# Process changelog

Package: `ic-host-process`. Unpublished replacement; no registry release.

## [0.80.0] - Unreleased

- Introduce one synchronous execution owner with explicit group/direct-child
  authority, null/inherited stdin, bounded stdout and captured/inherited stderr.
  Deliver bounded chunks to caller observers; retain status, communication failure
  and cleanup errors independently. No tool installation, sampling or IC policy.
- Move Build onto this owner, retaining its Cargo decoder, fallible diagnostic
  retention, sink errors and public failure categories in its private adapter.
  Reuse is limited to the reviewed replacement Build lifecycle/draining mechanism;
  no legacy process crate or old test tree was imported.
- Five independent process tests and the 19 existing Build tests pass. A separately
  selected Linux PTY test checks real inherited terminal input and stderr while
  preserving foreground process-group membership. Clippy and Rustdoc pass.
- Native macOS and current Canic signer group-capture migration remain gates.
  No performance or production-cutover
  claim is made. The crates.io sparse index selected published baseline 0.79.0.

### Cleanup and current Build qualification

- Add a private close-operation fault seam; no public testing API. Verify ordered
  group/direct/reap failure evidence, retained natural status, successful direct
  fallback with group refusal still visible, and no second signal after close.
- Independently observe Linux descendant identity/lifetime: owned-group cleanup
  stops its descendant; direct-child cleanup permits a descendant to complete only
  after the parent has closed. No borrowed PID receives a signal.
- Eight process tests, 19 Build tests and the separately selected PTY case pass.
  Current Canic artifact/metadata qualification also passes through the shared
  core; its changed diagnostic presentation is retained in the consumer adapter.

### Foreground consumer qualification

- Qualify the current Canic profile client's Direct-child invocation through a
  read-only export. Preserve inherited terminal input/stderr, the 16 KiB stdout
  ceiling, 300-second deadline, argument construction, reply decoding and no retry.
- Three ordinary fixture tests and one Linux PTY test pass, covering exact argv,
  binary argument-file choice, exact/over limits, nonzero status, invalid replies
  and one invocation per attempt. No credentials or network calls are used.
  Client Clippy, local supplier-edge and external-lock checks pass without a new
  product API or any sibling modification.
