# Current handoff — 2026-10-30

## Current adoption and local result

The latest task adopts published Platform 0.11.1 and Wire 0.10.0 together over
the pending adoption changes on `028bc592` (Release 0.71.0). Incoming dirty work
and untracked notes are retained in admission evidence. Only Platform, macros,
runtime guard, Wire and its derive crate change in the lock beyond this admission.
The inherited feature selections stay unchanged; Wire's optional endpoint and
Platform schema integrations stay disabled in Tooling. No Rust caller rewrite
was needed.

Platform no longer has a production Wire dependency or `wire-schema` feature.
Wire owns the optional `platform-types` integration, selecting Platform's principal
capability and TypeScript support without CDK/runtime. This dependency points from
Wire to Platform. The current consumers do not need the optional schemas.

Build's repaired test wiring is preserved: integration targets using shared
helpers explicitly require `test-support`; Make/CI all-feature selections retain
all tests. The CLI target still requires only `cli`, and the ignored optimizer
target keeps its existing admission. Default unit tests need no feature.

Wire is now 0.10.0 with its matching derive crate. Shared Tooling remains 0.27.11
at `62edfbd43087a718bf0283e9040676a49211976d`, independently verified against
all 148 declared payloads in the earlier adoption.

Tools' public registry principal has Platform-owned dependency identity, so
consumers exchanging these values must align Platform to 0.11. Testkit's public
limits and typed codec causes likewise require direct Wire dependencies to align
to 0.10. The draft stays 0.72.0 for both migrations; published 0.71 notes stay
unchanged. Native
codec bytes, protocol and admission rules stay unchanged, and optional Wire
endpoint integration remains disabled. Own workspace/package versions remain
0.71.0; index and HEAD are unchanged. No local
commit, push, release, publication, normal sibling checkout edit or artifact
cleanup ran.

Platform embeds its executor and system-call implementation. Wasm policies now
exclude external ic0/executor packages and check the actual Platform feature
selection: Testkit uses ic0 without runtime/executor; the private probe selects
all three. Codec, native-only and Metrics/Wire boundaries remain enforced.
Platform selects an internal singleton guard only with executor/runtime;
Testkit/Dev and Host/Tools policies exclude it, while the private Wasm probe
requires exactly one. No extra graph observation was added.

The refreshed module/lint rules are applied to maintained Rust wiring. Build's
shared Cargo/filesystem fixtures have one opt-in `test-support` library owner;
integration targets select it through explicit required features. Ordinary modules own
archive tests and integration-only Wasm fixtures. No crate or compatibility
reexport was added. Generated flag probes declare their cfg names. Lint
expectations retain reasons; three allows remain for integration-target subset
selection and an atomic API deprecated only on compilers newer than the MSRV.
The generated `include!` flag fixture tests source-path remapping, not shared
module wiring.

Artifact package rules and effective-feature assertions now consume one Shared
observation per profile. Wasm, encoding and text retain the same exclusions and
required identities. Validation-only profiles retain their previous policy.
The complete Host/Tools graph script makes 36 observations instead of 39 per
target. No cache, public mode, new dependency or supported contract was added.

## Coordinated adoption and dependency direction evidence

Evidence is `/tmp/ic-tooling-wire-platform.1UowyL/`, retaining admission dirty
source/untracked notes, lock/index/HEAD, registry update/fetch and focused logs.
Published Platform 0.11.1 records source commit
`1ed69142b0337311632db540cb70e40bbf059ee6`; Wire 0.10.0 records
`c9d4e7b45b9ba6d841d7e2a664877e3bea035a5b`.

Actual Rust 1.91 passes 17 tests: three registry, nine native Wire-call, four
native memory-adapter and one private protocol limit test. The private runtime
probe and selected Testkit/Native Wasm profiles compile. Strict all-target/all-
feature Clippy passes for Tools, Testkit, Testkit Native, Testkit Dev and the
private probe. Host/Tools and Testkit dependency policies pass.

