# Current implementation — 2026-11-17

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

Foundation 0.80.3, Process 0.80.4 and Testkit 0.108.0 are published from
`78f1d2fbfbda6cac3b39f49dfe8aaa3c26472135`, with all three annotated release
tags pushed on `main`. Registry checksums/downloads match verified upload bytes.
The release ships directory modules, compatible libproc/WAT/wasmparser ranges,
and Protocol 0.3's public encoder-error type identity. Protocol's compatible
minimum is 0.3.2; release resolution selects 0.3.3. Types remains 0.21.2 and
PocketIC retains its [qualified server pairing](../testkit-server.md#pocketic-client-requirement).
Linux archive, installed Testkit/CLI and Scheduling driver checks pass. Native
CI is asynchronous. CI diagnostics, CDPATH and documentation fixes are also
delivered; the private Tools ledger remains pending. See the
[release receipt](../release-ownership.md#foundation-0803-process-0804-and-testkit-01080-delivery).

Process 0.80.3 and Cargo metadata 0.1.0 are published from
`de86fe21dc746eda79e662008f0d670f20970ac7`, with annotated
`process-v0.80.3` and `cargo-metadata-v0.1.0` tags pushed on `main`.
Non-yanked registry archives match the verified uploads. Process adopts macOS-only
libproc 0.15.0; both macOS targets pass cross-compilation/lint, and the Linux
packaged Build-stack unit/terminal gate passes. Native runtime CI is asynchronous.
The registry-installed metadata editor passes Toko coordinated/multi-lock and
Scheduling Timers/Jobs transactions. Tooling #8 is closed. See the
[release receipt](../release-ownership.md#process-0803-and-cargo-metadata-010-delivery).

Testkit 0.107.4 is published from `67ac61f89a7311e278027bee3078de7e24a0a241`, tag
`testkit-v0.107.4`, with byte-identical non-yanked registry readback. It supplies
the optional [server CLI](../simulator-ownership.md), managed server, compatible
Protocol 0.2.7 and Types 0.21.2 requirements. Original PocketIC types and separate
default/held-reply/managed-server/CLI feature boundaries remain.
Artifacts/FS 0.80.2, Build 0.87.3, HTTP 0.80.0 and Text 0.77.2 are
published. `ic-cargo-qualification` remains private. [Release receipts](../release-ownership.md)
retain exact sources, archive checksums and acceptance. The release and its
subsequent documentation receipt are delivered directly on `main`.

Toko 0.44.11 is installed through its verified 49-file exporter snapshot from
`08d63df7661910554ef72980037f55edd914aba0`; both workspace markers/locks agree.

The delivered module-layout cleanup moves six Rust files into their existing directories
as `mod.rs`, preserving source bytes and module paths. Canic qualification
generators use the same layout and ordinary child discovery; historical consumer
acceptance is not renewed. Focused Clippy/formatting checks and 28 module tests
pass; logs are under `target/module-layout-review/`. The later release gate also
passes the installed Testkit checks, as recorded in the release receipt.

Qualification directory changes now ignore inherited `CDPATH`, removing the
caller's obligation to unset it. All 108 relative runner-startup cases and the
relative-source capture/drift checks pass; see the
[source-capture checks](../qualification/local-sources.md).

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
piped IO, original child failure and endpoint closure pass. Tooling #10 is closed
after native-host acceptance. Sibling sources are unchanged,
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
[Tooling #8](https://github.com/dragginzgame/ic-tooling/issues/8) is closed after
release metadata editor publication and installed acceptance. [Tooling #9](https://github.com/dragginzgame/ic-tooling/issues/9)
is closed after publication and registry-only 0.107.4/Protocol 0.2.7 resolution.

[CI run 43968510609](https://github.com/dragginzgame/ic-tooling/actions/runs/43968510609)
at `a00f697` passes library archives, CLI lifetime/failure cleanup and Build archives
on Linux, Apple Silicon macOS and Intel macOS. This completes Tooling #10's native
acceptance, including actual setup and the packaged command on all hosts.

Tooling #8's [metadata renderer](../cargo-metadata.md) is published as
`ic-cargo-metadata` 0.1.0, outside Build, with its own release group. It removes
the old Build CLI dependency while Toko retains transactions and delivery.
Focused tests and Linux archive admission pass. Current captured Toko coordinated
and multi-lock checks and Scheduling's real Timers/Jobs release transactions pass,
including rollback, preservation and downgrade refusal, with the registry-installed
command. Native CI is wired and remains asynchronous. The pre-existing Protocol
0.2.8 lock selection is retained in the delivered root lock; this release does not
claim new Testkit/Protocol qualification.

## Owners and remaining requirements

Local CI maintenance adds [bounded setup diagnostics](../qualification/ci-setup.md)
to the scaffold job. Offline failure/redaction/collection and upload-reporting
fixtures qualify the wiring; hosted upload/retrieval is not yet observed.

| Owner | Contract |
| --- | --- |
| Build | [Owned builds](../build-owned-artifact.md) |
| Cargo metadata command | [Release candidate rendering](../cargo-metadata.md) |
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
