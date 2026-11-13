# Tooling release validation

Release groups retain complete qualification of their own contracts. Tooling's
[adapter](../../scripts/release/adapter.sh) uses the existing Shared
[plan runner](https://github.com/dragginzgame/toko-tooling/blob/5806e6e4943f6be600cd53385498ce08a5f804c4/docs/releases/release-group-validation.md), without another release executor
or metadata renderer. Repository `make ci`, hosted `make ci-publication` and
publication gates retain their complete workspace coverage.

## Selection and conservative rollout

The [policy](../../ci/release-validation.json) accounts for all 35 targets in the
existing full CI gate and six focused alternatives. The
[coverage map](../../ci/release-validation-coverage.json) owns common checks,
explicit exclusion contracts, focused alternatives and full-gate fallback reasons.
The shared roster interface admits policy/group consistency; Tooling's thin
projection checks complete product coverage before emitting any stage.
Both full CI entrypoints and release selection consume this one ordered inventory;
Make recipes no longer duplicate its check list or supply text for a parser.
New, omitted, duplicate or unmapped stages refuse until coverage is explicit.
Fresh discovery and saved-baseline recovery share one
[baseline admission rule](../../ci/release-validation-baseline.jq).

| Group | Release qualification |
|---|---|
| Foundation | Full gate: primitive feature profiles, Build/Tools/Testkit reverse consumers, native/Wasm graphs and installed archives. |
| Build | Focused only when admitted below: artifact/cache and release-editor tests, normal/CLI graphs, private driver/archive checks, real Wasm-probe consumer and exact Build archive. Hosted optimizer qualification remains an independent CI obligation. Otherwise full gate. |
| Text | Focused only when admitted below: default/all-feature/YAML rendering, dependency graphs, Tools' `icp-project` consumer, private driver/report tests and installed Text archive. Otherwise full gate. |
| Tools | Focused only when admitted below: every isolated IC adapter profile, external identities, the private public-contract consumer, native/Wasm graphs and exact Tools archive. Otherwise full gate. |
| Testkit | Full gate: simulator/server lifecycle, public identities, feature boundaries, Wasm/probe singleton graphs and installed archives. |

Focused admission requires a completed, successful main-branch CI run at an
ancestor of the selected source. Each of Linux x86-64, macOS 15 Intel and macOS
15 Apple Silicon must have actually executed native CI/publication qualification
and retained successful qualification. A green docs-only run, incomplete matrix,
failed job, missing compiler lane or truncated response does not qualify.
All three jobs must also execute every group listed by `validation-roster.sh groups`
and retain their
matched-input comparison. Earlier full-CI runs without those proofs keep the
full release gate. Routine native CI runs the existing complete gate once.
Explicit qualification dispatches additionally run the Text, Tools and Build focused profiles on that
same source, lock, compiler and host. Their comparison artifact retains full-gate
elapsed time, focused commands/logs/timings and exact
installed-archive evidence; cache targets are excluded.
Discovery reads at most ten recent successful explicit dispatches once during
fresh preflight; it does not poll or wait for CI. Offline mode, unavailable GitHub access or absent
qualified ancestry selects the full gate. Raw discovery observations are retained.
Only main-branch `workflow_dispatch` qualification is discovered; routine pushes,
PRs, other branches/workflows/repositories and incomplete jobs remain ineligible.
Filtering discovery to qualification prevents routine pushes from crowding an
otherwise useful comparison out of the bounded search. Saved, previously qualified
push baselines retain their existing source/host/step and recovery checks.
Renew the comparison when its mapping, runner, relevant configuration or covered
contracts change, rather than for every push.

Routine workflow/ref runs retain newest-revision cancellation. Renew an explicit
comparison when its covered inputs change, not after every deployment. First
inspect existing explicit runs for the selected revision:

```sh
gh run list --workflow ci.yml --event workflow_dispatch --commit "$(git rev-parse HEAD)" \
  --limit 10 --json databaseId,headSha,status,conclusion,url
```

Record an existing in-progress matching qualification instead of dispatching it
again; continue delivery without waiting. Check the other selected workflows the
same way. A routine push runs full native CI but omits the focused comparisons,
so it cannot replace an explicit comparison. Retrying failed jobs in an existing
qualification preserves its identity; don't start another complete run merely
because a native setup failed. Keep explicit qualification's independent
concurrency: cancelling it in favour of routine CI would lose required evidence.

When a new comparison is needed, dispatch the existing four workflows on main
at one selected revision:

```sh
gh workflow run ci.yml --ref main
gh workflow run build-optimizer.yml --ref main
gh workflow run testkit-runtime.yml --ref main
gh workflow run browser-session.yml --ref main
```

Explicit dispatches have their own run identity and do not cancel or get cancelled
by routine pushes. CI dispatch always selects the complete gate, even for prose;
the required hosts, focused comparison, compiler status and archive custody are
unchanged. Record and verify the actual returned run SHAs: branch dispatch reads
main at each request, so mixed-source runs do not form one qualification record.
This is asynchronous qualification, not a release/publication command. A qualified
ancestor remains useful after a later push; current source changes still broaden
the release plan as below. Do not require a moving branch tip to stay unchanged
solely to recognize completed issue evidence.

Relative to that baseline, narrowing allows only the selected package's source,
tests and README, group ledgers/handoff, and release-owned version-only manifest
or lock changes. Changes to shared scripts, dependency selections, feature
contracts, compiler, public dependency identities or other package source select
the full gate. Unknown Cargo configuration/overrides also retain the full gate.
Text's reverse
owners must remain Tools and Dev, with runtime callsites confined to the checked
`icp-project` and archive diagnostics. New consumers require a
reviewed mapping. This is explicit contract admission, not automatic dependency
inference or an operator-controlled skip switch.

Tools has only Dev's development edge, with Rust callers confined to
`tests/public_contracts.rs`. Build has only Dev's normal edge; its
archive-qualification binary/tests, public contract and installed Wire test are
the admitted call surfaces. A new consumer, dependency kind or Rust call surface
selects the full gate. Per-group exclusion annotations explain every omitted
full stage; Tools/Build do not inherit Text's unrelated exclusion claims.
The new plans cannot use historical comparisons with a different group/coverage
mapping: dispatcher and workflow changes force full validation until a new three-host
comparison includes all three profiles. Mixed-group changes and Shared Tooling
refreshes still select the full gate.

Every focused plan retains snapshot/symlink/compiler/pin/format/document checks.
Tool admission includes host parsers, Cargo tools and the release editor, without
installing the simulator or optimizer for Text and Tools. Build retains
the complete product-tool admission for its real Wasm-probe consumer and existing
optimizer contracts. Preflight retains locked workspace fetch
and metadata admission. Focused lint precedes runtime and archive execution.
Focused archives use the same IC Build engine as the full public roster,
including normalized source/lock/provenance checks and extracted-package tests.
The archive set includes the selected packages' local normal, build and development
dependency closure, including aliases and optional/target edges. Only path-only
private development helpers omitted by Cargo packaging are excluded; other
private or outside-workspace path dependencies refuse. Dependency archives are
admitted and rebuilt, while runtime/doctest execution selects only the requested
packages. Without an explicit selection the driver retains whole-workspace tests.
The private archive driver still compiles its normal dependencies, including
Build and the client; focused selection excludes unrelated test/server execution,
not every possible dependency compilation. No archive stage is reused. Tooling's
archive driver may retain registry compiler outputs in its owned cache, but Cargo
cleans every extracted workspace package before fresh all-target/doctest execution.

## Source custody, receipts and recovery

Every target checks the complete checkout against the admitted Git source before
and after execution, permitting only the selected pending ledger. Shared also
checkpoints the plan, policy, recipes, lock, configuration and ledger bytes around
every stage. A required failure or source/input drift stops later stages. Omitted
stages retain explicit `not-applicable` events and their coverage reasons; they
are never reported as executed passes.

Version-5 release receipts include the canonical Shared run proof and bind the
source/group, plan and policy, environment, host/compiler observations and complete
gate inventory. Shared verifies the original inputs, ordered stage events,
timings, checkpoints and logs before the proof is saved and again before the
release receipt is sealed. Tooling retains only its gate pointer, product
selection and frozen receipt inventory; it does not parse generic events/timings.
Missing, failed, blocked, redirected or altered evidence cannot authorize release effects.

After version preparation, the original source checkpoint intentionally differs
from current manifests, lock and finalized ledger. Recovery compares the sealed
proof and every retained artifact byte against the original receipt, then checks
the exact prepared/committed payload and allowed source changes. It never creates
a replacement live proof from prepared versions. The private `retained-receipt`
operation only generates that frozen inventory for comparison with an existing
trusted receipt; it cannot authorize initial sealing or grant stage reuse.
Selection alone grants no reuse. Metadata preparation, unrelated-group preservation,
commit/payload custody, Git delivery and publication retain their existing checks.

Gate artifacts live in unique `.git/release-state/<group>-gate.*` directories.
Linked worktrees use `target/release-validation/<group>-gate.*` because Shared's
checkpoint paths must remain checkout-relative. Preserve those directories with
the release receipts; deleting artifacts invalidates evidence rather than granting
an implicit pass. Exact-version recovery retains its saved baseline and re-derives
the same selection without a new remote lookup. Fresh verification creates new
logs while preserving earlier attempts. Changed source/scope/configuration needs
a fresh admitted attempt. Historical version-2/3/4 receipts use their original
source and adapter; they are never relabelled as version 5.

### Collector-failure boundary

Tooling's `validation.sh run` executes a selected plan and then seals its Shared
proof. It is not a post-processing-only resume operation. `receipt` checks an
already complete proof; `retained-receipt` produces an inventory for comparison
with a trusted receipt.
The adapter does not currently expose a qualified operation to finish a failed
collector against an earlier successful run. Direct verifier invocation or a
PASS line cannot substitute for that missing admission.

Use the shared [failure-specific recovery decision](https://github.com/dragginzgame/toko-tooling/blob/dcc51b5cab3d9993f0668bc0902252364d7dc316/docs/releases/release-validation-reuse.md#failure-specific-recovery)
with the original gate, logs and receipts. Identify successful execution separately
from proof-publication failure. If the saved state cannot be admitted by an existing
recovery command, disclose that limitation and the selected rerun scope before a
new gate. Only the four qualified fixture stages and the separately admitted full
publication lint have the reuse contracts described below; native runtime results
cannot be reconstructed as new stage proofs. This limitation remains in #353.

## Selected adapter cases and unchanged-stage reuse

During release qualification, `tooling-command-check` runs the adapter fixture
with `--group` for the selected release group. Ordinary CI and CI/publication
continue to exercise all five groups, even when a release selection is inherited.
The remaining tooling roster and its first-failure behavior are unchanged.

The fixture-retention, product-evidence, workflow-selection and scratch-routing
stages use Shared's
[stage runner](https://github.com/dragginzgame/toko-tooling/blob/5806e6e4943f6be600cd53385498ce08a5f804c4/docs/releases/release-validation-reuse.md). It observes every shell source
under `scripts/`, the tooling roster, its observer, interpreter modules and the
resolved fixture tools. The workflow/scratch matrices additionally observe their
Make, CI, workflow, rule and companion-document trees, root toolchain/snapshot,
and the prepared workflow parser. Execution has an explicit environment containing PATH,
a fixed workspace temporary root, stage identity, parser selection and the C locale. Retention
renders only initialization/EXIT prefixes; it stops before Cargo, Git and product
bodies. Evidence qualification archives and checks its synthetic inputs, including
compiler-cache exclusions. Workflow/scratch matrices exercise their real routing
with synthetic product effects. The tooling roster owns all four stage selections.
Manifests, lockfiles and release ledgers are outside these invariant stages' input
closures. Consequently version-only changes can reuse them across release groups.
The adapter's group-specific metadata checks and the metadata-dependent release
and publication fixtures continue to run freshly; no version projection or
canonicalized manifest substitutes for current source admission.
Changes to an observed source, tool, environment, runner or host invalidate reuse.

Each gate records a new source/group admission with mode `executed` or `reused`.
Reuse retains the original execution source, group, proof and stdout/stderr;
Tooling seals those original bytes into its recovery receipt. Evidence lives
under `target/release-stage-cache`; deleting it invalidates dependent receipts.
Changed or missing proof/log bytes cannot be sealed or recovered as a pass.
Fixture replay grants no Rust, dependency, runtime or archive qualification.
Publication separately re-admits an existing full workspace lint through the
[committed release receipt](../publishing.md#registry-publication), with fresh
lint fallback. There is no whole-gate reuse or reinterpretation of older receipts.

The adapter fixture uses one actual invariant execution proof in each disposable
compact group. It retains real metadata, source/receipt sealing, prepared payload,
lint admission and corruption/recovery checks. Its script copy uses the existing
release-fixture preparer. `test-stage-reuse.sh` independently executes all four
production invariant stages across groups, including changed inputs/environment,
missing original logs and occupied destinations; `test-validation.sh` retains the
full roster and receipt inventory. No production stage is removed or synthesized.

### Deployment cost observations

[#353 — Reduce tooling-command qualification cost in deployments and avoid duplicate same-source CI](https://github.com/dragginzgame/ic-tooling/issues/353)
owns remaining deployment measurements. Normal tooling output now records every
fixture's route, fallback reason, status and elapsed seconds. A `stage-admission`
route is an attempt to admit proof, not a reuse claim: the resulting admission
separately reports `executed` or `reused` and its monotonic duration. A cold or
ineligible proof reports `no-admissible-matching-proof`; the Shared runner retains
individual invalid-proof diagnostics. Publication surfaces the lint admission's
refusal before its existing fresh-lint fallback. These messages grant no admission.

Foundation 0.76.5's authorized frozen `batch-eTIfLgJR` retains its validation log
under `.git/release-state/frozen/batch-eTIfLgJR/repository.git/release-state/`.
Its 32 rounded Linux stage durations sum to 682 seconds: tooling commands took
265, package qualification 106 and installed-workspace tests 105. All four
invariant fixtures executed. This is gate time, not total deployment latency;
the retained log cannot separately establish preparation, Git delivery, upload
and registry-availability time. Do not infer those times from file modification
timestamps or claim a cold checkout can reuse another checkout's proof.

Separately, [push CI at 2fc42c74](https://github.com/dragginzgame/ic-tooling/actions/runs/43202983937)
records 486 seconds for Linux tooling commands and 1,252 for Intel. Log boundaries
attribute approximately 310/728 seconds to the adapter and 95/278 to validation
fixtures. The [explicit run at the same source](https://github.com/dragginzgame/ic-tooling/actions/runs/43203026490)
also adds the focused comparison; it is not semantically identical to push CI.
Its Apple Silicon setup failed before qualification, so it supplies no complete
three-host comparison. Hosted timings do not substitute for deployment timings.

The focused Linux Foundation adapter fixture passed at 114.247 seconds before
and 108.110 after the fixture reduction, with product/setup effects substituted.
That is a modest local fixture improvement, not an end-to-end release or macOS
speedup.

The 2026-11-13 five-group deployment completed as frozen `batch-_HKWam5i`, from
source `918fd7bf` through release `bdb84eba`. All five selected complete gates:

| Group | Passed stages | Sum of rounded stage seconds | Tooling seconds |
| --- | --- | --- | --- |
| Foundation 0.77.0 | 32 | 610 | 253 |
| Build 0.84.0 | 32 | 467 | 246 |
| Text 0.75.0 | 32 | 451 | 254 |
| Tools 0.92.0 | 32 | 439 | 262 |
| Testkit 0.102.0 | 32 | 426 | 247 |

Foundation executed the four invariant fixture stages; subsequent groups reused
them through matching admissions. Every publication admitted its own full-workspace
lint receipt and freshly checked source/archive provenance. These observations
are gate durations, not total deployment time or a controlled speedup comparison:
compiler, source and cache warmth differ. The normal log does not separately time
preparation, Git delivery, uploads and registry availability; #353 remains open
for that missing phase evidence and its separate collector-recovery boundary.
Retain `target/deploy-minors-20261113.4mmynn/` and its durable log/readback copies
under `.git/release-state/preserve/deploy-20261113-minors.XUnm1U/`.

The subsequent Toko 0.41.0 patch batch `batch-4szjd3du` completed from corrected
source `d22fbd8f` through release `4ea33bbf`. Foundation 0.77.1 passed all 32 stages
in 668 rounded stage seconds (274 tooling); Build 0.84.1 passed all 32 in 510
(256 tooling). Foundation executed the four invariant fixtures; Build admitted
their unchanged proofs. Both publications reused their qualified lint receipts
with fresh source/archive admission. These remain gate-only observations; they
do not establish preparation, Git, upload or registry-availability durations.
Final logs, timings, registry readback and reconciliation evidence remain under
`.git/release-state/preserve/deploy041-20261113.jBRdpw/deployment-records/`.

The first attempt lost its entire ordinary `target/` tree externally, including
focused development logs; the cause is not established. Durable journals and
preservation records survived. Recovered `batch-K6l0kMh1` then stopped after seven
passes because the formatting fixture selected a retired workspace alias. It
prepared no version, tag or upload. The fixture now perturbs retained FS/Process
aliases; its focused check passed before a new capture. The corrected batch ran
both complete gates. No source-changing receipt reuse or collector recovery was
claimed, and the failed attempt's journal/logs remain preserved.

Earlier `batch-j5L1wRS7` stopped at the 23rd stage after 22 passes: a Build fixture
read Cargo's private build-script stdout path, changed in Cargo 1.100. No version,
tag or upload was prepared. The corrected fixture compares cached JSON exports
with its own `OUT_DIR` marker. Only that regression and affected Clippy ran before
the new capture; changed source/checkout identity required new gate admissions.
The failed batch remains retained and was not relabelled as current evidence.

The Toko 0.42.5 deployment used patch `batch-iZ1FgHB1` and minor
`batch-iZa8DNVj`: Foundation 0.77.2, Tools 0.92.1 and Testkit 0.103.0 each passed
all 32 stages. Rounded gate sums were 610/502/605 seconds, including 257/250/251
seconds of tooling fixtures. The patch batch admitted four invariant fixture
proofs across groups; the separate minor batch executed its own proofs. Observed
whole-command wall times were 1,624.82 seconds for the two-group patch and 866.71
for the minor, excluding planning, fixture qualification, failed attempts,
reconciliation and readback. These are observations, not controlled speedups;
the normal runner still lacks separate preparation/Git/upload/index timings.
Durable receipts, logs and archive readback are under
`.git/release-state/preserve/deploy0425-20261113.clk428/`. An earlier capture
stopped in the detached adapter before preparation: a duplicate consumer-root
assignment selected the helper directory. The corrected normal source passed
focused recovery checks, then recaptured and reran complete qualification.

## Qualification and limits

`make release-validation-plan-check` exercises all five group fallbacks against
the same full-gate roster, focused group selection, required/excluded failure
injection, discovery refusals, source/environment drift and receipt custody.
Its Make effects are substituted. `bash scripts/release/test-adapter.sh` retains
the real five-group metadata, rollback, commit and recovery contracts, also with
qualification effects substituted. Its disposable checkout uses three gate targets
(source admission, lint and tooling proof) plus the focused vocabulary. This
preserves actual selector/Shared-proof/receipt wiring without replaying 35 dummy
product commands for every recovery probe. The complete production policy and
roster-equivalence checks remain in `test-validation.sh` and ordinary CI.
Common source/proof corruption, setup refusal and staging probes execute once in
the five-group adapter run. Each group retains real metadata preparation, renderer
and graph rollback, other-group receipt refusal and committed-payload admission.
A single `--group` run includes the complete common probe set.
These fixtures do not establish native-host
execution or a runtime saving.
`bash scripts/release/test-adapter.sh --group testkit` selects one adapter owner
for a focused change. `bash scripts/release/test-stage-reuse.sh` executes the real
retention and evidence fixtures, proves cross-group reuse with unchanged inputs, and forces
fresh execution after changed/new shell inputs. Recovery fixtures additionally
reject changed stage identities and original execution logs.
`bash scripts/ci/test-workspace-validation.sh` isolates actual Make dispatch for
both full CI entrypoints under parallel Make. Its substituted check/publication
effects prove the 34-stage order after certificate owner extraction, one lint pass, pipe jobserver access
and failure stopping before later checks. The default Make fixture includes this
focused case for complete tooling qualification.

Actual focused commands select `RELEASE_GROUP=text`, `tools` or `build` with
`make release-scope-lint`, `release-scope-graphs`, `release-scope-tests`,
`release-scope-docs` and `release-scope-archives`. These are owner-specific checks;
Foundation and Testkit intentionally refuse those focused targets. Linux and both
native macOS hosts remain required; report their evidence separately. Preserve
matched-input comparison and command/timing observations before claiming savings.
Shared-input changes retain full release qualification until their own complete
native CI baseline exists. Historical rollout qualification completed in
[#314 — Qualify group-scoped release validation](https://github.com/dragginzgame/ic-tooling/issues/314);
that evidence does not qualify later runner or mapping changes.

For selection and command-routing development, run
`bash scripts/release/test-validation.sh --case selection`. It substitutes
product effects and proves per-owner omissions, missing-baseline fallback,
mixed-source fallback and refusal of new consumer surfaces; it does not qualify
native execution. Full CI keeps the complete fixture entrypoint.
