# Historical Scheduling evidence — superseded consumer selection

These experiments used the pre-rewrite Scheduling source then located at
`../ic-scheduling`. The current rewrite later moved to that same path from
`../ic-scheduling-new`; paths alone do not identify qualified source.
See [current requirements](README.md).

The old executable lane, four migration patches and two Jobs/Timers session
helpers were removed under [#4](https://github.com/dragginzgame/ic-tooling/issues/4).
There are no supported commands for replaying these experiments. The records
below preserve source identity, observations and limitations, not a compatibility
requirement or instructions to restore the old architecture. Original adapters
remain in Git history before their removal.

## Source and owned-build observations

The captured source HEAD was `0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`;
installed comparisons below used source tar SHA-256
`87a57d0902d16180c5fb358485953130ac96f08519087be2f20f20924779f146`.
No sibling source or product API was changed by those experiments.

The Timers build adapter selected local Build 0.87.1 and replaced manual Cargo
execution, artifact selection and capture with one owned-build operation.
Sixteen host checks and two installed subjects covered watchdog trap/upgrade and
Wire rejection before timer mutation. Jobs had five capture checks and installed
management/upgrade/watchdog checks for original Wasm and real Binaryen `-O3`,
`-Os` and `-Oz` captures. The [private observation design](observations.md)
records the receipt identity migration and conservative comparison boundary.

Jobs used held filesystem staging, retained complete `.capture.pending` bytes on
publication failure, and distinguished staging from visible-but-unconfirmed
publication. Producer identity controlled cleanup. These observations did not
establish full supplier cutover, native macOS acceptance, cohort comparability
or shared-target performance.

## Installed observations on 2026-11-15

All paths below are retained evidence locations, not active qualification inputs.

| Experiment | Export and evidence | Observed result |
| --- | --- | --- |
| Jobs controller recovery | `target/scheduling-consumer.APxGwn5w`; `target/jobs-snapshot-live.log`, `target/jobs-session-clippy.log` | Three subjects passed: watchdog delivery, synchronous trap rollback and controller snapshot recovery. All three instances were observed deleted; focused Clippy passed. |
| Timers borrowed server | `target/scheduling-consumer.WO25qvjZ`; `target/timers-session.log`, `target/timers-session-clippy.log` | Three installed subjects and missing-endpoint refusal passed. Three instances were observed deleted; focused Clippy passed. Receipt: `runtime/timer-qualification-1443319-0/runtime.json`. |
| Native snapshot negative control | `target/scheduling-consumer.N4k5TYay`; `target/timers-native-snapshot.log`, `target/timers-native-snapshot-clippy.log` | Unchanged restore passed; the first mutated restore trapped with `ic-timers: competing global timer owner`. Inventory remained `["Available"]` until outer-server teardown. Focused Clippy passed. |
| Uploaded snapshot comparison | `target/scheduling-consumer.7gD6PKbP`; `target/timers-uploaded-snapshot.log`, `target/timers-uploaded-snapshot-clippy.log` | Unchanged and two mutated restores passed, with three instances observed deleted and missing-endpoint refusal. Focused Clippy passed. Receipt: `runtime/timer-qualification-1502449-0/runtime.json`. |
| Direct-client completion | `target/scheduling-consumer.LT6i42GD`; `target/timers-direct-final.log`, `target/timers-final-clippy.log` | Runtime composition, watchdog, invalid ingress and repeated snapshot restoration passed. Five instances were observed deleted. |
| Final negative control | `target/scheduling-consumer.LbnVVWDG`; `target/timers-direct-negative.log`, `target/timers-final-negative-clippy.log` | Native restoration still trapped at the first mutated restore. Both final graphs contained PocketIC 16.1.0 and no Testkit library. External-lock admission passed in `target/timers-final-lock.log`. |

Jobs retained caller-owned simulated time, bounded progress and controller
selection. Only install-rate-limit rejection admitted its existing bounded
upgrade retry. Non-controller snapshot capture/restore refused without changing
raw stable memory. These were single-canister observations.

The positive Timers experiment stopped the canister, took and downloaded a native
snapshot to private local files, then uploaded it as a new PocketIC snapshot.
It deleted the native source snapshot and local transfer files before restoring
the uploaded snapshot. Heap observations and real callbacks survived unchanged
and mutated restores without resetting timers or replacing the instance. The
uploaded snapshot was deleted before instance teardown. This was evidence for
a server sharing the diagnostic filesystem, not a remote-server guarantee.

The final direct-client experiment preserved 60-second creation/request limits,
application topology, funding, anonymous callers and bounded ticking. Explicit
async deletion was followed by independent server inventory. None of these runs
proved cleanup after every panic/cancellation, multi-canister snapshot atomicity,
remote transfer bounds or a requirement to remove Testkit from the rewrite.
See [simulator failure boundaries](../../docs/simulator-ownership.md).
