# Testkit server command

Testkit 0.107.4 adds `ic-testkit-server` behind the optional `cli` feature for
[Tooling #10](https://github.com/dragginzgame/ic-tooling/issues/10). This version is
published on crates.io. The command owns setup/check/run policy;
the [managed-server library](testkit-server.md) owns asset selection, byte admission
and server lifetime. Process owns bounded execution and child groups, FS owns asset
publication, and Text owns diagnostic escaping. Consumers retain their runtimes,
instances, application assertions, simulation and retry policy.

## Ownership and reuse decision

The private qualification runner demonstrated this command workflow for archive
and Scheduling tests. Its orchestration is admitted as one coherent unit into
Testkit's product package; the private package and its second dependency graph are
removed. No historical launcher source, compatibility layer, pin catalog or tool
store is imported. The product command additionally preserves child stdin/output,
handles interruption signals, and retains decompression failures. Default Testkit
and held-only/managed-only users do not acquire Clap, CtrlC or Text. The binary is
in the existing Testkit release group; it needs no independently versioned package.

This removes the obligation to implement a launcher in each consumer. It follows
Toko's [PocketIC ownership contract](https://github.com/dragginzgame/toko-tooling/blob/08d63df7661910554ef72980037f55edd914aba0/rules/engineering-details.md#rust-workspaces-and-portable-tooling).
The command syntax below is a new supported contract, not a promise of legacy
0.106.0 CLI compatibility. The managed-server library API is unchanged.

## Installation and explicit preparation

Use the existing reviewed Toko installer to select 0.108.1 from the
consumer checkout. Installation is an explicit setup step; ordinary execution
adds `--check` to this selection to admit an already installed executable offline:

```sh
TESTKIT_SERVER_CLI="$(bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh \
  --consumer "$PWD" --package ic-testkit --version 0.108.1 \
  --bin ic-testkit-server --features cli --profile release)"
export TESTKIT_SERVER_CLI
```

Tooling developers can build the local candidate explicitly with
`make prepare-testkit-server-cli` and select
`TESTKIT_SERVER_CLI="$PWD/target/debug/ic-testkit-server"`. This is development
qualification, not registry installation or permission for consumers to depend on
a mutable sibling executable. The [release receipt](release-ownership.md#testkit-01074-publication)
records registry publication and installation evidence separately from development.

With the selected CLI and prepared absolute curl/gzip executables:

```sh
mkdir -p target/simulator
"$TESTKIT_SERVER_CLI" setup "$PWD/target/simulator/prepared" \
  "$PWD/target/simulator/setup-evidence" /usr/bin/curl /usr/bin/gzip
"$TESTKIT_SERVER_CLI" check "$PWD/target/simulator/prepared"
```

`setup DIRECTORY NEW_EVIDENCE_DIRECTORY ABSOLUTE_CURL ABSOLUTE_GZIP` is the only
network operation. Directory/evidence/tool arguments must be absolute without
CR/LF. Evidence must be new; the destination is created with private permissions
if absent, and its parent must exist. Curl configuration files are disabled;
HTTPS redirects, a 120-second curl deadline, 130-second Process deadline, 64 MiB
body limit and 128 KiB per captured output stream bound the download. Testkit
verifies its selected archive, bounds decompression, and publishes the executable
create-only. A separate offline check admits it again before success. Existing
invalid assets are refused, never repaired or overwritten. No automatic retry.

`check DIRECTORY` downloads nothing and prints only the admitted executable path
on stdout. Setup has the same success stdout. Diagnostics go to stderr; the gzip
archive stays in the new evidence directory. `POCKET_IC_BIN` and upstream server
search/download are not alternative sources. Tooling qualification selects
`TESTKIT_SERVER_DIRECTORY`, `TESTKIT_SERVER_ARCHIVE` and `TESTKIT_GZIP` explicitly;
Scheduling selects `SCHEDULING_SERVER_DIRECTORY`.

## Execution, output and failure

```sh
"$TESTKIT_SERVER_CLI" run "$PWD/target/simulator/prepared" \
  "$PWD/target/simulator/run-evidence" --ttl 300 -- cargo test --locked --offline
```

`run DIRECTORY NEW_EVIDENCE_DIRECTORY --ttl SECONDS -- COMMAND ARGS...` admits
prepared bytes offline and starts one loopback server. TTL defaults to 300 seconds
and accepts 1–3600. Startup has 30 seconds; server output has 1 MiB per stream;
server lifetime is command TTL plus 60 seconds. The child receives
`IC_TESTKIT_POCKET_IC_URL`; `POCKET_IC_BIN` is removed. Command construction and
Cargo policy stay with the caller. No Cargo invocation is built into Testkit.

The child uses inherited stdin, supporting pipes and files. Stdout/stderr are
forwarded byte-for-byte and also retained, each with a 16 MiB total byte budget.
This is intentional command IO passthrough, including binary/terminal bytes;
only Testkit-owned diagnostics are escaped. The command runs in an owned process
group. This is a bounded command runner, not a foreground-terminal/job-control or
daemonization service. Callers must exclude processes that escape the owned group.
Blocking output sinks/kernel operations are not hard real-time guarantees.

After normal/nonzero exit, spawn refusal, output refusal, timeout or cancellation,
Process closes the command group and the CLI stops and joins the server. SIGINT,
SIGTERM and SIGHUP set Process cancellation; startup/preparation observe it at their
bounded phase boundaries. Cancellation does not undo already published files or
remote effects. Forced termination such as SIGKILL cannot promise Rust cleanup.
Nonzero command/curl/gzip exit codes are preserved; signal termination, admission,
communication or cleanup failure returns 1 when no nonzero tool code exists.
Clap usage errors return 2. A successful command cannot conceal failed cleanup.
Secondary report-write failure cannot replace an already observed nonzero status.

Each new evidence directory retains bounded command/server streams, Process
reports and the started endpoint. External clients must finish before the child
exits. They do not hold a Rust `Instance`, so a zero abandonment count is not proof
of remote deletion. Scheduling and held-reply checks independently observe instance
inventory. Shutdown/deletion never implies restoration of time or data.

## Diagnostics and acceptance

[Tooling #3](https://github.com/dragginzgame/ic-tooling/issues/3)'s reporting obligation
is retained. At most 4096 diagnostic source bytes are shown through Text escaping,
with colons additionally escaped against CI workflow-command syntax. Raw bounded
download/decompression/startup stderr and original Process status remain available.
CI uploads named setup evidence with seven-day retention, excluding archives and
partial downloads. Evidence starts in an exclusive private directory; existing
or linked evidence is refused. Namespace writers must remain excluded.

`make test-pocketic-setup` uses a fake curl and real CLI: exit 56, escaped/raw
bytes, output overflow, checksum refusal, offline missing-asset refusal, invalid
TTL and stale/link refusal. With an existing `TESTKIT_SERVER_ARCHIVE`, it also
checks gzip exit 43 and successful preparation. `make test-testkit-server-cli`
uses a real prepared server for command IO, nonzero exit, timeout, SIGTERM, spawn
failure and secondary diagnostic failures, independently checking endpoint closure.

The archive gate installs the CLI from the extracted candidate with Cargo, then
uses that installed executable for these checks and both existing held-reply tests.
The native workflow runs the same gate on Linux and both macOS architectures.
Local evidence and remaining host/delivery limits are recorded in the
[current handoff](status/current.md). Earlier private-runner evidence remains under
`target/debt-held/` and `target/simulator-owner.g7XjBr1S/`; it is not product release
or native macOS evidence.

## Local candidate evidence — 2026-11-16

This preceding development evidence used Protocol 0.2.6. The final published
0.107.4 with Protocol 0.2.7 passed the complete archive gate separately; see its
[release receipt](release-ownership.md#testkit-01074-publication).

The corrected local Testkit 0.107.4 archive is
`target/issue-10/repackaging/package/ic-testkit-0.107.4.crate`, SHA-256
`9e83e20d1236e787b6839bf36d8c3cf68c68e097d4bf287f2b58fcdcb0b245e8`.
Cargo installed its CLI from the extracted archive into
`target/issue-10/installed-corrected/`, using a fresh compilation directory.
`--version` reports `ic-testkit-server 0.107.4`. That executable passes complete
setup fixtures (including real archive preparation and gzip exit 43), byte-exact
piped IO, success/nonzero/timeout/SIGTERM/spawn cleanup, and original exit retention
when secondary report writes fail. It also runs both held-reply installed tests;
both managed-server installed tests pass separately. Logs and source hashes are
under `target/issue-10/`; runtime evidence is
`target/testkit-server-cli.1aKreAs8/`.

The initial archive gate at `target/library-release.6q1V7BeI/` passed package/source,
Clippy, default/full library tests and documentation checks, then exposed Clap's
package-name default in the CLI version output. The corrected archive sets the
binary name explicitly. Its library source is unchanged from that gate; focused
Clippy and the corrected installed-command checks above pass. The initial gate
did not finish successfully, and its earlier library results are reused only for
those unchanged sources. Default/managed-only/held-only graphs exclude CLI
dependencies; the CLI graph excludes fixture WAT and Platform dependencies.
Formatting, baseline admission, ShellCheck and actionlint pass.

The current [Scheduling capture](../qualification/scheduling/README.md#current-runner-acceptance--2026-11-16)
passes real pressure/held-upgrade checks through the product command. This removes
the private launcher package without moving the consumer's client or assertions.
At that checkpoint, Toko installer `--describe` accepted the package/bin/features
selection, while registry installation, publication and native macOS acceptance
were outstanding. No hosted run or DB migration is claimed by those local results.

## DB adoption boundary and historical requirements

DB's migration is thin: select this executable through Toko, invoke setup explicitly,
use offline check for ordinary admission, and make `scripts/with-pocketic.sh` invoke
`run` with its prepared directory, a unique evidence directory and original command.
An existing externally supplied endpoint can still execute the DB command directly.
Keep DB's runtime, direct PocketIC client, missing/invalid endpoint test, raw-memory
rollback oracle and console assertions. Its standalone console test removes the
endpoint and exercises that command wrapper; piped input/output must still work.
The original evidence from DB #3 is retained in [Tooling #10](https://github.com/dragginzgame/ic-tooling/issues/10).
No DB source or adoption is changed by this supplier implementation.

[Earlier DB/Platform/Scheduling observations](https://github.com/dragginzgame/ic-tooling/blob/10e7d64e3103af84c18e433be434e4527dee75d6/docs/simulator-ownership.md)
retain their exact historical scope. Current inspected callers do not justify a
shared multi-canister snapshot-set abstraction. Such an operation needs an unfrozen
consumer's controller identity, ordering and partial-effect recovery contract;
this command does not introduce cached baselines or claim snapshot atomicity.
