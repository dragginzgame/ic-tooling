# Tooling release validation

Release groups retain complete qualification of their own contracts. Tooling's
[adapter](../../scripts/release/adapter.sh) uses the existing Shared
[plan runner](../release-group-validation.md), without another release executor
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
| Native | Focused only when admitted below: dependency-free native/Wasm graphs, allocation/unwind tests, documentation and installed Native archive. Otherwise full gate. |

Focused admission requires a completed, successful main-branch CI run at an
ancestor of the selected source. Each of Linux x86-64, macOS 15 Intel and macOS
15 Apple Silicon must have actually executed native CI/publication qualification
and retained successful qualification. A green docs-only run, incomplete matrix,
failed job, missing compiler lane or truncated response does not qualify.
All three jobs must also execute every group listed by `validation-roster.sh groups`
and retain their
matched-input comparison. Earlier full-CI runs without those proofs keep the
full release gate. Routine native CI runs the existing complete gate once.
Explicit qualification dispatches additionally run the Native, Text, Tools and
Build focused profiles on that
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

Routine workflow/ref runs retain newest-revision cancellation. For a qualification
comparison that must finish while delivery continues, dispatch the existing four
workflows on main at one selected revision:

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
Native must have no dependencies or workspace reverse consumers. Text's reverse
owners must remain Tools and Dev, with runtime callsites confined to the checked
`icp-project` and archive diagnostics. New consumers require a
reviewed mapping. This is explicit contract admission, not automatic dependency
inference or an operator-controlled skip switch.

Tools has only Dev's development edge, with Rust callers confined to
`tests/public_contracts.rs`. Build has only Dev's normal edge; its
archive-qualification binary/tests, public contract and installed Wire test are
the admitted call surfaces. A new consumer, dependency kind or Rust call surface
selects the full gate. Per-group exclusion annotations explain every omitted
full stage; Tools/Build do not inherit Native/Text's unrelated exclusion claims.
The new plans cannot use historical Native/Text comparisons: changes to the
mapping, dispatcher and workflow force full validation until a new three-host
comparison includes all four profiles. Mixed-group changes and Shared Tooling
refreshes still select the full gate.

Every focused plan retains snapshot/symlink/compiler/pin/format/document checks.
Tool admission includes host parsers, Cargo tools and the release editor, without
installing the simulator or optimizer for Native, Text and Tools. Build retains
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

## Selected adapter cases and unchanged-stage reuse

During release qualification, `tooling-command-check` runs the adapter fixture
with `--group` for the selected release group. Ordinary CI and CI/publication
continue to exercise all six groups, even when a release selection is inherited.
The remaining tooling roster and its first-failure behavior are unchanged.

The fixture-retention, product-evidence, workflow-selection and scratch-routing
stages use Shared's
[stage runner](../release-validation-reuse.md). It observes every shell source
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

## Qualification and limits

`make release-validation-plan-check` exercises all six group fallbacks against
the same full-gate roster, focused group selection, required/excluded failure
injection, discovery refusals, source/environment drift and receipt custody.
Its Make effects are substituted. `bash scripts/release/test-adapter.sh` retains
the real six-group metadata, rollback, commit and recovery contracts, also with
qualification effects substituted. Its disposable checkout uses three gate targets
(source admission, lint and tooling proof) plus the focused vocabulary. This
preserves actual selector/Shared-proof/receipt wiring without replaying 35 dummy
product commands for every recovery probe. The complete production policy and
roster-equivalence checks remain in `test-validation.sh` and ordinary CI.
Common source/proof corruption, setup refusal and staging probes execute once in
the six-group adapter run. Each group retains real metadata preparation, renderer
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

Actual focused commands select `RELEASE_GROUP=native`, `text`, `tools` or `build` with
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
