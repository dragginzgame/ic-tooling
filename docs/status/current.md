# Current implementation — 2026-11-15

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

GitHub checkpoint `607ff8aaa39206952bb2fef17a0f67ad8f071b16` is on `origin/main`.
Post-push review of the delivered adapters and qualification wiring found no new
defect. Process 0.80.2 implements [handoff/reaping](../process-handoff.md): 14 Process
and 19 Build tests pass, plus Clippy/Rustdoc. Current Canic's incomplete dependency
rename blocks viewer replacement qualification. Consumer runtime evidence remains tied
to the versions and snapshots recorded below; old Timers experiments are historical.
Seven [release groups](../release-ownership.md) own their ledgers; root changelog is
an index. All eight packages remain unpublished. Consumer migration, registry
publication and canonical cutover are separate; recheck registry identities first.
Toko 0.44.4 is pinned in both workspaces; snapshot/tools admission and 19 Build tests pass after renaming `qualification/owned-wasm-probe` to match its package.

## Implemented owners

- `ic-build` 0.87.3 owns bounded Cargo execution/decoding, exact selection, owned
  Wasm bytes and explicit cleanup. Private targets are mode 0700. No mutable
  Cargo path escapes. See [owned builds](../build-owned-artifact.md).
- `ic-host-artifacts` 0.80.1 owns bounded complete-stream read/copy and identity,
  independently of native filesystem/process/HTTP dependencies.
- `ic-host-fs` 0.80.1 owns held directories/files, bounded enumeration, cooperative
  locks and stage/write/seal/commit. Preserve retained failures and visible-but-
  unconfirmed effects; identity-checked removal still needs caller exclusion.
  See [custody](../filesystem-custody.md).
- `ic-host-http` 0.80.0 owns complete bounded response bodies, typed original
  transport/allocation causes and capped fallible growth. Reqwest/TLS/request
  policy stays consumer-owned. See [HTTP](../http-body.md).
- `ic-host-text` 0.77.2 owns escaping and bordered/plain tables with scalar Unicode
  widths. Query/DB use borders; Canic adds plain alignment. No progress, clipping
  or terminal policy. See [Text](../text-presentation.md).
- `ic-cargo-qualification` 0.1.2 is an external-lock subset admission executable,
  outside Build's graph. It is not an archive runner or metadata editor.
- `ic-host-process` 0.80.2 owns synchronous group/direct-child execution and bounded
  observer delivery. Build uses its core; eight native tests, the 19 existing Build
  tests and Linux terminal fixtures pass, including current Canic's foreground
  client with a harmless tool. Signer capture passes four tests; lock initialization
  passes diagnostic and app-build/refusal checks. See [execution](../process-execution.md).
- `ic-testkit` 0.107.1 implements the [shared Testkit contract](../testkit-held-replies.md):
  original PocketIC types and an owned update-reply catalog with complete admission.
  Bounded module generation and exclusive pending/release control pass focused
  installed tests and current Scheduling pressure/held-upgrade qualification. The new
  dependency selects thiserror 2.0.18; prior external tuples otherwise remain intact.

## Latest scoped evidence

Source heads, tar hashes, logs and exact commands remain in the linked recipes.
The next-version changes alone do not refresh earlier runtime evidence.

- [Canic](../../qualification/canic/README.md): current app-build proof passes
  duplicate-output preflight, config-relative recipes, lock initialization and
  preservation, exact output and serial partial failure. A fresh metadata slice
  passes a real-build control and ten pre-compilation refusals, including workspace
  membership and relative target paths. A newer Canic 0.210.0 snapshot with Build
  0.87.3 also passes six prepared-tool/wrapper refusals without executing wrappers
  or replacing output. Host/CLI Clippy passes for that snapshot. A later artifact
  run preserves symlink/hardlink/dangling outputs on compilation failure, then
  replaces them on success without mutating foreign bytes or creating missing
  referents. Exact identity, publication refusal and scratch cleanup pass.
  Earlier independent slices cover artifact default/explicit publication and
  refusal, 19 display tests, and six record-admission/recovery tests. Current Canic
  0.213.0 passes combined Build/Text/Process lock, signer, foreground/PTY and app checks:
  47 tests, 20 metadata/tool refusals and Clippy pass with metadata on local Process. Host publication
  and parent creation still use retained suppliers outside this migration.
  Retained desktop opener/viewer passes four tests; handoff/reaping design remains open.
