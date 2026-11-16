# Explicit prepared simulator and server ownership

Published in Testkit 0.107.3; [release and registry evidence](release-ownership.md#testkit-01073-publication)
records the verified archive and the additional release-time installed checks.

The optional `managed-server` capability supplies [Tooling #6](https://github.com/dragginzgame/ic-tooling/issues/6).
Testkit owns PocketIC asset selection and the dependency between an instance and
its server. Process owns spawning, fair bounded output, cancellation, group
termination and reaping. FS owns no-follow file admission and create-only asset
publication. Application fixtures, Tokio runtimes, funding, time and gateways
remain caller-owned. Default Testkit and the held-reply-only graph stay unchanged.

## Contract and reuse decision

`prepare` accepts an explicitly downloaded PocketIC 16.1.0 gzip archive, an existing
destination directory and an explicitly prepared absolute gzip executable. It
checks a compiled-in official archive hash, bounds decompression, checks the exact
executable hash, then publishes create-only through FS. No network access occurs
in this API. `PreparedServer::check` is offline and refuses missing, malformed,
aliased, writable-by-others, nonexecutable or checksum-mismatched assets. There is
no cache search, automatic download, overwrite or fallback to legacy Testkit.

`Server::start` checks prepared bytes again and starts one loopback-only server
through Process on a dedicated worker thread. After asset verification, explicit limits bound startup,
whole process lifetime and each captured stream. A private temporary port file
identifies readiness; startup failure cancels and joins the execution owner before
returning its original process/cleanup evidence. Kernel waits and filesystem IO
are not hard real-time guarantees. Observed namespace checks assume cooperative
writers and exclusion of concurrent mutation, as in the underlying FS contract.

`Server::instance` uses the caller's Tokio runtime and original PocketIC builder,
forcing the explicitly owned URL and a finite request deadline. It never launches
an upstream-discovered server. Each instance retains the server owner. Explicit
`Instance::close` attempts deletion before releasing that dependency and reports
attempt completion separately from timeout; it does not claim observed deletion.
Cancellation/panic during creation or deletion can leave remote effects. Explicit
server stop refuses while instances/creation futures retain ownership. Dropping
the last owner cancels and joins the server worker as fallback, including unwind;
it is not an asynchronous instance deletion promise. `client_mut` permits upstream
localhost gateways. Callers must not move/replace the upstream client through this
mutable borrow or attach independent clients that outlive the owner. The API
preserves original types rather than claiming to enforce those caller obligations.

The implementation is new orchestration over the existing admitted Process,
Artifacts and FS contracts. No historical Testkit lifecycle implementation or
generic harness is imported. The removed caller obligation is coordinating
unbounded child startup, pipe drains, asset verification and server-before-instance
teardown independently in every native test harness.

## Consumer requirements and scope

Canic was captured read-only at HEAD `e84702b3bdcf4eb209476aadd053df21f18dd8eb`
plus tracked edits and non-ignored additions, tar SHA-256
`9d5cd0cdab4dce808427caf8cc9694bd8615b9540b4758938dde6496bd53a515`, under
`target/issue-6/canic-review/`. Its qualification and DB witness currently use
legacy preparation/startup plus explicit instance close; CLI/signer fixtures
also borrow localhost gateways. Those are requirements, not new acceptance.
Canic's owning migration is [Canic #15](https://github.com/dragginzgame/canic/issues/15).
No Canic/Auth implementation is edited or migrated by this supplier slice.

Official archive digests come from the PocketIC 16.1.0 GitHub release metadata;
executable digests were measured after verifying and decompressing those archives.
Linux x86-64 and macOS x86-64/ARM64 have explicit selections. Unsupported hosts
refuse. Native macOS execution, publication and consumer migration are separate
from local implementation evidence.

## Preparation and use

Enable `managed-server` in the consumer's Testkit dependency. Preparation is an
explicit setup step; ordinary tests only call `PreparedServer::check`. The example
prints the current host's URL and archive hash:

```sh
cargo run -p ic-testkit --features managed-server --example prepare_server -- --asset
```

For Linux x86-64, an explicit download and offline preparation can be run as below.
On macOS use the host URL printed above and the explicitly selected gzip path.
The destination must already exist, and neither it nor the archive may be writable
by group/others. An existing invalid destination is refused, not repaired.

```sh
umask 077
mkdir -p target/prepared-pocketic
curl --fail --location --max-time 120 --max-filesize 67108864 \
  --output target/pocket-ic.gz \
  https://github.com/dfinity/pocketic/releases/download/16.1.0/pocket-ic-x86_64-linux.gz
cargo run -p ic-testkit --features managed-server --example prepare_server -- \
  target/pocket-ic.gz target/prepared-pocketic /usr/bin/gzip
```

The caller chooses limits and owns the runtime. Startup and stop are synchronous;
do them outside latency-sensitive runtime workers. For example:

```rust
use ic_testkit::{pocket_ic::PocketIcBuilder, server::{Deletion, Limits, PreparedServer, Server}};
use std::{path::Path, time::Duration};

fn run(prepared: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = Server::start(PreparedServer::check(prepared)?, Limits {
        startup: Duration::from_secs(30),
        lifetime: Duration::from_secs(180),
        stdout_bytes: 1_048_576,
        stderr_bytes: 1_048_576,
    })?;
    let runtime = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    runtime.block_on(async {
        let timeout = Duration::from_secs(30);
        let instance = server.instance(
            PocketIcBuilder::new().with_application_subnet(), timeout,
        ).await?;
        // Application fixtures use instance.client(); gateways use client_mut().
        assert_eq!(instance.close(timeout).await, Deletion::Attempted);
        Ok::<_, ic_testkit::server::Error>(())
    })?;
    let stopped = server.stop()?;
    // Assess stopped.process, stopped.output and stopped.abandoned in caller policy.
    // Intentional stop normally reports Failure::Cancelled, not successful exit.
    assert_eq!(stopped.abandoned, 0);
    assert!(stopped.process.status.is_some() && stopped.process.cleanup.is_empty());
    Ok(())
}
```

`Deletion::Attempted` is insufficient for a consumer claiming observed deletion.
The installed test independently reads the original server's instance inventory
before stopping it. A caller dropping an instance, cancelling creation/deletion,
or timing out deletion must retain the uncertainty; `Stopped::abandoned` counts
those operations. Process expiry can precede deletion, so a live owner does not
override the explicit lifetime budget.

## Focused local acceptance — 2026-11-16

The initial Linux x86-64 checks passed before committing the implementation:

- Four offline/startup tests cover missing/corrupt/nonexecutable/linked assets,
  changed identity, early exit, startup deadline, malformed readiness, and both
  output stream limits, retaining exit and cleanup evidence.
- The installed success test prepares the pinned archive, starts independent
  instances, exercises an original localhost gateway and held reply, independently
  observes both deletions, and confirms stop closes the endpoint. Dropping the
  server handle preserves a retained instance until its close.
- The installed failure test refuses failed or incorrect decompression without
  publication; cancelled creation, timed-out deletion and fixture unwind report
  three abandoned operations. Explicit stop and lifetime expiry reap the process.
- All-target Clippy with both features and Rustdoc with warnings denied pass.
  Default and held-only normal/build graphs exclude the new Host dependencies;
  managed-only has no direct Platform or WAT dependency.

Logs and dependency graphs are in `target/issue-6/`; installed logs are
`installed.log` and `installed-failures.log`. Repeat the focused cases with
`make test-testkit-server`, or, after explicit archive/tool preparation,
`TESTKIT_SERVER_ARCHIVE=/absolute/archive.gz TESTKIT_GZIP=/absolute/gzip make test-testkit-server-installed`.
The latter needs loopback socket access and runs only the two installed server
tests. Ordinary tests download nothing. This evidence does not qualify Canic's
application callers or native macOS, and does not claim a release or publication.
