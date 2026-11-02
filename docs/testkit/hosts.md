> Subsystem qualification and historical receipts imported from Testkit 0.61.0.
> Current workspace commands are in the [IC Tooling host guide](../hosts.md).

# Host support and qualification

## Current support

This section describes the current checkout. The
[ownership boundary](overview.md#scope-boundaries), workspace manifests and
Makefile define the product and its selected interfaces. The qualification
receipts below preserve earlier source identities, results and limitations;
their old commands and version numbers are historical evidence.

The private test-canister package is `ic-testkit-perf-probe` under
`crates/ic-testkit-perf-probe`. Cargo package selections use hyphens; Rust imports
and Cargo's generated `ic_testkit_perf_probe.wasm` retain underscores.

The private fixture reporter owns complete-sample floating-point statistics;
Testkit owns the public benchmark's checked integer averages.
Focused reporter qualification runs the `statistics::tests` and
`fixture_reuse_driver::report::tests` selections with
`cargo test -p ic-testkit-dev --lib --locked --offline`.
A live fixture-driver smoke qualifies collection and report wiring separately
from those numerical tests.

### Dependency roles and deployment boundary

The root catalog labels dependencies for the supported `ic-testkit` library/server
as production dependencies. The private development
package and PocketIC probe are tests and tooling packages. Publication flags alone
do not determine these roles. `serde_json` is production-classified because the
native product graph needs it transitively, even though local direct uses are tests.

Headed dependency groups use grouped Cargo ordering: `make fmt`, `make fmt-check`, hooks,
and the release adapter's `--manifest-order-grouped` check use the same policy. Labels do
not change Cargo resolution or enforce dependency exclusions.

Review each locked normal/build graph separately; native test dependencies must
not leak into the published Wasm library. The maintained feature contracts are:

| Package and target | Feature selection | Required boundary |
| --- | --- | --- |
| Native `ic-testkit-native` | All features | Only its own package identity; no normal/build dependencies |
| Wasm `ic-testkit-native` | All features | No exported APIs or normal/build dependencies |
| Native `ic-testkit` | Default | PocketIC/Candid allowed; no Wire, executor, Build, Host Text, Agent, parser or full CDK |
| Native `ic-testkit` | All features (`cli,wire,held-reply-fixtures`) | One Wire identity; otherwise the same native exclusions |
| Native `ic-testkit-dev` | All features | One Build, Platform, PocketIC and each Host package identity; tooling is intentional |
| Wasm `ic-testkit` | All features | One Platform/ic0 identity; no Metrics, Candid, PocketIC, Wire, Host, Build, Agent, parser, full CDK or executor |
| Wasm `ic-testkit-perf-probe` | Default | One Platform/Wire/executor/ic0 identity; no Candid, PocketIC, Host, Build, Agent, parser or full CDK |

Platform's private entrypoint macros are intentional in the probe graph. The
library's Wasm graph must also omit those macros. Check each selected native host
rather than assuming a Linux graph qualifies macOS. Cargo's reproducible inputs
are the lockfile, selected package, target, features and normal/build edges:

```bash
cargo tree --locked --offline -p ic-testkit --edges normal,build
cargo tree --locked --offline -p ic-testkit --all-features --edges normal,build
cargo tree --locked --offline -p ic-testkit --all-features --target wasm32-unknown-unknown --edges normal,build
cargo tree --locked --offline -p ic-testkit-perf-probe --target wasm32-unknown-unknown --edges normal,build
cargo tree --locked --offline -p ic-testkit-native --all-features --edges normal,build
cargo tree --locked --offline -p ic-testkit-native --all-features --target wasm32-unknown-unknown --edges normal,build
```

`make dependency-pins-check` now runs the direct declaration checks followed by
`make dependency-graph-check`, which applies this matrix through Shared Tooling's
canonical locked, offline graph observer. The native target comes from the selected
`rustc` host, and CI runs the gate on Linux and both supported macOS architectures.
Wasm graphs also exclude the native Clap command-line parser. CI and the release
gate already call the dependency policy target and explicitly prepare locked caches.
For a focused local run, prepare missing sources with `cargo fetch --locked` first;
the checks never fetch, resolve upgrades or install tools themselves
([#134 — Enforce selected library and probe resolved dependency graphs](https://github.com/dragginzgame/ic-testkit/issues/134)).

**Deployment boundary:** this repository has publication and test-canister
workflows, not an application deployment gate. Graph observation does not prove
which bytes were deployed. Deploying consumers must enforce their own target and
feature profiles, then bind locked build evidence and the final artifact digest
to deployed bytes. Linux and Wasm cross-compilation do not qualify native macOS
or installed-IC consumer journeys.

### Host and Build type boundary

The Host Artifacts, FS, Process and Text requirements select the 0.32 family;
the current lock selects published 0.32.0. Testkit 0.58 through 0.60 expose Host
0.31 startup/cleanup identities; the pending 0.61 release exposes Host 0.32.
Callers naming version-probe errors or command/server cleanup errors must align
their direct Host Process selection. The existing admission and cleanup policies
remain unchanged.

The private developer package passes Host limits and owned children into Build.
It selects published Build 0.24.0, whose exposed Host boundary is 0.32. The lock
must have one Host identity per family package, without aliases or
conversion wrappers. The native developer graph check enforces that requirement
alongside its single Build and Platform identities. The selected release editor follows that same Build lock
entry. This requirement does not add Build to the published library or Wasm.

Build renders the workspace-discovery command and decodes its complete response.
The benchmark driver captures it through Host with the existing 64 KiB per-stream
limits and no elapsed-time deadline. Failed invocations retain their original
typed Host cause and I/O error classification; incomplete output is never decoded.

### Published Platform boundary

Testkit selects registry `ic-platform` 0.5.0 with no default features. Its native
target enables `candid` and `pocket-ic`; the Wasm library selects `ic0` for marker logging,
execution counters and stable-page counts alongside the unconditional external principal identity. Candid, CDK
and host parsing stay out of that graph. The published library selects no executor,
certification, Agent, management DTO or `candid-parser` feature. Native PocketIC and other dependencies still
require transitive Candid; removing direct declarations does not remove that code.

The published archive checksum is
`071793369ed93c3a42dae9a19eedfa76856ca2d0b31f03de7167ae7905bdc4f4`,
matching Cargo's registry index. The archive provides the argument traits,
encoders, bounded decoders, five explicit limits and typed error sources used
here. Its decoder implementation matches the former local implementation; no
compatibility crate remains. External consumers must adopt Platform's limits
and error identities with Testkit 0.54; prior published packages remain available.

Testkit 0.56 changed those authored limits and error identities to Platform 0.2.
Those 0.56/0.57 callers select Platform 0.2 for limits and typed error matching;
the prior 0.1 identity is incompatible even though all five budgets and decoder
behavior are unchanged. Consumers of Testkit 0.55.2 and earlier still need the
older identity until they adopt the published Testkit release
([#133 — Align public decoder-limit identities with published Platform 0.2](https://github.com/dragginzgame/ic-testkit/issues/133)).

Testkit 0.58 and 0.59 use Platform 0.4. The pending 0.60.0 batch adopts Platform
0.5. Its `DecodeLimits` and typed Candid errors have that package identity, so
callers exchanging these values select Platform 0.5.
PocketIC, Candid DTOs and `fake_principal` retain `principal::ExternalPrincipal` (the
original upstream type); the distinct application `ic_platform::Principal` is
converted explicitly at an application/simulator boundary. No DTO, wire bytes,
reply budget, executor or persisted representation changes here.

Platform 0.4 separates raw runtime calls from Candid codec helpers and owns
runtime reject codes. Testkit's probe already supplies explicit Wire codecs;
its PocketIC-facing reject codes and DTOs retain the original external identities.
The signature features are not selected. No local replacement API is needed.

Platform also maintains the native PocketIC client and client-owned DTOs; callers import
its explicit module directly. Testkit retains
provisioning, server admission, startup/teardown, transport classification and
fixture policy. Platform 0.17's client preserves the PocketIC 16.1 protocol and
management DTOs 0.8.0; the server remains pinned to 16.1.0. The library's Wasm
normal/build graph must exclude the native client implementation, Candid, CDK
and host transport; the test probe's runtime graph is a separate intentional selection.

The 0.64 hard cut removes Testkit's forwarding module and convenience aliases.
Platform supplies placement, effective-principal, message-ID, HTTP gateway and
HTTP response identities directly. The former server-free tests of Testkit's
forwarding surface are retired; maintained fixture/lifecycle callers now compile
against Platform directly, preserving PocketIC's management 0.8 identity.
Platform's separate management feature selects a different upstream DTO family;
those identities are not interchangeable. Compiling local callers does not
qualify the siblings' installed-IC journeys or native macOS execution.

The private probe selects Platform's `cdk` feature and entrypoint macros, with
IC Wire framing and shared private method contracts for its native callers. Its
direct Candid/CDK declarations and policy exceptions are removed; Platform owns
the generator and maintained non-cryptographic runtime. The probe keeps explicit
Wire codecs and its pre-copy request guard. Its Wasm normal/build graph excludes
Candid, full `ic-cdk`, upstream `ic-cdk-macros`, PocketIC, parser and Agent.
The qualified registry executor 2.0.0 and ic0 1.2.0 identities remain shared.
Its intentional trap
still calls Platform's raw `ic0::trap` with the same bytes. This completes
Testkit's entrypoint migration under [Platform #1 — CDK call and entrypoint boundary](https://github.com/dragginzgame/ic-platform/issues/1);
other consumers retain their own acceptance. No runtime or codec is copied here.

Native controlled-protocol callers may enable Testkit's optional `wire` feature.
This selects published Wire/derive 0.5.0. Wire's decoding bound is explicit:
the native helpers require an owned response, while requests may borrow bounded
fields. The library enables its direct Serde dependency only with native `wire`.
The private canister's generic decoder requires owned requests because it releases
its input buffer.
Method identities, frame bytes, all four budgets and transport policy are unchanged.
Its query/update helpers use Wire's method types, identities and finite limits;
the private workload and installed probe qualify their actual calls. Default native
and all Wasm library graphs omit this optional dependency. The CI test target
selects `wire` explicitly, while the development package also selects it for its
fixture callers. Reply length checks follow PocketIC allocation. Auth and Wire
retain their own protocol, trust and replay qualification when adopting the helpers.

Platform now supplies `debug_print`, completing the removal of Testkit's direct
`ic0` dependency under [Platform #11 — Expose debug_print for Testkit benchmark marker emission](https://github.com/dragginzgame/ic-platform/issues/11).
The direct-dependency policy permits no workspace package to declare Candid,
CDK-family crates, `ic0`, PocketIC or the private Platform macro implementation directly.
One upstream `ic0` 1.2.0 identity remains transitive through Platform.
Counter 1, 65,536-byte stable-page conversion and marker formatting remain
unchanged. The live probe test checks marker spans, the intentional rejection
text and a successful query after the trap.

`make dependency-pins-check` runs Shared Tooling's direct-dependency checker
against `ci/cargo-dependency-boundaries.json`. It checks canonical package names
(including aliases), normal/build/dev and target-specific declarations for all
members. The adopted checker does not scan unused root catalog entries, arbitrary
future `ic-cdk-*` names, independent workspaces or transitive feature reachability.
The current repository has one workspace and no generated Candid DTOs/templates;
unused catalog entries and new externally maintained IC package families still require review;
the graph gate separately enforces the selected native/Wasm profiles above.

| Target | Required qualification |
| --- | --- |
| Linux x86-64 | Native builds, PocketIC lifecycle, provisioning and operator workflows; CI uses Ubuntu 24.04 |
| macOS 15 Apple Silicon | The same native contracts; CI uses `macos-15` |
| macOS 15 Intel | The same native contracts; CI uses `macos-15-intel` |
| `wasm32-unknown-unknown` | Portable benchmark/principal APIs; the private probe qualifies canister markers and Wire calls |

Use Bash 3.2 or newer, GNU Make, Git, Perl and the native Unix utilities named in
[Shared Tooling's setup prerequisites](../local-setup.md#bootstrap-prerequisites).
The workspace minimum and pinned development/release compiler are Rust 1.99.1.
Prepare its Wasm target explicitly. `make install-tools` prepares common host
and Cargo tools, builds the local owner CLI and explicitly provisions PocketIC.
Generic IC executables remain optional. `make tools-check` checks prepared
selections offline; it neither builds the CLI nor installs missing tools.
The manifest and tool catalogs own exact selections.

### Fixture Wasm encoding and validation boundary

Published Host Artifacts 0.30.0 owns fixture Wasm encoding and full validation.
The native-only `reply-fixtures` feature selects `wasm-encoding`; native development
dependencies select `wasm-validation`. Testkit supplies IC import/export names,
caller budgets, data placement and call order to Host's fixed-memory module
encoder. Host owns binary sections and instructions through its private encoder;
Testkit retains simulator installation and lifecycle policy. Platform continues
to own the Candid/CDK/PocketIC boundary.

The optional native `held-reply-fixtures` feature additionally selects Host's
`wasm-text` compiler for stateful single-use callbacks. Platform's native Candid
encoder and management types encode the required external HTTP barrier protocol;
application replies remain raw. The controller uses PocketIC's manual HTTP mock
facility, without real network access. Use manual progress and fixed simulated
time; never reinstall/restore with pending calls. See
[held provider replies](../../crates/ic-testkit/README.md#held-provider-replies)
for exact-request release, timeout, exhaustion and teardown behavior.

The native installed qualification is `cargo test -p ic-testkit --features
held-reply-fixtures --locked --offline --test pocket_ic_held_replies` with the
authenticated `POCKET_IC_BIN` from the existing offline server check. It proves
real inter-canister calls remain outstanding through unrelated committed ingress,
out-of-order exact replies, repeat/foreign-token refusal and timeout failure.
The existing all-feature native CI includes this target on all three hosts;
Wasm feature/graph checks explicitly include held fixtures while excluding their
native dependencies. These checks do not authorize provisioning a missing tool.

Fixture tests call Host's full validator with a 100,000-byte allowance, followed
by independent structural assertions. Validation covers every function body and
type; structural inspection alone is insufficient. The encoder and validator
remain absent from the Wasm library graph. Private native development tooling
may independently reach the parser through IC Build/Host. Qualify empty and
page-boundary fixtures, admission refusals and live exact query/update replies.

Host FS owns unique directory allocation and `OwnedDirectory` cleanup. Startup
and test directories keep mode 0700 subject to umask and cleanup on drop,
including unwind; caller-selected external startup logs survive teardown.
Qualification evidence uses unguarded allocated paths for intentional retention.
Worker capture uses Host's scoped `TemporaryFile`, cloning its descriptor and
closing/unlinking its name before spawning the worker. The private development
package explicitly selects FS's `temporary-file` feature. The published
library needs neither that feature nor `capability`; these empty-default
backends are selected only by actual callers. Testkit declares none of
`tempfile`, `wasm-encoder` or `wasmparser` directly; these remain transitive Host
implementation dependencies.

The public startup probe failure and command/server cleanup references expose
Host Process 0.31 types in the pending 0.58 batch. Released Testkit 0.56/0.57
uses Host 0.30; consumers select the matching family when naming those types. Server compatibility, deadlines, retained output
and cleanup behavior are unchanged. Qualification uses the selected registry
lock; source compilation and cross-target graphs do not establish native macOS
or deployed consumer acceptance.

The private driver now consumes registry Build 0.22.1 with the same Host 0.31
child and output-limit identities. Its actual Cargo selections, renamed probe
build and installed marker test qualify this caller without a compatibility
layer or unpublished source override. Other consumers retain their acceptance
under [Build #9 — Qualify consumers against the current exposed Host boundary](https://github.com/dragginzgame/ic-build/issues/9).

### Local PocketIC operation boundary

The server CLI and benchmark driver use Host's `SignalActions` for scoped Unix
registration and restoration. Each standalone process owns its dispositions and
supplies a static atomic-only callback: the CLI selects INT/TERM/HUP, while the
driver selects INT. Testkit retains cancellation, child teardown and interruption
exit policy. CLI child statuses use Host's numeric projection with Testkit's
129 unavailable-status and 1 unrepresentable-status fallbacks. Qualify real
interruption and descendant cleanup with `server_runner` and the isolated
benchmark driver interruption case; signal actions are process-global.

`pic::try_pocket_ic_operation` runs one synchronous operation and returns only
recognized transport panics as the existing `PocketIcOperationError`. Candid
calls reuse it. Upstream returned values/rejections and unrelated panic payloads
retain their behavior. The adapter adds no retries, deadlines or async polling
capture; it does not infer whether a submitted update executed or an instance
died. Internal cleanup/collect-all adapters retain their intentional broader
panic capture. Focused qualification selects `pic::transport::tests`,
`pic::calls::tests` and `pocket_ic_teardown`, including real reqwest refusal/reset
through the raw operation adapter, plus live Candid rejection and explicit docs.

The selected client's diagnostic shapes remain a maintained Testkit obligation;
normal support does not wait for upstream structured errors. The exact transitive
`thiserror` constraint is independent: Testkit explicitly selects the same
`=2.0.18` requirement as PocketIC under the maintainer's current decision.
See [the resolver constraint](POCKET-IC.md#resolver-constraint).

## Current focused qualification

Follow the [shared validation policy](../../DRAGGINZGAME.md#validation-and-evidence):
select checks for the affected owner, check for active validation first, and keep
its inputs stable. Prepare missing locked sources with `cargo fetch --locked`
before an offline run. These are focused choices, not a sequence to run for every
change. Use this workspace's `target/`.

| Affected boundary | Focused checks |
| --- | --- |
| Native library and provisioning | `cargo check -p ic-testkit --lib --locked`; `cargo test -p ic-testkit --lib --locked pic::server_assets::tests` |
| Operation errors and call decoding | `cargo test -p ic-testkit --lib --locked pic::transport::tests`; `cargo test -p ic-testkit --lib --locked pic::calls::tests` |
| Platform Candid integration | `cargo test -p ic-testkit --lib --locked pic::calls::tests`; `cargo test -p ic-testkit --lib --locked pic::transport::tests` |
| Canister library | `cargo check -p ic-testkit --lib --target wasm32-unknown-unknown --locked` |
| Development build/report adapter | `cargo check -p ic-testkit-dev --all-targets --locked`; `cargo test -p ic-testkit-dev --lib --locked` |
| Operator and private CLI parsing | `cargo test -p ic-testkit --features cli --bin ic-testkit-server --locked`; `cargo test -p ic-testkit --features cli --test server_runner --locked`; `cargo test -p ic-testkit-dev --lib --locked arguments`; `cargo test -p ic-testkit-dev --lib --locked fixture_reuse_protocol` |
| Benchmark parser and example | `cargo test -p ic-testkit --test benchmark --locked`; `cargo test -p ic-testkit --example benchmark_report --locked` |
| Shared snapshot and real tooling callers | `make shared-tooling-check`; `make tools-check dependency-pins-check`; select the affected hook/release-adapter checks from the Makefile |
| Formatting and API documentation | `make fmt-check`; `RUSTDOCFLAGS="-D warnings" cargo doc -p ic-testkit --no-deps --locked`; `cargo test -p ic-testkit --doc --locked` |

For live tests, build the owner CLI and admit the already prepared bundle:

```bash
make build-server-cli
server="$(target/debug/ic-testkit-server check)" || exit
POCKET_IC_BIN="$server" cargo test -p ic-testkit-dev --test canister_benchmark --locked
```

Select a relevant `pocket_ic_*` integration target for runtime contract changes.
The live marker test consumes IC Build's retained artifact and validates actual
Testkit emission/decoding; generic artifact-cache regression tests belong to
IC Build. The optional full fixture-reuse workload remains outside ordinary CI.
Shared Tooling's exported fixtures remain reviewed snapshot payloads for focused
changed-contract qualification. Routine CI uses the consumer checks below.

Use `make compiler-check` to admit the single local compiler selection, then
choose the affected package/target profile. Native CI and release qualification
use the same Rust 1.99.1 checks; no independent older-compiler lane remains.
The private development harness and its Host/Build dependencies are native-only;
compiling every workspace target for Wasm does not qualify a supported product.
Release execution retains separate maintainer authority.
Passing Linux checks or cross-compilation does not qualify native macOS.

## Shared tooling qualification boundary

The current snapshot selects Shared Tooling 0.25.8 at
`0a2d2ee3b5e0ae77755c2915ac28836d4d5ba359`. Clap 4.6.7, already selected in the
lockfile, owns all maintained Rust CLI schemas, typed values, defaults, help,
version and parse diagnostics. Testkit retains server admission/lifecycle and
exit-status policy; the private benchmark retains workload relationships such as
workers not exceeding iterations and distinct modes/capacities. Native path bytes,
empty child arguments and the private worker's positional protocol remain intact.
Clap is a native-only product dependency and stays out of library/probe Wasm graphs.

CI failure collection uses Shared's `collect-tool-evidence.sh` orchestrator.
Testkit selects outcomes, product logs and linked-parent diagnostics; Shared owns
selector/archiver receipts and refusal of failed selector output as path authority.
`selection-status` retains the original tool failure; collection returns nonzero
for incomplete evidence. Testkit's archive/upload/download fixture retains checks
for redirected parents, partial archives, exact payload bytes and hosted bindings.

The release-adapter fixture copies its declared helper closure through Shared's
canonical fixture preparer. The adapter's companion declaration owns that inventory;
a new shared helper no longer requires another manually copied file list here.
The graph observer and fixture preparer are explicit snapshot entrypoints.

The ordered CI/release gate runs the existing Clippy selection after tool,
dependency and formatting admission, before guard fixtures, product checks,
tests and package verification. Publication performs source/tag admission and
then that same Clippy gate before registry observation or upload. Sequential
recipes preserve refusal under parallel Make; lint success does not replace
the remaining release checks.

The locked IC Build 0.22.1 release editor replaces Shared's retired Perl
manifest/lock renderers. `make install-tools` prepares the selected executable;
`make tools-check` and release preparation admit it offline. Standard release
commands select it without per-shell exports. Fresh preflight prepares tools
after source admission; saved recovery uses its admitted lock. The low-level
renderer has an explicit 8 MiB allowance per metadata document, not a Cargo
build-output or execution deadline. Testkit retains metadata originals,
publication ordering and same-contract interruption recovery.

The canonical no-symlink checker covers indexed and tracked/nonignored working
paths through `make no-symlinks-check`, including the CI/release roster. Package
archives are checked separately during adoption; graph admission alone does not
prove the contents or dependency closure of deployed Wasm.

The portable CI job admits this checkout's prepared tools, dependency selections
and snapshot through its real Make targets under the system Bash and its declared
temporary-directory context. It no longer replays six complete synthetic producer
suites. The adopted Shared Tooling revision dispatches all six from its own
`scripts/ci/test-portable-tools.sh` on Linux and both supported macOS architectures.

| Shared implementation proof | Testkit consumer proof retained |
| --- | --- |
| `test-host-tools.sh`, `test-ic-tools.sh`, `test-rust-tools.sh`: installer, catalog and tool-store cases | Native `make install-tools` and `make tools-check`; local release guards verify setup/check ordering, missing tools and failure propagation |
| `test-cargo-tool-versions.sh`, `test-dependency-pins.sh`: version-query and declaration-parser cases | Prepared Cargo-tool admission and `make dependency-pins-check` against this checkout's actual catalogs, manifests and workflow |
| `test-release-runner.sh`: generic release state machine and recovery | Local release-command, metadata and guard fixtures exercise Testkit's adapter and Make wiring, including input changes and rejected phases |

The existing hook checks, failure-evidence archive/transport qualification and
product tests remain in CI. The canonical exporter retires the six unused producer
matrix roots and their fixture-only companions at the same Shared revision.
Shared Tooling retains those matrices; they are not needed to execute Testkit's
consumer checks.

Runtime tooling is selected explicitly: the dependency-pin checker, Cargo-tool
cache-key observer, installed hook, Cargo version observer, failure-evidence
action and shared evidence collector are snapshot roots. These helpers used to
arrive only through producer fixture roots; deleting a generic regression suite
must not remove a consumer's real checker or evidence path. Required companions
remain exporter-derived, with no local copies or alternate implementations.
This selection follows the [shared consumer policy](../consuming-snapshots.md#consumer-qualification-selection).
It makes no measured gate-duration claim. Native macOS and the GitHub artifact
round trip still require their configured CI runs.

## Historical qualification

Earlier receipts, including their original pending headings, are preserved in
[the qualification archive](qualification-history.md). Use the current sections
above for new work.
