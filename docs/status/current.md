# Current handoff — 2026-11-13

## Deployment preparing — Toko 0.42.5 and Wire 0.22

The authorized roster is Foundation 0.77.2 and Tools 0.92.1 in one patch batch,
then Testkit 0.103.0 in a minor batch. Each uses the managed checkout under this
repository's `target/release-frozen/`, with durable journals under
`.git/release-state/frozen/`. Reconcile the patch batch before capturing the minor;
the adapter cannot mix increments in one batch. No sibling delivery or cleanup.

Entry source, locks, supplier indexes and archive comparisons are preserved at
`.git/release-state/preserve/deploy0425-20261113.clk428/`.
Freeze the reviewed current locks, including Platform 0.32.2, Types 0.20.2,
Runtime/macros/guard 0.22.2, Wire 0.22.1 and Mio 1.2.5. The compatible patches
were already present at entry; no dependency update belongs in delivery. Their
declared minimums remain the deliberately adopted compatible requirements.

Foundation includes a changed packaged inspector setup README. Tools changes its
normal Platform/Types minimums; Testkit changes the exposed Wire generation.
Build 0.84.1 and Text 0.75.0 remain Git-only maintenance: every non-Cargo/provenance
archive file is byte-identical to publication; normalized manifests differ only
in development tooling/lints, with no new normal/build dependency contract or API.
Compatible supplier projections during release preparation do not force their
publication. Private qualification versions remain 0.96.0.

Fresh registry review finds the selected Platform family, Wire, PocketIC and Mio
current. Toko 0.42.6 is available but all 116 installed payloads match 0.42.5 in
bytes and mode; defer its producer-only fixture update to the next tooling refresh.
Retain the reviewed native catalog and Build 0.73.0 metadata editor; optional
cargo-sort-derives 0.15.0, yq 4.54.3, cloc 2.12 and editor 0.84.1 updates belong
to the next tool-maintenance batch. Cloc's selected source-counting flow does not
expand archives. jq, ripgrep, Binaryen, ShellCheck, Actionlint and Gitleaks match
their current releases. Required gates and final archive admission remain intact.
Main is current and unprotected; planned registry versions are unoccupied.

The first frozen-hook check retained `target/tmp/frozen-release.cwzRRh/`. Its
synthetic Testkit patch heading competed with real pending 0.103.0, and admission
correctly refused. Fixture copies now receive one independent synthetic ledger;
the real ledgers/history are untouched. No public release or upload occurred.
Rerun the frozen-hook check from corrected source; do not mutate its old capture.

## Toko Tooling 0.42.5 — local refresh

Both workspace pins and locks select published 0.42.5. The only installed payload
change anchors fixture-retention helpers at their installed location while
generated probes retain the consumer root. Native tool pins, the baseline,
entrypoints, hooks and product release requirements stay unchanged. Preserve
the pending vendor-layout, Wire/Platform and other local work.

Evidence and entry source/locks are retained under
`.git/release-state/preserve/toko0425-adoption.QStckC/`.
Authenticated installation, both-workspace snapshot selection, existing pinned-tool
admission, dependency pins, formatting and 665 local document links pass. Product
retention checks pass for premature completion, success and retained failure
through aliased scratch/cwd. A focused real consumer probe also verifies installed
helper lookup after changing cwd while retaining the distinct consumer root and
failure evidence. No producer-wide matrix or product rebuild was needed.
Both locks preserve every non-tooling entry block byte-for-byte; entrypoints and
hooks are unchanged. Native macOS and full release qualification were not run.
No commit, push, publication or sibling change is included.

## Toko Tooling 0.42.4 — local refresh

Both workspace pins and locks select published 0.42.4. Three installed payloads
change: vendor-aware release-command substitution, Cargo-tool setup maintenance
recovery, and changed-tree diagnostics. The baseline, native tool pins, entrypoint
roster, hook configuration and product release requirements stay unchanged.
Pending Wire/Platform and vendor-migration work is preserved.

