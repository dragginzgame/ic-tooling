# IC Tooling qualification

Toko Tooling is selected as `toko-tooling = "=0.40.1"` in each workspace's
`[workspace.dependencies]`: the product root and `qualification/Cargo.toml`.
Every real package inherits it unconditionally in
`[dev-dependencies]` and lists it in `package.metadata.cargo-machete.ignored`;
the virtual roots are not packages. Commit the declarations, both `Cargo.lock`
files and `.toko-tooling.snapshot` together. To adopt another release, edit both
pins and their documented exact-pin exceptions in
`ci/dependency-pinning-exceptions.json`, run `cargo update -p toko-tooling` and
`cargo update --manifest-path qualification/Cargo.toml -p toko-tooling`, then
`make toko-tooling-update`.
Use `make toko-tooling-install` to install the existing Cargo.lock selection
and `make toko-tooling-check` for offline package/pin/lock/snapshot admission.
That check admits both workspaces against the same installed snapshot.

Explicit `make deps-update` runs `cargo update -v` sequentially in the product
and qualification workspaces, with Cargo's normal output and no separate
metadata or registry audit. Cargo owns resolution and network/offline settings.
Optional `DEPENDENCY_PACKAGE=<name>` and `DEPENDENCY_PRECISE=<version>` selectors
apply to both roots; `DEPENDENCY_MANIFEST=<relative/Cargo.toml>` adds another
explicit workspace. The command never rewrites requirements or synchronizes
selections. Updates are not atomic: the first Cargo failure stops execution,
preserves its status and names attempted workspaces whose locks may have changed;
there is no automatic rollback. Inspect those diffs and qualify adopted changes.
Avoid dependency updates during qualification or after freezing release inputs.
The former `deps-review` and `deps-help` commands are removed. Supplier freshness
review and adoption/deferral decisions remain required at applicable maintenance
and release boundaries. See the
[dependency maintenance contract](https://github.com/dragginzgame/toko-tooling/blob/45c75d1841a291df1c7e7d943ab107d5c3822d47/docs/validation/cargo-graph-checks.md#repository-dependency-maintenance).

The snapshot contains the baseline, canonical root files and executable inputs.
Shared reference guides stay upstream at the commit pinned by the links below;
product documentation remains local under its existing topics. Update selections
through the installer rather than copying or editing managed reference files.

The nine product packages remain in `crates/`. The private development driver
and Wasm probe live in the independent `qualification/` workspace, with their
own lockfile and private version. Run their Cargo commands with
`--manifest-path qualification/Cargo.toml`. Formatting, full lint, documentation
and installed tests select both roots. `make install-server` explicitly fetches
both locked graphs; standalone checks remain offline.
Installation remains explicit; Rust imports are unnecessary. Direct publication
and release adapters check tooling before effects. See the
[root selection guide](https://github.com/dragginzgame/toko-tooling/blob/6e364f0c8d2729d57566aa740a9cd951c744d707/docs/adoption/consuming-snapshots.md#cargo-selection-and-installation).

Local shell/browser fixture defaults and explicit outside-target selections are
documented in [qualification scratch ownership](design/fixture-scratch.md).

## Platform and Wire adoption

Tooling owns qualification of the published supplier selections it adopts
([#349](https://github.com/dragginzgame/ic-tooling/issues/349)). Platform/Wire
releases do not wait for Tooling's ranges, an older registry Testkit client or
these composition checks. A supplier's tests using an older published client do
not qualify the current Tooling working tree. Inspect the actual selected graph
before choosing required API/type alignments; compatible optional patch refreshes
do not require unrelated consumer releases.

| Consumer profile | Supplier boundary and exposed identities |
| --- | --- |
| Native Testkit default | Published upstream `pocket-ic` owns its native Principal/management identities and transport. Private snapshot management encoding uses Platform's owned codec/DTOs. Portable principal generation retains Types; Runtime and Wire remain absent. |
| Native Testkit `wire` | The same native client plus optional Wire. `WireCallSpec` limits and typed codec causes have Wire identity; graph qualification selects only this feature, independently of CLI/held fixtures. |
| Testkit Wasm | Types only; no Platform facade, Runtime, native transport or Wire. |
| Private Wasm probe | Runtime's `cdk`, its matching singleton guard and qualified upstream `ic0`, plus Types and Wire; not a second simulator or production adapter crate. |
| Host Tools adapters | `candid` selects Platform's owned binary codec; `agent` and `ledger-evidence` select their explicit adapters; `registry` selects Types directly. Textual service inspection is removed. Default Tools has neither supplier. |

The native Wire adapter and private probe select Wire 0.21. Decoding follows
upstream Postcard, including redundant varints within width bounds; successful
decoding does not promise byte-identical re-encoding. Keep original frames for
exact retries, signatures and hashes. This adoption preserves encoder bytes,
method identities and explicit budgets; there are no browser Wire bindings here
to regenerate.

The current adoption selects Platform 0.32.0 and Types 0.20.0 in both roots,
with Runtime/macros/guard 0.22.0 for the private probe. These supplier minors
adopt Rust 1.100.0 and lint/must-use policy. Host Tools exposes Platform codec,
ledger and registry identities; Testkit's portable principal helper exposes Types.
Align callers' direct Platform/Types selections when exchanging those values.
The pending Tools 0.92.0 and Testkit 0.102.0 minors already cover this identity and
compiler transition. Native PocketIC identities remain upstream 16.1. The locked
Wire 0.21.5 selection stays unchanged: its optional Platform Types integration is
not selected here. Existing codec budgets, method identities and serialized data
contracts remain; this adoption requires no caller source or storage migration.

Foundation and Build's normal/build graphs forbid the entire Platform family
and higher product layers; development fixtures may still use integration owners.
The existing graph checks own these exclusions and the probe's executor singleton.

After explicit tool setup and `cargo fetch --locked`, use
`make dependency-adoption-check` for a focused composition run. It never installs
tools, upgrades suppliers or publishes. It prints consumer HEAD/dirty state and
the locked supplier versions/sources actually exercised. It then reuses the
existing isolated graph checks and Host Tools adapter tests, default Testkit DTO
identity and transport/snapshot refusal tests, Wire-only adapter tests, installed
PocketIC 16.1 certificate/metrics/management/snapshot contracts, and the private
Wire canister's request/reply/state/failure test. The private driver is explicitly
a Wire-enabled harness; its workspace features do not prove the default graph.

`make tools-features-check` and `make testkit-graph-check` remain the cheaper
graph-only entries. The latter now includes a separate Wire-only native graph;
all-feature qualification is additional coverage. Keep installation, transport,
protocol codecs and caller recovery with their current owners.

This adoption command is not a new supplier or release gate. The existing
[release plans](design/release-validation.md) and their 32-stage full CI roster
are unchanged. Testkit retains the complete gate; Tools can narrow only through
its admitted three-host baseline and directly affected adapters. A future supplier
API/type break selects the affected Tooling minor and caller migration; it does
not force generic Foundation/Build releases. Native macOS 15 Intel and Apple
Silicon and product release obligations remain independent of focused Linux proof.

Host profiles below remain independent. Build and Testkit share the root
workspace and release matrix, with separate optimizer, browser-session and
PocketIC lifecycle workflow jobs. `make testkit-check` runs aggregate Testkit
qualification using the prepared server; `make testkit-wasm-check` qualifies deployed profiles. `make testkit-package-check`
qualifies all nine public archives through Build. Private packages remain
excluded from publication. Native macOS results must come from native runners.
The driver's retained attempt holds selection diagnostics beside Build's
`archives/` qualification directory. Its separate `target/archive-build-cache`
retains registry compiler outputs across attempts. Before every test run, Cargo
cleans only the extracted workspace packages from that owned cache, so fixed tar
mtimes cannot let a changed same-version payload reuse its old build. A lock
serializes invalidation and tests; unowned or redirected cache paths refuse.
Archive/source/graph verification and both all-target/doctest commands always run.
Compiler outputs are excluded from evidence uploads; original diagnostics remain.
The generic Build convenience API still uses a fresh target for every attempt.

Plain `cargo clean` removes this workspace's complete `target/`, including nested
qualification caches and evidence. Builds/releases do not clean automatically.
Release receipts and durable deployment logs under `.git/release-state/` survive
that cleanup; separate `.tools` selections, the shared tool store and explicitly
chosen fixture roots retain their own cleanup scope. Cargo's normal workspace
target is unchanged, and operator target/profile/package selections remain valid.
Linked-worktree [release-plan artifacts](design/release-validation.md) live under
`target/release-validation/`; preserve them with their receipts before cleanup.
Deleting those artifacts invalidates that qualification evidence.

Native hosted CI uses `make ci-publication`: prerequisite admission, publication's
source and workspace lint gate, then the publication dry run, followed by the
remaining CI checks with `CARGO_NET_OFFLINE=true`. The full lint runs once; a
failure stops subsequent stages even under parallel Make. Standalone `make ci`
retains its own full lint gate and does not run the publication dry run.
Both are full qualification commands, subject to the shared validation policy.

The workspace minimum and pinned development/release compiler are Rust 1.100.0.
`make compiler-check` reads the workspace manifest and toolchain pin and requires
matching actual Cargo/rustc identities. Native CI and release qualification run
that admission once, then retain native, isolated-feature, Testkit Wasm and
unsupported-filesystem checks on the same compiler. The existing GitHub `msrv`
status depends on all native results; it performs no duplicate build or cache setup.

Both owning workspace roots apply the
[common Rust lint policy](../TOKO-TOOLING.md#common-rust-lint-policy), and all eleven
members inherit their owner's tables. Reviewed pedantic API/style checks remain;
blanket nursery/restriction groups are removed and function length is non-blocking.
Safe library/CLI roots forbid unsafe code. FFI and qualification targets inherit
deny; the existing reviewed process/signal exceptions stay scoped. Generated
exports are checked by the Wasm probe. Only the baseline-pool module selects
`future_not_send`: its recipe contract promises Send futures, unlike local adapters
and caller-owned writers.
Strict Clippy and Rustdoc commands retain `-D warnings`; no documentation gate or
runtime ownership boundary is relaxed by this adoption.

Required hosts are Linux x86-64 and macOS 15 on Intel and Apple Silicon.
Successful native jobs retain small `*-qualification-*` artifacts with
the clean source revision, exact Cargo.lock, compiler/Cargo identities and declared policy, actual
OS/architecture/version, selected profile and completion marker. These are
produced after the job gates pass and retained for 14 days; cache restoration
does not itself qualify a source or host. Compilation caches exclude attempt-specific archive targets. Hosted uploads/cache effectiveness
remain distinct from local script qualification. The dedicated archive compiler
cache can be restored separately; extracted workspace packages are rebuilt before
tests regardless of cache restoration.
Native CI explicitly adds `wasm32-unknown-unknown` to the declared development
compiler after preparing it, before running the deployed Testkit profile checks.
The FS package also owns the [native file CLI](file-cli.md). Its actual-executable
fixture uses the focused command in that guide; native CI includes it through
the existing package gate.

The imported filesystem/process APIs retain their platform gates; unsupported
platforms receive their existing explicit refusal rather than simulated behavior.

`ic-host-fs` can compile as a library for `wasm32-unknown-unknown`, but this is an
unsupported execution target. Its three regular lock-file openers return
`RegularFileLockError::UnsupportedPlatform` before filesystem effects; other APIs
retain their individual platform gates or native standard-library errors. Unix
descriptor-only APIs stay Unix-gated. This makes no promise of usable filesystem
IO in a canister, and downstream Wasm graphs should continue excluding Host FS
and native Build/Testkit harnesses. `make check-fs-non-unix` compiles the library, CLI,
test-support feature and non-Unix lock admission test without executing them.
It requires the Wasm standard-library target to be prepared explicitly. Native
CI installs that target for the repository pin and includes this unsupported-target
compilation alongside the deployed Testkit profiles.

Select one changed crate with `PACKAGE=<crate>` for `make check`, `make clippy`,
`make docs-check` or `make test`. Artifact checks use its actual enabled gzip,
and Wasm features; production filesystem dependencies select none of them.
Check target-directory ownership before compilation. Preserve failure logs and artifacts.
The Testkit and feature-graph qualification targets above are aggregates;
select the affected Cargo test or profile below for ordinary development.
Build integration tests using shared helpers require `test-support`; Make/CI
all-feature selections retain them. A focused invocation is `cargo test -p ic-build
--features test-support --test artifact_helpers --locked --offline`; select `cli`
alone for its release-metadata CLI target. Default unit tests need no feature.
Tooling changes use the affected consumer check: `make release-adapter-check`,
`make release-validation-plan-check`, `bash scripts/ci/test-make-snapshot.sh` or
`bash scripts/ci/test-host-fixture-retention.sh`. The shared
[validation policy](../TOKO-TOOLING.md#validation-and-evidence) owns execution cadence.
Select `bash scripts/ci/test-make-snapshot.sh --case release-routing` for ordinary
release Make dispatch and delivery refusal, with runner effects substituted.
This consumer fixture also owns outer/inner runner failures and conflicting
release selection. The generic minimal-checkout dispatch fixture is not repeated:
it cannot model this repository's independent qualification admission.
It does not qualify frozen delivery hooks. Installed frozen-hook qualification
uses `bash scripts/release/test-frozen.sh`; it retains actual metadata/Git and
publication-dry-run evidence, with product gate bodies substituted. Complete
installed release and registry proof remain separate.
For receipt/prepared-version changes shared by all groups, select a representative
owner with `bash scripts/release/test-adapter.sh --group testkit`; the default
entrypoint retains all five groups for complete release qualification.
The compact adapter fixture seals one real invariant proof; the complete
production fixture roster remains qualified by the dedicated stage/gate tests.
For publication lint fallback diagnostics and first-failure stopping, use
`bash scripts/publish/test-workspace.sh --case lint-fallback` (effects substituted).
For full-CI roster and lint/publication ordering, use
`bash scripts/ci/test-workspace-validation.sh`; it runs the actual Make routing
with substituted check/build/publication effects and no Cargo compilation.
For explicit tooling-fixture membership/order and first-failure stopping, use
`bash scripts/ci/test-make-snapshot.sh --case tooling-roster`. It retains the
actual Make/selector wiring with substituted fixture bodies; the default
entrypoint retains the broader Make contracts.

Fixture retention checks execute each real initialization/EXIT boundary with
failure injection and relative, aliased temporary parents after changing cwd.
They do not execute product fixture bodies. The `tooling` roster explicitly runs
each Make, release, publication and qualification fixture once during complete CI;
ordinary development selects the affected entrypoint directly.

Build's `artifact_helpers` Cargo-membership fixture deliberately uses the native
system temporary directory outside this checkout. It qualifies implicit excluded
packages without an enclosing Cargo workspace; placing it under `target/` would
change Cargo's ancestor lookup. Its created directory is removed on success and
retained on assertion failure. Other fixtures keep their caller-selected scratch.

Standard `make fmt` and `make fmt-check` report each manifest/Rustfmt pass and
its elapsed time. An unchanged successful selection can reuse its formatting
receipt after prepared-tool admission; `FORMAT_RUST_ARGS=--no-cache` runs all
passes explicitly. The Host Make fixture disables reuse for its substituted
Cargo, while the formatting-hook fixture uses the actual prepared tools.

`make clippy-workspace` is the existing full-workspace, all-target/all-feature
warning-denied lint gate. CI/release validation runs it before expensive tests
and builds after prerequisite admission. Publication and its dry run use that
same gate before packaging or registry dispatch; ordinary package checks keep
their selected scope. These entrypoint rules do not authorize a full local gate.
Text, Tools and Build release plans can select package/consumer lint and the
[focused contracts](design/release-validation.md) after baseline admission;
Foundation, Testkit and uncertain/shared changes retain the full gate. Tools/Build
need a new matched three-host comparison before they can narrow. CI and
publication keep their complete gates. The `release-scope-*` targets in that
guide expose the selected owner checks without executing a release.
Independent group versions are literal in every member; root Cargo metadata has
no shared version. Public release and publication Make/CLI commands require an
explicit group. CI's read-only `workspace.sh qualify` mode still checks every
public package with one full lint pass; it cannot upload.

`make test-artifacts-minimal` checks the artifact library without optional
features. `make test-artifacts-wasm` checks inspection without the encoder or
validator. Testkit owns direct raw-reply encoding and its independent byte-contract
and installed query/update checks.
`make test-artifacts-text` checks optional WAT compilation without a production
parser or validator. Source/output admission, symbolic names, stateful instructions
and separate full validation are covered in its all-feature profile.
Testkit development dependencies select full validation for generated fixtures.
The feature guard checks the Artifacts profiles on Linux and both macOS target graphs, including exclusion
of wasmparser's validation feature unless explicitly requested. These graph checks
do not replace native qualification. Both capabilities require the declared
compiler and native Linux/macOS evidence; native macOS remains separately reported.
The direct-dependency policy restricts cap-std, cap-fs-ext and tempfile to FS,
wasmparser and wat to Artifacts, and direct wasm-encoder to Testkit, including
dev/build and aliases.
All normal/build profiles except explicitly selected text compilation exclude
wat/wast. FS defaults, CLI, Process and the inspector exclude the cap/tempfile
backends. Independent `capability`, `temporary-file` and combined FS profiles
require only their selected backends on native targets; the non-Unix combined
profile excludes both. `make check-fs-non-unix` also compiles the unsupported-target contract.
FS tests cover held-parent operations, link/special-file refusal, bounded entries,
temporary-file cleanup and the read-only no-follow descriptor opener.
For feature qualification, run the FS library tests with no default features,
then with `capability`, `temporary-file` and all features. The latter includes
the real CLI tests. Native CI's all-feature package gate retains both backends'
regressions; Linux and both macOS graph checks independently enforce selection.
`make test-tools-default` checks response and ICP interpretation without
Platform or Candid. `make test-tools-candid` checks opt-in Candid decoding/service
facts. `make test-tools-interface` adds Wasm export
interpretation. `make test PACKAGE=ic-host-text` checks bounded line normalization,
escaping, progress frames and Unicode table alignment.
`make test-tools-registry` checks golden Protobuf tags and shard admission.
Single-feature Tools targets compile all targets under their isolated feature
selection, then run that feature's tests. Default and combined profiles retain
their complete suites; shared response/ICP cases need not repeat in each feature.
For Tools' independent `icp-ledger` profile, run `cargo test -p ic-host-tools
--no-default-features --features icp-ledger --lib icp_ledger::tests --locked
--offline` using the pinned compiler. Its fixtures encode the official
fields independently; the feature graph excludes Platform, Candid and Agent.

Text's optional `yaml` feature and Tools' `icp-project` require Rust 1.100.0.
Qualify YAML with `cargo test -p ic-host-text
--features yaml --all-targets --locked --offline` and the equivalent Tools
`--features icp-project` selection. Tests cover ignored-field duplicate/tag/merge
policy, UTF-8/byte admission, alias handling and no external expansion. The byte
allowance is not a parser memory/CPU budget; backend recursion/repetition checks
are retained. The dependency guard confines `yaml_serde` to Text, rejects competing
and retired frontends and excludes YAML from defaults and unrelated profiles.
YAML-enabled graphs require the maintained `libyaml-rs` backend and exclude
`serde_yaml` and `unsafe-libyaml`. This is the YAML Organization's maintained
fork of the same low-level Rust port, not a rewrite that eliminates unsafe Rust.
Both licenses and compiler floors were reviewed: `yaml_serde` is MIT/Apache-2.0
with Rust 1.82; `libyaml-rs` is MIT with Rust 1.60. The public Host contract stays
on Rust 1.100.0. All-feature and isolated YAML checks use the same pinned compiler
and native CI lane.

Platform owns the strict certificate/hash-tree decoder and its grammar, recursion,
byte-limit and error tests. Tooling's `ic-testkit-dev --test public_contracts`
retains Testkit's public subnet-metrics DTO identity check within ordinary workspace
tests. It does not repeat Platform's decoder suite. Every selected Tools graph
excludes Ciborium; the ledger profile retains its upstream Minicbor dependency.

`make test-tools-agent` exercises native construction, exact binding and response
ceilings. Its Wasm graph check observes dependency exclusion, not Tools execution.

Qualify dependencies for the actual target and selected features. Package-selected
graphs differ from a combined workspace build: native Platform features can unify
Agent dependencies into other consumers of the same package. Agent builders retain
caller-selected identity and signing, so operation review must name the identity
and secret-bearing path. See [the crypto qualification owner](https://github.com/dragginzgame/ic-tooling/issues/240)
for the current graph evidence and Platform correction dependency. Named-target
macOS graphs establish selection only; native execution remains separate evidence.

The optional `icp-project` feature qualifies bounded effective-YAML projection;
default and other unrelated profiles exclude its YAML parser.

`make test-tools-encoding` checks canonical unsigned LEB128 admission. The decoder
adds no dependencies. `make tools-features-check` excludes Candid and its parser
from default and interface-only normal/build graphs. `make test-tools-candid`
checks owned binary decoding and its public consumer; its graph also excludes
registry Candid. Textual service inspection and its combined test target are
removed. Agent retains upstream native transport; ledger evidence uses the owned
codec. Neither enables the textual parser.
Registry selects `ic-platform-types` directly, without the facade or codec. Its
native and Wasm graph policies retain one canonical Principal identity. `ic-interface` adds
Host Artifacts and Wasm parsing. The standalone Wasm inspector
retains its parser-free graph. These non-YAML profiles must qualify on Rust 1.100.0
and native Linux/macOS matrix as the existing libraries.

Platform 0.23 delegates IC system calls and the executor to independent Runtime
0.19. Testkit's portable library has no Runtime dependency; the private
performance probe enables Runtime's executor through `cdk`. Wasm compilation
checks this selection separately from native execution and installed IC behavior.
Platform selects its internal singleton guard only for executor/runtime
users. The graph policies exclude it from Host/Tools and Testkit, and require it
for the private probe's Wasm runtime.
Consumers exchanging Platform-owned registry principals with Tools must align
the canonical `ic-platform-types` 0.18.1 identity exposed by Platform 0.23.0.
Platform's certificate codecs return the same upstream certificate/hash-tree
identities as native Agent. Pass decoded values directly between those APIs.
Tools profiles without Agent retain their declared dependency exclusions.
Platform has no production Wire or Host Artifacts dependency. Public caller tests
check Principal/Account core frames, limits and type identities. Production
profiles use only Wire's core codec and do not select its optional metadata bridge or
`ic-wire-endpoint`. Platform's separate groups preserve
independent Types releases; incompatible Types changes still require explicit
consumer and schema adoption.

Native Testkit depends on published `pocket-ic` directly under the maintainer's
approved native exception. Callers import its async client and native DTOs through
Testkit's native-only upstream re-export. Native snapshot preparation and held
fixtures select Platform's owned Candid and management adapters. Graph checks exclude
`ic-platform-pocket-ic`; Wasm excludes both clients and registry Candid/management.
The installed PocketIC server remains pinned to 16.1.0. Qualify upstream panic,
best-effort deletion and coherent snapshot preparation through the
[migration](testkit/client-migration.md), not the retired fork's guarantees.

Native and Wasm Testkit graphs exclude Metrics; the library has no recording
or instrumentation dependency. Default native and all-feature Wasm Testkit still
exclude Wire; Host/Build retain their Metrics exclusions.

Native CI prepares pinned Rust and the complete common toolsets, then verifies
the shared snapshot, declarations, formatter prerequisites, actual hook behavior,
release adapter,
common tool/LOC command wiring,
minimal artifact tests and each selected package with all features in sequence.
Release metadata backup/install/restoration and receipt custody use pinned Shared
helpers. Their synthetic failure matrices stay with Shared; Host's adapter fixture
checks real rendering, disposable Git and committed-payload contracts for both
release groups, with substituted setup/full qualification. Toko Tooling owns group
rendering and transactions; source custody and the complete gates remain local.
See [release ownership](design/release-groups.md).
The consumer's [tooling-check roster](../ci/tooling-checks.tsv) selects ordinary
command checks, the appropriate Make/retention fixtures and reusable stage names
in its optional third column. Release gates can reuse retention and archive-evidence
fixture results through exact-input admission; ordinary CI executes both fixtures.
Keep behavioral
assertions independent of that roster when extending a selected check.
Toko Tooling owns the synthetic Make-consumer, host/IC/Rust installer and LOC parser
matrices in `toko-tooling/scripts/ci/test-portable-tools.sh`. Host retains its actual
Make routing, aggregate failure ordering, LOC arguments/status and pipe-jobserver
checks in `scripts/ci/test-make-snapshot.sh`, plus prepared-tool admission and
native qualification. Native identity admission and pipe-jobserver observation
use the pinned Shared helpers; Host retains the runner matrix, selected macOS
major, PATH setup and real Make callers. Their synthetic observer matrices stay
with Shared. Common fixture copying and per-target unsafe-mode probes
use Shared helpers; exhaustive flag aliases and inherited/replaced combinations
remain producer checks. Native CI's shared common-tool action retains setup/check
logs and publishes PATH only after both succeed. Snapshot checks retain all
selected bytes and modes;
the CI cache-key helper is an explicit snapshot root for its workflow caller.
The release gate uses the common host and Cargo toolsets. Host has no product
IC executable caller, so its aggregate setup and CI do not select the IC bundle.
Shared 0.24.0's common Cargo set contains cargo-sort and cargo-sort-derives only.
Host has no extractor invocation: IC Build owns native extraction. Refresh the
common PATH through `make install-rust-tools`; retained extractor bundles are
not deleted or silently adopted as a product selection.
The individual IC setup/check commands remain available for explicit use.
Explicit setup runs `make install-tools`;
`make tools-check` verifies the set offline. Selected release preflight prepares
it; standalone verification never installs tools. Existing bundles and failed
evidence are retained. Rust toolchain bootstrap remains explicit.
Native CI uses the shared Cargo-tool build identity to restore builds.
Cache hits still run complete setup and admission;
cache transport failure falls through to setup. Main saves only after admission.
This cache does not qualify native macOS behavior or guarantee a setup duration.
`make formatting-hook-check` runs the shared adoption checker with Host's real
Cargo manifest, Rust input and Make formatters. Generic formatter prerequisite
and hook implementation matrices remain in Toko Tooling's own CI. Host keeps
prepared-tool admission and its actual Make failure-propagation checks.
The caller selects direct roots; the shared fixture copier follows their
declared companions, including new unstaged helpers.
Failed Git observations, partial staging, formatting failures and
unrelated edits are covered in disposable checkouts; the real index, working
files and hook configuration are preserved. The frontend formatter companion
retires with its unused generic fixture; Host has no frontend build step.
Both native macOS jobs run this same target through `make ci`; local Linux and
Bash 3.2 passes remain separate evidence under
[Host #55](https://github.com/dragginzgame/ic-host-tooling/issues/55).
Shared 0.3.1 setup checks IC platform/catalog admission and Rust/Cargo
availability before downloading common tools. These preflight checks do not
install a toolchain or replace the offline verification of installed tools.
Release preparation uses the admitted IC Build metadata editor with source-bound
member identities, with rollback qualification in disposable release fixtures.
`make install-tools` includes the pinned release-only editor; `make tools-check`
admits it offline. The published editor retains its own Rust 1.91 floor; the current workspace
requires and builds with Rust 1.100.0.
The editor selects published IC Build 0.73.0 with `cli` for group manifest edits.
Both native CI cache paths include its tool-store entries; its pin participates
in the cache key. `make release-adapter-check` exercises actual rendering and
disposable Git with substituted setup/full qualification, including first-use
setup and busy refusal.
CI runs the release, publication, qualification-record and Make adapter suites
once through the retention check, using a relative aliased temporary parent.
Their standalone focused targets remain available; CI does not repeat them in
the ordinary temporary directory. The retention failure probes still exercise
each admitted entrypoint before setup or other effects.
`make check-no-symlinks` checks indexed and tracked/nonignored working paths in CI
and the release gate. Publication checks complete regular source notices and the
prepared archive's exact notice bytes before upload.
The root catalog groups all twelve local members by release ownership, with full
versions and their group changelog paths, including private and unused entries.
External production-only and mixed production/test dependencies have separate
blocks; no current external dependency is build-only or test-only. Catalog entries
add no graph edges until members inherit them. Release admission checks every
entry's version, actual member path and owning group's dependency roster.
Clap is shared by production CLIs and maintained examples. FS gates its binary
and normal Clap edge behind `cli`; generic Artifacts/FS/Process library graphs
exclude it. Clap owns typed arguments, help, version and diagnostics; CLIs retain
their admission, output and operation semantics. Existing feature matrices use
Toko Tooling's Cargo graph checker for canonical names/counts while retaining Host's
enabled-validator/compiler feature checks. These are not deployed-artifact proof.
The Process alias is a test-only dependency of Tools' HTTP fixture and is excluded
from the other packages' normal/build graphs. Process itself remains a production
package. Tokio stays in the production category because Agent transport also
uses it transitively, although Host's direct Tokio declaration is dev-only.
`make publish-command-check` exercises the workspace's real publication adapter,
including package selection, provenance, Cargo/log failures, interruption and
retained attempt locks. Toko Tooling owns the generic publication-runner
regression matrix; this consumer does not replay it separately.
Host's direct-dependency guard selects Toko Tooling's
`ci/external-ic-dependencies.txt` inventory and composes it with stricter local
`ci/cargo-dependency-boundaries.json` rules, including renamed, development, optional and
target-specific declarations. The broader Platform boundary also requires review
of new IC package identities and generated imports; the finite policy does not
automatically classify every future package. General-purpose dependencies remain
with their consuming crates. Current Host declarations already satisfy the rule.
The production section uses the canonical role header and grouped Cargo sorting:
`fmt`, `fmt-check` and release source qualification use the same ordering.
Prepared/committed release checks sort-check the exact saved root manifest;
unchanged member manifests belong to the previously qualified source.
No release-only executable extends the aggregate common toolset.
CI also owns native macOS execution; Linux qualification is not macOS evidence.
The reviewed Toko Tooling snapshot owns `ci/tool-versions.env` and `ci/ic-tools.tsv`.
Host uses these common defaults without catalog exceptions; Rust compiler policy
remains local. Host, IC and Rust exceptions, if
needed later, require independent explicit selection and native qualification.
The common catalog selects Binaryen 133 for all three supported hosts.
Explicit IC setup,
offline admission and optimization/execution smoke pass on Linux. Both native
macOS setup/check jobs passed on Host 0.12.9
([Host #53](https://github.com/dragginzgame/ic-host-tooling/issues/53)); later
changes retain their own qualification obligation.
The consolidated workspace owns Build's native optimizer workflow, including
original/transformed Wasm execution through its pinned Node runtime. Shared
0.26.2 requires explicit IC executable selection: Make declares `IC_TOOLS :=
wasm-opt`, and the workflow's direct offline check selects `--tool wasm-opt`.
Host libraries themselves acquire no optimizer dependency.
Pin changes use explicit IC setup, preserving old bundles, and need native
acceptance independently of snapshot integrity.
Shared 0.2.0 adoption removed its PocketIC rows while retaining the other tools.
Host library gates have no PocketIC callers, so no simulator dependency or setup
is added.
Consumers that need a simulator use Testkit's qualified setup/check/run contract.
The shared installer validates the complete catalog but installs only the
selected wasm-opt executable. Explicit `make install-ic-tools` prepares this
selection while preserving previous bundles, receipts and failed evidence.
Tool reuse compares validated records, so comments and record order do not
require downloads or replace installation provenance. Changed tool selections
and malformed records still fail admission before executable checks.
The workflow runs on pull requests and pushes to `main`; it cannot qualify
uncommitted local bytes. A scoped contribution PR exercises both macOS 15
architectures and Linux against the candidate source before release. Bind
results to that PR's tested commit and keep consumer rehearsals separate.
Earlier Toko Tooling host-fixture failures on macOS remain historical evidence
in the original repository. The refreshed fixture has local Linux execution with
substituted host identities here; native macOS qualification remains separate.

## Dependency exclusions and deployment evidence

`make candid-boundary-check` is the lightweight, offline
[maintained Rust Candid source check](design/candid-boundary.md). The common
`dependency-pins-check` stage runs it before dependency metadata admission.
Its focused consumer rejection control is
`bash scripts/ci/test-product-candid-boundary.sh`; neither command compiles Rust.

`make tools-features-check` checks transitive normal/build dependency exclusions
for the native target (or `CARGO_BUILD_TARGET`), including the combined Tools
features. `bash scripts/ci/check-tools-features.sh TARGET` inspects a named supported
Linux/macOS target without claiming native execution. The checker is in CI and
release validation. Cargo aliases are reported by their resolved package names.
Before those graph checks, Toko Tooling's canonical dependency-boundary checker reads
the common external IC inventory together with
`ci/cargo-dependency-boundaries.json` and rejects direct Candid, parser,
certification and enumerated CDK dependencies in every workspace member, including
renamed, dev/build and target-specific declarations. Platform owns those upstream
edges. Unused root catalog declarations and future CDK package names still require
review; the checker covers declared member edges and exact policy names.
Generic Host and Build profiles also reject Wire, Testkit and known higher
product families on transitive normal/build edges. Generic Host excludes Build;
Build and generic Host exclude Platform. Tools retains its explicitly selected
Platform profiles. Testkit's optional Wire and the private performance probe keep
their separately admitted graphs.
`bash scripts/ci/test-tools-layering.sh` exercises the actual policy against
small local Cargo graphs with renamed transitive normal/build edges, including
older published package identities. These are graph fixtures, not downloads or
compilation of those historical packages. The fixture is part of
`make tooling-command-check`.
These checks preserve Tools/Text/inspection ownership boundaries; role comments
are explanatory and are not parsed as an executable exclusion policy.

The test-only Process alias is graph-checked out of the other production owners;
the selected Process root is intentionally excluded from that dependency rule.
There is no canister build or deployment entrypoint in Host. This repository publishes
libraries and native CLIs. It cannot certify a downstream deployment's features,
transitive test-only exclusions or final artifact digest. Deployment owners must
enforce their own exclusions against their locked target/feature graph and bind
successful build evidence to the installed artifact. Toko Tooling's sorter, pin checker
and snapshot verification do not provide that admission. Further test-only catalog
entries require explicit graph exclusions; comments alone do not activate them.
Native macOS and deployment evidence remain distinct from graph checks.

The focused `test-tools-icp-project`, `test-tools-icp-ledger`,
`test-tools-ledger-evidence` and `test-text-yaml` targets exercise each named
feature with default features disabled. `ci` runs them once on the admitted minimum/development compiler. For a focused
check, run `make compiler-check` and the selected target from this checkout.

CI outer job ceilings are 90 minutes for native qualification, 45 minutes for
runtime and optimizer jobs, and 10 minutes for browser fixtures. The status-only compiler gate has a
5-minute ceiling. These allow
headroom over the 0.66.3 run (native Intel macOS about 35 minutes, runtime about
7 minutes, optimizer about 2 minutes); they do not impose library deadlines.
Runtime fixtures live below `runner.temp/portable-fixtures`, with command output
in `portable-regression.log`, so the shared failure collector can retain them.

The main CI workflow uses Toko Tooling's [change classifier](https://github.com/dragginzgame/toko-tooling/blob/5806e6e4943f6be600cd53385498ce08a5f804c4/docs/validation/ci-change-selection.md)
and the exact product prose paths in `ci/docs-only-paths.txt`. Admitted prose
changes run document links under the existing native/MSRV job names, without
compiler setup, Cargo caches, product tests or native qualification receipts.
Their summaries explicitly identify documentation-only validation. Lint/security
still runs. A failed classification fails dependent jobs; unknown, code, mixed,
policy and packaged-document changes retain full qualification on all three
native hosts. Release gates and the separate product workflows remain unchanged.
`bash scripts/ci/test-product-ci-selection.sh` exercises the actual workflow
commands and fail-closed guards in disposable Git history.

`make tools-report` runs every existing offline tool check and retains all failures;
`make tools-check` remains the fail-fast admission gate. Installer `--describe`
prints read-only selection JSON without installing or qualifying executables.
Native CI selects the shared action's Cargo-tool cache with the release-editor
recipe digest; restore is followed by ordinary setup and admission.

Offline Cargo-tool admission returns status 75 for an admitted busy selected
slot. The release adapter preserves that refusal without dispatching setup.
Read-only admission waits are explicit at the shared installer interface; see
[the bounded wait contract](https://github.com/dragginzgame/toko-tooling/blob/5806e6e4943f6be600cd53385498ce08a5f804c4/docs/adoption/local-setup.md#standard-cargo-tool-build-environment).

The optional artifact profile targets run their own Wasm inspection, encoding or
text tests with defaults disabled. Minimal core tests run once; the complete
workspace suite separately covers all-feature composition. Native and Wasm Testkit graphs exclude Metrics. The installed Wire
control-canister check remains in the private development package.

## Configured evidence IO helpers

`ic_host_artifacts::artifact::budget::ReadBudget` admits caller-selected per-stream
and aggregate byte ceilings. The caller supplies metadata length and retains
paths and freshness policy; success charges returned bytes, growth charges the
limit sentinel, and failed IO/allocation conservatively charges the attempted
ceiling plus one. Structured errors distinguish file, aggregate and host-accounting
limits. It does not open or freeze a file.

Host FS capability entries expose `open_regular_file_with_permissions` for explicit
forbidden mode-bit admission on the opened descriptor. This never repairs modes
or follows observed links. `durable::ExclusiveFileLock` owns a nonblocking acquired
descriptor lock with explicit release and best-effort Drop, including duplicated
descriptors; callers retain coordination-file contents and namespace custody.

Host HTTP’s portable `endpoint` feature exposes `endpoint::parse_http_endpoint` admits strict credential-free
HTTP(S) base URLs. It does not select endpoints, redirects, retry or trust roots.