An isolated offline `platform-types` consumer graph resolves exactly one Wire
0.10.0 and one Platform 0.11.1, without Candid, CDK, Platform macros or a runtime
guard. Metadata confirms Platform has no production Wire declaration or retired
schema feature, and Wire's optional production edge points to Platform. This is
a dependency-graph proof, not installed canister or schema runtime qualification.
No optional integration was enabled in the real workspace.

Root manifest sorting, documentation links and diff checks pass. Only the five
coordinated package selections change from admission; earlier local work, Build's
repair, own 0.71.0 versions, real index and HEAD remain preserved. No native macOS
execution, installed IC experiment, hosted dispatch, full workspace gate, release,
publication, cleanup or Git mutation ran.

## Prior Wire 0.9 adoption evidence

Evidence is `/tmp/ic-tooling-wire-refresh.9bpaQ5/`, retaining incoming dirty work,
untracked notes, lock/index/HEAD, registry update/fetch and focused logs. Published
Wire 0.9.0 records source commit `edb5d1b1286c73ea8fc8afa6cb28339e96fd3174`.
Its facade Rust sources are identical to 0.8.0; the changed optional endpoint
dependency is not selected here. Consumer examples and draft release notes now
name the matching direct Wire 0.9 dependency. No Rust caller changes were needed.

Actual Rust 1.91 passes nine native Wire-call tests and the private protocol's
largest-control-values test, and compiles the private probe for Wasm. Strict
all-target/all-feature Clippy passes for Testkit, Testkit Dev and the private
probe. Host/Tools and Testkit dependency policies, root manifest sorting,
documentation links and diff checks pass. Only the two Wire lock packages change
beyond admission; Platform selections, Build's repair, own 0.71.0 versions, real
index and HEAD are preserved. No native macOS execution, installed IC experiment,
hosted dispatch, full workspace gate, release, publication, cleanup or Git
mutation ran.

## Prior Platform 0.10 adoption evidence

Evidence is `/tmp/ic-tooling-platform-refresh.yu2V68/`, retaining clean admission,
lock/index/HEAD, registry resolution/fetch and focused logs. Published Platform
0.10.0 records source commit `421edbc90494f7dec7f03bac64b0708e80943acb`.
Its facade source changes from 0.9.1 add the prepared-call module and its runtime
export; existing modules are unchanged. No Rust caller rewrite was needed.

Actual Rust 1.91 passes three registry, nine native Wire-call and four native
memory-adapter tests. The private runtime probe and selected Testkit/Native Wasm
profiles compile with Platform 0.10.0. Strict all-target/all-feature Clippy passes
for Tools, Testkit, Testkit Native, Testkit Dev and the private probe. Host/Tools
and Testkit dependency policies, root manifest sorting, documentation links and
diff checks pass. Lock changes are confined to the three Platform packages;
the real index, HEAD, Build's repaired wiring and own 0.71.0 versions are preserved.
No native macOS execution, installed IC experiment, hosted dispatch, full workspace
gate, release, publication, artifact cleanup or Git mutation ran.

## Prior Build archive repair evidence

The original failure remains at
`target/package-qualification-1040156-0/`. Evidence for the repair is
`/tmp/ic-tooling-build-package-fix.OoQTaq/`, retaining admission, original stderr,
real packaged archives, extracted sources and focused logs. Actual offline,
locked, all-feature Cargo packaging with `--no-verify` succeeds for Build 0.70.5;
extracted tests run separately below. No version bump was needed. The final
archive's normalized manifest has no self
dependency and retains all twelve integration targets, with ten requiring
`test-support`, CLI requiring only `cli`, and optimizer ungated as before.