Evidence and entry source/locks are retained under
`.git/release-state/preserve/toko0424-adoption.nylSKP/`.
Authenticated installation and normal tool setup/admission pass, reusing existing
selected tools. Both-workspace snapshot admission, dependency pins, formatting
and 665 local document links pass. Product frozen/captured release routing and
delivery refusal pass with runner effects substituted. Both locks preserve every
non-tooling entry block byte-for-byte; entrypoints and hooks are unchanged.
Producer maintenance-failure matrices are not replayed against the live tool store.
Native macOS and full product release qualification were not run.
No commit, push, publication or sibling change is included.

## Toko Tooling 0.42.3 — local refresh

The requested crate is now published. Both workspaces select 0.42.3; its only
installed payload change clarifies domain-library versus canister-endpoint
ownership in the baseline. Existing Testkit codecs remain transport helpers,
while the private Wasm probe owns its test endpoints. No Rust or caller migration
is needed. Preserve the pending vendor-layout, Wire and Platform work.

Evidence and resumed entry source/locks are retained under
`.git/release-state/preserve/toko0423-adoption.PdH6Zq/`.
Snapshot installation, both-workspace selection and existing pinned-tool admission
pass. Dependency pins, formatting and 665 local document links pass. Every other
installed payload hash/mode, all entrypoints and the hook configuration are
unchanged; both locks preserve every non-tooling entry block byte-for-byte.
Reused the 0.42.1/0.42.2 caller evidence because no executable changed. No product
rebuild, runtime suite, native macOS qualification or full release gate was run.
No commit, push, publication or sibling change is included.

## Toko Tooling 0.42.2 — local refresh

Both workspace pins/locks select published Toko 0.42.2. The authenticated preview
changes only the installed package-admission checker; the vendor layout,
entrypoint roster, native tool pins and product release requirements are unchanged.
Pending Wire/Platform adoption and all other local work are preserved.

Evidence and entry source/locks are retained under
`.git/release-state/preserve/toko0422-adoption.76ydqT/`.
Installation and pinned-tool admission pass. Both-workspace snapshot selection,
repository-file admission, dependency pins, formatting and 665 local document
links pass. The focused product source/archive acceptance and rejection fixture
passes with Cargo/publication effects substituted. The unchanged vendor wiring
retains its 0.42.1 caller evidence; no unrelated fixture or full product suite was
replayed. All 116 installed files are admitted; entrypoints and hook configuration
are unchanged. Both locks preserve every non-tooling entry block byte-for-byte.
Native macOS qualification was not run. No commit, push, publication or sibling
change is included.

## Toko Tooling 0.42.1 — vendor migration

Both workspaces select the published 0.42.1 crate. The authenticated exporter
installed 116 managed files with the vendor layout, retaining root canonical
files and the root snapshot receipt. The old managed executables/catalogs retired
through source-ownership proof; product scripts, policy and workflows stay local.
Release/publication entrypoints and shared actions are explicit selections.
Make, script, action, fixture and documentation callers now use the vendor paths;
there are no compatibility wrappers. Historical reports and release evidence keep
their original paths. Pending Wire 0.22.0 and Platform family selections remain.

Evidence, entry source and both locks are preserved at
`.git/release-state/preserve/toko0421-adoption.4NCBXc/`.
Standard tools are prepared/admitted, and the installed hook selects
`vendor/toko-tooling/.githooks`. Snapshot admission, package inventory, formatting,
local doc links, shell/workflow lint and the installed updater's unchanged preview
pass. Product-owned Make/CI routing, hook preservation, Candid admission, scratch,
evidence, retention and reuse checks pass. Vendor inputs now invalidate reuse;
the CI runner has an explicit consumer root. Local EXIT tests cover premature
completion, success and retained failure through aliased scratch/cwd; the generic
lifecycle matrix stays upstream. The focused Build release adapter and publication
admission checks pass with product/setup/delivery effects substituted. Group plan,
full-roster, stage-failure, source-drift and receipt-recovery fixtures pass with
Make effects substituted. Native macOS and complete product release qualification
were not run. Both locks preserve all non-tooling entry blocks byte-for-byte.
New vendor paths are intent-to-add for source-enumerating checks; no file contents
are staged. No commit, push, publication or sibling change is included.

