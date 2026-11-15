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

GitHub checkpoint `1f288290269b7ca063e81f80f508856d9fe3aa00` is on `origin/main`.
Post-push review found no new actionable issue in version wiring and tool-admission
qualification. The working tree was clean after delivery. Build 0.87.3, Text 0.77.2
and Tools 0.1.2 remain unpublished drafts; local follow-up adds output-alias
qualification and a combined Canic Build/Text app workflow. Scheduling source
selection is corrected to the rewrite; old Timers experiments are historical.
Foundation and HTTP are unchanged. Runtime evidence remains tied
to the versions and snapshots recorded below.
Six [release groups](../release-ownership.md) own their ledgers; root changelog is
an index. All seven packages remain unpublished. Consumer migration, registry
publication and canonical cutover are separate; recheck registry identities first.
Toko 0.44.3 is pinned in both workspaces; its 48-file snapshot is verified.

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

- `ic-testkit` 0.107.0 starts the [shared Testkit contract](../testkit-held-replies.md):
  original PocketIC types and an owned update-reply catalog with complete admission.
  Bounded module generation and exclusive pending/release control pass focused
  installed tests; consumer replacement qualification remains open. The new
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
  refusal, 19 display tests, and six record-admission/recovery tests. Other Host
  publication, parent creation and lock setup remain unmigrated. Recipes are not
  implicitly combined except the app target: a fresh Build 0.87.3/Text 0.77.2
  graph passes app checks, thirteen fleet/placements tests and Host/CLI Clippy.
  Changed sibling contexts can invalidate individual patches.
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
  refusals. The replacement is present but not selected here yet.
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
Managed-server lifetime/cancellation, multi-canister partial snapshot effects,
remaining Platform drivers, Profiler ownership/native acceptance, full watchdog
comparability, archive acceptance and native macOS qualification remain open.
Borrowed-server success does not justify a wrapper; upstream creation can panic
or leave remote effects after cancellation, and async deletion is best-effort.
See [simulator ownership](../simulator-ownership.md).

These slices assume trusted source/tools, cooperative process groups, exclusive
child reaping and caller exclusion of namespace/content writers. Identity checks
are not hostile-writer atomic compare-and-unlink. App builds are serial, not an
all-or-nothing transaction. HTTP/installed fixtures require loopback. No production
consumer has migrated; no performance or peak-memory improvement is claimed.
Shared-target reuse still needs matched evidence.
