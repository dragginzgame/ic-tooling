# Current handoff — 2026-10-29

## Scope and source

The local fix for [#263](https://github.com/dragginzgame/ic-tooling/issues/263)
is complete and unstaged over admission HEAD
`cc9e544d1236c23c78d2732cfd04080e7e38052a` (Release 0.69.8). Pending compatible
notes are 0.69.9; package versions remain 0.69.8. No staging, commit, push, tag,
release, publication or tool download ran. The user's incoming Cargo.lock update
is preserved byte-for-byte. No unrelated Cargo/rustc process was visible at
admission. Sibling working checkouts and existing artifacts remain intact.

## Snapshot correction

PocketIC's native take/load semantics restore heap while retaining the current
global timer and low-memory-hook state. Testkit now converts captured snapshots
to metadata-upload semantics, transferring the module, globals, heap, stable
memory, certified data and chunk store together with timer/hook state. Transfers
use bounded management messages without a shared server filesystem. Missing
timer/hook metadata, incomplete regions and invalid chunk-store data refuse
capture. Initial capture needs an extra temporary snapshot slot, memory and
cycles; warm loads reuse the uploaded snapshot.

Exact capture senders and restore funding remain intact. Rollback attempts all
known snapshot deletions, preserving primary failures and failed cleanup IDs.
Lost creation replies can leave unknown IDs requiring caller reconciliation.
Typed rejection and transport causes retain the pool's fatal-error and single
dead-transport-rebuild policy. This fixes test baselines; it does not qualify
production backup or cross-version recovery.

Callers must stop/drain the whole target set before capture or restore, disable
automatic progress, exclude concurrent operations, and restart only after the
whole capture or restore/reset succeeds. Generic snapshot helpers do not own
canister lifecycle. The standalone recipe, benchmark workload and maintained
multi-canister example implement that contract. Absolute timer deadlines are
restored unchanged; current simulated time is not rewound.

## Evidence and limits

Evidence is retained at `/tmp/ic-tooling-263.WQ4ch3/`, selected by
`/tmp/ic-tooling-263-current`. It includes admission source/lock/diff, the previous
handoff, commands, attempted and final checks, benchmark JSON, and final source
capture. Earlier failed compiler/lint attempts remain alongside corrections.

Actual Rust 1.91 Linux focused tests pass: 60 cases spanning snapshot rollback,
slot refusal, exact senders/funding, timer/hook/heap/stable/chunk-store restoration,
pool acquisition/recovery, standalone lifecycle and driver reporting. Pool
coverage includes 100 consecutive restores and both explicitly selected
dedicated-server transport recovery cases. Live cases use the existing PocketIC
16.1.0 binary admitted offline. The final timer regression fails against the
preserved admission implementation and passes with this fix.

The actual benchmark driver completes two fresh and two pooled tasks with a
single worker and capacity one, including cold capture and warm restore. This is
a functional smoke test, not a performance conclusion. Warning-denied all-target,
all-feature Clippy passes for Testkit and Testkit Dev on prepared Rust 1.99.1.
Formatting, local documentation links and diff checks pass. No full workspace or
release gate ran. Native macOS 15 Intel and Apple Silicon qualification did not
run; Linux evidence is not native macOS evidence.

Issue #263 remains open pending delivery and consumer qualification. After a
corrected publication, Canic needs adoption and the lifecycle contract in both
Root baseline owners, then its retained four-case sequence must pass together.
Read-only Canic inspection does not qualify that consumer or authorize its
release. Its diagnostic work and evidence remain untouched.

## Preserved earlier evidence

The previous cleanup is released in admission HEAD. Its evidence remains at
`/tmp/ic-tooling-cleanup-0698.LehyNb/`; earlier records remain under
`/tmp/ic-tooling-cleanup-0697.OHn8IL/`, `/tmp/ic-tooling-262.zTQbiB/`,
`/tmp/ic-tooling-cleanup-0696.7aCypD/` and `/tmp/ic-tooling-deps-0696.OJreg2/`.
Those qualification and supplying-owner obligations retain their recorded scope;
this fix adds no real optimizer or native macOS qualification. Shared Tooling
stays at revision `d08cbcafbaff4227e798a890232e2248bf167504` (144 files).

Migration/retirement backups remain under `/tmp/ic-tooling-migration/`. Original
Host archive refs, `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans,
failed attempts and earlier backup/stash records remain preserved through the
previous handoffs. No cleanup or external issue-state change ran.