## Platform and Wire — local adoption

Both roots select Platform 0.32.1, Types 0.20.1 and Wire 0.22.0. The private probe
selects Runtime/macros/guard 0.22.1. Registry metadata and authenticated published
archives establish these current non-yanked versions. Platform patches were
already resolved at task entry; the manifest minimums now describe that adoption.
Targeted Cargo updates changed only Wire in the entry locks. Preserve the local
Toko 0.41.1 adoption and unrelated dependency selections.

Wire's minor requires caller alignment for shared trait/error identities; codec
bytes, limits and payload admission remain unchanged. Platform's patch releases
preserve the existing DTO/codec surfaces. PocketIC remains the external native
16.1 client; optional Wire Types integration remains disabled.
Pending Tools 0.92.1 and Testkit 0.103.0 ledgers do not change package versions.

Evidence and entry source/lock preservation are under
`.git/release-state/preserve/platform-wire-adoption.r7HEQw/`.
The focused Linux `dependency-adoption-check` and `testkit-wasm-check` pass:
isolated native/portable graphs, Host Tools codec/Agent/Registry/ledger adapters,
snapshot DTO/refusal and Wire-only helper tests, installed PocketIC stable-memory
restore, and the private Wire canister's state/typed-failure test. Strict Clippy
passes for both affected public libraries with all features and for the private
probe on Wasm. Pin checks, formatting and 665 local documentation links pass.
Both entry locks preserve every non-Wire package block byte-for-byte. No caller
source change or persisted-state migration was needed. Full release qualification
and native macOS were not run.

The previous ordinary `target/` tree, including Toko 0.41.1 raw logs, was removed externally before this task;
source edits and the durable release records survived. No active build using
this repository's target was observed before adoption. No delivery is included.

## Toko Tooling 0.41.1 — local adoption

Both roots now select published Toko 0.41.1. The authenticated snapshot retains
all recorded entrypoints and changes only the baseline, package-inventory labels
and tooling-selection verifier. Native tool pins and product release requirements
remain unchanged. Both lockfiles update only the tooling package.

The binary-only `ic-testkit-dev` package uses the newly admitted
`target.'cfg(any())'.dev-dependencies` source marker, inherited from the same root
pin with its existing cargo-machete exemption. Library packages retain their
unconditional dev-dependencies; no dummy import or lint allowance was added.
Bootstrap used the released installer located through locked Cargo metadata.

Adoption evidence is under `target/toko0411.gXudiy/`. Standard tool setup/admission,
115-file snapshot and both-workspace selection checks, eleven-package inventory,
pin/inheritance checks, all five group graph admissions, formatting and 665 local
documentation links pass. The private `check_packages` binary's unit-test target
compiles with Cargo warnings denied; no package-qualification execution was run.
Both locks preserve unrelated package blocks byte-for-byte and the entrypoint
roster is unchanged. Native macOS and full release qualification were not run.
No commit, push, tag, publication or sibling change is included.
Foundation's pending 0.77.2 notes do not change package versions.

## Toko Tooling 0.41.0 — deployment delivered

Both roots select published Toko 0.41.0; the authenticated 115-file snapshot and
all recorded entrypoints remain. `make deps-list` reports all eleven packages
offline. Five unused product catalog aliases and their group dependency aliases
were removed; all nine product group members remain. Tooling adoption changed only
Toko lock blocks; deployment also includes the reviewed CC patch. Standard setup admitted the
new installer-policy receipts while retaining old tool generations.