Actual Rust 1.91 passes the final extracted archive's helper test with
`test-support` and all five CLI tests with only `cli`. Source-side checks pass
two default-feature unit tests and the selected integration helper; strict
all-target/all-feature Build Clippy also passes. Manifest sorting, documentation
links and diff checks pass. The lock changes only the self edge; real index, HEAD
and own versions are preserved. No full coordinated package suite, release gate,
native macOS execution, publication, artifact cleanup or Git mutation ran.

## Prior Wire follow-up evidence

Evidence is `/tmp/ic-tooling-wire-adoption.LysnjP/`, retaining clean admission,
lock/index/HEAD, registry update/fetch and focused command logs. Published Wire
0.8.0 records source commit `25a7212269ca4fe1257cf4d3d22efbbe040cb84b`.
Consumer README/overview and draft release notes document the direct Wire 0.8
alignment. No Rust caller edits or new dependency/feature owner were needed.

Actual Rust 1.91 passes nine native Wire-call tests and the private protocol's
largest-control-values test, and compiles the private canister probe for Wasm.
Strict all-target/all-feature Clippy passes for Testkit, Testkit Dev and the private
probe, including native consumer integration target compilation. Host/Tools and
Testkit dependency policies, root manifest sorting, documentation links and diff
checks pass. Index, HEAD and own versions remain unchanged. No native macOS
execution, installed canister experiment, hosted dispatch, full workspace gate,
Git mutation, release or publication ran.

## Prior Platform follow-up evidence

Evidence is `/tmp/ic-tooling-platform-adoption.mWKv3B/`, retaining incoming
changes, untracked source, lock/index/HEAD, registry resolution and focused logs.
Platform 0.9.1's published facade provenance names source commit
`8e21a5ea96eb6b2cabc92be8cffc7df3ac9745aa`. The facade and macro Rust sources
are identical to 0.9.0; this patch changes packaging/feature ownership of the
singleton guard. Previous unaffected source checks remain evidence for their
original inputs rather than new 0.9.1 qualification.

Actual Rust 1.91 passes three registry and nine Wire-call tests with Platform
0.9.1. Testkit/Native's selected Wasm profiles and the private runtime probe compile
on that compiler. Host/Tools and Testkit graph policies, focused ShellCheck, root
manifest sorting, documentation links and diff checks pass. Lock and index are
unchanged from this follow-up's admission; own versions and HEAD remain unchanged.
No native macOS execution, installed runtime experiment, hosted dispatch, full
workspace gate, release, publication or Git mutation ran.

## Earlier adoption evidence and limitations

Evidence is `/tmp/ic-tooling-upstream-adoption.HB3Bzo/`, including admission
source/index/lock/HEAD, pinned supplier sources, exporter/verification logs,
registry update/fetch, resolved metadata and focused command logs. Linux strict
all-target/all-feature Clippy passes for Build, Process, FS, Tools, Testkit,
Testkit Native and Testkit Dev. Host/Tools and Testkit dependency policies pass;
the latter checks embedded runtime features without adding graph observations.

Actual Rust 1.91 passes 86 focused tests: 60 Tools, 13 Build flag/archive/workspace
fixtures, nine Wire calls and four native memory adapter tests. The initial Tools
run passed 59 tests and hit the sandbox's loopback-bind restriction on one fixture;
that fixture passes with permitted local networking. Rust 1.91 also compiles
Testkit/Native's selected Wasm profiles and the private probe. Formatting,
Build manifest sorting, focused ShellCheck and documentation links pass.
Initial compile/fixture/tool-path failures and corrected runs are retained.

No full workspace/release gate, native macOS execution, installed IC experiment,
hosted CI dispatch or publication ran. Native macOS 15 Intel and Apple Silicon
qualification remains separate from Linux execution and Wasm compilation.

## Upstream contributions and adoption boundary

