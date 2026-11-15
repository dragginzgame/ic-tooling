# Required workloads and migration boundaries

This register selects behavior, not old exports. Historical source evidence is
in commit `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3` under
`docs/design/consumer-evidence.txt` and `inventory.tsv`. The reviewed production
baseline is `a00369fc29fa837c6deb022cceb7e09aa447292c` in `../ic-tooling`.
These are historical observations, not fresh consumer execution or an exhaustive
current fleet audit. Use each active consumer’s current local working tree for qualification;
[source snapshots](qualification/local-sources.md) preserve edits without modifying
the sibling. Historical commits do not establish current compatibility. Auth is on hold; Storage,
Observability and Wire are frozen by maintainer direction. Their obligations stay
recorded, but no migration or design expansion is active for those repositories.

| Consumer/workflow | Required outcomes and refusals | Proposed destination | Status / gate |
| --- | --- | --- | --- |
| Scheduling rewrite | Shared client/type identity and bounded held-reply fixtures for Once/Jobs pressure, watchdog and upgrade refusal | Requirements-driven Testkit; caller-owned policy | Active source `../ic-scheduling-new`; [current requirements](../qualification/scheduling/README.md). No demonstrated old build-driver or cached-snapshot requirement; old experiments are historical |
| Backup payload/local publication | Exact persisted bytes/hash, bounded reads/HTTP, original I/O failures, no foreign cleanup, distinguish visible-but-not-durable | Bytes + filesystem custody; narrow HTTP adapter | Current local filesystem, byte streams and HTTP body adapter qualified; legacy digest values/in-memory hashing remain; see [contract](filesystem-custody.md) |
| Canic host/artifact/CLI | Correct build and publication, destination authority, locks and required terminal output | Build/custody plus explicit presentation adapter | [Local record admission](../qualification/canic/README.md) uses byte/filesystem owners; [current CLI Text](../qualification/canic/README.md#current-cli-text) also qualifies; [owned Build](../qualification/canic/README.md#current-owned-build) qualifies artifact/app recipes, including serial partial failure; parent creation/publication still use old Host FS |
| Auth / Storage packaging | Exact package/archive and external-lock acceptance, usable released artifacts | Development-plane qualification executable | [Auth external-lock/package path qualified](../qualification/auth/README.md); Auth is now on hold; Storage archive migration is frozen; released metadata editor remains unmigrated |
| DB simulation/snapshots | Explicit server/instance lifetime, caller identity, stable-memory oracle, partial snapshot effects | Simulator session | [Current DB rollback](../qualification/db/README.md) passes with direct upstream async client, explicit endpoint and observed deletion/server survival; [Jobs controller recovery](../qualification/scheduling/README.md) also passes; managed server and multi-canister partial snapshot effects remain pending |
| Platform / Query | Supported simulator/Wire behavior and upstream type identity | Simulator / narrow Wire adapter if needed | [Platform raw_rand and Agent](../qualification/platform/README.md) use direct upstream PocketIC; remaining Platform drivers are open. [Current Query Text](../qualification/query/README.md) and [DB console Text](../qualification/db/README.md#current-console-text) pass without expanding the API |
| Profiler observations | Required process measurements on Linux and native macOS; no borrowed-PID kill authority | Profiler-owned observation or independently justified leaf | Stage 5; `../ic-profiler` is absent, current checkout/status requested; historical imports do not admit a replacement |
| Current native-tool execution | Bounded capture/deadlines with owned-group cleanup, plus direct-child foreground terminal communication | Process 0.80.1 shared with Build; caller keeps signing/decoding/retry policy | [Current caller review](process-execution.md); combined current Canic Build/lock/signer/metadata/foreground qualification passes |
| Desktop viewer opener | Return while the opener may still run; literal URL argument, original spawn refusal, no forced teardown | Caller chooses launch policy; handoff/reaping ownership still to be designed | Current retained caller and loopback viewer pass fixture qualification; synchronous Process `run` is not a substitute |
| HTTP / Text / Wire leaves | Bounded complete body, escaping/Unicode/table output, encode-call-decode once and committed mutation after decode refusal | Individually admitted adapters | HTTP, escaping, bordered tables and Canic plain tables admitted; other Text and Wire workloads remain open |
| Product release | Exact candidate archives, source/tool/host evidence, stage failures and partial publication recovery | Selected Toko runner plus product recipes | Stage 6; no new generic release engine |

Old caches, forwarding aliases, hidden dispatch, generic public test support and
undemonstrated wrappers are excluded from the new core. This does not declare an
active consumer retired. No consumer has migrated; all retain the old release.

Physical crates and release groups are not inherited. The first implementation
package owns only build-to-artifact. Stage 2 extracts byte admission and filesystem
custody for demonstrated independent Backup use; Process now owns shared execution. Full portable
byte and simulator consumers must not acquire the build operation's native graph.
