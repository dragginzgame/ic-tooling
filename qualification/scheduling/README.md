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
Testkit implementation. The new Testkit 0.107.0 package currently admits client
types and reply catalogs only; it cannot yet replace these runtime fixtures.

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