[Shared Tooling PR #327 — Carry structured Cargo options through publication adapter](https://github.com/dragginzgame/shared-tooling/pull/327)
is open from isolated branch `fix/publication-options-326`, commit `339facd34f306408590a329347b950a93563aefd`.
It implements supplier issue #326 with explicit manifest/target paths, feature
selections and check-only dirty admission. Shared path admission is reused by the
adapter and runner; source checks, exact argv, retained attempts and no-retry
semantics remain. Its pending notes originally selected 0.27.11; the separately
released 0.27.11 governance refresh does not include that contribution. No supplier
merge or release ran in this task. IC Tooling's publisher stays unchanged
until appropriate supplier integration/adoption preserves all stronger consumer
provenance, licensing, source, roster and admission-lock requirements.

[IC Observability PR #102 — Allow Metrics arithmetic without native Wire metadata](https://github.com/dragginzgame/ic-observability/pull/102)
is open from isolated branch `feat/arithmetic-without-native-wire`, commit `a885fb3db851a8d8f45b02134ea852cf410e51e7`.
It implements supplier issue #101 with a positive default-enabled `native-wire`
feature. Defaults retain metadata; disabling them excludes native Wire. Serde
and numeric admission remain unconditional, recording independent, and Wasm
excludes native Wire even with all features enabled. Existing no-default-features
schema callers must explicitly select the feature, so pending supplier notes
select 0.19.0 with migration guidance. Package versions and lock stay unchanged.

[IC Tooling #268 — Delegate Testkit scaled benchmark averages to ic-metrics](https://github.com/dragginzgame/ic-tooling/issues/268)
remains consumer adoption work after that supplier profile is integrated and
released. Testkit's existing Metrics/Wire graph guards remain; no source-path,
Git dependency, compatibility arithmetic or dependency-boundary bypass was added.

## Prior batch evidence and limitations

Evidence is `/tmp/ic-tooling-converge-0706.sw5uqp/`. It retains admission/final
source, lock/index/HEAD, actual graph logs, a before/after command trace and seven
feature-policy perturbations. The trace proves 39 to 36 observations and identical
package rules for the three converged profiles. The actual Linux graph policy,
renamed/transitive layering regressions, feature rejection/status propagation,
focused ShellCheck, documentation links and diff checks pass.

Both suppliers use separate source clones under that root; sibling checkouts were
not changed. Shared's publication-adapter and attempt fixtures pass with Cargo/Git
effects substituted, including literal argument bytes, options/path refusal,
dirty upload refusal, drift, logger failures, interruption and retained locks.
Its focused ShellCheck, Bash syntax, companion closure and doc checks pass.

Metrics strict library/test/all-feature Clippy passes on Rust 1.99.1. Actual Rust
1.91 passes six minimal ratio tests, eight default metadata/transport tests and
fifteen minimal doctests; all-feature Wasm library compilation passes. Selected
native minimal/default and Wasm graphs prove Wire placement and Platform exclusion.
Actual declaration policy/refusal fixtures, formatting, focused ShellCheck and doc
checks pass. Initial test-fixture failures remain retained with their corrections.
No full workspace/release gates, native macOS execution, installed IC experiment
or registry publication ran. Hosted qualification is asynchronous; native macOS
acceptance remains separate from Linux execution and Wasm compilation.

## Earlier evidence and obligations

The prior handoff is preserved in the admission archive and at
`9af35e5c:docs/status/current.md`. Its original sources and evidence remain at
`/tmp/ic-tooling-cleanup-0705.Tfe0mO/`, `/tmp/ic-tooling-deadlines-0704.y8lrZV/`
and `/tmp/ic-tooling-cleanup-0703.mBTxZ4/`. The existing unsupported Process/Wasm
sampling-import failure remains as recorded there; it was not requalified here.
Profiler #7, Platform #95, snapshot/adoption obligations and production backup
qualification remain with their existing owners. Preserve all earlier conflict,
grouping/native qualification, supplier adoption, migration/retirement archives,
Host refs, failed preparation/package evidence, release plans, locks and stash
backups linked by the retained handoff. No older evidence was reclassified.
