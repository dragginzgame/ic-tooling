# Current handoff — 2026-10-30

## Current adoption and local result

The latest request adopts IC Wire 0.8.0. The previous Shared/Platform adoption and
cleanup are committed in `9f3282d5` (commit title `0.71.0`; actual workspace version
still 0.70.5). This follow-up leaves local unstaged changes over that clean
admission. Root Wire minimum and the registry Wire/derive selections move from
0.7.25 to 0.8.0; no other lock package changes. Shared Tooling remains 0.27.11
at `62edfbd43087a718bf0283e9040676a49211976d`, independently verified against
all 148 declared payloads in the earlier adoption. Platform remains 0.9.1.

Platform-owned public type identities change across this minor boundary;
Tools callers exchanging those types must align Platform to 0.9. Testkit callers
supplying Wire limits or matching typed codec causes must align direct Wire to
0.8 because those exposed types change dependency identity. Native codec bytes,
protocol and admission rules stay unchanged; optional Wire endpoint integration
remains disabled. The draft stays 0.71.0, carrying the earlier unpublished 0.70.6
graph cleanup and both dependency migrations. Own
workspace/package versions remain 0.70.5; index and HEAD are unchanged. No local
commit, push, release, publication, normal sibling checkout edit or artifact
cleanup ran.

Platform embeds its executor and system-call implementation. Wasm policies now
exclude external ic0/executor packages and check the actual Platform feature
selection: Testkit uses ic0 without runtime/executor; the private probe selects
all three. Codec, native-only and Metrics/Wire boundaries remain enforced.
Platform 0.9.1 selects an internal singleton guard only with executor/runtime;
Testkit/Dev and Host/Tools policies exclude it, while the private Wasm probe
requires exactly one. No extra graph observation was added.

The refreshed module/lint rules are applied to maintained Rust wiring. Build's
shared Cargo/filesystem fixtures have one opt-in `test-support` library owner;
integration tests select it through a self dev dependency. Ordinary modules own
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

## Latest Wire follow-up evidence

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
