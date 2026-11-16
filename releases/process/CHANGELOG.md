# Process changelog

Package: `ic-host-process`. Unpublished replacement; no registry release.

## [0.80.2] - Unreleased

- Compose current Canic Host's Process and filesystem adapters: nine focused
  Process checks pass alongside 52 paired filesystem cases and two custody tests.
  Its native normal/build graph excludes legacy FS/Process; CLI/runtime and native
  macOS qualification remain separate. No Process API change is introduced.
- Start the next unpublished line after GitHub checkpoint `607ff8a`. Update local
  Process selections without changing its synchronous API or implementation.
- Define [desktop-opener handoff](../../docs/process-handoff.md): the worker keeps
  child/reaping ownership while the caller observes completion. Observer drop
  must not kill the child or abandon reaping. Implementation and native acceptance
  remain pending; prior consumer runtime evidence stays tied to 0.80.1.
- Implement `launch` and `Completion`: return after spawn, retain worker reaping
  custody after observer drop, and preserve cached status or original wait error.
  Six new tests include independent Linux reaping evidence and injected setup,
  interruption and channel failures. All 14 Process and 19 Build tests, Clippy
  and Rustdoc pass. Synchronous execution remains unchanged.
- Prepare export-only viewer migration. Current Canic metadata refuses an
  incomplete `ic-wasm-store-protocol` workspace rename, so replacement consumer
  qualification remains pending; no historical fallback or sibling repair.
- Qualify current Canic 0.214.0 viewer replacement after the workspace rename
  settles, including Linux reaping after observer drop and CLI Clippy.
- Integrate handoff with the combined execution recipe: 52 selected tests, 20
  metadata/tool refusals, real app builds and Clippy pass with only local Process
  in the resolved graph. Preserve the new locked Store-source selection and
  consumer-owned output location; a tiny borrowed-source fixture stays unchanged.

## [0.80.1] - Unreleased

- Start the next unpublished line after GitHub checkpoint `f086547`; the public
  Process API and implementation are unchanged.
- Migrate current Canic signer version/principal capture in a read-only export.
  Replace legacy capture/success/completeness calls with one bounded group run
  and a private consumer adapter retaining status, failure, cleanup and bytes.
  Preserve exact version admission, principal parsing, limits and no retry.
- `make test-canic-signer` uses harmless local fixtures for arguments, version
  refusal, invalid replies, output limits, nonzero exit and deadline. It checks
  the local supplier edge and external lock, then runs consumer Clippy and tests.
  No keys, network calls, sibling edits or native macOS acceptance are implied.
- Strengthen signer capture qualification on current Canic 0.213.0: distinguish
  exact stdout/stderr limit failures, deadline and missing-executable failures;
  verify original nonzero status, retained bytes and cleanup evidence independently.
  Four focused tests and consumer Clippy pass. No Process API change is needed.
- Qualify Cargo lock initialization through the group owner in a current Canic
  export. Retain bounded escaped Cargo diagnostics, exact status and cleanup
  evidence, absent-lock initialization and existing-lock preservation. Two
  diagnostic tests, all-target Host Clippy and real app-build/refusal checks pass.
  The export retains other Build suppliers; combined migration remains separate.
- Follow up with combined current Canic qualification: Build 0.87.3, Text 0.77.2
  and Process 0.80.1 pass app builds, lock refusal/preservation, diagnostic and
  display checks in one captured working tree. Keep the standalone evidence tied
  to its earlier inputs; no production cutover or complete legacy removal claimed.
- Include all four signer fixture tests in that same combined export. Standalone
  and combined recipes share fixture preparation and test invocation, removing
  duplicate wiring while retaining separate source/graph admission for each run.
- Add Linux `make test-canic-process` for same-capture group and direct-child
  qualification. Refresh the foreground migration for Canic's package-directory
  rename and share its fixture runner. The combined run passes 44 tests including
  a real PTY interaction, real app-build/refusal checks and Host/CLI/client Clippy.
  Ordinary app qualification retains its platform scope; macOS remains open.
- Migrate prepared Cargo metadata capture in the combined Process export. Share
  one private Cargo adapter with lock initialization, preserving separate stdout
  budgets, diagnostic rendering and failure evidence. Remove Host's direct legacy
  Process dependency and obsolete evidence conversions. Forty-seven tests, all
  20 metadata/tool refusals, real app builds and Clippy pass; no new public API.
- Record the remaining CLI desktop-opener boundary with four current-source
  fixture/viewer tests and Clippy. Running-child relinquishment is distinct from
  synchronous close; no replacement handoff API or reaping guarantee is claimed.

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
