# Current DB external-session qualification

Current recipes select the next unpublished Build 0.87.2 / Text 0.77.1 where
used. Their patches and locked graphs admit current local sources; historical
runtime evidence below remains bound to its recorded checkpoint inputs.


`make test-db-consumer` snapshots current `DB_SOURCE` (default `../ic-db`), including
uncommitted source and unignored additions, without modifying the sibling. It
applies [the migration](external-session.patch) and requires a graph with PocketIC
16.1.0 and no Testkit library. See the [ownership decision](../../docs/simulator-ownership.md).

The patch removes the old managed-instance wrapper from the driver and updates its
callers to borrow the upstream async client and explicitly call async `drop`.
The current consumer's original rollback assertions remain unchanged. Provisioning
moves from an ignored library test to the released CLI at `make m0-setup`; instance
use requires the launcher-provided endpoint. This does not claim all driver commands
or all integration subjects have been executed, nor modify DB's actual checkout.

Without `DB_TESTKIT_CLI`, the installed Toko checker selects Scheduling’s prepared
0.106.0 CLI with feature `cli`; no automatic installation occurs. An explicit
`DB_TESTKIT_CLI` overrides this selection.

Prepare Rust 1.100.0 with wasm32-unknown-unknown, cached dependencies, the released
`ic-testkit-server` 0.106.0 CLI, and an authenticated PocketIC 16.1.0 binary first:

```sh
DB_TESTKIT_CLI=/absolute/path/to/ic-testkit-server \
DB_SERVER_DIRECTORY=/absolute/path/to/prepared/server-assets \
make test-db-consumer
```

The server asset directory defaults to the sibling's `.tools/m0-pocketic`. Nothing
is downloaded. The CLI and server byte identities are recorded and checked after
execution. Loopback and simulator subprocess access are required.

The rollback canister is compiled from this snapshot into a private per-run Wasm
target. Cargo artifact records select exactly one module; its identity is recorded
before execution and checked afterward. Native driver builds may reuse a separate
host compilation cache. Source, metadata, exact Wasm, logs and inventories stay
under the printed `target/db-consumer.*` export.

The runner executes two subjects:

- Missing external endpoint refuses, even with an unusable `POCKET_IC_BIN` selected.
- Original `message_rollback`: nine fault/commit modes, raw backing-memory equality
  after traps, retained mutation after normal error replies, row/index checks and
  duplicate-key refusal without allocation changes.

An independent read-only server inventory begins empty. After client close, it
must report one `Deleted` instance while remaining reachable. The command-scoped
launcher then shuts down its own server. This observation does not strengthen the
upstream best-effort deletion API or prove cleanup after cancellation/failure.

Latest Linux run passed both subjects and the deletion/reachability oracle at
local HEAD `416a518ad722aeb46e28051194dcf039637c1bd2` plus uncommitted edits.
Export: `target/db-consumer.mQ9DgBhA`; source snapshot SHA-256:
`d56b48f9dcdac20834a9d3145d4bd7eb150079f56382528b26a9b168b35b646e`.
Log: `target/db-consumer-runner.log`. Warning-denied Clippy passed for all affected
driver targets on an identical source snapshot; `target/db-consumer-clippy.log`.
External-lock admission confirmed that retained registry identities/checksums did
not change. Existing Wire/Storage releases and legacy host text/artifact uses are
unchanged; those suppliers were not migrated. No production cutover or performance
claim is made, and no new Tooling product crate was needed for this slice.

## Current console Text

`make test-db-text` applies `text.patch` after `external-session.patch`, selecting
new Text 0.77.0 and its `escape`/`bordered_table` operations. It builds only the
current records canister and runs the exact existing `console_inspects_and_queries`
subject, plus the external-endpoint refusal. The ordinary rollback target remains
separate. This console test exercises schema/status tables, query/aggregate and
cursor output, exact numeric values, help and invalid commands through subprocesses.
No DB product code is copied into Tooling, and the sibling stays read-only.

The console is a local fixture, not Canic integration or a production DB client.
Frozen Wire/Storage implementations remain selected releases. Text adds no runtime,
server or terminal framework. The outer prepared CLI owns server shutdown; remote
instance observations are recorded separately from console output acceptance.

Console Linux evidence on 2026-11-15: current HEAD
`832ca7d661cb0ef1cd2e705e9919a406b547a566` plus edits, source tar SHA-256
`cab88c46bd8bbc903a22ce571c77bba86859c2b3274f5115ce3bc4c22661e595`.
Export `target/db-consumer.A8BzC7Hl`, log `target/db-text-run.log`: the exact
console subject and missing-endpoint refusal passed. All 30 instances were
independently observed deleted, with the external server reachable; selected CLI,
server and Wasm hashes were unchanged. Console/records-test warning-denied Clippy
passed against identical source in `target/db-text.PafdI5mw` (log
`target/db-text-clippy.log`); retained external-lock identity admission passed.
This does not extend the prior nine-mode rollback evidence to the changed DB
source or establish cleanup after panic/cancellation. The console acceptance
uses released Storage/Wire; their repositories remain frozen and untouched.
