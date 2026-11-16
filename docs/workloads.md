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
| Scheduling rewrite | Shared client/type identity and bounded held-reply fixtures for Once/Jobs pressure, watchdog and upgrade refusal | Requirements-driven Testkit; caller-owned policy | Active rewrite source `../ic-scheduling` (formerly `ic-scheduling-new`); [current requirements](../qualification/scheduling/README.md). No demonstrated old build-driver or cached-snapshot requirement; old experiments are historical |
| Backup payload/local publication | Exact persisted bytes/hash, bounded reads/HTTP, original I/O failures, no foreign cleanup, distinguish visible-but-not-durable | Bytes + filesystem custody; narrow HTTP adapter | Current local filesystem, byte streams and HTTP body adapter qualified; legacy digest values/in-memory hashing remain; see [contract](filesystem-custody.md) |
| Canic host/artifact/CLI | Correct build and publication, destination authority, locks and required terminal output | Build/custody plus explicit presentation adapter | [Local record admission](../qualification/canic/README.md) uses byte/filesystem owners; [current CLI Text](../qualification/canic/README.md#current-cli-text) also qualifies; [owned Build](../qualification/canic/README.md#current-owned-build) qualifies artifact/app recipes, including serial partial failure; Canic now owns held report-directory publication; earlier adapters are historical |
| Auth / Storage packaging | Exact package/archive and external-lock acceptance, usable released artifacts | Development-plane qualification executable | [Auth external-lock/package path qualified](../qualification/auth/README.md); Auth is now on hold; Storage archive migration is frozen; released metadata editor remains unmigrated |
| DB simulation/snapshots | Explicit server/instance lifetime, caller identity, stable-memory oracle, partial snapshot effects | Simulator session | [Current DB rollback](../qualification/db/README.md) passes with direct upstream async client, explicit endpoint and observed deletion/server survival; [Jobs controller recovery](../qualification/scheduling/README.md) also passes; managed server is published; multi-canister snapshot sets require a demonstrated current caller |
| Platform / Query | Supported simulator/Wire behavior and upstream type identity | Simulator / narrow Wire adapter if needed | [Platform raw_rand and Agent](../qualification/platform/README.md) use direct upstream PocketIC; remaining Platform drivers are open. [Current Query Text](../qualification/query/README.md) and [DB console Text](../qualification/db/README.md#current-console-text) pass without expanding the API |
| Profiler observations | Retired by maintainer confirmation on 2026-11-16 | No replacement admitted | Removes this consumer's Stage 5 observation/native-acceptance obligation; historical evidence is retained |
| Current native-tool execution | Bounded capture/deadlines with owned-group cleanup, plus direct-child foreground terminal communication | Process 0.80.2 shared with Build; caller keeps signing/decoding/retry policy | [Current caller review](process-execution.md); combined current Canic Build/lock/signer/metadata/Store-source/foreground qualification passes |
| Desktop viewer opener | Return while the opener may still run; literal URL argument, original spawn refusal, eventual reaping without forced teardown | Process `launch` keeps worker custody independently of caller completion observation | Current Canic 0.214.0 replacement and loopback viewer pass; Linux reaping after observer drop is observed; native macOS remains open |
| HTTP / Text / Wire leaves | Bounded complete body, escaping/Unicode/table output, encode-call-decode once and committed mutation after decode refusal | Individually admitted adapters | HTTP, escaping, bordered tables and Canic plain tables admitted; other Text and Wire workloads remain open |
| Product release | Exact candidate archives, source/tool/host evidence, stage failures and partial publication recovery | Selected Toko runner plus product recipes | Stage 6; no new generic release engine |

Old caches, forwarding aliases, hidden dispatch, generic public test support and
undemonstrated wrappers are excluded from the new core. This does not declare an
active consumer retired. Scheduling has published its replacement-based migration;
Canic records local managed-server adoption in [Canic #15](https://github.com/dragginzgame/canic/issues/15).
Other migration claims remain scoped to the owning records, not inferred fleet completion.

Physical crates and release groups are not inherited. The first implementation
package owns only build-to-artifact. Stage 2 extracts byte admission and filesystem
custody for demonstrated independent Backup use; Process now owns shared execution. Full portable
byte and simulator consumers must not acquire the build operation's native graph.

## Legacy capability disposition

This is a capability-family map, not an export-count or LOC completion percentage.
Reviewed legacy source is clean at `a00369fc29fa837c6deb022cceb7e09aa447292c`.
The current-source inspection on 2026-11-15 included Canic, Backup, Query, DB and
Scheduling, including their working-tree edits. Canic HEAD was
`521b054d425ad152a84c5cecb8fc80d53b459ac6`; Backup HEAD was
`b7c2c6ae754a5ae2452e65b9be537b517614ebd2`. This inspection identifies work; it
is not a stable-source execution qualification or an exhaustive fleet audit.
Frozen consumers were not reopened. Absence from these callers does not prove
that an old export can be removed or its workload retired.

“Implemented” below covers the stated replacement contract only. “Displaced”
names the owner of required behavior; it does not imply production migration.
“Unresolved” keeps the old capability available until its requirement is decided.

| Legacy family | Disposition | Replacement or remaining obligation |
| --- | --- | --- |
| Cargo build invocation, messages, selection and captured artifact | Implemented core | Build owns execute/select/capture/cleanup; callers receive owned bytes, not a mutable Cargo output path. |
| Shared target reuse and build observations | Deferred | Private builds work; shared leases need a measured workload. Workspace/reporting observations are not admitted wholesale. |
| Native Cargo commands and broad flag/environment helpers | Unresolved / caller-owned selection | Existing prepared-tool admission covers the Wasm build slice, not every native command or general Cargo interface. |
| External lock admission | Displaced and implemented | `ic-cargo-qualification` owns the development executable; ordinary Build does not. |
| Package archives and source inventories | Frozen delivery workload | Auth is on hold and Storage is frozen. Exact archive/candidate acceptance still belongs in final release qualification. |
| Release metadata editor | Displaced, not implemented here | Development/release tooling, not Build's runtime API; determine required callers and Toko ownership before replacing it. |
| Bounded byte read/copy and content identity | Implemented core | Artifacts admits complete streams with exact/maximum length and original failure evidence. |
| Digest representation and incremental producer hashing | Partly implemented and qualified | Complete bounded `hash_with` passes independent tests and current Backup assembly comparison. Restore baseline fails before migration; digest representation and multi-step hash state remain unresolved. |
| Gzip, chunk digests, matching writers and generic artifact trees | Unresolved | No new generic family admitted. Backup's own assembled-payload hash is a concrete requirement, not authority to import the old tree framework. |
| Wasm inspection/validation/custom sections and WAT compilation | Partly displaced; broader surface unresolved | Testkit generates its required held-reply module; Build admits its selected artifact. Neither replaces the complete legacy Wasm utilities. |
| Batch-label validation and general buffer/budget wrappers | Unresolved / private where needed | Required limits exist within new operations; a reusable public utility needs a caller contract. |
| Held file/directory access, enumeration and identity | Implemented core | Filesystem provides the qualified Backup custody operations; hostile namespace writers remain outside the contract. |
| File staging, seal/commit, locks and visible-but-unsynced evidence | Implemented core | File publication preserves independent effects/durability. Held-parent sidecar open/admit/acquire passes four independent tests and a 39-case current Canic comparison with released-lock interoperability. |
| Parent creation and path-based publication | Implemented and qualified for artifact/config/record output | Held-parent publication passes artifact checks and a combined 37-case retained-read/publication comparison. Locks, explicit preparation, staging allocation and unrelated artifact/path helpers still use old FS. |
| Directory commit, retained trees, non-aliasing export and verified copy | Directory publication implemented and qualified; other capabilities unresolved | Held no-replace directory publication passes five independent cases and 16 current Canic starter/fleet comparisons. No generic tree traversal/recovery framework is admitted. |
| Scratch files, owned-directory guards and path helpers | Partly private/displaced | Build owns its scratch lifecycle; consumer-owned paths/recovery stay with consumers. Remaining standalone helpers need individual requirements. |
| Bounded group/direct-child execution | Implemented core | Process owns stream limits, cancellation/deadlines, signaling and cleanup evidence; callers own decoding and retry. |
| Background handoff and reaping | Implemented core | Worker keeps child custody after observer drop; no daemonization or readiness guarantee. |
| Process identity/sampling, clocks, affinity and descendant observations | Profiler workload retired | No replacement justified by this consumer. Execution/reaping and test-only `/proc` observations do not imply a public observation API. |
| Framing, signal/status helpers and launch gates | Partly private; remainder unresolved | Build/Process own necessary internal mechanisms. No standalone compatibility family is admitted without a current requirement. |
| Complete bounded HTTP response | Implemented adapter | HTTP owns body admission; consumers own Reqwest clients, TLS and request policy. |
| Terminal escaping and table rendering | Implemented adapter | Text covers escaped bordered/plain tables and width-aware alignment. |
| Line fitting/clipping and broader terminal policy | Unresolved | Not implied by table width handling; no completion claim for legacy `fit_line`. |
| PocketIC type identity and held-reply fixtures | Implemented core | Shared re-export, catalog/module generation and exclusive reply control; current Scheduling comparisons pass. |
| Prepared server provisioning and managed-server lifetime | Library published; product CLI pending | Testkit owns pinned offline preparation and bounded lifetime; its optional [setup/check/run command](simulator-ownership.md) replaces the private qualification runner in 0.107.4. |
| Instance attach/create/install/retry and time helpers | Partly displaced | Current consumers use upstream clients and caller-owned runtime/policy. Managed lifetime, abandonment and gateway acceptance exist in [Testkit](testkit-server.md); caller-specific creation/install/retry policy remains local. |
| Snapshots, standalone fixtures and cached baselines | Partial upstream qualification; broader scope unresolved | DB single-controller rollback evidence exists. Multi-canister partial effects remain open; a generic cache/pool is excluded by design. |
| Testkit Wire | Frozen | No replacement package or API admitted while Wire remains frozen. |
| Generic public test-support modules and compatibility forwarding | Excluded by design | New tests stay with their owner; no old test tree or compatibility scaffold is copied to increase apparent coverage. |

The last row and generic caches/pools are explicit architecture exclusions.
Apart from the explicitly retired Profiler workload, the other unimplemented
families are **not** declared unnecessary or retired.
Seven legacy package areas having new packages therefore says little about
whole-surface completion: Build, Artifacts, Filesystem and Testkit still have
material capability gaps beyond their implemented cores.

## Current decisions and retained evidence

The completed Canic parent/publication sequence is retained in the
[historical register](https://github.com/dragginzgame/ic-tooling/blob/10e7d64e3103af84c18e433be434e4527dee75d6/docs/workloads.md#next-implementation-slice-canic-parent-authority-and-publication)
and [consumer qualification](../qualification/canic/README.md). It is not the next
implementation slice. Current Canic owns report-directory publication; obsolete
patches must refuse changed source rather than reintroduce old helpers.

[Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2) records
completed owner-side installed acceptance and published Timers/Jobs. Its Linux
PocketIC 16.0.0 results do not relabel older Tooling 16.1.0 captures or establish
native macOS acceptance. Canic's local acceptance and delivery remain distinct.

Current DB rollback and Platform management-snapshot codec fixtures do not
demonstrate a shared multi-canister snapshot-set workflow. Keep that design
capability deferred until a current unfrozen owner defines its partial-effect
contract; do not build a cache or infer global workload retirement. The maintainer
explicitly confirmed Profiler's retirement on 2026-11-16, resolving its observation
owner and native-acceptance question without introducing an unused API.

Native host evidence is recorded in the [handoff](status/current.md); consumer
archive and deployment acceptance remain separate. Shared build caching stays
deferred until matched measurements demonstrate a workload.
