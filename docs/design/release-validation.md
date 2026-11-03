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
explicit exclusion contracts and full-gate fallback reasons. The selector
compares the actual Make dispatch with that roster; a new or omitted obligation
refuses until the mapping is updated and qualified.
The policy owns the ordered stage inventory once; coverage annotations do not
repeat it. Fresh discovery and saved-baseline recovery share one
[baseline admission rule](../../ci/release-validation-baseline.jq).

| Group | Release qualification |
|---|---|
| Foundation | Full gate: primitive feature profiles, Build/Tools/Testkit reverse consumers, native/Wasm graphs and installed archives. |
| Build | Full gate: artifact/cache protocols, release editor, optimizer ownership, private drivers and exact archives. Hosted optimizer qualification remains an independent CI obligation. |
| Text | Focused only when admitted below: default/all-feature/YAML rendering, dependency graphs, Tools' `icp-project` consumer, private driver/report tests and installed Text archive. Otherwise full gate. |
| Tools | Full gate: separate IC adapter feature profiles, external identities, registry-only consumers and installed archives. |
| Testkit | Full gate: simulator/server lifecycle, public identities, feature boundaries, Wasm/probe singleton graphs and installed archives. |
| Native | Focused only when admitted below: dependency-free native/Wasm graphs, allocation/unwind tests, documentation and installed Native archive. Otherwise full gate. |

Focused admission requires a completed, successful main-branch CI run at an
ancestor of the selected source. Each of Linux x86-64, macOS 15 Intel and macOS
15 Apple Silicon must have actually executed native CI/publication qualification
and retained successful qualification. A green docs-only run, incomplete matrix,
failed job, missing compiler lane or truncated response does not qualify.
All three jobs must also execute the focused release contracts and retain their
matched-input comparison. Earlier full-CI runs without those proofs keep the
full release gate. Native CI runs the existing complete gate once, then both
focused profiles on that same source, lock, compiler and host. Its comparison
artifact retains full-gate elapsed time, focused commands/logs/timings and exact
installed-archive evidence; cache targets are excluded.
Discovery reads at most ten recent successful runs once during fresh preflight;
it does not poll or wait for CI. Offline mode, unavailable GitHub access or absent
qualified ancestry selects the full gate. Raw discovery observations are retained.
Successful pushes and explicit `workflow_dispatch` qualification are eligible;
PRs, other branches/workflows/repositories and incomplete jobs remain ineligible.

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
`icp-project`, benchmark driver and archive diagnostics. New consumers require a
reviewed mapping. This is explicit contract admission, not automatic dependency
inference or an operator-controlled skip switch.

Every focused plan retains snapshot/symlink/compiler/pin/format/document checks.
Tool admission includes host parsers, Cargo tools and the release editor, without
installing the simulator or optimizer. Preflight retains locked workspace fetch
and metadata admission. Focused lint precedes runtime and archive execution.
Native/Text archives use the same IC Build engine as the full public roster,
including normalized source/lock/provenance checks and extracted-package tests.
The private archive driver still compiles its normal dependencies, including
Build and the client; focused selection excludes unrelated test/server execution,
not every possible dependency compilation. No archive stage is reused.

## Source custody, receipts and recovery

Every target checks the complete checkout against the admitted Git source before
and after execution, permitting only the selected pending ledger. Shared also
checkpoints the plan, policy, recipes, lock, configuration and ledger bytes around
every stage. A required failure or source/input drift stops later stages. Omitted
stages retain explicit `not-applicable` events and their coverage reasons; they
are never reported as executed passes.

Version-3 release receipts bind the source/group, plan and policy, environment
hashes, host/compiler observations, complete gate artifact hashes and each
required stage's successful log/timing evidence. Missing, failed, blocked,
redirected or altered evidence cannot authorize subsequent release effects.
Selection grants no reuse. Metadata preparation, unrelated-group preservation,
commit/payload custody, Git delivery and publication retain their existing checks.

Gate artifacts live in unique `.git/release-state/<group>-gate.*` directories.
Linked worktrees use `target/release-validation/<group>-gate.*` because Shared's
checkpoint paths must remain checkout-relative. Preserve those directories with
the release receipts; deleting artifacts invalidates evidence rather than granting
an implicit pass. Exact-version recovery retains its saved baseline and re-derives
the same selection without a new remote lookup. Fresh verification creates new
logs while preserving earlier attempts. Changed source/scope/configuration needs
a fresh admitted attempt. Historical version-2 receipts use their original source
and full-gate adapter; they are never relabelled.

## Qualification and limits

`make release-validation-plan-check` exercises all six group fallbacks against
the same full-gate roster, focused Native/Text selection, required/excluded failure
injection, discovery refusals, source/environment drift and receipt custody.
Its Make effects are substituted. `bash scripts/release/test-adapter.sh` retains
the real six-group metadata, rollback, commit and recovery contracts, also with
qualification effects substituted. These fixtures do not establish native-host
execution or a runtime saving.

Actual focused commands are `RELEASE_GROUP=native` or `RELEASE_GROUP=text` with
`make release-scope-lint`, `release-scope-graphs`, `release-scope-tests`,
`release-scope-docs` and `release-scope-archives`. These are owner-specific checks;
the four other groups intentionally refuse those focused targets. Linux and both
native macOS hosts remain required; report their evidence separately. Preserve
matched-input comparison and command/timing observations before claiming savings.
The rollout itself changes shared inputs, so it retains full release qualification
until its own complete native CI baseline exists. Issue
[#314](https://github.com/dragginzgame/ic-tooling/issues/314) tracks delivery and
consumer qualification.
