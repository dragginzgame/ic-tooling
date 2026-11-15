# Required workloads and migration boundaries

This register selects behavior, not old exports. Historical source evidence is
in commit `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3` under
`docs/design/consumer-evidence.txt` and `inventory.tsv`. The reviewed production
baseline is `a00369fc29fa837c6deb022cceb7e09aa447292c` in `../ic-tooling`.
These are historical observations, not fresh consumer execution or an exhaustive
current fleet audit. Refresh each consumer's exact committed source before its
migration; never export unrelated dirty work.

| Consumer/workflow | Required outcomes and refusals | Proposed destination | Status / gate |
| --- | --- | --- | --- |
| Scheduling builds | Exact selected Wasm, complete process/protocol evidence, bounded diagnostics, stable captured bytes, cancellation/cleanup | Build operation | First slice specifies the core; installed Scheduling and required build-script facts remain Stage 3 |
| Backup payload/local publication | Exact persisted bytes/hash, bounded reads/HTTP, original I/O failures, no foreign cleanup, distinguish visible-but-not-durable | Bytes + filesystem custody; narrow HTTP adapter | Stage 2 filesystem qualified against pinned committed source; legacy digest/HTTP suppliers remain; see [contract](filesystem-custody.md) |
| Canic host/artifact/CLI | Correct build and publication, destination authority, locks and required terminal output | Build/custody plus explicit presentation adapter | Stage 3; no old path-helper family admitted automatically |
| Auth / Storage packaging | Exact package/archive and external-lock acceptance, usable released artifacts | Development-plane qualification boundary | Stage 3; verify actual selected Toko interface and displaced editor destination |
| DB simulation/snapshots | Explicit server/instance lifetime, caller identity, stable-memory oracle, partial snapshot effects | Simulator session | Stage 4; no pool/baseline wrapper by default |
| Platform / Query | Supported simulator/Wire behavior and upstream type identity | Simulator / narrow Wire adapter if needed | Evidence gap; required workload must be resolved before these consumers migrate |
| Profiler observations | Required process measurements on Linux and native macOS; no borrowed-PID kill authority | Profiler-owned observation or independently justified leaf | Stage 5; destination unresolved, old supplier stays active |
| HTTP / Text / Wire leaves | Bounded complete body, escaping/Unicode/table output, encode-call-decode once and committed mutation after decode refusal | Individually admitted adapters | Add when consuming slice requires them, not as a final-stage prerequisite bottleneck |
| Product release | Exact candidate archives, source/tool/host evidence, stage failures and partial publication recovery | Selected Toko runner plus product recipes | Stage 6; no new generic release engine |

Old caches, forwarding aliases, hidden dispatch, generic public test support and
undemonstrated wrappers are excluded from the new core. This does not declare an
active consumer retired. No consumer has migrated; all retain the old release.

Physical crates and release groups are not inherited. The first implementation
package owns only build-to-artifact. Stage 2 extracts byte admission and filesystem
custody for demonstrated independent Backup use; process remains private. Full portable
byte and simulator consumers must not acquire the build operation's native graph.