- [Query](../../qualification/query/README.md): refreshed current source passes
  20 presentation/output tests with Text 0.77.1, including placements partial JSON
  and rejection escaping. CLI/test Clippy and retained external-lock admission pass.
- [DB](../../qualification/db/README.md): console schema/query/cursor/refusal proof
  passes with Text; all 30 instances report deletion with the external server alive.
  Earlier rollback evidence covers nine modes, raw stable bytes, normal-error
  committed mutation and explicit endpoint refusal. These are different snapshots.
- [Platform](../../qualification/platform/README.md): raw_rand and Agent use direct
  PocketIC. Codec rejection/cleanup/cancellation and four identity/certificate
  cases pass, including Principal mismatch refusal and observed instance deletion.
  Other Platform drivers remain unqualified; no new simulator product exists.
- [Scheduling](../../qualification/scheduling/README.md): active source is now
  `../ic-scheduling-new`. Its executor/Once drivers require the Testkit client
  boundary and held-reply fixtures. Do not remove that dependency as a design goal.
  Old Build/Jobs/Timers migrations and native-snapshot experiments target the old
  repository and are historical only. The [Testkit contract](../testkit-held-replies.md)
  is defined. Focused pressure/held-upgrade tests pass with released Testkit, as do
  endpoint refusal, observed instance deletion and driver Clippy. Replacement
  controller passes independent identity, expiry, repeated release and progress
  refusals. The 0.107.1 replacement export now passes the same pressure/held-upgrade
  assertions, exact supplier selection, retained external locks, driver Clippy,
  endpoint refusal and observed instance deletion. A later focused slice passes
  watchdog admission/pressure and Jobs pressure with released and replacement
  suppliers; separate source captures differ only in unselected retention fixtures.
  A single-capture comparison also passes watchdog trap/owner-command behavior,
  refusal backoff and malformed/stale-envelope upgrade reconstruction with both
  suppliers. Both instances are observed deleted; no new product API was needed.
  The synchronous executor also passes three lifecycle and three admission cases
  with both suppliers on one capture. Its selected replacement graph excludes
  optional fixture dependencies; endpoint refusal and instance deletion pass.
  Other modes are not qualified; see the Scheduling record for exact identities.
- [Backup](../../qualification/backup/README.md): current-source byte/filesystem
  and HTTP migrations preserve metadata, transfer binding, budgets and evidence.
  The 65 custody/payload cases and 13 HTTP cases have separate recorded runs.
  Legacy public digest values/in-memory hashing remain a consumer-contract decision.
- [Auth](../../qualification/auth/README.md): pinned archive/external-lock evidence
  is historical at `e52a40be75dd9809d0528189d84b43fe96952ad9`; do not continue it.

Product evidence: six Testkit unit tests and two installed fixture/controller
tests pass, plus default/feature Clippy and docs; see the Testkit contract for scope.
19 Build tests; two byte tests and Wasm Clippy; ten filesystem
failure tests; two HTTP body tests; three Text escaping/layout tests. Relevant
Clippy, Rustdoc, formatting and Toko admission pass as recorded by their slices.
HTTP/Tokio stay outside Build and portable bytes. No full release gate is claimed.

## Remaining gates and limitations

Continue from current unfrozen workloads in the [register](../workloads.md).
The [native-execution review](../process-execution.md) identifies current Canic
group-capture and foreground-child requirements that Build's private runner cannot
serve unchanged. The shared core passes cleanup/lifetime tests and current Canic
artifact/metadata and foreground-client qualification. Signer group capture remains open;
no Canic product code moves here. `../ic-profiler` is absent and its current
checkout/status has been requested. The inspected `ic-platform-new` has no current
Tooling dependency; old Platform fixture results must not drive that rewrite.
Managed-server lifetime/cancellation, multi-canister partial snapshot effects,
remaining Platform drivers, Profiler ownership/native acceptance, broader Scheduling
acceptance, archive acceptance and native macOS qualification remain open.
Borrowed-server success does not justify a wrapper; upstream creation can panic
or leave remote effects after cancellation, and async deletion is best-effort.
See [simulator ownership](../simulator-ownership.md).

These slices assume trusted source/tools, cooperative process groups, exclusive
child reaping and caller exclusion of namespace/content writers. Identity checks
are not hostile-writer atomic compare-and-unlink. App builds are serial, not an
all-or-nothing transaction. HTTP/installed fixtures require loopback. No production
consumer has migrated; no performance or peak-memory improvement is claimed.
Shared-target reuse still needs matched evidence.
