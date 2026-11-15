# Historical Scheduling evidence — superseded consumer selection

These runs target `../ic-scheduling`, not the selected `../ic-scheduling-new`.
They are historical experiments, not current rewrite acceptance or a requirement
to remove Testkit. The old Make target names below describe recorded commands;
see [current ownership and reproduction boundaries](README.md).

# Scheduling owned-build qualification

Current recipes select the next unpublished Build 0.87.3 / Text 0.77.2 where
used. Their patches and locked graphs admit current local sources; historical
runtime evidence below remains bound to its recorded checkpoint inputs.


`make test-scheduling-consumer` snapshots the current local `SCHEDULING_SOURCE`
(default `../ic-scheduling`) without modifying it, including local edits and
non-ignored additions, then applies
[the migration](owned-build.patch), and verifies that the Timers driver resolves
its build dependency to local `ic-build` 0.87.1, with no legacy Build anywhere in
the graph. No sibling edits are made. Sixteen Timers host tests (including three new
observation tests) and two exact installed subjects run in that export:
watchdog survival through trap/upgrade, and Wire rejection before timer mutation.
Jobs adds five capture tests and installed management/upgrade/watchdog qualification
of the original Wasm and real Binaryen `-O3`, `-Os`, `-Oz` captures.
This is focused installed evidence, not the complete watchdog/cohort release roster.

See [source selection](../../docs/qualification/local-sources.md). The current
checkout is clean at `0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`; its patch
still applies, so the prior installed evidence remains relevant to unchanged
inputs. No new installed run was performed for this source-selection change.

Prepare Rust 1.100.0, its Wasm target and the consumer's locked dependencies first.
The selected source checkout also needs its prepared Binaryen tool bundle; the
local export's Toko admission script checks it offline without installing anything.
Select an already prepared `ic-testkit-server` 0.106.0 CLI and its asset directory:

```sh
SCHEDULING_TESTKIT_CLI=/absolute/path/to/ic-testkit-server \
SCHEDULING_SERVER_DIRECTORY=/absolute/path/to/ic-testkit-server-assets \
make test-scheduling-consumer
```

The CLI checks the server bytes offline before use. PocketIC requires local
loopback access. `SCHEDULING_SOURCE` may select another current local checkout;
its snapshot must admit the migration patch. Scratch, original diagnostics, captured Wasm and runtime
receipts remain under the printed `target/scheduling-consumer.*` export; Cargo's
host cache is under `target/scheduling-consumer-build`. No implicit tool download,
registry publication or protected release artifact is involved.

The consumer now calls one owned build instead of creating/locking a shared target,
collecting every Cargo message, separately checking execution/protocol, selecting a
mutable path and capturing it afterward. The complete owned bytes are published
into the invocation's diagnostic directory for the existing installed harness.
Source/configuration drift checks, before/after tool identities, exact installed
artifact hashes, subject admission and receipt validation remain in that driver.
No original test bodies or supplier implementation are copied into new products.

The dependency uses its canonical `ic-build` name. Timers owns its private bounded
configuration admission and receipt evidence; the old generic observation/selection
types and supplier are removed. See the [observation contract](observations.md),
including the explicit recipe-identity migration and conservative comparison rules.
Jobs capture uses held filesystem staging, exact byte admission and explicit
producer cleanup. Registry Host and Testkit remain unmigrated.
There is one Timers compile/capture path, with no fallback
to the old execution API. This does not establish complete Scheduling supplier
cutover, cohort comparability, native macOS acceptance or shared-target performance.

PocketIC 16.1.0 pins `thiserror` to 2.0.18. Product requirements now admit that
compatible version; the product lock still selects 2.0.21. The consumer lock keeps
retained registry versions/checksums, adds the local suppliers plus `const-oid`,
and removes the unused old Build graph. The private parser uses the already locked
TOML library directly. Full Stage 3 still includes Auth/Storage and Canic workflows;
remaining Host/Testkit responsibilities stay explicit. Shared incremental targets
need measured justification first.

Jobs retains complete `.capture.pending` bytes when destination publication fails;
its error identifies staging versus visible-but-unconfirmed destination outcomes.
Successful capture retires only the observed producer identity and keeps the log.
Existing recovery staging refuses further capture until the caller reconciles it.
The CLI and optimizer flags are unchanged. Source/destination parents passed to
external tools remain stable caller-owned paths. The migration declares explicit
product workspace roots in the export to disambiguate Cargo inheritance with
local suppliers from another workspace; sibling manifests are untouched.

## Direct Jobs session and controller recovery

