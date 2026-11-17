# Testkit: first replacement contract

Published Testkit 0.109.0 adds [bounded manual clock progression](#bounded-manual-clock-progression)
through the exclusive controller. Its new `ControlError` variants require
exhaustive callers to update their matches. See the [release receipt](release-ownership.md#testkit-01090-delivery).

Earlier Testkit 0.108.0 selects Protocol `>=0.3.2, <0.4.0`. That release
changes the public `CompileError::Encoding` payload to Protocol
0.3's type identity; callers naming it must align their dependency. The HTTP
encoder source and Types selection are unchanged. See the current
[adoption evidence](qualification/platform-adoption.md#local-protocol-032-adoption).

Earlier Testkit 0.107.4 selected Protocol `>=0.2.7, <0.3.0`.
The [compatibility correction](qualification/testkit-types-compatibility.md)
permits Types `>=0.21.2, <0.22.0`. Scheduling has completed its Runtime migration;
current pressure and held-upgrade checks pass through the product server command.
Client types
and catalog admission, module generation and exclusive request control are
implemented with independent tests. Scheduling's earlier Once aggregate, paired
metrics-feature probe and executor comparison pass on their recorded source
captures with both suppliers. Current and historical evidence is separated in
[Scheduling qualification](../qualification/scheduling/README.md).
Other consumer migrations remain open. Package native
acceptance and the later Protocol migration's local checks are distinguished in
the [release receipts](release-ownership.md#independent-library-publication). This
defines the acceptance contract. The active rewrite is now `../ic-scheduling`.
Its source selection and focused baseline live in
[Scheduling qualification](../qualification/scheduling/README.md). Earlier
old-Scheduling snapshot experiments are historical and do not define this scope.

## WAT and validator dependency requirements

The 2026-11-17 individual pin review changes WAT to `1.263.0`
(`>=1.263.0, <2`) and the test-only wasmparser to `0.263.0`
(`>=0.263.0, <0.264`). Both locked versions remain the latest non-yanked releases.
Only Wasm bytes cross between the compiler and validator; no shared Rust type or
exact-version coupling requires matching pins. WAT's public error remains on the
same compatible major line. Our generated template uses established Wasm syntax;
the contract preserves reply bytes and module limits, not one compiler's exact
module encoding. WAT does not promise SemVer stability for every evolving text
format feature, so actual compiler upgrades still run module validation, opaque
reply/export checks and installed held-reply checks when emitted behavior changes.
PocketIC retains its separate [client/server constraint](testkit-server.md#pocketic-client-requirement).

## Platform 0.34 migration review

### Protocol 0.2.7 selection

The compatible Protocol requirement now starts at 0.2.7, with only `http-request`.
This removes the exact-patch conflict from Tooling #9 without adding an adapter;
the encoder source is unchanged. Fresh package and installed-fixture evidence is
recorded in the [release receipt](release-ownership.md#testkit-01074-publication).

### Earlier published Protocol 0.2.6 selection

Testkit 0.107.3 selected Protocol 0.2.6, with only `http-request`; Types remains
0.21.2. The HTTP encoder source is identical to 0.2.5, while new Caffeine and
management-snapshot features remain disabled. Six fixture unit tests, all-feature
Clippy and graph admission pass. The [adoption record](qualification/platform-adoption.md)
binds the verified archive and focused evidence, retaining earlier installed
acceptance for the unchanged encoder. This update is published in Testkit 0.107.3;
the [release receipt](release-ownership.md#testkit-01073-publication) adds fresh
archive acceptance and verified registry provenance.

### Earlier Protocol 0.2.5 dependency refresh

The earlier development selection was Protocol 0.2.5 with only `http-request` and
Types 0.21.2, verified against the registry on 2026-11-16. Both archives identify
source `9dfdc0083fa5d6e56da7b90c04c20de4b2d3cc9f`; checksums and the repository-wide
inventory are in the [adoption record](qualification/platform-adoption.md).
Protocol adds explicit version-one pricing through a separate encoder; Testkit
keeps `encode_http_get_args` and unspecified pricing. Types source is unchanged
from 0.21.1, but explicit Principal callers must align their Cargo selection with
0.21.2. Registry/placement features remain disabled. Protocol retains encoding
ownership; no new fixture mechanism or request policy is introduced. This is
local dependency adoption, not a new Testkit publication.

Focused Linux evidence in `target/platform-adoption-next/` passes six unit
tests, warning-denied all-target Clippy and both separately selected installed
module/controller tests. Independent inventory observes all three instances
deleted. Default Testkit excludes Protocol, Types and WAT; the fixture graph
selects only Protocol's `http-request` feature. Source and prepared-tool hashes
remain unchanged across execution, and formatting checks pass. These are current
development checks, not a new archive release or native macOS qualification.

### Published migration evidence

The 0.107.1 release selected Protocol 0.2.3 with only `http-request`,
and Types 0.21.1 for public Principal arguments. Protocol's narrow
`encode_http_get_args` replaces the old HTTP DTO and generic Candid encoder.
The legacy Platform 0.33.7 and Types 0.20.4 dependencies are removed; no unused
certificate-only Platform 0.34 dependency or compatibility facade is added.
This removes Testkit's obligation to assemble generic management DTOs and opt
into their broad codec/type graph. Protocol owns wire encoding; Testkit still
owns fixture URLs, slot bodies, reply identity and the module-byte ceiling.

[Platform #5](https://github.com/dragginzgame/ic-platform/issues/5) supplied the
missing encoder. The verified Protocol 0.2.3 archive has SHA-256
`9fe6bc76a461f60b585bee7d6e5117bbe6e63de87ef3cfe4982f0895c00fad91`, from
source commit `cbbf26ade6644ed6fbcadfe3ccbafe809c362f25`. The earlier Protocol
0.2.2 inspection and feature-refusal probe remain historical evidence under
`target/dependency-adoption-review`, not an active migration blocker.

Explicit Principal callers must select Types 0.21.1; text-inferred adapters need
no Rust changes. PocketIC's original client/types and canonical text conversion
remain unchanged. `CompileError::Encoding` now carries Protocol's `EncodeError`,
preserving output-limit versus allocation refusal. Protocol uses a narrower
Candid type table, so generated module bytes can change; byte-for-byte equality
with the old encoder is not claimed. Required HTTP fields and original callback
payloads are preserved and independently observed through the simulator.

The installed module test now asserts exact URL inventory, GET, empty headers,
slot-body bytes, response cap, full replication and legacy pricing, then releases
in reverse order and checks original page-spanning payloads, rejection and
exhaustion. Unit acceptance separately checks encoder-limit propagation and the
complete module's exact/one-byte-short cap. Default graph guards now also exclude
Protocol. All implementation edits stay in this repository.

Final formatted archive run `target/library-release.sCTEKGgV` passes six unit
tests, both installed tests, default and feature-enabled Clippy/Rustdoc, and
default-graph isolation. All three instances are independently observed deleted;
source/tool hashes remain unchanged. The feature graph contains one Types 0.21.1
and Protocol 0.2.3, with no old Platform or Types 0.20.4. The preceding
`target/library-release.mVj1vUCI` run passed before formatting; use the final run
for the current archive's source identity. Older Types-only evidence remains
historical, not qualification of this encoder.

## Purpose and ownership

Testkit owns reusable IC test mechanisms. For this slice those are a single
PocketIC client/type boundary and deterministic held-reply fixtures. The package
removes the need for each consumer to generate barrier Wasm, inspect simulator HTTP
requests, correlate replies or track uncertain release attempts itself. Removing
Testkit dependencies is not a success criterion.

Preserve `ic_testkit::pocket_ic` as the shared upstream identity boundary: sync and
async clients, builders, identifiers and errors must be the original upstream
Rust types, not copied types or newtype lookalikes. Fixture operations must accept
that same async client. Qualify resolved package IDs across these edges. A new
client wrapper, Tokio runtime, server launcher, instance pool, automatic retry,
snapshot abstraction and consumer scheduler do not belong to this slice.

The consumer chooses the prepared server, owns runtime and instance lifetime,
creates/funds the target, installs generated bytes once, drives ticks, selects
release order and asserts its scheduling behavior. Testkit owns bounded fixture
construction, request-token provenance, release admission and explicit bounded
clock steps through that controller. Explicitly selected
`held-reply-fixtures` functionality must not enter product canisters or ordinary
Build/byte libraries. No application-specific Scheduling code moves here.

## Fixture and request state

1. **Construction:** admit the complete immutable update-method/reply catalog and
   caller limits; bind one controller to one server, instance and target canister.
   Return generated Wasm and an exclusive controller. Do not install, fund, start a
   server, download tools or tick. Repeated names consume distinct slots in catalog
   order; each slot can be used at most once. Payload bytes are not a routing key.
2. **Installation:** the caller installs those exact bytes into the bound target
   once. Reinstalling, upgrading or restoring that target while its controller is
   in use is outside the contract. A controller is not reconstructible over live
   calls and must not be cloned into competing release authorities.
3. **Pending:** observe without advancing simulated time, ticking or settling.
   Only expose requests belonging to this fixture's known slots. Return opaque
   tokens bound to server, instance, fixture, slot, subnet and upstream request ID.
   The slot index may be readable; callers cannot synthesize token authority.
4. **Release:** check client identity and manual-progress mode, then verify the
   exact token is still pending. Unknown, foreign, expired and consumed tokens
   refuse before dispatch. Consume local release authority before sending the
   simulator response. The response only opens a barrier: the original callback
   returns its original immutable application bytes. Reordering releases must not
   exchange replies or resume another request.
5. **Uncertainty/abandonment:** a panic, cancellation or transport failure after
   dispatch can leave settlement uncertain. Do not retry a consumed token or
   promise rollback. Caller teardown of the isolated instance abandons pending
   work. Dropping a fixture/controller has no hidden simulator operations.

Allow unrelated calls and multiple independent fixtures in an instance, but never
release another fixture's requests. This is trusted test control, not a security
boundary against malicious same-instance code or concurrent controller recreation.

## Bounds and failure contract

| Boundary | Required behavior |
| --- | --- |
| Catalog | Check slot count, aggregate method-name bytes and aggregate reply bytes with overflow-safe arithmetic before generation; repeated names count per slot |
| Methods | Update-only; reject empty/invalid names, unsupported query slots and unrepresentable Wasm addressing; opaque reply bytes remain exact |
| Generated artifact | Enforce a caller-selected compiled-module byte cap and retain the compilation/admission failure category |
| Manual progress | Refuse automatic/live progress at construction, observation, clock progression and release; caller must not enable it concurrently |
| Pending observation | Return only admitted slots. The upstream HTTP inventory may allocate before filtering; do not claim a hard total-memory/response-byte bound without a separate admitted transport |
| Release identity | Reject wrong server/instance and foreign/stale/repeated tokens before mocking; matching URL text alone is insufficient |
| Time and money | Caller supplies barrier cycles and canister funding; never silently top up. Held clock changes use `Fixture::advance_time` and one explicit processing round; HTTP expiry rejects rather than releases application bytes |
| Cleanup | Preserve admission failures versus uncertain remote effects; no destructor cleanup promise, automatic replay or invented transactional guarantee |

Default fixture calls accept no attached cycles. Cycle-acceptance extensions,
query-reply fixtures and other old exports need their own current consumer evidence
before admission. HTTP-barrier encoding, generated-Wasm correctness and downstream
compiler dependencies need explicit review; the old implementation is evidence,
not source authorized for wholesale import. Native macOS acceptance remains a gate.

## Focused acceptance

The qualification-only `--held-replies` selector runs existing rewrite tests with
released Testkit, establishing a behavioral baseline rather than validating a
replacement. It selects only:

- **Pressure:** 64 declarations admitted and the 65th refused; eight held flights;
  cancelled queued work stays unstarted; reverse release preserves counts; four
  later requests complete exactly once; foreign timer interference remains visible.
- **Held upgrade:** a real pending reply survives refused upgrade; duplicate/stale
  consumer envelopes are inert; releasing the original reply settles the original
  attempt; fresh armed demand reconstructs after an allowed upgrade.

These tests do not by themselves cover all fixture admission failures. Before a
replacement is accepted, add independent tests for exact/over-limit catalogs and
module size, malformed names/query slots, byte-exact replies across a Wasm page,
wrong-instance and cross-fixture tokens, repeated/expired release, auto-progress
refusal, and release uncertainty. Keep positive and refusal cases narrowly selected.
Use two fixtures with differing bytes and deliberate release order so correlation
is independently observable. Do not copy the old test tree or substitute the
rewrite's full Once/Jobs/metrics roster for these checks.

## Implementation sequence

1. Establish the focused current-consumer baseline and record source, lock, compiler
   and prepared server identity. Preserve Testkit imports and caller-owned policy.
2. Introduce an unpublished replacement Testkit package only after reviewing its
   dependency graph and version identity. Keep the upstream type boundary and
   optional held-reply owner explicit; no generic lifecycle framework.
3. Implement the smallest bounded fixture mechanism from the state contract above,
   reviewing any reused coherent unit individually. Validate it with new focused
   failure tests before migrating the consumer export to its API.
4. Run the same held-reply consumer selection against the replacement, check exact
   supplier identities and compare required observations. Publication, sibling
   migration, broad installed suites and performance claims remain separate work.

## Initial admitted implementation

The package version advances from the consumer-selected released 0.106.0 line;
registry identity must be rechecked before eventual publication. No old module,
generator or test tree is copied. The package re-exports PocketIC 16.1.0's original
types. The default has no held-reply catalog dependency; the explicit
`held-reply-fixtures` feature adds `held::{Catalog, Reply, Limits, Budget, Error}`.

Every `Reply` is an update slot, so query mode cannot be requested. Catalog
construction validates all slots, aggregate byte budgets and names before copying
any name/reply data. Each retained buffer uses fallible exact reservation. Repeated
names remain separate ordered slots, and returned views borrow the immutable owned
catalog. Slot count/immutable byte offsets are constrained to nonnegative Wasm32
representation; generation must separately account for encoded overhead and the
compiled-module cap. Empty catalogs and empty replies are valid. This is no promise
of an allocator-overhead or future compiler-memory bound.

Three focused tests cover aggregate exact/over limits, malformed later names,
duplicate ordering and independent ownership of opaque page-spanning bytes. They
do not cover allocator fault injection or unimplemented runtime/token failures.
Default and feature-enabled Clippy and warning-denied Rustdoc pass. The upstream
client adds its native simulator graph only to Testkit; Build and portable bytes
do not acquire it. PocketIC's exact thiserror 2.0.18 requirement changes the root
selection from 2.0.21 within the already declared range; other prior external tuples
are retained. Focused affected-package tests provide the dependency-change check.

The first runtime implementation must consume this catalog rather than establish
a second admission owner. Current Scheduling qualification continues to use
released Testkit; its positive held-reply baseline is not evidence for this partial
replacement. No legacy compatibility facade is added.

Initial Linux evidence is retained in `target/testkit-catalog-tests.log`,
`target/testkit-clippy.log`, `target/testkit-default-clippy.log` and
`target/testkit-doc.log`. The shared error selection passes 19 Build, two byte,
ten filesystem, two HTTP and three qualification-executable tests in
`target/testkit-shared-error-tests.log`. Formatting, tooling/snapshot admission
and the seven-package release map pass. Graph captures
`target/testkit-build-isolation.txt` and `target/testkit-bytes-isolation.txt` contain
neither Testkit/PocketIC nor HTTP dependencies. These are scoped development checks,
not registry, native-macOS or installed replacement-fixture acceptance.

## Generated module slice

`Catalog::compile(target, call_cycles, module_bytes)` now emits an update-only
module from the admitted catalog. It performs no simulator operations and grants
no release authority. The future bound controller must use this same generator.
The module has one used flag and one immutable callback per slot; duplicate methods
consume their slots in original order. HTTP success ignores the response body and
returns the slot bytes. HTTP rejection, dispatch refusal and exhaustion reject.
No cycles-acceptance import is emitted, and memory has a fixed page count.

Reuse review: the old fixture's HTTP-barrier protocol is behavioral evidence.
This implementation freshly generates one narrow module shape; it does not import
the old fixture module, mixed reply modes, lifecycle helpers or test tree. Platform
0.33.7's maintained HTTP DTO and bounded Candid encoder own the request encoding;
Platform Types 0.20.4 owns target identity. WAT 1.263.0 compiles escaped names/data;
Wasmparser 0.263.0 is an independent test-only validation dependency. They remain
outside default Testkit's direct dependency edges and outside Build/portable bytes.

Address accumulation is checked, encoded requests use the caller's module cap,
and data alone exceeding that cap refuses before compilation. The final compiled
length is checked before returning bytes. Compiler memory is not bounded by the
artifact cap; WAT source and compiler allocations are not fallible catalog storage.
Encoding, representation, compilation and module-budget failures remain distinct.

Five unit tests pass, including exact compiled cap versus one byte less, empty
catalogs, duplicate escaped Unicode exports, independent Wasm validation and opaque
page-spanning data. One explicitly selected installed test uses two targets and four
held calls, reverses response order, returns different original payloads despite a
foreign mock response body, rejects an HTTP failure and refuses exhausted slots.
It uses prepared PocketIC 16.1.0 and the released launcher solely as external test
infrastructure. Logs: `target/testkit-generation-tests.log`,
`target/testkit-generation-clippy.log`, and `target/testkit-module-runtime.log`.
Reproduce the installed test with `make test-testkit-module` inside an externally
owned `IC_TESTKIT_POCKET_IC_URL` session. The normal catalog target skips it.

The generator runtime test uses native PocketIC mocking as its oracle, not the
replacement controller. Scheduling remains on released Testkit. macOS and
consumer replacement qualification remain open.

## Exclusive controller slice

`Fixture::new` binds an admitted catalog to the upstream async client and target,
checks manual progress and uses the same generator with a fresh wire namespace.
It does not install, fund or tick. `wasm()` borrows the exact installation bytes;
`pending()` returns opaque `Request` values with a readable `slot()`;
`release()` requires mutable exclusive controller access and the original token.
The caller obligation removed is maintaining HTTP request correlation and local
release-attempt state. No old controller implementation was copied.

A token retains private shared binding identity (server, instance and fixture
namespace), slot and upstream request/subnet identity. Its owning controller is
not cloneable. Before dispatch, release checks server/instance, manual progress,
local token ownership, unconsumed slot and equality with the complete current
upstream request. It then consumes local authority before polling dispatch.
PocketIC transport panics remain panics; cancellation or panic does not restore
consumed authority. This is not a remote transaction or retry guarantee.

Each construction uses a checked monotonic process-local namespace in the HTTP
URL; even another constructor for the same target cannot observe the first one's
requests. Cross-process reconstruction, target reinstall/upgrade/restore and
concurrent automatic progress remain outside the contract. Pending results are
filtered and deduplicated to at most the admitted slot count; the upstream
inventory still allocates before filtering. Drop performs no remote operations.

Six unit tests now pass, including injected dispatch cancellation and panic after
local consumption. These injections test the local uncertainty boundary, not a
network fault or remote rollback. The separately selected installed controller
check uses two fixtures with different replies and reverse release. It proves
wrong-server/instance, foreign-controller, altered request/subnet, consumed-token,
expired-request and automatic-progress refusal. A fresh same-target controller
observes no old requests. This original slice excluded held clock changes; the
bounded progression contract below supersedes that restriction. Automatic progress
is tested only after held calls have settled.

Reproduce with `make test-testkit-control` inside the externally owned server
session. Linux logs: `target/testkit-control-tests.log`,
`target/testkit-control-runtime.log`, `target/testkit-control-clippy.log`,
`target/testkit-control-default-clippy.log`, `target/testkit-control-doc.log`.
The runtime Make target also verifies the installed Toko snapshot. Testkit 0.107.1
passes the current Scheduling export's focused pressure/held-upgrade qualification
and a later watchdog admission/pressure plus Jobs pressure selection;
see its [source and results](../qualification/scheduling/README.md). No sibling
edits or registry publication are implied. Those early slices did not qualify other modes.

The same 0.107.1 draft also passes a single-source-capture released/replacement
comparison for Scheduling watchdog traps/owner commands, refusal backoff and
upgrade reconstruction. This qualifies the unchanged upstream client/type boundary
for those scenarios; it does not enlarge the held-fixture API. The exact selector,
source identity, limitations and cleanup evidence are in the Scheduling record.

The default-feature boundary is now exercised by Scheduling's real synchronous
executor driver on one captured source tree with both suppliers. Three lifecycle
and three admission checks preserve exact reply/event bytes. Its selected
normal/build tree excludes the optional Platform fixture and WAT dependencies;
workspace-wide Once feature selection is not used as proof of executor isolation.
The Scheduling record retains the graph, source and independent cleanup evidence.

The current executor refresh passes against source bytes identical to the full
Once aggregate and paired metrics-feature comparison at Scheduling HEAD
`ee445be226f3ba679e3798d2f7687f2c4d8b0534`. Both released 0.106.0 and replacement
0.107.1 preserve the six executor assertions; the replacement's selected graph
still excludes optional fixture dependencies. All three comparisons have explicit
endpoint refusal and observed instance-deletion evidence. See the Scheduling
record for exact capture hashes and logs. This completes these Linux consumer
checks, not native macOS acceptance, production migration or registry publication.

## Bounded manual clock progression

[IC Tooling #11 — Support bounded clock progression with held Testkit replies](https://github.com/dragginzgame/ic-tooling/issues/11)
demonstrates a current Canic requirement: hold an authentic callback across its
30-second receiver receipt window while preserving the independent HTTP outcall.
Testkit extends its existing exclusive controller; no alternate controller,
application policy, runtime or lifecycle owner is introduced. This removes the
caller's need to bypass Testkit with raw clock changes or duplicate the controller.
The generated Wasm, original client/type identities, immutable bytes, finite
slots and consumed-before-dispatch release boundary are unchanged.

`Fixture::advance_time(&mut self, &PocketIc, Duration)` admits a nonzero duration
of at most `Fixture::MAX_CLOCK_STEP` (60 seconds), checks nanosecond arithmetic,
sets the clock forward and executes one processing round. It returns the original
PocketIC `Time` observed afterward. The round may add zero or one nanosecond;
overflow admission reserves that extra nanosecond. A larger observed deviation
refuses further use. This bounds simulated progression, not network latency or
the wall time needed to execute a round. No slot is consumed and no successful
HTTP response is mocked by this operation. It can execute unrelated instance work
and timeout rejection callbacks; it is not a scheduler or a promise that nothing
else runs. Ordinary caller-owned manual ticks remain available.

Observation, clock progression and release must be serialized across **all**
clients/controllers using the instance. Mutable controller access serializes its
own clock/release operations; callers retain instance-wide exclusion. Concurrent
automatic progress, raw time changes, rollback/reinstall and reconstruction remain
unsupported. Foreign clients and automatic progress are refused before mutation.
Zero/over-limit steps and representational overflow leave local reply authority
and the simulator clock unchanged. Repeated admitted steps are allowed; none
renews an existing request's deadline.

On the qualified PocketIC 16.1.0 client/server, the HTTP barrier's original timeout
is 60 simulated seconds from **HTTP dispatch**, which occurs in a later round than
ingress submission. Its inventory remains pending at the exact deadline and
expires strictly after it. A real token can be released at that deadline; one
nanosecond later, after the operation's processing round, it is absent and release
returns `NotPending`. An expired callback rejects the original call without the
catalog's application bytes. PocketIC's original request inventory remains the
authority; Testkit neither estimates dispatch from first observation nor extends
deadlines. Consumers must allow for the original HTTP budget and their own
additional execution rounds; a 31-second step does not promise that an already
aged request remains live. See upstream's [manual HTTP mocking contract](https://github.com/dfinity/ic/blob/master/packages/pocket-ic/HOWTO.md).

Clock authority is marked uncertain before the first remote mutation is polled.
Cancellation, transport panic, or a divergent final observation leaves this
controller refusing `pending`, `release` and further clock changes with
`ClockUncertain`. Do not retry/reconstruct control; stop all control on the instance
and use its caller-owned teardown. A completed operation alone clears that state.
As with existing release cancellation, refusal is not remote rollback or cleanup.
New public `ControlError` variants require Testkit 0.109.0 for exhaustive matches.
Publication status is recorded in the [release receipt](release-ownership.md#testkit-01090-delivery).

Focused acceptance uses `make test-testkit-control` within an explicitly owned
server session. The new installed case verifies the 31-second continuation,
exact-deadline reply, one-nanosecond-late refusal without application-byte release,
client/token/consumed/automatic refusals and unchanged pending request identities.
Unit tests cover zero/excess steps, arithmetic overflow and interrupted clock
dispatch. The archive's installed-fixture runner also selects this case and
independently verifies deletion of all five instances across its three cases.
Canic's same-receiver enrollment/revocation qualification stays with Canic; these
producer checks do not claim its adoption or shorten its production deadlines.

Linux acceptance on 2026-11-17 passes all-feature/all-target Testkit Clippy,
warning-denied fixture Rustdoc, eight ordinary fixture tests and the three
installed cases through the normal archive-installed runner. Independent inventory
reports five deleted instances. Logs and tool/server evidence are retained under
`target/issue-11/` (`clippy.log`, `unit.log`, `docs.log`, `installed.log`,
`installed/instances-after.json` and `installed-runtime/`). The initial boundary
probes remain there too; only the final wired run qualifies the final source.
The later release archive gate and publication admission are recorded in the
[release receipt](release-ownership.md#testkit-01090-delivery). No new macOS or
Canic execution is claimed by these focused checks.
