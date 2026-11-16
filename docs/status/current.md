# Current implementation — 2026-11-16

## Authority and delivery

The maintainer accepted [design revision 2](../DESIGN.md). Required outcomes drive
interfaces; old Rust exports, packages and release groups do not. Consumer code
stays in its owner; patches here are qualification migrations only.

Use current local sibling working trees, including edits, through
[read-only snapshots](../qualification/local-sources.md). Refuse capture drift or
stale patches; historical passing runs do not qualify changed source. Auth is on
hold; Storage, Observability and Wire are frozen. Canic is being rewritten and its
current requirements must justify further APIs.

`../ic-tooling` remains the read-only production reference, observed at
`a00369fc29fa837c6deb022cceb7e09aa447292c`. Superseded transplant/design evidence is
retained at `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`; preserve those deletions and
historical evidence. No old product source/test tree was admitted wholesale.

Previous GitHub checkpoint: `b195328a566cbb147646380e75ee10e8643eb4e0`.
This checkpoint adds the composed Canic preparation/recovery qualification below.
Runtime evidence remains tied to the captured source and suppliers in the linked
qualification recipes; old Timers experiments are historical. Seven [release groups](../release-ownership.md) own their ledgers; root changelog is
an index. All eight packages remain unpublished. Consumer migration, registry
publication and canonical cutover are separate; recheck registry identities first.
Toko 0.44.4 is pinned in both workspaces; snapshot/tools admission and 19 Build tests pass after renaming `qualification/owned-wasm-probe` to match its package.

## Owners and evidence entrypoints

Use `make deps-list` for local versions. Read only the contract affected by the task:

| Owner | Contract |
| --- | --- |
| Build | [Owned builds](../build-owned-artifact.md) |
| Artifacts | [Producer commitments](../byte-commitments.md) |
| FS | [Preparation](../directory-preparation.md), [publication](../directory-publication.md), [sidecar locks](../sidecar-locks.md) |
| HTTP | [Bounded bodies](../http-body.md) |
| Text | [Presentation](../text-presentation.md) |
| Process | [Execution](../process-execution.md), [handoff/reaping](../process-handoff.md) |
| Testkit | [Held replies](../testkit-held-replies.md), [simulator ownership](../simulator-ownership.md) |

`ic-cargo-qualification` is private external-lock subset admission, outside Build;
it is not an archive runner or metadata editor. HTTP/Tokio stay outside Build and
portable bytes. Testkit's thiserror 2.0.18 selection is scoped to that graph.

Consumer recipes retain source hashes, commands, results and limitations:
[Canic](../../qualification/canic/README.md),
[Query](../../qualification/query/README.md), [DB](../../qualification/db/README.md),
[Platform](../../qualification/platform/README.md),
[Scheduling](../../qualification/scheduling/README.md),
[Backup](../../qualification/backup/README.md) and
[Auth](../../qualification/auth/README.md).
Canic has Linux Build/Text/Process evidence; Query and DB have scoped Text evidence.
Scheduling's rewrite has paired held-reply, pressure, watchdog, Once, metrics-feature
and executor evidence; old Scheduling experiments do not qualify it. Platform's
old driver evidence must not drive the replacement. Auth evidence is historical
at `e52a40be75dd9809d0528189d84b43fe96952ad9`; its migration remains on hold.
Product and consumer test inventories stay in these existing contracts/recipes;
version bumps do not refresh them. No full release gate is claimed.

## Remaining gates and limitations

[Backup assembly](../../qualification/backup/README.md#complete-producer-commitments) passes four current tests with both suppliers and Clippy; restore remains blocked by its earlier 15 baseline failures.
Canic held preparation/publication now includes recovery and release writers: 47 paired cases plus two allocation identity/collision cases pass. Live signing/submission remains outside this evidence; artifact CLI passes separately. Next: artifact inspection and app-output path resolution.
The [native-execution review](../process-execution.md) records the shared Process
owner used by Build and current Canic group capture, foreground execution and
desktop handoff. Combined Linux qualification covers artifact/metadata, lock setup,
signer capture, Store-source lookup and viewer reaping; native macOS remains open.
No Canic product code moves here. `../ic-profiler` is absent and its current
checkout/status has been requested. The inspected `ic-platform-new` has no current
Tooling dependency; old Platform fixture results must not drive that rewrite.
Managed-server lifetime/cancellation, multi-canister partial snapshot effects,
remaining Platform drivers, Profiler ownership/native acceptance, archive acceptance,
native macOS qualification and production consumer migration remain open.
Borrowed-server success does not justify a wrapper; upstream creation can panic
or leave remote effects after cancellation, and async deletion is best-effort.
See [simulator ownership](../simulator-ownership.md).

These slices assume trusted source/tools, cooperative process groups, exclusive
child reaping and caller exclusion of namespace/content writers. Identity checks
are not hostile-writer atomic compare-and-unlink. App builds are serial, not an
all-or-nothing transaction. HTTP/installed fixtures require loopback. No production
consumer has migrated; no performance or peak-memory improvement is claimed. Shared-target reuse needs matched evidence.