Apply `jobs-session.patch` after `owned-build.patch`. Jobs selects upstream
PocketIC 16.1.0 directly and requires `IC_TESTKIT_POCKET_IC_URL`; it cannot select
or spawn a server. Timers retained released Testkit at that checkpoint; its later
direct-client slice is documented below. The runner checks the
Jobs reachable dependency graph separately, since workspace metadata includes
Timers. No shared simulator crate or snapshot manager was introduced.

With the same prepared `SCHEDULING_TESTKIT_CLI` and
`SCHEDULING_SERVER_DIRECTORY`, run `make test-jobs-snapshots`. This focused mode
skips Timers and optimization variants, builds current Jobs Wasm privately, and
runs exactly the existing watchdog-delivery, synchronous-trap and controller
snapshot-recovery subjects. Missing named tests fail before execution. It
independently checks that all three instances report deletion while the external
server stays reachable. The released CLI remains an explicit development server
owner, not a migrated product implementation.

The consumer retains initial-predicate/30-tick bounded progress, explicit simulated
time changes and a four-attempt recovery upgrade budget. Only install-rate-limit
rejection permits the existing 600-second/two-tick cooldown. Snapshot stop, take,
restore and start select the default anonymous controller explicitly. The recovery
subject additionally refuses a non-controller for capture and restore, preserves
raw stable memory after those refusals, and compares restored bytes exactly with
the original independent snapshot. It then upgrades and checks pending job state.
These are single-canister observations, not multi-canister atomicity or cleanup
promises after cancellation; upstream limitations are in
[simulator ownership](../../docs/simulator-ownership.md).

Linux evidence on 2026-11-15: snapshot HEAD
`0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`, source tar SHA-256
`87a57d0902d16180c5fb358485953130ac96f08519087be2f20f20924779f146`,
export `target/scheduling-consumer.APxGwn5w`. All three live subjects and inventory
checks passed (`target/jobs-snapshot-live.log`). Harness all-target Clippy with
warnings denied passed (`target/jobs-session-clippy.log`); the same source and
patch were used. The first sandbox run built/captured Wasm but could not bind
loopback; only the live step was repeated with loopback permission. Product APIs,
frozen siblings and production Scheduling were not modified.

## Timers borrowed-server qualification

`make test-timers-session` selects the existing owned-build and Jobs migrations,
then applies `timers-session.patch` to the current Timers harness. Use the same
prepared `SCHEDULING_TESTKIT_CLI` and physical `SCHEDULING_SERVER_DIRECTORY` paths
as above. It does not run Jobs' optimizer roster or the full Timers suite.

The outer released CLI owns one server process and its retained logs. Each Timers
subject requires `IC_TESTKIT_POCKET_IC_URL`, creates its own application-subnet
instance with the existing 60-second startup/request limits, and explicitly closes
that instance. The startup patch removes per-subject server spawning, log-directory
allocation and server TTL selection. The current patch also removes Testkit
instance/tick/install helpers and selects PocketIC 16.1.0 directly, including
explicit creation timeout and asynchronous deletion. The runner checks the normal,
build and dev graph contains PocketIC and no Testkit library. No product simulator
API is introduced. The current positive target additionally applies
`timers-uploaded-snapshot.patch`, replacing cached-baseline helpers with the
upstream download/upload operations described below. Its existing provenance recorder still hashes the
prepared `POCKET_IC_BIN` even though the harness does not spawn it.

The runner checks missing-endpoint refusal with an unavailable binary before any
remote instance exists. The unchanged consumer driver then selects exactly the
Runtime composition, trap/upgrade watchdog, invalid-Wire ingress and
cached-baseline restoration subjects. Runtime composition uses two instances to
exercise both plain and prepared calls through the migrated tick-observation loop. The last subject repeats unchanged and mutated restores in one instance,
checks timer ownership and observes actual callbacks after restoration. An
independent `/instances` query must report five deleted instances while the outer
server remains reachable. This observes successful cleanup for this run; it does
not establish deletion on panic/cancellation or multi-canister atomic restoration.

Linux evidence on 2026-11-15: source HEAD
`0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`, source tar SHA-256
`87a57d0902d16180c5fb358485953130ac96f08519087be2f20f20924779f146`, export
`target/scheduling-consumer.WO25qvjZ`. All three exact installed subjects and the
missing-endpoint refusal passed (`target/timers-session.log`); inventory reported
`["Deleted","Deleted","Deleted"]`. Focused runtime-test warning-denied Clippy
passed (`target/timers-session-clippy.log`). The consumer retained its source/tool
receipt at `runtime/timer-qualification-1443319-0/runtime.json` inside the export.
Loopback access was required. No sibling source or product API changed.

## Native snapshot negative probe

`make test-timers-native-snapshot` uses the same prepared tools as
`test-timers-session`, but uses the direct `timers-session.patch` without the
uploaded-snapshot fix in its isolated export. This selects native PocketIC
take/load snapshot calls in the existing timer-restoration subject.
All other harness, funding, caller, callback and instance choices stay the same.
The ordinary Timers target uses the passing upstream download/upload preparation
described below; the negative probe deliberately omits that additional patch.

