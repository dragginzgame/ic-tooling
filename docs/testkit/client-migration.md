# Async-only PocketIC client

The next Testkit minor release removes the blocking client. Testkit 0.86 and
earlier retain the old API. The maintained implementation remains in this
package; PocketIC server 16.1, external DTO identities, protocol bytes, typed
failures and snapshot transfer budgets are unchanged.

## Imports and calls

Import `ic_testkit::pocket_ic::{PocketIc, PocketIcBuilder, ClientError}`.
The public `nonblocking` module is removed. All network operations are async.
Replace `builder.build_async().await` with `builder.build().await`, and
`config.build_instance_async(builder).await` with
`config.build_instance(builder).await`. Existing blocking callers add
`.await` to construction, network calls, Wire calls and lifecycle helpers.
There is no compatibility wrapper, private runtime or automatic client Drop I/O.
Read `client.instance_id` instead of `instance_id()`, and use
`SnapshotTransfer::advance(...).await` instead of `advance_async`. The obsolete
builder-thread spawn/disconnection startup errors are removed.

The caller owns a Tokio runtime with I/O and time enabled. Async tests can use
`#[tokio::test]`; synchronous test coordinators can own a runtime at their
application entrypoint. Libraries must not rebuild blocking wrappers per method. Keep the runtime alive
through every client/pool operation and explicit close. Reused HTTP connections
belong to that runtime; do not create a runtime per pooled lease. Blocking test
coordinators need a live multithread runtime so network drivers keep progressing.

`PocketIc::new(server_url).await` and
`PocketIc::list_instances(server_url).await` require explicit endpoints.
Builders without a URL return `ClientError::MissingServerUrl`. Provision with
`ic-testkit-server setup` or `pic::server_assets::setup`, admit installations
with `check`, and start a selected binary through `PocketIcStartupConfig`.

## Explicit teardown and cancellation

Await `owned.close().await` on a `PocketIcInstance<PocketIc>` to delete its
instance while retaining the managed server. For a raw client, await
`client.drop().await` while its server remains available. Rust Drop performs
no client network calls. Dropping an owned server still terminates/reaps its
process group and stops all its instances. Borrowed servers stay caller-owned.

Dropping a network future does not prove that its operation was rejected.
Preserve accepted operation/ingress identities and reconcile uncertain effects;
do not replay updates automatically. Cancelling managed construction reaps its
owned server. A borrowed-server creation may need caller reconciliation after
cancellation or a lost response. Explicit close is also fallible and must not
be retried implicitly. Snapshot capture rolls back known work on returned failures;
cancelling its future cannot run async rollback. Retain isolated server custody or
reconcile temporary snapshots explicitly on a borrowed server.

## Helpers and baseline pools

Await installation, snapshot capture/restore, diagnostics, simulated-time helpers,
held-reply inspection and Wire calls. Pure Wasm fixture generation and benchmark
analysis remain synchronous. Retry/readiness callbacks are async callables.

Implement asynchronous `PocketIcBaselineRecipe` lifecycle methods, including
reset and validation. Recipe futures are Send and metadata is Send + Sync.
`StandaloneCanisterRecipe` builders return a Send future. A closure that owns
shared inputs can clone them before returning `async move { ... }`.

Await `pool.acquire().await`. Tokio owns FIFO capacity admission and queued
cancellation. Cancelled preparation retains its slot value for teardown and forces
a rebuild with `RebuildReason::PreparationCancelled`; it never exposes partially
reset state. Fatal failures, one permitted recovery build and both recovery causes
remain distinct. Dropping a lease returns its slot; it does not delete the baseline.
Await `pool.close().await` after all leases end, before stopping a borrowed
server; it returns every instance deletion failure. Do not discard those failures.

Wire requests, local baseline metadata and diagnostic writers may be non-Send;
use their futures on the calling runtime. The pool's separate Send requirements
permit concurrent recipe preparation.

## Dependency and provenance boundaries

The client is native-only. Wasm measurement retains Runtime/Types without HTTP,
server, Tokio, tempfile or Candid. Principal, management SDK 0.11, certification and
transport identities stay unchanged. Platform supplies Candid/management adapters
without a PocketIC feature. Earlier Platform client imports must migrate directly
to Testkit; no compatibility exports are supplied.

[Provenance](../../crates/ic-testkit/pocket-ic-provenance/README.md) retains the
exact donor and upstream reconstruction. The maintenance patch records this cut;
original donor/upstream hashes are unchanged. Use the focused commands in
[host qualification](../hosts.md). Linux evidence is separate from native macOS
Intel and Apple Silicon qualification.
