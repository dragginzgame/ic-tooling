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
| Canic host/artifact/CLI | Correct build and publication, destination authority, locks and required terminal output | Build/custody plus explicit presentation adapter | [Local record admission](../qualification/canic/README.md) uses byte/filesystem owners; [current CLI Text](../qualification/canic/README.md#current-cli-text) also qualifies; [owned Build](../qualification/canic/README.md#current-owned-build) qualifies artifact/app recipes, including serial partial failure; parent creation/publication still use old Host FS |
| Auth / Storage packaging | Exact package/archive and external-lock acceptance, usable released artifacts | Development-plane qualification executable | [Auth external-lock/package path qualified](../qualification/auth/README.md); Auth is now on hold; Storage archive migration is frozen; released metadata editor remains unmigrated |
| DB simulation/snapshots | Explicit server/instance lifetime, caller identity, stable-memory oracle, partial snapshot effects | Simulator session | [Current DB rollback](../qualification/db/README.md) passes with direct upstream async client, explicit endpoint and observed deletion/server survival; [Jobs controller recovery](../qualification/scheduling/README.md) also passes; managed server and multi-canister partial snapshot effects remain pending |
| Platform / Query | Supported simulator/Wire behavior and upstream type identity | Simulator / narrow Wire adapter if needed | [Platform raw_rand and Agent](../qualification/platform/README.md) use direct upstream PocketIC; remaining Platform drivers are open. [Current Query Text](../qualification/query/README.md) and [DB console Text](../qualification/db/README.md#current-console-text) pass without expanding the API |
| Profiler observations | Required process measurements on Linux and native macOS; no borrowed-PID kill authority | Profiler-owned observation or independently justified leaf | Stage 5; `../ic-profiler` is absent, current checkout/status requested; historical imports do not admit a replacement |
| Current native-tool execution | Bounded capture/deadlines with owned-group cleanup, plus direct-child foreground terminal communication | Process 0.80.2 shared with Build; caller keeps signing/decoding/retry policy | [Current caller review](process-execution.md); combined current Canic Build/lock/signer/metadata/Store-source/foreground qualification passes |
| Desktop viewer opener | Return while the opener may still run; literal URL argument, original spawn refusal, eventual reaping without forced teardown | Process `launch` keeps worker custody independently of caller completion observation | Current Canic 0.214.0 replacement and loopback viewer pass; Linux reaping after observer drop is observed; native macOS remains open |
| HTTP / Text / Wire leaves | Bounded complete body, escaping/Unicode/table output, encode-call-decode once and committed mutation after decode refusal | Individually admitted adapters | HTTP, escaping, bordered tables and Canic plain tables admitted; other Text and Wire workloads remain open |
| Product release | Exact candidate archives, source/tool/host evidence, stage failures and partial publication recovery | Selected Toko runner plus product recipes | Stage 6; no new generic release engine |

Old caches, forwarding aliases, hidden dispatch, generic public test support and
undemonstrated wrappers are excluded from the new core. This does not declare an
active consumer retired. No consumer has migrated; all retain the old release.

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
| Process identity/sampling, clocks, affinity and descendant observations | Unresolved | Profiler checkout remains absent. This is not covered by execution/reaping or test-only `/proc` observations. |
| Framing, signal/status helpers and launch gates | Partly private; remainder unresolved | Build/Process own necessary internal mechanisms. No standalone compatibility family is admitted without a current requirement. |
| Complete bounded HTTP response | Implemented adapter | HTTP owns body admission; consumers own Reqwest clients, TLS and request policy. |
| Terminal escaping and table rendering | Implemented adapter | Text covers escaped bordered/plain tables and width-aware alignment. |
| Line fitting/clipping and broader terminal policy | Unresolved | Not implied by table width handling; no completion claim for legacy `fit_line`. |
| PocketIC type identity and held-reply fixtures | Implemented core | Shared re-export, catalog/module generation and exclusive reply control; current Scheduling comparisons pass. |
| Prepared server provisioning/CLI and managed-server lifetime | Displaced operationally; replacement ownership open | Qualification still runs released launcher 0.106.0. External tool reuse is not a recoded setup/check/run CLI or managed-server implementation. |
| Instance attach/create/install/retry and time helpers | Partly displaced | Current consumers use upstream clients and caller-owned runtime/policy. Cancellation/partial effects and required managed ownership still need explicit acceptance. |
| Snapshots, standalone fixtures and cached baselines | Partial upstream qualification; broader scope unresolved | DB single-controller rollback evidence exists. Multi-canister partial effects remain open; a generic cache/pool is excluded by design. |
| Testkit Wire | Frozen | No replacement package or API admitted while Wire remains frozen. |
| Generic public test-support modules and compatibility forwarding | Excluded by design | New tests stay with their owner; no old test tree or compatibility scaffold is copied to increase apparent coverage. |

The last row and generic caches/pools are explicit architecture exclusions.
The other unimplemented families are **not** declared unnecessary or retired.
Seven legacy package areas having new packages therefore says little about
whole-surface completion: Build, Artifacts, Filesystem and Testkit still have
material capability gaps beyond their implemented cores.

## Next implementation slice: Canic parent authority and publication

The complete producer operation is implemented; see [its contract](byte-commitments.md).
Four current Backup assembly tests pass with both suppliers and Clippy. Restore
remains blocked by its earlier baseline failures; no full Artifacts removal is claimed.

[Directory preparation](directory-preparation.md) now implements explicit child
creation relative to held custody. Four independent tests pass, including retained
prefixes after child/parent synchronization failures. It removes manual sequencing
and reopening a prepared path without importing consumer path policy.

Current Canic artifact, configuration and retained-record output pass held-root
preparation composed with file stage/write/seal/commit. Nineteen writer/recovery
comparisons preserve create-only evidence, original permissions and signing
authority. A combined 37-case comparison now also qualifies retained reads with
all these writers. Explicit signed-workflow parent preparation remains selected
from the released supplier. [Directory publication](directory-publication.md)
passes independent tests and 16 current starter/fleet comparisons, preserving
retained staging and visible-but-unconfirmed outcomes. Next review remaining
staging allocation, artifact/path helpers and explicit preparation by required caller;
the completed slices do not establish full filesystem dependency removal.
Keep Canic serialization, staging names and retry/recovery decisions with its owner.
The [sidecar operation](sidecar-locks.md) now owns open/create/admit/acquire with
four independent tests and 39 composed Canic cases. Store/configuration locks
interoperate with the released helper; remaining preparation/staging/path calls
still retain their suppliers. Store/plan preparation now passes 41 composed cases:
Store locks through the prepared directory and plan publication uses held custody
for both writes. The composed recipe now also retains starter/fleet staging
handles from allocation through publication, with two independent collision and
identity tests. Naming and the collision budget stay in the private consumer
adapter; no new Tooling API is added. Starter member preparation and publication
now use that held root and returned leaves, passing the same composed comparison.
Fleet staging now similarly retains prepared root/child handles through ordered
member writes; the 41 paired cases and two custody tests pass. Recovery/release
writers now also retain prepared custody, with six publication cases expanding
the comparison to 47. Live submission/terminal-result handling remains outside
runtime evidence. Artifact inspection and app-output path resolution remain
separate migrations; released test helpers are retained as independent oracles.

Continue server/snapshot
ownership only from an unfrozen consumer's required lifecycle. Native macOS and
candidate-archive qualification remain independent release gates throughout.
