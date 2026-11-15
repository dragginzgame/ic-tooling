# Streaming Cargo artifact selection

The first replacement change removes full-message retention from the Build Wasm
example and real shared-target collision fixture. It follows the current
Scheduling build caller, which retains every message before calling
`select_cdylib_wasm`. The original Scheduling checkout has not been edited.
A [pinned migration patch](../../qualification/scheduling/README.md) is compiled
and checked against Scheduling's existing host tests in an isolated source export.

## Consumer migration

Construct `CdylibWasmSelector::new(manifest, recipe, max_outputs, max_path_bytes)`
before execution, while holding the caller's source/target custody. Feed each
message by reference to `observe` from the existing `execute_cargo` callback.
Handle diagnostics and required build-script fields in that same callback, then
discard the message. Return observation errors through the callback so the decoder
refuses admission while Host continues draining and collecting cleanup evidence.

After execution, require process success, successful message decoding and admitted
build facts, in that order. Call `finish` and capture the returned path before
releasing the target lock. Keep the capture owner through every consumer use.
The [executable example](../../crates/ic-build/examples/wasm_build.rs) implements
this sequence. Its limits of eight candidates and 64 KiB of encoded path bytes
are example policy, not library defaults.

The existing slice-based selector remains supported and delegates to the same
implementation without imposing new limits on existing callers. A public
streaming entrypoint is necessary because Scheduling owns the current message
buffer outside Build; a private accumulator alone cannot remove that buffer.

## Protected behavior and limits

- Exact canonical manifest and library/named-example selection; duplicate paths
  count once and missing/ambiguous output remains a refusal.
- Exactly one successful build-finished record. Missing, failed or duplicate
  completion cannot admit an artifact.
- Original filesystem errors are returned by `observe`. A failed observation
  permanently prevents `finish` from succeeding even if its error is ignored.
- Inclusive candidate-count and cumulative encoded-path-byte ceilings are checked
  before inserting each distinct path. Diagnostics and build-script messages are
  not retained by the selector. Limits exclude allocator overhead, manifest and
  recipe storage, the currently decoded message and caller-owned observations.
- No filesystem fallback, retry, tool-probe removal or new capture authority.
  Canonicalization observes paths; it does not replace caller custody.

## Evidence and performance

`make test-build-selection` checks protocol/failure behavior and executes two real
Cargo Wasm builds with custom library names and a shared target. The fixture
asserts that the first captured bytes survive the second build overwriting the
reported path, and that required build-script input evidence survives streaming.
The unit regression feeds 10,000 diagnostics plus repeated candidate messages
under a one-candidate/exact-path-byte allowance.

The demonstrated structural improvement is retained selection storage bounded by
caller limits instead of retaining all Cargo messages. It removes the complete
message vector and subsequent build-script scan from the real fixture, and the
vector from the runnable example. No wall-time, RSS, overall LOC reduction or
Scheduling speedup is claimed. `make test-scheduling-consumer` additionally
qualifies source-inventory, process refusal and receipt-publication behavior
against the actual host consumer. It compiles the migrated callback but does not
run the complete PocketIC workflow. Remaining full-message consumers may have other
observation requirements and require their own migration review.
