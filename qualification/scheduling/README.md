# Scheduling rewrite: current consumer requirements

The maintainer selected `../ic-scheduling-new` as the active Scheduling consumer.
`make test-scheduling-consumer` snapshots that working tree, including tracked edits
and non-ignored source, with the shared read-only capture helper. It applies no
old Scheduling migration patch and does not modify the sibling. An explicit
`SCHEDULING_SOURCE` override must satisfy the same rewrite package/graph checks.

The current target admits the locked dependency graph and checks only the host
`once-driver` and `executor-driver` targets offline. Both must select Testkit
0.106.0, with the same supplier package ID; Once enables `held-reply-fixtures`.
Neither driver may depend directly on PocketIC. This is source/graph and caller
compilation evidence, not installed-runtime proof or acceptance of a replacement
Testkit implementation. `make test-scheduling-replacement` separately selects
local Testkit 0.107.1 in the captured export and runs the focused installed checks
described below. It does not change the sibling's supplier selection.

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

The fixtures use `RawReplyMethod`, `RawReplyLimits`, `ReplyMode` and
`HeldReplyFixture::{new, wasm, pending, release}`. Limits, funding and request
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

[Earlier Scheduling experiments](historical.md) used `../ic-scheduling`. They
retain their source IDs, logs and patches, but do not qualify this rewrite or
justify its architecture. Their direct-client migration is not the accepted target.
The active Makefile no longer exposes the old Jobs snapshots/Timers session/native
snapshot commands. `scheduling-historical.sh` retains reproduction only with
`HISTORICAL_SCHEDULING_EVIDENCE=1` and an explicit old `SCHEDULING_SOURCE`; it is not
called by an active target. Existing prepared binaries in the old checkout can
remain tool assets for other consumers; that does not select its source for this
rewrite's qualification.

## Initial corrected-source evidence

On 2026-11-15 the runner captured HEAD
`7715fc81d533bbd9bed613a62c80f6f9d18a4a37` plus local edits, source tar SHA-256
`229d27dd3a9cbaa07b8bc21bef7d575223abeb6dd581cfea79e6571411ac521f` in
`target/scheduling-new.qz5U96ZW`. Testkit graph admission and both host-driver
checks passed (`target/scheduling-new-admission.log`). An explicit override to
old `../ic-scheduling` refused the rewrite source contract before driver compilation
(`target/scheduling-old-source-refusal.log`); the historical runner refuses an
unqualified invocation (`target/scheduling-history-guard.log`). This did not run
PocketIC, deploy canisters, modify sibling files, or qualify any new Testkit API.

## Focused held-reply baseline and next contract

The [first Testkit contract](../../docs/testkit-held-replies.md) defines shared
ownership, bounded catalog/module admission, opaque request identity, exclusive
release authority and uncertain-effect handling. It is an implementation contract,
not a claim that the replacement runtime fixtures or all their failure tests exist.

Run `make test-scheduling-held` with `SCHEDULING_POCKET_IC_BIN` set to an
absolute executable path for a prepared local server. The optional
`SCHEDULING_TESTKIT_CLI` selects the released 0.106.0 launcher; otherwise the
installed Toko checker selects its already-prepared Scheduling tool receipt.
That old checkout supplies only a tool asset, never consumer source. No tool is
installed. This recorded run uses server 16.1.0; the rewrite's earlier 16.0.0
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

Next: implement the small shared owner and its independent failure cases, then
run this same consumer selection against the replacement. Do not copy the old
fixture or test tree wholesale, or expand into the full installed roster.

## Replacement Testkit 0.107.1 qualification

`make test-scheduling-replacement` captures current rewrite source, admits its
released graph, then applies `held-selection.patch` and
`replacement-testkit.patch` only to that export. The latter changes the supplier
to this checkout and replaces old fixture descriptors/constructors with
`held::{Catalog, Reply, Limits, Fixture}`. It keeps consumer assertions, funding,
installation, tick budgets and release order intact. Principal conversion uses
canonical text at the native-client/Platform identity boundary. No consumer code
is added to the product Testkit crate.

The complete Once driver must compile, so the patch adapts Jobs-pressure and
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

This qualifies an unpublished replacement in a disposable current-source export.
The sibling remains unchanged and continues selecting released Testkit. Broader
Once/Jobs/watchdog/calendar execution, macOS and production cutover remain separate.

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