Both roots now deny unused Cargo dependencies/workspace metadata, missing lint
inheritance and unknown Cargo lints, plus invalid Rustdoc tables. Removed unused
metadata and Testkit's redundant homepage; no dummy imports or lint allowances.
Both-workspace all-target/all-feature Clippy with warnings denied, strict Rustdoc,
portable Testkit/probe Wasm Clippy, snapshot/tool admission, pin checks and all
five release-group graph admissions passed before capture.
The Build adapter's real metadata/rendering/receipt checks pass with an empty
alias roster; its setup/product gates were substituted. Native macOS is unrun.

The [publishing guide](../publishing.md) records independent native test suppliers
and optional-upgrade deferrals. Required gates and frozen admission remain.
Rust 1.100's `DropGuard` replaces the test-only
produced-value destructor; `Path::trim_prefix` preserves optional manifest prefixes
and `Vec::from_fn` constructs indexed package fixtures.
[#355](https://github.com/dragginzgame/ic-tooling/issues/355) has Linux qualification
for clean/warning, rendered/plain JSON, cold/cached artifacts and compiler denial.
Native plain JSON preserves stderr ownership and compiler guards. Keep #355 open
for native macOS acceptance. The complete Linux release gates also passed these
fixtures from source and extracted archives.

Frozen `batch-4szjd3du` completed from corrected source `d22fbd8f`. All six packages
are published and non-yanked. Downloaded registry archives match index checksums
and exact group release commits; remote main and annotated tags were verified.

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Foundation (five packages) | 0.77.1 | `698aab14b0618c06df10692a21952f413fda3502` | `foundation-v0.77.1` |
| Build | 0.84.1 | `4ea33bbfedc0f9a8e57152b93325fc834e3f1890` | `build-v0.84.1` |

The ordinary checkout reconciled cleanly to `4ea33bbf`; this handoff is subsequent
Git-only documentation. Both lockfiles preserve unrelated package blocks
byte-for-byte against entry copies, including reviewed CC 1.11.2 and Toko 0.41.0.
Normalized Text/Tools/Testkit payload review shows
only development tooling/lints and homepage metadata; deliver this maintenance
through Git and retain Text 0.75.0, Tools 0.92.0 and Testkit 0.102.0.
Private qualification packages remain 0.96.0; no sibling source or release was
included. No artifact or historical checkout was retired.

Both complete 32-stage Linux gates passed, including strict lint/docs, native
and Wasm feature boundaries, installed PocketIC, extracted archives and installed
workspace tests. Rounded stage sums were 668/510 seconds, including 274/256
seconds of tooling fixtures. Only the four unchanged invariant fixture proofs
were reused across groups; both publications admitted their full lint receipts
and freshly checked source/archive custody. These are gate times, not total
deployment latency or controlled speedup measurements.

Explicit qualification is asynchronous: [CI](https://github.com/dragginzgame/ic-tooling/actions/runs/43434488465),
[Build optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/43434492809),
[Testkit runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/43434497041) and
[Browser fixture session](https://github.com/dragginzgame/ic-tooling/actions/runs/43434501685).
All four returned main at exact release `4ea33bbf`; they were queued/in progress
at handoff. Their independent dispatch identities survive the subsequent ordinary
documentation push. No fresh native macOS 15 Intel or Apple Silicon pass is
claimed. Keep #355 and #354 open for their native macOS acceptance.

Retain entry source/locks and final logs, timings, registry archives/readback,
receipts and reconciliation evidence at
`.git/release-state/preserve/deploy041-20261113.jBRdpw/`.
The completed journal/Git remain under `.git/release-state/frozen/batch-4szjd3du/`;
its managed checkout and current raw evidence are under `target/release-frozen/`
and `target/deploy041.q8E0gh/`.

During the first attempt the entire ordinary `target/` tree disappeared through
an external action whose cause is not established. Earlier `target/` evidence
paths mentioned below are historical and no longer available; durable copies
under `.git/release-state/preserve/` remain. The original focused Toko/Rust 1.100
logs were lost; the corrected batch's full gate logs are retained.
`batch-K6l0kMh1` recovered its checkout after external target loss, then stopped
after seven passing stages at a formatting fixture selecting the retired HTTP
alias. No release was prepared. Retain its journal/logs; the corrected fixture
selects retained FS/Process aliases and passed before the new source capture.

## Five-group deployment delivered

Frozen `batch-_HKWam5i` completed from reviewed source `918fd7bf`. All nine public
packages are published and non-yanked; downloaded registry archives match Cargo
index checksums and their exact group release commits. Remote main and annotated
tags were verified. The ordinary checkout reconciled cleanly to `bdb84eba`.

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Foundation (five packages) | 0.77.0 | `f366520eb6830d049c0622adec8e68ba7d329ced` | `foundation-v0.77.0` |
| Build | 0.84.0 | `a86d688ad39d69ca68843b64f7e2967e5352d673` | `build-v0.84.0` |
| Text | 0.75.0 | `42956d02b58d0ff5d1b9a68d34729e80cb5a7f39` | `text-v0.75.0` |
| Host Tools | 0.92.0 | `1c32f4c0df1b78aec118bc25cab871543d6f03ce` | `tools-v0.92.0` |
| Testkit | 0.102.0 | `bdb84eba0a382d65d87f6bbda5c6886df34ff2a7` | `testkit-v0.102.0` |

Private qualification packages remain 0.96.0. No sibling source or release was
included. No artifact or historical checkout was retired.

Retain `.git/release-state/frozen/batch-_HKWam5i/`, its managed checkout under
`target/release-frozen/`, and `target/deploy-minors-20261113.4mmynn/`.
`.git/release-state/preserve/deploy-20261113-minors.XUnm1U/` holds full entry
source/locks, untracked files, final logs and registry readback evidence. Both
lockfiles' non-product package blocks remain byte-identical to those entry copies;
release preparation changed only owned product identities. Do not reapply old
preservation locks over these supplier selections.

## Qualification and deployment cost

All five complete 32-stage Linux gates passed: strict Clippy/Rustdoc, native and
isolated feature profiles, Wasm, unsupported-filesystem compilation, installed
PocketIC, extracted archives and installed workspace tests. Every publication
reused its qualified full lint receipt and freshly checked source/archive custody.
Rounded gate totals were 610/467/451/439/426 seconds in supplier order, including
253/246/254/262/247 seconds of tooling fixtures. Later groups reused the four
unchanged invariant fixture stages. These are gate times, not total deployment
latency or controlled speedup measurements; preparation/Git/upload/index phases
lack separate timings. See [the observations](../design/release-validation.md#deployment-cost-observations).

[#353 — Reduce tooling-command qualification cost in deployments and avoid duplicate same-source CI](https://github.com/dragginzgame/ic-tooling/issues/353)
remains open for missing phase evidence and the documented collector-recovery
boundary. No collector-only recovery or whole-gate reuse was added.

First capture `batch-j5L1wRS7` stopped at archive qualification after 22 passing
stages. A Build fixture read Cargo's private build-script stdout path, changed
in 1.100. It created no prepared version, tag or upload. The fix in `918fd7bf`
compares cached JSON exports with a fixture-owned `OUT_DIR` marker; package IDs,
cached exports and custody remain checked. The exact regression and strict Build
Clippy passed before recapture; all subsequent archive gates passed. Retain the
failed journal, checkout and logs without relabelling them.

[CI on the final release](https://github.com/dragginzgame/ic-tooling/actions/runs/43418366917)
is asynchronous. Fresh native macOS 15 Intel and Apple Silicon qualification is
not claimed by the Linux evidence. Keep
[#354 — Adopt Rust 1.100.0 and the common lint policy across maintained workspaces](https://github.com/dragginzgame/ic-tooling/issues/354)
open until its native macOS acceptance is demonstrated; do not infer passes.

## Shipped compiler, tooling and suppliers

The preceding five-group release selected Rust 1.100.0 and Toko Tooling 0.40.1; all eleven
members inherit the owning compiler/lint policy. All 115 managed paths and
entrypoints remain. Cargo tools/editor are admitted for Rust 1.100.0, retaining
older generations. Safe roots forbid unsafe code; reviewed FFI and the pool's
Send contract keep their narrow scopes. Directory modules preserve public paths.
Earlier compiler, lint-consumer and tooling-rule proofs remain in
`target/toko040-rust1100.2MC9BA/` and `target/toko0401.XXEGwz/`.

Both roots select Platform 0.32.0 and Types 0.20.0; the private probe selects
Runtime/macros/guard 0.22.0. Wire resolves to 0.21.5 with optional Types integration
disabled. Native PocketIC remains upstream 16.1. Tools/Testkit minors cover exposed
Platform/Principal identities. Focused adoption evidence remains under
`target/platform-adoption.2onG3b/`; the final full gates cover the released graphs.

The Foundation ledger records bounded tool deferrals: keep Toko's reviewed
cargo-sort-derives 0.14.1, yq 4.47.2 and cloc 2.10 catalog, plus the admitted Build
0.73.0 release editor, for this batch. Revisit newer suppliers in tool maintenance.
Cloc 2.12 fixes archive expansion security; the selected LOC flow counts source
files without expanding archives. Do not hand-edit managed tool catalogs.

## Consumer acceptance and open work

[#352 — Compiler/wrapper identity or complete refusal facts for build reuse](https://github.com/dragginzgame/ic-tooling/issues/352)
is shipped in Build 0.84.0. `ConfigurationFile.compiler_overrides` now contains
`CompilerOverride::Build`/`Environment` facts, preserving empty, forced, relative
and shadowed declarations without values or a precedence/executable resolver.
Consumers must migrate vectors/matches and independently bind compiler identity
or refuse reuse. Canic owns adoption in
[Canic #247 — Bind effective Cargo-configured wrappers in build reuse evidence](https://github.com/dragginzgame/canic/issues/247).

[#263 — Cached snapshots and timer state](https://github.com/dragginzgame/ic-tooling/issues/263)
remains open for Canic's exact two-case acceptance of the correction first shipped
in Testkit 0.101.0. Native snapshots preserve timer/hook, globals, heap/stable,
Wasm, certified data and chunks through upstream PocketIC and Platform codecs.
Two slots per target and transfer cycles are required; stop/drain/start, sender,
funding and partial-failure recovery remain caller-owned. No client fork, timer
reset, second transport or time rewind. Installed regressions pass on Linux.
[Timers #87 — Snapshot regression qualification](https://github.com/dragginzgame/ic-timers/issues/87)
tracks separate coverage; no sibling edits or qualification are included here.

Next independent fixes remain outside this deployment:

1. Preserve exact paths in [Build tool probes](../../crates/ic-build/src/tools/probe/mod.rs)
   and check raw directory names before archive normalization.
2. Propagate filesystem errors from [Cargo configuration selection](../../crates/ic-build/src/cargo/environment/mod.rs).
3. Reject malformed successful [process observations](../../crates/ic-host-process/src/testing/mod.rs).

Canic's separate Build migration is
[Canic #762 — Explicit maintenance and pruning](https://github.com/dragginzgame/canic/issues/762).

## Retained historical evidence and contracts

The [pre-deployment handoff](https://github.com/dragginzgame/ic-tooling/blob/918fd7bffc93b51fabb573a18235085bff040978/docs/status/current.md)
records prior delivered Foundation 0.76.5 (`batch-eTIfLgJR`) and Testkit 0.101.0
(`batch-Aps5IIGp`), failed attempts, preservation refs and exact recovery paths.
Retain their original journals/adapters; never relabel old receipts as current.
Existing `preserve/*` refs and `.git/release-state/preserve/` remain intact.
The retired client archive is `/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`;
original source is `0c17900090f0c2d2ebf3182598b49e651223845e`.

Keep two-pass generated-root discovery, Darwin `File::sync_all` durability and
discovery error downcast identity. Retain the snapshot cleanup callback's explicit
future type; attempted `AsyncFnMut` alternatives failed its Send lifetime contract.
