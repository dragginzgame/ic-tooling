# Current implementation — 2026-11-16

## Authority and scope

[Design revision 2](../DESIGN.md) is accepted. Required outcomes drive interfaces;
no old product source/test tree was admitted wholesale. Historical production
reference: `a00369fc29fa837c6deb022cceb7e09aa447292c`; superseded design checkpoint:
`cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`. Preserve that evidence.
Canonical repository: `dragginzgame/ic-tooling`; predecessor: `ic-tooling-legacy`.
Historical checkout names are not current source selections.

Use [read-only current-source snapshots](../qualification/local-sources.md),
including sibling edits. Refuse capture drift and stale patches. Consumer product
code stays in its owner. Auth is on hold; Storage, Observability and Wire are
frozen. Backup adoption remains user-owned. Stay on `main`; ordinary fixes remain
local, and hosted CI runs asynchronously.

## Latest delivery

Testkit 0.107.4 is published from `67ac61f89a7311e278027bee3078de7e24a0a241`, tag
`testkit-v0.107.4`, with byte-identical non-yanked registry readback. It supplies
the optional [server CLI](../simulator-ownership.md), managed server, compatible
Protocol 0.2.7 and Types 0.21.2 requirements. Original PocketIC types and separate
default/held-reply/managed-server/CLI feature boundaries remain.
Artifacts/FS 0.80.2, Process 0.80.2, Build 0.87.3, HTTP 0.80.0 and Text 0.77.2 are
published. `ic-cargo-qualification` remains private. [Release receipts](../release-ownership.md)
retain exact sources, archive checksums and acceptance. The release and its
subsequent documentation receipt are delivered directly on `main`.

Toko 0.44.11 is installed through its verified 49-file exporter snapshot from
`08d63df7661910554ef72980037f55edd914aba0`; both workspace markers/locks agree.

The CLI reuses the managed-server/Process/FS owners and removes the private runner.
Scheduling and archive scripts require explicit prepared tools/directories;
no sibling tool lookup remains. Setup keeps bounded diagnostics and original
failure status. Run preserves piped stdin and child stdout/stderr, with bounded
capture, interruption handling and cleanup. The archive gate installs the actual
packaged CLI; CLI dependencies remain optional. Library APIs are unchanged.
The workload register now separates current adoption from historical captures.
DB already owns direct-client/Text adoption, so its obsolete Tooling migration
patches/runners are retired. [Tooling #10](https://github.com/dragginzgame/ic-tooling/issues/10)
owns this supported setup/check/run successor. DB #3 is closed as a
misclassified consumer mirror; its source/acceptance evidence is retained there.
The CLI is published and installed through Toko; offline tool/server admission,
piped IO, original child failure and endpoint closure pass. Tooling #10 retains
native-host acceptance under asynchronous CI. Sibling sources are unchanged,
and DB adoption remains with its owner.

Focused Linux evidence from the installed 0.107.4 archive passes setup/admission,
byte-preserving IO, success/nonzero/timeout/signal/spawn cleanup, secondary report
failure, both held-reply tests and both managed-server installed tests. Current
Scheduling pressure/held-upgrade checks pass through the product command. See
[archive evidence](../simulator-ownership.md#local-candidate-evidence--2026-11-16)
and the [Scheduling capture](../../qualification/scheduling/README.md#current-runner-acceptance--2026-11-16).

## Current acceptance and consumer boundaries

[Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2) is complete:
Timers 0.46.0 and Jobs 0.39.0 were published with Runtime/guard 0.23.14 and Testkit
0.107.2. Its owner records full installed lifecycle/watchdog/Jobs acceptance on
Linux/PocketIC 16.0.0. This supersedes the historical Timers build blocker in
[Tooling #5's record](../qualification/testkit-types-compatibility.md).
Current local Scheduling now selects Testkit 0.107.3; Tooling's adapter admits that
manifest and preserves rewrite ancestry/source-drift checks. Earlier Tooling
PocketIC 16.1.0 comparisons remain bound to their recorded captures.

[Canic #15](https://github.com/dragginzgame/canic/issues/15) records completed local
Testkit 0.107.3 adoption, removal of legacy lifecycle imports, real gateways and
application/failure checks. That closure records local work, not delivery by this
repository. Canic already owns report-directory publication through new FS;
[old qualification patches](../../qualification/canic/README.md#current-source-review--2026-11-16)
correctly refuse changed owners. Canic's broader Platform work remains separate.
[WasmStore #3](https://github.com/dragginzgame/ic-wasm-store/issues/3) is also closed.
Tooling #5 and Tooling #7 are closed; supplier fixes are published.
[Tooling #8](https://github.com/dragginzgame/ic-tooling/issues/8) remains the separate
release metadata editor requirement. [Tooling #9](https://github.com/dragginzgame/ic-tooling/issues/9)
is closed after publication and registry-only 0.107.4/Protocol 0.2.7 resolution.

Delivered [CI run 43954042484](https://github.com/dragginzgame/ic-tooling/actions/runs/43954042484)
passed the Build stack on all three native hosts and the library lane on macOS
Intel. Linux library checks failed because an extracted optional registry Host
dependency was not fetched before offline tests; the delivered gate fixes that setup
omission. macOS ARM setup received HTTP 403 from the asset download. Its diagnostics
worked; no upstream service repair or ARM library acceptance is claimed. These
results qualify that earlier source only, not the new CLI/Protocol release.

## Owners and remaining requirements

| Owner | Contract |
| --- | --- |
| Build | [Owned builds](../build-owned-artifact.md) |
| Artifacts | [Producer commitments](../byte-commitments.md) |
| FS | [Preparation](../directory-preparation.md), [publication](../directory-publication.md), [locks](../sidecar-locks.md), [custody](../filesystem-custody.md) |
| HTTP / Text | [Bounded bodies](../http-body.md), [presentation](../text-presentation.md) |
| Process | [Execution](../process-execution.md), [handoff/reaping](../process-handoff.md) |
| Testkit | [Held replies](../testkit-held-replies.md), [managed server](../testkit-server.md), [server command](../simulator-ownership.md) |

Build does not own archive running or release editing. HTTP and simulator graphs
stay outside Build/portable bytes. [Workloads](../workloads.md) distinguish accepted
cores from migration and unresolved requirements. Snapshot sets need a concrete
unfrozen caller and partial-effect contract; no current inspected caller justifies
restoring the old cache/pool framework. The maintainer confirmed Profiler is retired
on 2026-11-16; its observation owner/native acceptance are no longer outstanding.

[Backup](../../qualification/backup/README.md) retains assembly passes and earlier
restore baseline failures; next adoption is user-owned. [Query](../../qualification/query/README.md),
[DB](../../qualification/db/README.md) and [Platform](../../qualification/platform/README.md)
retain scoped historical evidence. Full consumer archive/deployment and new native
host acceptance remain separate delivery work. Async deletion is best-effort;
creation cancellation can leave remote effects. Custody assumes cooperative
writers/groups and exclusive child reaping. No performance improvement is claimed.
