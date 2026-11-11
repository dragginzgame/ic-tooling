# Crate ownership and extraction contract

## Coordinated tooling workspace

The Host primitives retain the existing boundaries below, plus the explicitly
approved optional HTTP boundary described here. `ic-build` now shares
this repository and owns generic Cargo execution, build input evidence and
retained artifact caches. `ic-testkit` owns simulator integration. The unused
native allocator package is retired; it was not folded into Testkit.
IC Storage owns backing-memory IO metering in its test-support API. `ic-testkit-dev` and
`ic-testkit-perf-probe` are private qualification packages. Build and Testkit
depend on Host; Host does not depend on either. The private Testkit driver
may also depend on Build. The product workspace contains independently versioned
Foundation and integration [release groups](design/release-groups.md); private
qualification packages have an independent workspace and lockfile. These
groups preserve the capability boundaries and optional consumer features.

The maintainer approved `ic-host-http` under
[#117](https://github.com/dragginzgame/ic-tooling/issues/117). It accepts an
already-created native reqwest response and owns bounded body accumulation and
typed partial-failure evidence. It reuses Artifacts' bounded allocation primitive.
It never creates requests or selects status, TLS, compression, redirect, retry,
timeout, decoding, redaction or retention policy. Query and Blob retain their distinct status and
failure-evidence contracts; consumer adoption is separate from this implementation.

## Direct Types, Runtime and external IC owners

Testkit selects published `ic-platform-types` for its existing external Principal
identity. The portable library excludes Runtime, the facade, executor, macros
and Candid. The private performance probe selects
Runtime's CDK entrypoints and one guarded executor directly. Native simulator
operations use published upstream `pocket-ic` directly under an explicit native
dependency-boundary exception. Its native-only `pocket_ic` re-export exposes
that exact dependency so consumers need no independent client selection.
Testkit's `pic` owns harness policy and server
custody; it owns no client fork. Held fixtures select Platform's Candid and
management adapters. The [client migration](testkit/client-migration.md) describes
upstream identities, panics, best-effort deletion and the removed transfer engine.

### Public supplier identities

Tooling owns configuration and recovery classifications for its adapters.
`ic_host_tools::candid::decode::DecodeLimits` expresses caller policy and is
converted privately to Platform's limits. Its decode errors retain the original
typed codec cause. Configuration does not require a direct Platform dependency;
supplier-error downcasts retain explicit supplier-version coupling. Upstream
PocketIC owns its separate native protocol codecs and DTOs.

The public-surface inventory for
[#340](https://github.com/dragginzgame/ic-tooling/issues/340) retains these
deliberate interoperability boundaries:

| Surface | Canonical owner / release group | Why identity is retained |
| --- | --- | --- |
| Application Principal | `ic-platform-types` / Types | Shared application identity, independent of facade releases. |
| Simulator Principal and management DTOs | Upstream `pocket-ic` | Testkit uses the upstream async client directly; convert explicitly to application Principal identity at consumer boundaries. |
| Simulator subnet metrics | Upstream `pocket-ic` | The dependency owns read-state transport and decoding; Testkit does not maintain another codec. |
| Candid trait bounds, `Nat` and decoder causes | Platform's owned Candid codec | Typed external Candid exchange retains explicit work/allocation limits; registry Candid is not the direct codec owner. |
| Agent builder, agent and SDK errors | Upstream `ic-agent`, exposed through Platform | Callers deliberately configure and use the SDK; no competing agent wrapper or lifecycle owner. |
| ICRC ledger `Value` in `LedgerBlock` and chain results | Platform / Platform | Standard external ledger schema and hashing have one owner. This optional adapter intentionally shares its selected Platform generation; unrelated error/configuration adapters do not. |
| Wasm facts consumed by `interface::exported_methods` | `ic-host-artifacts` / Foundation | Borrowed, already-inspected facts preserve one parser, export model and admission owner. |
| Project YAML failure | `ic-host-text` / Text | Typed cause of the delegated YAML interpreter; no second parser contract. |
| Wire call limits and codec failures | `ic-wire` / Core | The caller explicitly selects the owned RPC codec and its admission/error contract. |

`LedgerTip` remains Tooling's passive index/hash anchor for chain verification,
owned by Host Tools' Tools group. Constructing it proves no authentication.
Query owns certificate verification and its authenticated-result API; its
[boundary migration](https://github.com/dragginzgame/ic-query/issues/118) can
project to this existing anchor when invoking Tooling. Do not create a second
passive `LedgerTip` owner or move certificate codecs into either consumer.
Shared DTO/SDK generation upgrades still need joint identity checks; they are
distinct from an unrelated facade release changing adapter error/configuration
identities. Foundation/Text public-value upgrades likewise retain their explicit
group contracts rather than acquiring duplicate models here.

Host Tools selects Platform for binary Candid decoding; textual service inspection is retired.
Platform owns certificate/hash-tree CBOR and typed IC packet codecs; Tools retains
service-to-Wasm projections and caller-owned aggregate-byte accounting.
Testkit delegates native simulator transport and protocol codecs to upstream
PocketIC. Owned application protocols use IC Wire.

<a id="fleet-candidcdk-boundary"></a>

The [historical boundary evidence](extraction-history.md) records the original
Candid/CDK consolidation. It does not describe today's split package graph.

## Profiler native mechanics

The authorized Host extraction from Profiler commit
`70907ffa3b1a61c89929b02ff8d8e40509de9f8e` owns raw procfs/libproc/Mach
observations and Unix workload gate/custody mechanics in `ic-host-process`.
Profiler retains its serialized capture DTOs, pinned-process admission, selected
roots, cadence, aggregation and diagnostics. Platform retains native IC adapters
and matching build inputs. This introduces no upward Host dependency. Native
source provenance and safety comments are retained; there is no cryptographic
implementation transfer.

## Current implementation

The [source inventory](../ci/extraction-sources.json) retains original extraction identities.

Dependencies point down from ic-host-process to ic-host-fs and from each to
ic-host-artifacts. Tools selects Artifacts only for explicit Wasm profiles;
its production graph has no filesystem or process dependency. No generic crate
depends on Tools, Canic, IcyDB or an IC runtime. Test/example artifact features
do not become production filesystem dependencies. Tools' default response
interpretation adds no generic Host dependencies; `icp-project` selects Text.
Native Candid extraction belongs to IC Build's `artifacts::candid`; generic line
normalization belongs to `ic_host_text::normalize`.
Shared method identities remain unconditional. Bounded reply decoding selects
`candid` through Platform's owned codec, without registry Candid or its parser. `ic-interface` adds generic Wasm facts and export projection without
Platform, Candid or filesystem/process dependencies.
Replica revision selection, install thresholds and resource-report policy belong
to the consuming application. Host Artifacts retains generic Wasm facts; Tools
no longer owns an `ic-limits` profile or a reference-policy CLI example.

- ic-host-artifacts owns pure ordered nonempty/unique batch-label admission,
  declared-file directory checksums, stream identities,
  bounded read/write/copy mechanics,
  including allocation-free visitation with caller-owned scratch and callbacks,
  optional gzip encoding/decoding, optional Wasm
  structural facts, encoded code/data statistics, bounded custom-section framing,
  full core-module validation, optional core WebAssembly Text compilation.
- ic-host-fs owns no-follow directory hashing and ordered sync/checksum traversal
  over caller-held descriptors,
  held directory capabilities with private cap-* traversal and bounded enumeration,
  fresh child-directory allocation with explicit permissions and collision bounds,
  standard cache-directory tag recognition and durable replacement with explicit
  suffix bytes and permissions,
  pathname/descriptor reads, explicit invocation-directory
  cleanup custody and the durable module's replace,
  create-new, private-file, parent synchronization and descriptor-lock operations.
  `durable::SharedFileLock` explicitly unlocks on guard drop before closing;
  consumers retain shared guard lifetimes, paths and cache/maintenance policy.
  Its `scratch::create_private_bytes` shares private, non-durable invocation-file
  creation; consumers retain byte budgets, successful cleanup and effect recovery.
  Its separate `scratch::TemporaryFile` owns scoped named-file cleanup through
  private tempfile allocation; retained files, unnamed files and directory custody
  are separate contracts. The [capability guide](capability-fs.md) identifies
  Query's remaining cache-specific adapter and discovery responsibilities.
  Read-only and ordered-sync visitors share checksum collection while retaining
  their released descriptor interfaces and distinct effects.
- ic-host-process owns executable resolution/admission, installed-version
  observation, explicit execution context,
  output limits, deadlines, direct-child/explicit group cleanup evidence, native
  process-table/RSS observations and separate Git queries.
  Its `framing` module owns bounded newline admission for buffered readers and
  incremental output chunks. Schemas, EOF-tail policy, deadlines, diagnostics and
  recovery remain with each process consumer.
  Its Unix `custody` module also owns inherited regular-sidecar descriptors and
  exact existing-sidecar quiescence. Callers retain paths, environment names,
  durable identity records, wait settings, command/effect admission and recovery;
  custody neither kills processes nor implies a remote effect outcome.
- ic-host-text owns bounded UTF-8 line normalization, pure terminal-safe escaping,
  and optional typed YAML decoding with input-byte and tag/merge admission.
  The private YAML backend is owned here; schemas, filesystem/includes and
  application configuration rules remain with consumers. Tools' `icp-project`
  selects this Text feature rather than another parser implementation. Its private
  maintained `yaml_serde` backend uses `libyaml-rs`; all Text profiles
  use the workspace's declared compiler minimum. Text also owns
  explicit column clipping, Unicode display-width tables and pure single-line
  progress frames. Its `timestamp` module owns canonical UTC second spelling and
  Gregorian conversion, formatting unsigned Unix seconds and parsing zero through
  `i64::MAX`. Callers retain clocks, freshness and collection-time interpretation.
  Its `decimal` module owns exact two-place rounding/rendering with explicit
  scaling; consumers retain normalization, units, sentinels and raw report fields.
  Its `quantity` module also selects caller-provided decimal power-of-ten units
  for arbitrary-precision decimal strings, with two-place half-up rounding and
  unit promotion. Its `percent` module renders exact `u64` ratios to one decimal
  place. Consumers retain unit names, unavailable-value fallback, checked total
  construction and report semantics; neither operation selects output or policy.
  Its `duration` module owns positive whole-second parsing, rounded single-unit
  display and truncating two-unit display. Clock selection, deadlines, freshness,
  persisted units and output remain caller-owned. Source and six tests are retained
  from Host PR #162 at `58e03fbb0b2c2e13816a1c34fb7da6965d68942e`.
  Progress callers retain terminal detection, wrapping limits, output, flushing,
  failure policy and cleanup timing; the helper never selects a sink or writes.
  Escaping borrows unchanged input through `Cow<str>` and owns changed text;
  tables keep safe cell borrows and return one owned rendered string.
  Cells, alignments and formatting options are explicit; reports and CLIs retain
  headers, rows, column choices, size/truncation policy, output, terminal detection,
  colour and progress lifecycle. It depends on no other Host or IC owner. This fifth generic
  boundary was explicitly selected by the maintainer under
  [#110 — Host text ownership](https://github.com/dragginzgame/ic-host-tooling/issues/110).
- ic-host-tools owns Candid service-method interpretation,
  reply decoding with explicit byte/work and caller-owned aggregate limits,
  IC method export projection over generic Wasm facts and JSON/plain-hex ICP responses.
  Its `icp` module owns pure call/status argv
  construction and successful status JSON's null/string installation projection.
  Bounded inventory and decimal-string cycle-balance projections also live here;
  sorting, name admission, manifest fallback and funding policy stay local
  ([#335](https://github.com/dragginzgame/ic-tooling/issues/335)).
  Executable/context admission, capture bounds, execution, targets, full status
  reports and lifecycle stay with callers. The implementation and four tests are
  taken from reviewed Host PR #101 at `38b114bb30f12e9a6e255ccd85d6b7344331d763`.
  CLI labels, preambles and any Unicode whitespace normalization remain with
  consumers rather than a second presentation adapter in the codec.

IC Build owns native Candid extraction orchestration over Host's hashing and
execution primitives. It retains complete bounded raw evidence and source/tool
identities without normalization or publication policy. Consumers needing text
call `ic_host_text::normalize::normalize_lines` explicitly with an output allowance;
UTF-8, normalization, grammar and expected-service admission remain separate decisions.
The [0.24 migration](changelog/0.24.md) records producer delivery and consumer actions.

The optional tools `registry` feature supplies focused Registry Protobuf
projections and canonical canister-range shard-key/interval admission. It selects
Types and Prost directly, without the Platform facade, Candid, Agent, certificate
parser, filesystem or process owner. Public Principal identity is unchanged.
Callers must enforce input/work budgets and own discovery, trusted versions,
certification, full range ordering, completeness, replay and report policy.
The projections preserve Query 0.57.0's tags/discriminants and are not a complete
upstream Registry schema; unknown fields retain normal Prost behavior.

The maintainer confirmed on 2026-11-10 that the five Registry projection files
`registry/proto/{id,node,registry,routing,subnet}.rs` and the Ledger projection
`icp_ledger/protobuf/mod.rs` under `crates/ic-host-tools/src/` were authored by
the team. Their upstream schema references document wire compatibility.

The optional native Tools `agent` feature shares response-bounded SDK construction
and lazy exact endpoint/ceiling binding using Platform's supported Agent API.
The caller retains endpoint admission, trust roots, identity, historical policy,
deadlines and every network call. Defaults exclude transport, and
the Wasm graph excludes it even with the feature selected. Process and Tokio
are dev-only fixtures; no lifecycle owner or automatic network activity is added.

The optional tools `encoding` feature decodes a complete canonical unsigned
`u64` LEB128 value without allocation or new dependencies. It rejects truncated,
overflowing, nonminimal and trailing encodings while inspecting at most ten bytes.
Callers retain field names, semantic limits, transport and trust policy.

Platform's `certification-cbor` feature owns strict CBOR decoding through
`ic_platform::certification::cbor::strict`, including its `DecodeLimits` and
typed errors. Host Tools no longer provides a `certificate` feature or module.
The codec returns certificate/delegation/hash-tree values from Platform.
These retain upstream `ic-certification` type identities. It admits complete
input bytes before decoding with the IC Agent's `serde_cbor` codec and its
built-in recursion limit, returning one fully consumed value. The codec has no depth
override or competing CBOR parser. Direct Serde visitors enforce actual array/byte kinds, exact tree tags
and arities and 32-byte pruned digests without a generic CBOR payload tree.
Certificate/delegation fields reject unknown, duplicate, missing or invalid
values. These stricter schema adapters remain necessary: upstream storage
deserialization also accepts text/integer-array byte projections and does not
detect every duplicate-null field. Errors distinguish byte allowance, trailing
data and CBOR/shape failures.
Parser allocations use ordinary upstream Vec/Box storage; a byte bound is not a
heap/CPU quota. No authentication is implied. Agent verification, network trust
roots, combined certificate/witness budgets, witness commitment and application
leaf interpretation remain consumer-owned. The original extraction is recorded
in [#109](https://github.com/dragginzgame/ic-host-tooling/issues/109); its codec and
grammar tests now belong to Platform. Consumers select that owner directly.

Platform's `ic-protocol` feature also owns typed read-state envelope encoding,
response decoding and subnet-metrics leaf decoding. Testkit retains the HTTP
exchange, endpoint/status handling, certificate tree lookup and optional-result
behavior. This move adds no verification, retry or replay policy.

Source tests move with their owner; public artifact projection tests belong to
ic-host-artifacts. Runnable examples retain explicit dependencies on each owner.
No generic compatibility reexports remain in ic-host-tools.

`wasm::encode_custom_section` encodes an isolated custom section from the existing
borrowed `CustomSection` view. Name and complete payload lengths must fit Wasm's
u32 domain; the complete encoded-byte allowance is checked before fallible
allocation. The optional Wasm feature reuses the existing locked `leb128` integer
writer; no filesystem/process or Candid dependency is added. Names, raw payloads,
metadata schemas, duplicate/append policy and aggregate module budgets stay with
callers. They assemble and validate the containing module. Independent malformed-
parser fixtures remain separate from encoder round-trip coverage.

Testkit owns its raw-reply fixture generator and uses the optional native
`wasm-encoder` dependency directly. One admission pass checks IC names, caller
budgets and reply extents before emitting the fixed imports, memory, functions
and data. Export-name extents are checked before emission, and complete encoded
output must fit u32. There is no generic call-module
DTO layer or instruction escape hatch. Artifacts retains structural inspection,
full validation, custom-section framing and WAT compilation.

`wasm-validation` adds `wasm::validate` and owned `ValidationError`, using the
private parser's full validator rather than structural inspection. It implies
`wasm`, leaving inspection-only graphs free of validation machinery. Consumers
retain target/proposal admission, execution and installation policy. Testkit can
replace its direct full-validation calls without weakening fixture coverage;
its adoption and dependency removal require the supplying Host release.

The independent `wasm-text` feature owns `wasm_text::compile`, using `wat` with
component support disabled. It admits UTF-8 source length before compilation,
then checks complete output length and core-module framing. These are byte
allowances, not CPU/heap quotas, and compilation is not full type/instruction
validation. Backend diagnostics are owned strings and may contain source excerpts.
Backup retains its stateful fixture sources and lifecycle; it can call this API
without a direct WAT dependency. Query's raw-reply fixture fits Testkit's existing
fixture builder, so it can eliminate its generated WAT instead. Neither route
requires a general instruction API or upstream encoder/parser re-export.

`read::open_regular_file_no_follow` exposes the existing Unix read-only opener's
regular descriptor admission without consuming bytes. Backup can sync its
reference descriptor and Canic can inspect a lock using that same standard handle.
Paths, permission admission, error mapping and locking decisions stay local.
Process's existing `signal::SignalActions` handles installation/restoration for
Testkit/Query-style exclusive registrations. Canic's cooperative signal-hook
callbacks and terminal restoration have distinct semantics; they are not replaced
by an exclusively owned disposition guard. No second signal abstraction is added.

Textual Candid service inspection is removed. `interface::exported_methods`
borrows function export facts without reparsing Wasm, preserving internal
methods, empty names and mode differences for caller admission. Method identities
remain available without Wasm or Candid support. Select `ic-interface` for export
projection.
Expected interfaces, CDK allowlists, type compatibility, payload annotations and
Canic endpoint presentation remain consumer policy. IC Build owns build/cache
orchestration separately; these libraries do not install tools or build canisters.

Within `ic-host-fs`, `read` is the sole owner of required/optional/private reads.
Its metadata-returning hash variants retain the initial metadata from the same
descriptor used for bounded hashing. Identity-only hash calls project that result;
Process consumes the paired observation for its executable permission policy,
after byte/digest admission. Metadata can become stale under concurrent writers;
neither paired observation nor hashing establishes atomic pathname execution.
Its Unix `tree` module owns no-follow descriptor traversal and bounded directory
hashing; `ic-host-artifacts::artifact::tree` owns exact checksum framing. Ordered
sync/checksum traversal belongs to FS `durable::tree`. Backup retains checksum
records, expected-identity admission, complete artifact publication policy and
retained byte custody. Callers admit and hold the root, select
depth/byte limits, and prevent concurrent namespace or content changes where
stable evidence is required.
Observed directories always use directory-only opens and descriptor type checks;
callers do not choose kernel flags. Native open errors may precede type callbacks
when an entry is replaced during traversal.
Following paths, final-component no-follow admission and already-open descriptor
reads remain distinct contracts. `read_file_prefix` permits a larger regular
file and retains length from the same admission observation, returning bounded
raw bytes. Consumers retain decoding and omitted-byte presentation; concurrent
writers can invalidate the observed length. Optional reads reuse the same bounded
stream engine and return `ArtifactError`; private reads add typed permission, link-count
and fixed-length rejection. `hash_file_no_follow` combines the same Unix opener
and descriptor admission with the artifact owner's constant-memory hash traversal;
metadata and streamed bytes have independent bounds. It rejects final symlinks,
while existing `hash_file` continues to follow them. Ancestor confinement,
concurrent-writer custody and exact expected-length checks remain caller-owned.
`open_entry_beneath` selects a regular file or directory through held root and
parent descriptors, refusing traversal, observed symlinks and special entries.
It does not guarantee immutable bytes or continuing pathname containment.
Durable publication and locks do not own a second
reader hierarchy. Lock errors retain typed admission and original I/O causes;
durable creation errors retain `NamedWriteError` in `RegularFileLockError::Publication`.
The durable directory owner sequences a caller-verified existing tree through
no-replace sibling publication and parent synchronization. It does not create,
clean, hash or traverse that tree. The verification callback owns complete
child/root synchronization and exact retained-byte comparison before the rename;
the consumer owns namespace exclusion, recovery meaning and retry authority.
Post-rename failures are explicitly distinct from before-publication failures.
`durable::write_retained_at_with` completes create/produce/sync for a caller-named
entry without unlink, rename or retry. Typed phase failures and panics preserve
the original entry, including partial evidence whose durability is unconfirmed.
`try_lock_exclusive` makes one nonblocking attempt on a caller-owned regular
descriptor; contention retains native `WouldBlock`. Blocking callers use
`lock_exclusive` on that descriptor. Observed waits compose the opener with
`lock_exclusive_with_wait` and caller-owned polling/reporting intervals. No filesystem latency
deadline, retry controller or stale-lock removal is introduced.
`open_regular_lock_file_with_parents` is that single admission owner, exposed
without acquisition so consumers choose shared, blocking, immediate or observed locks.
It preserves existing bytes and returns a close-on-exec read/write descriptor.
Consumers retain stable namespace, clone lifetime and explicit unlock policy;
opening does not establish root confinement or lock ownership.
Existing lock entries skip publication staging and synchronization; only missing
entries invoke durable creation. Final descriptor checks still enforce regular
no-follow admission, and namespace removal may return a native error without retry.
`open_private_regular_lock_file` reuses that descriptor admission for direct
owner-only sidecar creation in an existing parent, without staging or durability
syncs. It preserves existing bytes, inode and permissions. The caller owns lock
acquisition, namespace trust, descriptor inheritance and descendant custody;
adding this opener does not transfer command lifecycle or recovery to Host.
The [0.4 notes](changelog/0.4.md) describe the required consumer changes.

`durable::write_with(path, options, producer)` is the single generic pathname
writer. `write_bytes(path, options, bytes)` uses it with explicit publication mode
and permissions, returning `NamedWriteError<io::Error>` rather than discarding
publication state or cleanup evidence. Consumers retain error projection
and recovery decisions; an after-publication failure requires reconciliation.
`write_existing_parent_validated_with` reuses descriptor publication after
directory-only opening of an existing selected parent. It refuses missing or
special-file parents without creating them, follows parent aliases and selects
the directory once across renames. `write_at_validated_with` accepts the held
parent directly. Both require explicit result admission after file sync and
before publication; no-op admission is caller-selected. The forwarding variants
retire without aliases. Neither supplies root confinement or lifecycle policy.
The [0.10 migration](changelog/0.10.md) replaces the old two-argument writer and
removes the `write_typed_with` name without a compatibility alias.

`durable::write_named_with` exposes an absolute owned staging pathname to external
producers through the existing publication engine. Callers supply `WriteOptions`
for publication mode and staging permissions. Filesystem code owns exclusive
creation, descriptor lifetime, identity checks, cleanup and sync/rename outcomes.
The callback owns executable admission, process completion, bounded output
validation and any domain interpretation. Its original error and cleanup evidence
remain distinct, as does a final sync failure after publication. Parent hierarchy
control and exclusion of concurrent namespace mutation are caller prerequisites;
this is neither arbitrary-file adoption nor a child sandbox.

`durable::write_validated_with` adds a closed-writer admission phase to that same
engine. Read-only descriptor custody preserves inode identity while permitting
prepublication executable probes. Consumers retain tool versions, distribution
hashes, probe arguments/environment and bundle layout. All writable clones and
probe processes must finish before publication; read-only custody does not
prevent an external writer from changing bytes.

The first import preserves implemented guarantees. It does not establish root
confinement for following pathname APIs, immutable source bytes, process sandboxing,
descendant cleanup, hardware-independent crash durability or safe paid-effect retries.
Query's capability confinement and refresh leases remain consumer-owned.
Backup's checksum records and admission policy, multi-file publication and
journals remain consumer-owned; generic hashing and framing use the Host owners.
`ic-host-process` opt-in testing also supplies an acknowledged direct-child crash
barrier. It owns marker timing and direct-child reaping; consumers retain crash
point selection, fixture evidence and recovery assertions.
Further adoption must retain each caller's independently established guarantees.

`ic_host_process::tool::capture_command` reuses that same execution engine with a
caller-configured `std::process::Command`. The caller's executable/version policy,
arguments, cwd, environment and platform setup are retained; stdin becomes null
and stdout/stderr are bounded pipes. Unlike the exact-byte admission performed by
`AdmittedTool`, this entry performs no executable identity or version checks.
Ambient inheritance and PATH resolution follow the supplied command. This avoids
reconstructing commands and dropping consumer-owned setup. Neither entry owns
process groups, retries or paid-operation recovery. `ToolError::evidence` and
`execution_error` borrow the existing capture and failure objects for consumer
error projection; no parallel error model or automatic diagnostic output is added.
Completed invocation APIs return `CompletedExecution` with a required exit status,
including nonzero and signalled exits. `ExecutionEvidence` retains optional status
for failures. Strict callers use `require_success`; version, Git and Candid
admission apply it before interpreting output. A completion does not imply that
external effects succeeded or transfer a retained child. Nonzero/signalled leaders
always undergo the selected ownership's cleanup and reaping, even in retain mode.

`capture_group_command` opts the same capture loop into `OwnedChild` group
cleanup. It establishes a new group and signals its remaining members before
reaping on leader exit or failure. Original failures retain separate group,
direct-child fallback and reap errors. This adds no retries, inherited input,
cancellation controller, implicit handoff or paid-operation recovery. Escaped
processes remain outside its scope; signalling does not prove descendant exit.

`communicate_child` uses that same IO engine with an existing `OwnedChild`.
Public `OwnedChild::spawn_direct` exposes the existing direct-child constructor
used by ordinary capture. It preserves inherited or configured process groups
and cleans up only the direct child, allowing foreground callers to retain group
membership. Group-owning `spawn` requires an explicit cleanup policy. Terminal
control, signal handling and descendants remain consumer-owned; communication cannot force EOF
from descendant-held pipes under direct ownership.
Callers configure stdin/stdout/stderr before spawn, select borrowed input and
output budgets, and supply a prompt cancellation predicate. Piped IO is fair,
bounded and nonblocking; absent output pipes preserve inherited/file/null IO.
There are no IO worker threads, global signal handlers or retries. Errors retain
the canonical execution/cleanup evidence. Invalid deadlines leave the existing
owner and pipes untouched. Communication time excludes earlier spawning;
consumers retain end-to-end timing and input-size policy.
Capture, communication and executable admission use `OutputLimits`, with
`timeout: Some(duration)` or `None`. No deadline still enforces retention,
polls cancellation during communication and applies cleanup; it may wait indefinitely
for child exit or descendant-held pipe EOF. Cleanup timing is independent.

`communicate_child_with_observer` projects nonempty byte chunks from
that same engine. Per-stream ordering is retained; chunk boundaries and ordering
between streams are unspecified. Each stream chooses `OutputLimit::Terminate(n)`
for hard overflow (only admitted bytes reach the observer), or
`OutputLimit::Truncate(n)` to keep a bounded prefix and forward every drained byte.
Truncation sets the corresponding evidence flag; zero retention stores no output.
Version, Git and Candid parsing require complete evidence even if truncation was
selected. Other consumers can call `CompletedExecution::require_complete`, retaining
their own child-disposition responsibility when using retained ownership.
Observers return `io::Result<()>`; the first sink error becomes
`ExecutionFailure::Observer` and stops observation through the same cleanup path.
Retained bytes, observed truncation and secondary cleanup failures remain available.
Observer rejection takes precedence over hard overflow in the same chunk.
Callbacks run synchronously and must return
promptly. The cancellation callback can project caller-owned heartbeat events
even while output is silent. Callback unwinding closes pipes and attempts the
owner's cleanup before resuming the panic; an unreaped child remains owned for
explicit recovery. There is no separate reader loop, event schema or scheduler.

`SuccessfulExit::Cleanup` preserves ordinary group cleanup on leader exit.
`SuccessfulExit::Retain` keeps the successful leader reserved until the consumer
admits output and rechecks cancellation/deadlines, then explicitly chooses
wait/termination or handoff. Failure still cleans the group. No automatic
background ownership transfer occurs. Broken stdin pipes preserve command
status/diagnostics and do not establish that input was consumed. The new failure
categories and removed obsolete output-pipe categories are detailed in the
[0.7 notes](changelog/0.7.md).

`ic_host_process::child::OwnedChild` shares child spawn/reap/cleanup mechanics
with the direct-child capture engine. Its public spawn establishes a new group
while preserving caller IO and command settings. Ordinary waits reserve an exited
leader until group signalling finishes, then reap it. `poll_exit` can instead
observe status without signalling/reaping while the caller admits IO and checks
cancellation. Only explicit `handoff` of a still-owned, successful leader reaps
without group cleanup; the consumer then owns readiness, stop and recovery.
Running/failed/already-reaped leaders cannot be handed off. Observing success
alone leaves Drop/termination cleanup intact. No reusable PID/group authority
is returned after reaping. The caller drives polling,
natural waiting or cancellation; Drop provides best-effort cleanup, including
unwinding. Typed cleanup evidence remains separate from the original operation
failure. No application readiness, global signal handler, lifecycle controller,
retry, descendant-exit barrier or process-tree containment is introduced.
Exclusive reaping and an unchanged child group are required. Group spawning
requires `CleanupPolicy`: `KillAndWait` has no wall-clock bound and only reaps
the direct child; `TermThenKill` uses caller-owned grace and reap limits.
TERM leaves the leader reserved until group KILL, even if it exits during grace.
Repeated termination and Drop share the original reap allowance rather than
restarting it. Timeout retains the child for explicit caller recovery; dropping
it may leave an unreaped child until parent exit. No reaper thread is installed.
Explicit natural waiting and successful background handoff keep their contracts.
TERM, group KILL, direct-child fallback and reap errors remain separate from the
original execution failure. See the [0.8 contract](changelog/0.8.md).

The sole-zombie macOS check uses one private fixed-buffer libproc call, explicitly
approved by the maintainer. Explicit Unix `signal::SignalActions` additionally
permits scoped FFI for caller-selected action installation, rollback and
restoration. Its unsafe entrypoint requires async-signal-safe callbacks and
exclusive process-global registration custody; cancellation and shutdown policy
remain consumer-owned. Unsafe code stays denied elsewhere. Other process
syscalls use safe rustix APIs; libc is Unix-scoped without a bindgen toolchain.

`AdmittedTool::admit_version` shares the exact admission engine for tools whose
installation the caller already trusts. It takes a `VersionSpec` and a mandatory
exact version identity, records the bounded executable identity and admits exact
version output; later runs enforce that observed digest. It removes consumer-side
hash/admit composition without
claiming that observed bytes are a trusted published pin. Pinned admission takes
the same `VersionSpec` and exact version plus a mandatory digest, checked before
executing the version probe.
Installation provenance, per-host pins and selection of either contract remain
consumer policy; no host catalog or version-range resolver is introduced.

Additional reusable mechanics are extracted from committed sibling source:

- `artifact::hash_reader` captures a bounded stream identity once;
  `ArtifactIdentity::verify` admits that identity against caller-selected authority.
  The combined `verify_reader` wrapper is retired. Hash/limit failures precede
  digest comparison; a mismatch retains both authority and observed identity.
- `artifact::hash_gzip` reuses bounded hashing over the existing single-member
  gzip decoder. Callers select representation and admit input/payload bounds;
  the unused mixed raw/gzip auto-detection wrapper is retired.
  `gzip_matches` reuses `MatchingWriter`
  to compare exact decoded bytes without retaining another payload copy. Input
  admission, CRC/length and trailing-data checks have one owner shared with
  `decode_gzip`. Complete compressed/raw input is still a caller-owned slice;
  Wasm admission, representation selection and deployment policy stay local.
- `ic_host_artifacts::artifact::chunk_digests` computes ordered fixed-size chunk
  digests and the complete raw input identity through the existing bounded read
  traversal. Callers select a nonzero chunk size, input-byte allowance and maximum
  digest count; compression interpretation and upload policy stay local.
  `CopyError` can project into `io::Error`, preserving native input/output causes
  and retaining non-I/O copy failures as typed causes at publication boundaries.
- `ic_host_artifacts::artifact::HashingWriter` hashes accepted writes through the
  existing bounded writer, returning the original sink and raw size/digest. It
  needs no complete serialized buffer. Short writes count and hash only accepted
  bytes; a failed producer yields at most an observed prefix, never a completion
  receipt. Encoding, producer success, flush/sync and publication stay local.
- `ic_host_artifacts::artifact::MatchingWriter` compares a produced byte stream
  with borrowed expected bytes. It keeps consuming after a mismatch so serializer
  errors remain visible; callers own successful completion and canonical ordering.
- `ic_host_artifacts::artifact::encode_gzip` writes one zero-timestamp gzip member
  through a bounded caller-owned sink. Callers select compression, source admission,
  compressed byte budgets and publication; output compatibility is bound to the
  selected compression backend rather than promised across dependency upgrades.
- `ic_host_fs::durable::write_with` accepts a streaming producer and returns its
  result after the existing durable replacement sequence completes. Byte-based
  writes share that same commit engine. A returned producer error preserves the
  old destination and attempts to remove only owned staging. Panics or process
  interruption can retain staging. Post-rename sync failure still needs caller
  reconciliation.

- `ic_host_fs::path::canonicalize_allow_missing` resolves existing symlinks and
  normalizes missing suffixes against an explicit absolute base for relative
  paths, including dangling symlink targets. It resumes existing-component
  resolution after parent traversal and retains native directory requirements.
  Its third argument requires the consumer's limit on simultaneously active
  missing-target expansions; `usize::MAX` explicitly selects no added depth bound.
  It neither confines a root nor creates files. Hard-link identity, opening and
  concurrent pathname revalidation remain consumer-owned.
- `ic_host_fs::durable::lock_exclusive_with_wait` acquires a caller-owned regular
  descriptor with explicit polling and contention observations, retrying
  interrupted acquisition. Pathname callers first use
  `open_regular_lock_file_with_parents` and retain that descriptor through the
  wait and protected operation. Polling and report cadence, opening, deadlines,
  lock namespace, descriptor clones and final-owner unlock remain consumer-owned.
  The [0.13 migration](changelog/0.13.md) removes the fixed-cadence pathname wrapper.

Unix native device/inode observations belong to `ic_host_fs::identity`, shared
by filesystem publication and Process custody. File-kind admission, locks,
namespace stability, retention formats and lifetime authority stay with callers;
the identity neither freezes content nor prevents inode reuse after close.

The FS package also owns the [native file command adapter](file-cli.md), which
uses its existing no-follow reads and durable writer. It selects no batch limits,
installation pins, product schemas or recovery policy. Auth retains its browser
contracts and lockfile adapter; its file commands retire only after published
consumer qualification.

The exact source revisions, file digests and selected mechanics are recorded in
[source provenance](../ci/extraction-sources.json). These APIs add no JSON schema,
cache policy, root confinement, tool pin or process-tree ownership. Backup's private
parent/file modes and crash barriers are stronger than ordinary durable replacement;
they must not be replaced by `write_with` without preserving those contracts.
`durable::prepare_private_parents` creates only missing owner-only directories and
synchronizes each new directory and parent link. Ancestor symlinks and `..`
refuse before creation, including through already-existing descendants. Callers
intentionally selecting an alias supply its canonical physical path; namespace
stability and root confinement remain caller-owned. Backup retains its own JSON
encoding, descriptor-relative publication and pre/post-publication crash barriers.

Consumers depend directly on each crate owner. Historical extraction provenance
does not select current consumer versions or authorize dependency upgrades.
The manifests permit crates.io publication through the
[explicit publication procedure](publishing.md). This contract assigns ownership;
GitHub issues own follow-up tracking.

The standalone `crates/ic-host-wasm-inspect` CLI package composes artifact Wasm facts
with bounded filesystem reads, without process execution or the Candid parser.
The library boundaries stay independent. Explicit paths, four budgets and
performance interpretation stay caller-owned. The standard selected Cargo
installer can prepare its published binary without feature flags. Consumers own
binary selection, installation and qualification; Host supplies no Metrics
integration or canister build workflow.
Optional expected-digest verification and export/custom-section details are also
owned by this CLI; the duplicate filesystem `inspect_artifact` example is retired.
Default TSV output remains stable, and the filesystem development profile no
longer selects the Wasm parser for that example.

The optional `icp-project` tools profile parses bounded effective YAML produced by
`icp project show` and projects exact environment/network facts. It selects Text's
optional YAML decoder; the default tools graph excludes Text and that parser. ICP
owns implicit-environment and recipe resolution. Callers retain command execution,
input-byte budgets, selected identities, local funding and lifecycle. This module does
not reconstruct effective configuration from raw manifests.

The opt-in `ic-host-process::testing::http` module owns bounded synthetic request
reading, including phased request-line admission and remaining-deadline socket
reads. Its one-request `HttpFixture` composes that reader with HTTP 200 framing,
explicit caller-selected response bytes, request budgets and one total deadline;
it cancels socket IO and joins its worker on Drop. `testing::net::accept_tcp_until`
shares bounded acceptance with custom-response fixtures. Payload schemas,
application assertions, simulator lifecycle and evidence files stay consumer-owned.
HTTP fixtures are portable; executable fixtures and crash barriers remain Unix-only.

## Logical directory sizes from IC Build

The metadata-only walker and its two filesystem regressions come from
[IC Build 0.12.0](https://github.com/dragginzgame/ic-build/blob/2598a417a96834d3139751adf549126add666c4b/crates/ic-build/src/artifacts/cache_fs.rs),
through [PR #173 — Extract logical directory-size observation](https://github.com/dragginzgame/ic-host-tooling/pull/173).
The algorithm retains iterative traversal, metadata lengths, saturating sums
and original I/O failures. Host owns this pathname observation; callers retain
namespace custody, resource admission, cache policy and cleanup. Build caller
replacement requires published APIs and its own qualification.

## Canic machinery extraction

The additive APIs below originate in Canic's `bd75bdecc` source snapshot and
extend existing workspace members. Consumer adoption and supplier publication
are separate from this extraction.

| Existing owner | API | Consumer responsibilities |
| --- | --- | --- |
| Build | `ic_build::tools::install` | Archive acquisition, retry, exact hash pins, extraction and executable admission |
| Build | `ic_build::tools::toolchain` | Compiler command/environment, target selection, output and inventory bounds, subsequent hashing |
| Build | `ic_build::provenance::git` | Git command/environment, output bounds, redaction and evidence persistence |
| Build | `ic_build::cargo::exports` | Selected keys, Cargo message capture, current recipe and producer identity, persistence |
| Build | `ic_build::artifacts::environment_comparison` | Private key generation/custody, input filtering, bounds and private storage |

Environment comparison uses HMAC-SHA256 with distinct key-ID/value domains and
length-prefixed fields. It provides private equality diagnostics, not cache or
release authority. It reuses the workspace's HMAC 0.13 and SHA-256
implementation; tags and key identifiers must not be published. Git observations
retain raw NUL-separated porcelain status, which may contain sensitive paths.
Tool publication preserves filesystem publication errors and their custody
information. Runtime-library bundles use Unix symlink selectors; other platforms
refuse that bundle operation explicitly.

Failed Git/compiler probes retain Host's typed execution evidence inside their
I/O error. Build delegates installation scratch allocation to Host FS with an
explicit 64-attempt bound and ordinary directory permissions. Before bundle
publication, failed removal retains `BundleCleanupError<E>` inside
`InstallError::Io::source`: the outer path identifies the unremoved bundle,
the causal chain retains the original caller error, and `cleanup` retains the
removal failure. Once the selection is published, a later sync failure never
triggers bundle deletion.