This target succeeds only when the attempted migration is rejected at restore
cycle 1 after mutation with `ic-timers: competing global timer owner`. A generic
compile/test failure is insufficient, and an unexpected successful run requires
reassessment. The failed test can leave its instance alive; the outer CLI owns
server-process teardown. This negative probe claims neither explicit instance
deletion nor panic/cancellation recovery. See the [snapshot requirement](../../docs/simulator-ownership.md#native-snapshots-are-not-yet-a-timers-replacement).

Linux evidence on 2026-11-15: the same Scheduling HEAD and tar hash as the
borrowed-server proof above, export `target/scheduling-consumer.N4k5TYay`.
The targeted negative gate passed (`target/timers-native-snapshot.log`); detailed
expected failure is in `native-snapshot.log` inside that export. The unchanged
restore succeeded before the first mutated restore trapped. Inventory was
`["Available"]` before outer-server teardown, not a deleted-instance result.
Runtime-test warning-denied Clippy passed (`target/timers-native-snapshot-clippy.log`).
An earlier broader direct-client experiment reproduced the same trap; the retained
probe narrows the evidence to the snapshot substitution alone. No product or
sibling source changed, and no snapshot subsystem was copied into product crates.

## Upstream uploaded-snapshot fix

The positive Timers target applies `timers-uploaded-snapshot.patch`: take a native
snapshot while stopped, download it through PocketIC into a uniquely allocated
private directory, and upload it through PocketIC as a new snapshot. Native source
snapshot and local transfer files are removed before restoration. The test loads
the uploaded snapshot for the unchanged control and both mutated restores, checks
heap observations, then rearms and observes real timer callbacks in the same
instance. No timer reset, replacement instance, funding top-up or weakened assertion
is introduced. The uploaded snapshot is explicitly deleted before instance close.

This is a qualified fix for the local server sharing the diagnostics filesystem.
PocketIC owns serialization and transfer operations; no legacy snapshot code or
new product API is imported. The selected anonymous sender is the controller.
Remote-only servers, upstream transfer bounds, failed transfer cleanup and
panic/cancellation recovery remain separate obligations. Transfer failures may
retain snapshot IDs or files for reconciliation; this fixture does not claim the
legacy generic helper's rollback guarantees. Plain native take/load remains the
negative control and is still not equivalent.

Linux evidence on 2026-11-15: same recorded source HEAD and tar hash, export
`target/scheduling-consumer.7gD6PKbP`. All three installed subjects, including the
unchanged and two mutated restores, passed (`target/timers-uploaded-snapshot.log`).
Inventory confirmed three deleted instances with the external server alive;
missing-endpoint refusal also passed. Focused runtime-test warning-denied Clippy
passed (`target/timers-uploaded-snapshot-clippy.log`). The receipt remains at
`runtime/timer-qualification-1502449-0/runtime.json` inside the export. This evidence
replaces the snapshot-helper requirement only for this local qualification path.

## Direct-client completion

The current `timers-session.patch` also replaces Testkit instance/tick/install
calls with direct PocketIC use across the runtime fixture. Each caller owns a
`PocketIc` value, borrows it for operations and explicitly awaits deletion. The
harness keeps 60-second creation/request limits, application topology, funding and
anonymous callers. It ticks once before every observation within the original
positive round budget and avoids changing time for zero-duration advances. The
uploaded-snapshot patch applies on top; the negative control uses the base patch
alone. The superseded separate native-snapshot patch is removed, leaving one owner
for each transformation. The released CLI remains only the external bootstrap.

Fresh Linux evidence on 2026-11-15 uses the same source HEAD/tar hash above.
Positive export `target/scheduling-consumer.LT6i42GD` passes four exact subjects:
Runtime composition (plain/prepared calls), watchdog trap/upgrade, invalid ingress
and repeated snapshot restores. Inventory reports five deleted instances while
the server is alive (`target/timers-direct-final.log`). The negative control in
`target/scheduling-consumer.LbnVVWDG` still traps at the first mutated restore
(`target/timers-direct-negative.log`). Both graphs contain PocketIC 16.1.0 and no
Testkit library, including dev dependencies. Both focused runtime Clippy checks
pass (`target/timers-final-clippy.log`, `target/timers-final-negative-clippy.log`).
External-lock subset admission passes (`target/timers-final-lock.log`): retained
registry tuples are unchanged and unused Testkit suppliers disappear. These
focused subjects do not qualify the whole runtime/cohort roster or improve the
upstream panic/cancellation/deletion guarantees. Sibling source stays unchanged.
