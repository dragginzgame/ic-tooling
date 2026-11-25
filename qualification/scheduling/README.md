# Scheduling rewrite: current consumer requirements

`make test-scheduling-consumer` snapshots `../ic-scheduling`, including tracked
edits and non-ignored source. It verifies rewrite ancestry from
`f382bacac5461ea965e5813b56f15bd4757bf596`, refuses capture drift and admits one
registry Testkit 0.110.4 shared by the executor and Once drivers. No direct
PocketIC dependency or historical-source fallback is admitted. Installed modes
require explicit `TESTKIT_SERVER_CLI` and `SCHEDULING_SERVER_DIRECTORY`; see the
[product command](../../docs/simulator-ownership.md). Tools are hashed before/after.

The local replacement adapter switches only the Testkit dependency to this
checkout's 0.110.5. Both drivers keep their client and held-reply-fixture boundary.
The captured lock already selects Protocol 0.3.10, Types 0.21.5 and Runtime
0.23.27. The replacement must preserve existing external identities, including
the original source and checksum. The old Protocol lock exception is removed.

`bash scripts/qualification/scheduling-consumer.sh admit-replacement` passed
source, graph and lock admission and Clippy for both drivers. See the
[current Platform adoption record](../../docs/qualification/platform-adoption.md#platform-0351-adoption).
The installed Once selector patch no longer applies to the current capture.
The installed runner refuses that patch before execution.

## Earlier runner acceptance — 2026-11-16

[Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2) is completed.
Its owner published Timers 0.46.0 and Jobs 0.39.0 with Runtime/guard 0.23.14 and
Testkit 0.107.2 after installed checks on Linux/PocketIC 16.0.0.

The Protocol 0.2.7 release check repeats pressure/held-upgrade acceptance on this
same source identity, using local Testkit 0.107.4 and the product CLI:
`target/scheduling-new.wJUWIZWv/`, log
`target/deploy-01074/scheduling-held-qualified.log`. Exact supplier admission,
Clippy, instance deletion and tool stability pass. A deliberately added unrelated
registry identity still refuses in `target/deploy-01074/unrelated-drift.log`.

Product-command capture at HEAD `df5d14a482973e427ea79cae022bd38971b39fa7`, tar
SHA-256 `35181357d57eb68d42af60567d795bc08c958930efe99d26c8b56ace8e0376c2`,
passes source/graph admission, local Testkit 0.107.4 Clippy/external-lock checks,
and real pressure/held-upgrade checks through `ic-testkit-server` with prepared
PocketIC 16.1.0 on Linux. Instance deletion and selected tool hashes pass
independently. Evidence: `target/scheduling-new.FsYDrjyH/`, log
`target/issue-10/scheduling-held.log`. This removes the private launcher package;
the consumer retains its client, held fixtures and application assertions. It
does not rerun all owner acceptance, qualify other hosts or change the sibling.

The preceding private-runner capture is historical:

Read-only capture at HEAD `df5d14a482973e427ea79cae022bd38971b39fa7`, tar SHA-256
`4e8db311c967299edc969a3a8310c13f5f6e6a1d1571c64605c8e86a6d460d3e`, passes
registry driver admission and local-supplier Clippy/external-lock checks. The
refreshed private selector preserves the consumer's new focused options. Local
Testkit 0.107.3 passes pressure and held-upgrade checks through the new private
managed-server caller with prepared PocketIC 16.1.0 on Linux. Instance deletion
and tool hashes pass independently. Evidence: `target/scheduling-new.dsY6YyGG/`,
log `target/debt-scheduling-held.log`. This focused pass does not rerun all owner
acceptance from Scheduling #2, test other hosts or change the sibling.

## Historical captures

The remaining dated captures retain their original versions, commands and limits.
They are evidence, not current setup instructions or current blockers.

## Platform Runtime refresh — 2026-11-16

Later current-source capture at the same HEAD `3d2b3fd3775252e779181c4175a8ecf661afb60b`
has tar SHA-256 `56aad13d860ef1f920c4d4b0a54bf7ba3ab3721143aff4fcef33328f8399d030`.
It selects Runtime 0.23.13 (`call-ic0`, `system-raw`) and Types 0.21.2 in the
qualification manifest, while its qualification lock still contains Runtime 0.22.5.
Published Testkit 0.107.1 requires exact Types 0.21.1, so Cargo refuses the registry
baseline before compilation. Normal admission/comparison commands remain strict;
they do not fall back to the earlier source or rewrite the consumer's baseline.

The manifest-only local-supplier adapter now matches this capture. Applying it
and the existing focused selector only to the exported source allows offline
resolution with local Testkit, Protocol 0.2.5 and Types 0.21.2. Both host drivers
pass all-target warning-denied Clippy. The subsequent Wasm `once-probe` build fails
with 17 errors at removed Runtime imports, executor helpers and call/completion
methods. No installed or paired acceptance is claimed for this capture, and the
resolved export is not claimed to preserve the old lock's external selections.

Evidence is under `target/platform-adoption-next/`: `scheduling-metadata.log`,
`scheduling-local-resolution.log`, `scheduling-local-metadata.json` and
`scheduling-local-build.log`. [Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2#issuecomment-6926773677)
tracks owner-side probe/lock migration. Compatible Testkit publication is separate
Tooling delivery work. No sibling source was edited or probe implementation copied.

## Published supplier refresh — 2026-11-16

This earlier passing capture predates the Runtime refresh above.

The read-only capture records HEAD `3d2b3fd3775252e779181c4175a8ecf661afb60b`
plus current tracked edits and non-ignored additions, source tar SHA-256
`01b8658271082d2c6afb977ea8ac5ea5cfcfe49b05272aa788baf9c0b4fdc21b`.
Its lock selects registry Testkit 0.107.1 with the verified publication checksum
`a4fe4ea7882118e731ef1ebe8c668fc5dfa5252c1c972b1de68bc444868fae42`.
Pressure, held-upgrade, Jobs-pressure and watchdog callers already use the new
catalog/fixture API. The former API migration patch correctly refuses this source;
it has been reduced to the manifest-only local-supplier substitution.

Fresh paired acceptance in `target/scheduling-new.faFrl9X7` uses that same capture
with registry and local Testkit 0.107.1. Both pass pressure and held-upgrade,
missing-endpoint refusal and independent observation of one deleted instance.
Pressure preserves the 64/65 admission boundary, eight held flights and twelve
single starts; upgrade refusal preserves the original settlement. Tool hashes
match before and after both runs. Driver Clippy and external-lock subset admission
pass. The selected owner is still the original Testkit/PocketIC type boundary;
no sibling implementation or assertion was changed.

The command was `scheduling-consumer.sh comparison held-replies`, with explicitly
prepared PocketIC 16.1.0 and launcher 0.106.0. The first sandbox attempt compiled
but could not bind loopback; only the later live run supplies installed evidence.
Logs are under `target/toko-adoption-0.44.8/scheduling-held-comparison-live.log`.
This refresh does not rerun or relabel older aggregate, metrics, executor or macOS
acceptance. No new public Testkit API or consumer migration obligation is introduced.

## Requirements from the actual callers

The rewrite's `AGENTS.md` and `qualification/README.md` explicitly require
`ic-testkit::pocket_ic` rather than a direct PocketIC dependency. Preserve that
consumer boundary while designing the replacement; removing Testkit is not a goal.

| Current caller | Demonstrated requirement | Ownership |
| --- | --- | --- |
| `executor-driver/src/main.rs` | Synchronous PocketIC builder/client and shared IC identifier/error types; prepared server binary | Shared Testkit client boundary; caller chooses topology, funding and assertions |
| `once-driver/src/main.rs` | Async client with explicit instance deletion and caller-owned Tokio runtime | Same client supplier/type identity; caller owns scheduling and runtime |
| `once-driver/src/pressure.rs` and `jobs_pressure.rs` | Bounded generated reply fixture, eight held flights, exact pending requests and deliberate release order | Reusable Testkit held-reply mechanism; consumer owns pressure/admission expectations |
| `once-driver/src/held_upgrade.rs` | Hold a real reply through upgrade refusal, then release and observe original settlement | Testkit reply control; consumer owns upgrade authority and recovery assertions |
| `once-driver/src/watchdog.rs` | Held request admission, cadence/coalescing while flights remain pending, explicit release | Same shared fixture; consumer owns simulated-time/tick policy |

The fixtures use `held::{Catalog, Reply, Limits, Fixture}` and
`Fixture::{new, pending, release}` with its owned `wasm` bytes. Limits, funding and request
budgets are selected by the consumer. Do not replace this with copied per-driver
fixture machinery. Preserve exact request selection, type identity, bounds and
failure behavior before admitting a new shared Testkit contract.

This rewrite builds its probes using Cargo commands in its Makefile. It does not
have the old `ic-timers-pocketic` driver, Jobs capture executable, or cached-baseline
snapshot workflow. There is no current evidence that it needs the old snapshot
migration or an `ic-build` adapter. Its Once command runs a broad installed roster;
the focused `--held-replies` selection below now supplies the first retained-supplier
baseline for replacement work.

## Historical evidence boundary

[Earlier Scheduling experiments](historical.md) used the pre-rewrite source then
located at `../ic-scheduling`. Their source IDs and results remain historical;
they do not qualify this rewrite or justify its architecture. The executable
historical lane, its exclusive patches and Jobs/Timers session helpers are removed
under [#4](https://github.com/dragginzgame/ic-tooling/issues/4). There is no supported
reproduction command or compatibility lane for that source. Current rewrite
qualification retains its own selectors, patches and explicit prepared tools.

## Initial corrected-source evidence

On 2026-11-15 the runner captured HEAD
`7715fc81d533bbd9bed613a62c80f6f9d18a4a37` plus local edits, source tar SHA-256
`229d27dd3a9cbaa07b8bc21bef7d575223abeb6dd581cfea79e6571411ac521f` in
`target/scheduling-new.qz5U96ZW`. Testkit graph admission and both host-driver
checks passed (`target/scheduling-new-admission.log`). An explicit override to
old `../ic-scheduling` refused the rewrite source contract before driver compilation
(`target/scheduling-old-source-refusal.log`); the now-removed historical runner's
old guard result is retained in `target/scheduling-history-guard.log`. This did not run
PocketIC, deploy canisters, modify sibling files, or qualify any new Testkit API.

## Focused held-reply baseline and next contract

The [first Testkit contract](../../docs/testkit-held-replies.md) defines shared
ownership, bounded catalog/module admission, opaque request identity, exclusive
release authority and uncertain-effect handling. It is an implementation contract,
not a claim that the replacement runtime fixtures or all their failure tests exist.

Run `make test-scheduling-held` with `SCHEDULING_POCKET_IC_BIN` set to an
absolute executable path for a prepared local server. Set `SCHEDULING_TESTKIT_CLI`
to an absolute executable path for the released 0.106.0 launcher. Both variables
are required for installed selections; no tool is installed implicitly. Earlier
runs located these assets through the old checkout's receipt. This recorded run
uses server 16.1.0; the rewrite's earlier 16.0.0
observations remain separate.

`held-selection.patch` adds a private `--held-replies` driver selection in the
export. It preserves Testkit imports and calls only the existing pressure and
held-upgrade subjects. For this mode the outer launcher owns server process/TTL
and logs; the driver requires its endpoint, creates one instance and explicitly
closes it. The runner checks missing-endpoint refusal and independently observes
one deleted instance before the outer server stops. This removes reliance on
waiting for an implicitly spawned server to expire while holding an output pipe.
It does not strengthen upstream panic/cancellation cleanup or bound HTTP inventory
allocation. Other driver modes retain their original source semantics.

Linux evidence on 2026-11-15: the same source HEAD/tar hash as initial admission,
export `target/scheduling-new.KVxhWPge`. Both subjects pass
(`target/scheduling-held-final.log`), covering eight held flights, reverse release,
queued cancellation/rearm and live-reply upgrade refusal followed by settlement.
The endpoint refusal, deleted-instance observation and before/after server/launcher
hash checks pass. Focused driver Clippy passes
(`target/scheduling-held-final-clippy.log`). All runtime checks use released
Testkit 0.106.0; no replacement Testkit acceptance, supplier-wide failure coverage,
performance result or sibling modification is claimed.

That initial baseline preceded the shared owner and its independent failure
checks. The implementation and subsequent comparisons below completed that slice;
current supplier selection is described at the top of this document.

## Replacement Testkit 0.107.1 qualification

At the initial 2026-11-15 qualification, `make test-scheduling-replacement` captured
rewrite source, admitted its released graph, then applied `held-selection.patch`
and the former API migration patch only to that export. The latter changed the
supplier to this checkout and replaced fixture descriptors/constructors with
`held::{Catalog, Reply, Limits, Fixture}`. It kept consumer assertions, funding,
installation, tick budgets and release order intact. Principal conversion used
canonical text at the native-client/Platform identity boundary. No consumer code
was added to the product Testkit crate.

The complete Once driver had to compile, so the former patch adapted Jobs-pressure and
watchdog call sites too. Only pressure and held-upgrade are executed here; adapting
and compiling those other call sites does not qualify their installed behavior.
Executor and Once must resolve the same sole local Testkit 0.107.1 package.
The external-lock subset checker verifies retained registry/source/checksum tuples;
removed old supplier dependencies are permitted, changed retained selections are not.

On 2026-11-15 the focused run passed against rewrite HEAD
`832c631cb3203d11bd7bcb3669da34d5b09d431b` plus its captured working tree, source-tar
SHA-256 `4fde9e153f591c886b1a8d87104c37f5b1f7869cc4d3646d8e3d97e156f524de`.
Export: `target/scheduling-new.kzVt9PvL`; log:
`target/scheduling-replacement.log`. Pressure retained the 64/65 declaration bound,
eight held flights, cancellation and twelve single starts with reverse release.
Held-upgrade retained live-upgrade refusal, inert duplicate/stale envelopes,
original settlement and fresh-demand reconstruction. Both driver Clippy checks,
external-lock admission, missing-endpoint refusal, instance deletion (`["Deleted"]`)
and prepared-tool identity checks passed. The server was prepared PocketIC 16.1.0;
released Testkit's CLI owned server lifetime only. It was not linked into the driver.

That run qualified the then-unpublished replacement in a disposable export.
The sibling was unchanged and still selected released Testkit at that checkpoint.
Later dated sections retain their own broader acceptance evidence.

## Watchdog and Jobs held-pressure selection

`make test-scheduling-pressure-baseline` uses released Testkit 0.106.0;
`make test-scheduling-pressure` uses the local 0.107.1 draft. Both select
`--held-pressure` through the consolidated `held-selection.patch` and run only
`watchdog::admission`, `watchdog::pressure` and `jobs_pressure::check`.
The shared runner checks the selected completion sentinel, missing endpoint,
observed instance deletion and unchanged prepared tools. An empty HTTP inventory
is required between each subject, before later Jobs time advancement. No product
implementation or consumer assertions were changed for this slice.

Both suppliers pass the existing obligations:

- Watchdog admission: eleven opportunities produce one physical held request;
  watchdog flight settlement remains independent of the application reply.
- Watchdog pressure: eight held flights leave cadence advancing; releasing one
  admits the original coalesced callback, then all eight blockers finish.
- Jobs pressure: the shared declaration/flight bounds, queued pause, isolated
  uncertain-occurrence recovery and healthy subsequent progress remain observable.

Evidence on 2026-11-15 uses rewrite HEAD
`832c631cb3203d11bd7bcb3669da34d5b09d431b` plus current working-tree captures:

| Supplier | Export | Source-tar SHA-256 |
| --- | --- | --- |
| Released 0.106.0 | `target/scheduling-new.FsISg5Hu` | `ca7a0f4f252dd600e1b8c80d6c9d0cacb4b53ed1b683619462ffe0ea1e9679ea` |
| Replacement 0.107.1 | `target/scheduling-new.sunzOmZX` | `196e6ab08c835e2666e34900dcac5cd7352ae5c7793d9925e766cc5886fa4cf4` |

Logs are `target/scheduling-pressure-baseline.log` and
`target/scheduling-pressure-replacement.log`. Both report deleted instances and
identical prepared tool hashes. The replacement also passes external-lock subset
admission and both driver Clippy checks. Source comparison found changes only in
`qualification/once-driver/src/retention.rs` and
`qualification/once-probe/src/retention.rs`; neither retention scenario was selected.
These are independently captured runs, not identical-source binary or performance
comparisons. No historical source fallback or sibling edits were used.

The earlier paragraph limiting execution to pressure/held-upgrade describes that
first selection only. The new slice qualifies these three additional subjects;
watchdog trap/refusal behavior, retention, metrics, calendar, other Jobs lifecycle
modes and macOS were outside that slice. The following comparison adds the
watchdog trap/refusal cases. No broader shared Testkit API was needed.

## Single-capture watchdog and lifecycle comparison

`make test-scheduling-lifecycle` selects `comparison lifecycle`: capture current
source once, run released Testkit, apply the replacement patch within that same
export, then run Testkit 0.107.1. Each run has a separately owned server/instance
and its own `baseline/` or `replacement/` evidence directory. The runner retains
original metadata and lock before migration and checks supplier identity, external
lock retention and driver Clippy. It does not recapture source between suppliers.
The selectors now live in one `held-selection.patch`; the temporary separate
pressure-selection patch was folded into it.

Only `watchdog::check`, `refusal::watchdog` and `lifecycle::check` are selected.
The original consumer assertions verify committed successors after work traps,
owner commands winning over callback decisions, cadence without bypassing delivery
backoff, one funded completion, malformed/stale dispatch rejection and future
deadline reconstruction across upgrade. These use the original PocketIC types;
they add no shared fixture or scheduler policy to Testkit.

Both suppliers passed on 2026-11-15 with source HEAD
`832c631cb3203d11bd7bcb3669da34d5b09d431b` and source-tar SHA-256
`e8160ba7a4f51dcae734c4f297f8a8aceae9e097e4fe14906fa809838908a0d9`.
Export: `target/scheduling-new.VR3dItWO`; log:
`target/scheduling-lifecycle-comparison.log`. The selected scenario logs match.
Both branches pass missing-endpoint refusal and observed instance deletion;
prepared PocketIC 16.1.0 and launcher hashes remain unchanged. External-lock
admission and replacement driver Clippy also pass. This is functional comparison,
not a performance measurement or full aggregate-suite qualification. Sibling
source, retained registries and publication state remain unchanged.

## Default-feature executor comparison

`make test-scheduling-executor` selects `comparison executor` in the same capture
and server runner. `executor-endpoint.patch` changes only the qualification
entrypoint's prepared-server selection to an explicit external endpoint; the
original synchronous PocketIC client, three lifecycle cases, three admission
refusals and expected event bytes are unchanged. The outer launcher owns the
server and the synchronous client owns its instance. This removes per-driver
server ownership from the qualification adapter without adding a Testkit wrapper.

The runner builds `executor-probe` and `executor-driver` explicitly. It captures
that driver's selected normal/build dependency tree, rather than inferring feature
isolation from workspace-wide metadata. The replacement must select local Testkit
0.107.1 and exclude `wat`, `ic-platform` and `ic-platform-types` from this graph.
The held-reply catalog/generator remains available to Once through its explicit
feature; it is not imposed on the executor. Direct upstream PocketIC and its own
native dependencies remain present as intended.

Both suppliers passed on 2026-11-15 from one source capture at HEAD
`832c631cb3203d11bd7bcb3669da34d5b09d431b`, source-tar SHA-256
`1d97ffdb0868e6f35f01231906c3543ba36d993a86afd2d528a6f4ab3cc57039`.
Export: `target/scheduling-new.MSReobiV`; log:
`target/scheduling-executor-comparison.log`. Baseline and replacement scenario
logs match, including reply/event bytes. Both runs reject a missing endpoint and
report exactly one deleted instance. Prepared tool hashes, external-lock retention
and replacement driver Clippy pass. Graph evidence is in each run's
`driver-tree.txt`. No product implementation change, sibling edit or full suite
was required; this does not imply native macOS acceptance or production cutover.

## Retained callback and dropped-handle comparison

`make test-scheduling-retention` selects `comparison retention` through the same
capture/server runner. It calls the current consumer's unchanged `retention::check`
and checks empty HTTP inventory before deleting the instance. Only the exported
entrypoint and supplier bindings change; no Scheduling implementation is copied
into Tooling. The selector patch preserves the current consumer's additional
focused Jobs/after-completion routes, and its Toko 0.44.4 selection is retained.

Released Testkit 0.106.0 and replacement 0.107.1 both passed on 2026-11-15 from
HEAD `ba581e1d344573b832ba27162e6255ec184d1d21`, source-tar SHA-256
`47a8bf166f59c590747dfa0a984cc7e52cac448ed77feca98ec43e7f43131d9a`.
Export: `target/scheduling-new.PimVaIJv`; log:
`target/scheduling-retention-comparison.log`.

The original assertions preserve 64 retained captures through 960 rejected drops
and repeated settlement (96 future drops, 192 callbacks). Dropping armed public
handles still lets 32 async Once and 32 watchdog callbacks settle; later cadence
does not repeat stopped watchdogs. Both scenario logs match. Recorded Wasm page
counts are observations, not a Tooling memory or performance improvement claim.

Both runs refuse a missing external endpoint and report exactly one deleted
instance. Prepared server/launcher hashes, exact replacement supplier identity,
external-lock admission and driver Clippy pass. This adds retention acceptance;
it does not qualify the aggregate suite, other Jobs/calendar modes, native macOS
or production cutover. The sibling source and product API remain unchanged.

## Jobs lifecycle and calendar comparison

`make test-scheduling-jobs-lifecycle` selects `comparison jobs-lifecycle` in the
existing runner. The export calls unchanged `jobs::calendar`, `jobs::check`,
`jobs::lifecycle` and `jobs::downtime`, checking empty HTTP inventory between
subjects. It preserves the current consumer's focused retry selector. Neither
the sibling nor the Testkit product API changes.

Released 0.106.0 and replacement 0.107.1 both passed on 2026-11-15 from HEAD
`bc36b9110b19171f068e763697751ca6f94dfe0b`, source-tar SHA-256
`557f20fe58c3c5e2099bbe2df3e943c3b2ae7096cfc0f75abf4a2e51b0db405c`.
Export: `target/scheduling-new.NiElDo6D`; log:
`target/scheduling-jobs-lifecycle-comparison.log`.

Five calendar scenarios retain their exact next-occurrence results. Seven Jobs
modes check occurrence context, skip-late behavior, edit/pause precedence,
recovery isolation and reentrant future destruction. Upgrade checks preserve
selected occurrences/revisions and paused state without replaying settled work.
Two downtime modes execute overdue work once or skip it according to restored
lateness policy, with future successors and no catch-up burst.

Both runs pass missing-endpoint refusal and report exactly one deleted instance.
Prepared tool hashes, external-lock admission, exact replacement identity and
driver Clippy pass. Scenario logs match; calendar instruction counts are recorded
consumer observations, not a Tooling performance claim. Other Jobs subjects,
aggregate-suite execution, native macOS and production migration remain outside
this evidence.

## Jobs retry, reconciliation and completion comparison

`make test-scheduling-jobs-settlement` selects `comparison jobs-settlement`.
It runs the unchanged `jobs_retry::check`, `jobs_retry::recovery` and
`jobs_completion::check` subjects, with empty HTTP inventory between subjects.
The export retains the consumer's newly added unresolved/lifetime selectors.
No product API or sibling source is changed.

Released Testkit 0.106.0 and replacement 0.107.1 passed on 2026-11-15 from HEAD
`bc36b9110b19171f068e763697751ca6f94dfe0b`, source-tar SHA-256
`865baa453960352bfe6595f13d88d1102bc2468bf11840316d0f4554aec71747`.
Export: `target/scheduling-new.uHcqC3ty`; log:
`target/scheduling-jobs-settlement-comparison.log`. This capture includes newer
working-tree changes than the preceding Jobs lifecycle run; the results do not
refresh that earlier slice's evidence.

Eleven retry modes preserve capped budgets, stable effect identity, pause/upgrade
state, owner edits, cancellation and settlement. Seven reconciliation modes check
explicit retry safety, paused upgrades, exact authority and refusal when retry
is unavailable. Eleven completion modes check real awaits, completion-relative
deadlines, pause/upgrade, edits, overflow, recovery and terminal cancellation.
Uncertain work is not retried merely because time passes or the owner resumes it.

Both suppliers pass all 29 scenarios with matching scenario logs, missing-endpoint
refusal and exactly one observed deleted instance per run. External-lock admission,
exact replacement selection, driver Clippy and prepared-tool hashes pass. These
are consumer functional assertions, not a performance comparison. Other subjects,
native macOS, aggregate-suite acceptance and production cutover remain separate.

## Lifetime custody and unresolved upgrade comparison

`make test-scheduling-uncertainty` selects `comparison uncertainty`. The exported
entrypoint runs unchanged `lifetimes::check` and `jobs_unresolved::check`, with
empty HTTP inventory between subjects. The consumer's current diagnostics
selector remains intact. No consumer policy or implementation moves into Tooling.

Released Testkit 0.106.0 and replacement 0.107.1 passed on 2026-11-15 from HEAD
`bc36b9110b19171f068e763697751ca6f94dfe0b`, source-tar SHA-256
`c6d201893987fa419ebda9c010476a815e0774797906836570f899bae979dd55`.
Export: `target/scheduling-new.hwf6Dljw`; log:
`target/scheduling-uncertainty-comparison.log`. This is a newer working-tree
capture than the settlement comparison; earlier evidence remains snapshot-bound.

Eight lifetime modes check transient settlement, trapped custody, reentrant
capture release and stale queued replacement. A cleanup trap must preserve
uncertainty or fault closed, without replay. Seven unresolved-work modes exercise
real upgrades during running/factory/post-await work, repeated reconstruction,
retry budgets, owner edits and catch-up custody. Time advancement and resume do
not replay restored work; retirement or retry follows explicit reconciliation.

Both suppliers pass all 15 scenarios with matching logs, missing-endpoint refusal
and exactly one observed deleted instance each. External-lock admission, exact
replacement selection, driver Clippy and prepared-tool hashes pass. No shared
Testkit API change was necessary. This does not qualify other subjects, the full
aggregate suite, native macOS or production migration.

## Jobs deadline, progress and catch-up boundaries

`make test-scheduling-jobs-boundaries` selects `comparison jobs-boundaries` in
the existing runner. It calls unchanged `jobs_initial::check` for recurring and
one-shot work, `jobs_boundaries::check`, `jobs_boundaries::ahead` and
`jobs_catch_up::check`, checking empty HTTP inventory between subjects.

Released Testkit 0.106.0 and replacement 0.107.1 passed on 2026-11-15 from HEAD
`f382bacac5461ea965e5813b56f15bd4757bf596`, source-tar SHA-256
`ee7f488adfdb6e621bac9522240dd36177a1c7a9daa35b8cc927d5ab3958b3e8`.
Export: `target/scheduling-new.Xrf6gCqk`; log:
`target/scheduling-jobs-boundaries-comparison.log`. Earlier comparisons remain
bound to their own captures rather than being promoted to this newer source.

Six recurring and eight one-shot admission modes preserve exact occurrence bytes
for zero, overdue and future deadlines, skip-late behavior and reconstruction.
Boundary assertions cover impossible schedules, exhausted progress, three exact
catch-up plans and progress ahead of time under both lateness policies. Eleven
catch-up modes include a full 64-occurrence batch, paused upgrades, schedule edits
and recovery from first/later traps without replenishing committed batch quota.

Both suppliers produce matching scenario logs and pass missing-endpoint refusal,
exactly one observed deleted instance, external-lock admission, exact replacement
selection, driver Clippy and prepared-tool hashes. Instruction counts printed by
the consumer are observations, not a Tooling performance claim. No product API
or sibling source changes were needed.

The inspected Once driver still has uncovered inline aggregate cases and separate
after-completion, diagnostics, general refusal and cost subjects. These focused
comparisons do not establish full aggregate-suite, native macOS or production
migration acceptance, nor do they refresh prior subjects against this capture.

## Completion cadence, diagnostics and delivery refusal

`make test-scheduling-completion-refusal` selects `comparison completion-refusal`.
It runs unchanged `after_completion::check`, `metrics::check` and `refusal::check`,
with empty HTTP inventory between subjects. These are Scheduling-owned assertions;
no diagnostics or scheduler product policy is added to Testkit.

The rewrite is now at `../ic-scheduling`. Git ancestry from the previously
qualified rewrite commit was verified before selecting this path. An explicit
`SCHEDULING_SOURCE=../ic-scheduling-legacy` run refuses before capture, recorded in
`target/scheduling-legacy-source-refusal.log`. Only the prepared server/launcher
assets are read from the legacy checkout; it supplies no consumer source.

Released 0.106.0 and replacement 0.107.1 passed on 2026-11-15 from HEAD
`6c658af8ae0f2ddd8a06a974f850b2c8e1bd4976`, source-tar SHA-256
`6a67d7f162e0e3a829eed3f936e067fbb3bb15cbc2ae7ef257c934febf061f7a`.
Export: `target/scheduling-new.GjZEIbi2`; log:
`target/scheduling-completion-refusal-comparison.log`.

Five after-completion modes preserve completion-anchored successors, stop and owner
override, overflow and trap barriers. Diagnostics preserve exact committed counts,
reentrant reads and initialization. Actual low-cycle delivery refusals retain a
later retry opportunity without accepting work; funding cannot bypass the retained
deadline, and the later delivery completes exactly once.

Both suppliers produce matching scenario logs and pass missing-endpoint refusal,
exactly one observed deleted instance, external-lock admission, exact replacement
selection, driver Clippy and unchanged prepared-tool hashes. The quoted cycle cost
is a consumer observation, not a Tooling performance result. This closes the three
subjects listed above for this capture; inline aggregate cases, cost subjects,
native macOS and production migration remain separate gates.

## Complete Once aggregate comparison

`make test-scheduling-aggregate` selects `comparison aggregate`. Its entrypoint
adapter selects the external server then falls through to the original complete
Once roster, without changing assertions, ordering or consumer cleanup. This
includes the six inline lifecycle/authentication cases, every subject in that
roster and its control/dispatch cost probes. The separately invoked paired
metrics-feature probe and executor driver are not part of this command.

Released Testkit 0.106.0 and replacement 0.107.1 both passed on 2026-11-15 from
HEAD `ee445be226f3ba679e3798d2f7687f2c4d8b0534`, source-tar SHA-256
`175eb603c5549de1186d1fef93159bc2678a65e6d513de1a15a983e9732d55a1`.
Export: `target/scheduling-new.f0HO0V8G`; log:
`target/scheduling-aggregate-comparison.log`. Both 101-line scenario logs match,
including recorded samples. No performance improvement is inferred from that
equality; dispatch prefix samples do not represent whole-message cost.

Both branches pass missing-endpoint refusal and report exactly one deleted
instance. External-lock admission, exact replacement identity, driver Clippy and
prepared-tool before/after hashes pass. Earlier isolated runs remain separate
evidence, but this capture now establishes complete Once roster composition.
Native macOS, the paired metrics-feature probe, and production migration remain
outside this result.

The initial attempt stopped before compilation because the prepared tools under
the removed legacy checkout no longer existed (`target/scheduling-new.1w9WebUU`).
The successful run explicitly selected PocketIC from
`../ic-backup/.tools/testkit/16.1.0-x86_64-linux/pocket-ic` and launcher 0.106.0
from the installed Toko tool store. Their SHA-256 values exactly match the previous
successful completion/refusal run: server
`b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db`
and launcher `47bb1fd4174bd7ee48b77f32ddb579252c3c6e1e7cd860a1d9f256d167b616e5`.
Only these prepared executables are reused; no Backup or legacy consumer source
is selected. The runner now requires both explicit executable paths and reports
missing files clearly. Missing-launcher refusal is recorded in
`target/scheduling-missing-launcher.log`; no installation is implicit.

## Paired metrics-feature comparison

`make test-scheduling-metrics` selects `comparison metrics-cost`. Each supplier
run builds the existing `metrics-probe` with metrics disabled and enabled in
separate Cargo invocations, retaining both Wasm files before the next build.
The original `metrics_cost::check` assertions run against an externally owned
server; the adapter adds only endpoint selection and a completion sentinel.
Hashes check that retained probe bytes stay unchanged throughout the run.

Released Testkit 0.106.0 and replacement 0.107.1 passed on 2026-11-15 from HEAD
`ee445be226f3ba679e3798d2f7687f2c4d8b0534`, source-tar SHA-256
`175eb603c5549de1186d1fef93159bc2678a65e6d513de1a15a983e9732d55a1`.
Export: `target/scheduling-new.9BWzKmCx`; log:
`target/scheduling-metrics-comparison.log`. The source tar is identical to the
completed Once aggregate capture; no intervening source change is inferred away.

Each branch runs three samples for each feature configuration. Both preserve
callback counts and stable final state. Disabled metrics return six zero counters;
enabled metrics return `[2, 2, 0, 1, 0, 0]`, with zero overflow flags in both.
Disabled/enabled Wasm sizes are 89,674/97,350 bytes. Setup and execution-prefix
instruction samples are recorded by the original driver; they are not whole-async
lifetime costs or evidence of a Tooling performance improvement.

The corresponding disabled/enabled Wasm files are byte-identical across suppliers,
and scenario logs match. Both runs pass missing-endpoint refusal, exactly one
observed deleted instance, external-lock admission, exact replacement selection,
driver Clippy and prepared-tool integrity. The complete Once aggregate and this
separate feature comparison are now qualified on identical source bytes. Executor
evidence remains tied to its earlier capture; native macOS and production cutover
remain open. No product implementation or sibling source was changed.

## Executor refresh on the aggregate source

`make test-scheduling-executor` passed with released 0.106.0 and replacement 0.107.1
on 2026-11-15 from HEAD `ee445be226f3ba679e3798d2f7687f2c4d8b0534`, source-tar
SHA-256 `175eb603c5549de1186d1fef93159bc2678a65e6d513de1a15a983e9732d55a1`.
Export: `target/scheduling-new.PWX0Vw62`; log:
`target/scheduling-executor-current.log`. Its source tar is byte-identical to
the complete Once aggregate and paired metrics-feature captures above.

All three lifecycle and three admission checks pass with exact reply/event bytes;
the two scenario logs match. The replacement driver selects local Testkit 0.107.1
without optional `wat`, `ic-platform` or `ic-platform-types` dependencies in its
normal/build graph. Both runs refuse a missing endpoint and report exactly one
deleted instance. External-lock admission, driver Clippy and prepared-tool hashes
pass. No new adapter or product code was required for this refresh.

The Once aggregate, paired metrics-feature probe and executor are therefore
qualified against both suppliers on identical current source bytes. This closes
the earlier mismatch in capture ages; future source edits still require their own
qualification. Native macOS, other consumers and production migration remain
separate acceptance work. None of these packages has been published by Tooling.
