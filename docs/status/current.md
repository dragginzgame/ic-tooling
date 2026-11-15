# Current implementation — 2026-11-15

## Scope and source authority

The maintainer accepted [design revision 2](../DESIGN.md) and staged implementation.
Required outcomes drive interfaces; old Rust exports and crate topology do not.
Current contracts: [owned builds](../build-owned-artifact.md),
[filesystem custody](../filesystem-custody.md), [HTTP bodies](../http-body.md).
The [workload register](../workloads.md) preserves remaining obligations.

Active qualification uses current local sibling working trees, including edits,
through [read-only snapshots](../qualification/local-sources.md). Source drift or
stale patches refuse; historical pinned runs do not qualify changed current source.
Consumer code stays in its owner; patches here are qualification adapters only.
Auth is on hold; Storage, Observability and Wire are frozen. Canic is being rewritten:
its current requirements, not historical implementation, must justify further APIs.

`../ic-tooling` remains a read-only production reference, observed at
`a00369fc29fa837c6deb022cceb7e09aa447292c`. Superseded transplant/design evidence
is retained in Git at `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`;
`ci/extraction-sources.json` is historical. Preserve the prior copied-tree deletions.
No old product source/test trees were admitted into the replacement.

GitHub checkpoint `10c73ddb36869a8937184bb57f4d19e10868f3b5` is on `origin/main`.
The following DB, Jobs, Platform, Query and Canic increments form the next
GitHub-only checkpoint; none is a crates.io release.
Five [release groups](../release-ownership.md) now own their ledgers under
`releases/`; package versions are unchanged and the root changelog is an index.
All six packages remain unpublished; consumer migration, publication and canonical
cutover are separate delivery work. Recheck registry versions before release.
Toko 0.44.3 is pinned in both workspaces; its 48-file snapshot is verified.

## Implemented owners

- `ic-build` 0.87.1 owns spawn, bounded Cargo decoding, exact selection, owned
  bytes and explicit process/private-target cleanup. Its scratch is mode 0700;
  success exposes neither a mutable target path nor independent unchecked results.
- `ic-host-artifacts` 0.80.1 owns bounded complete-stream read/copy and identity,
  independently of native filesystem, process and HTTP dependencies.
- `ic-host-fs` 0.80.1 owns held directories/files, explicit create/open, bounded
  enumeration, cooperative locks and consuming stage/write/seal/commit states.
  Commit distinguishes unpublished, committed and visible-but-unconfirmed effects.
  Retention precedes effects; cleanup and producer removal refuse observed foreign
  identities. Retained recovery writes remain distinct from named publication.
- `ic-host-http` 0.80.0 owns complete bounded response bodies only. It preserves
  original transport/allocation causes, refuses excess before copying, and caps
  requested buffer growth. Consumers select Reqwest/TLS directly; no client facade,
  request policy, retry, digest dependency or new byte-buffer API was introduced.
- `ic-host-text` 0.77.0 owns pure terminal escaping and bordered/plain tables with scalar
  Unicode widths. Query/DB use borders; current Canic adds plain aligned tables; no progress, clipping
  or terminal policy is included. See [Text](../text-presentation.md).
- `ic-cargo-qualification` 0.1.0 is an external-lock subset admission executable,
  not a Build library dependency, archive runner or metadata editor. Its new name
  also requires registry rechecking before publication.

## Consumer evidence

[Backup](../../qualification/backup/README.md) uses local HEAD
`203b2083009b3aad9b5892a4023fc4ea5ee6d7a5` plus current uncommitted work.
Snapshot identities and logs are recorded there. The patch preserves its metadata
and transfer-binding records and 13-entry budget. Bounded reads and exact payload
streams use the new byte owner; Backup owns commitment comparison. Persisted bytes
remain unchanged. No old filesystem or HTTP supplier remains in its selected graph;
legacy digest values/in-memory hashing remain explicit. The HTTP slice keeps direct
Reqwest configuration and TLS, deadline, encoding, certification and no-retry policy.

[Scheduling](../../qualification/scheduling/README.md) is still clean at
`0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`; its current-source patch applies.
Timers uses one owned build instead of target locking, message collection and path
capture. Private bounded config admission and request-derived receipts preserve
source/tool checks. [Recipe generations](../../qualification/scheduling/observations.md)
refuse legacy/new comparisons without changing report schemas. Jobs uses retained
staging for capture and identity-checked producer cleanup. Jobs uses the upstream
async client with an explicit external endpoint. Timers Testkit and other Host
responsibilities remain unmigrated; no legacy Build remains. PocketIC pins
thiserror 2.0.18, which product
requirements admit while the product lock retains 2.0.21.

Canic’s [Text qualification](../../qualification/canic/README.md#current-cli-text)
passes 19 current CLI display tests with plain aligned tables. Earlier
[record admission](../../qualification/canic/README.md) passed against local
`df9321fc17c99c93e07d631173200a99c2a890bc` plus edits. Its [owned Build recipe](../../qualification/canic/README.md#current-owned-build)
now qualifies independently; publication, parent creation and locking retain old Host. [Auth evidence](../../qualification/auth/README.md)
is historical at `e52a40be75dd9809d0528189d84b43fe96952ad9`; do not continue it.

[DB external ownership](../../qualification/db/README.md) uses current local
`416a518ad722aeb46e28051194dcf039637c1bd2` plus edits. Its driver uses upstream
PocketIC directly, requiring an explicit endpoint; no Testkit library remains.
The prepared released CLI still owns the external server as a development bootstrap.
No new product API was needed; [the ownership decision](../simulator-ownership.md)
records upstream panic, cancellation and best-effort deletion limits.

[Platform raw_rand and Agent](../../qualification/platform/README.md) use current
local fixtures with direct PocketIC. Their drivers no longer own server startup;
the qualification command supplies the endpoint. Agent checks the actual Principal
package identity across its client boundary. Other Platform drivers remain open;
[current Query Text](../../qualification/query/README.md) now passes its 19
presentation/output tests. No shared
simulator API was introduced.

Latest relevant Linux evidence, scoped to the snapshots linked above:

- Current DB console: the existing installed schema/query/cursor/refusal subject
  passes with new Text. All 30 instances report deletion; the server survives.
  Console/test Clippy and retained external-lock admission pass.
- Text: three escaping/table contract tests, product Clippy/Rustdoc and Toko
  admission pass. Current Query’s 19 selected tests, CLI Clippy and retained
  external-lock identity admission pass against the recorded dirty-source snapshot.
- Platform Agent: all four identity cases pass query/update/read-state and wrong-root
  refusal; driver Clippy, actual Principal identity and deliberate mismatch refusal
  pass. Missing endpoint refuses, instance deletion and server survival are observed.
- Platform raw_rand: both installed codec subjects, warning-denied driver/Wasm
  Clippy, source/graph admission, missing-endpoint refusal and observed deletion
  with external-server survival pass; prepared tool identities remain unchanged.
- Jobs: three installed watchdog/trap/snapshot subjects pass, including rejected
  controller identity and exact raw-memory restoration. Three deleted instances
  and server survival were independently observed; warning-denied Clippy passes.
- DB: missing-endpoint refusal and the original nine-mode rollback subject pass;
  the independent inventory reports deletion while the external server survives.
  Warning-denied driver Clippy and retained external-lock identity admission pass.
- Build: 19 tests, including real offline Wasm and process/protocol/cleanup refusals.
- Bytes: two tests and Wasm Clippy; filesystem: ten custody/publication failure tests.
- HTTP: two new local-body tests and 13 current Backup HTTP cases; product/consumer
  warning-denied Clippy, product Rustdoc, formatting and tooling admission pass.
  Cargo graph checks keep HTTP/Tokio outside Build and portable bytes.
- Backup's preceding byte/filesystem slice: 65 tests (54 store/payload/capture,
  five ledger/advance, six metadata/transfer), Clippy and unchanged external-lock
  identity admission. That run was not repeated for the separate HTTP slice.
- Scheduling: 16 Timers and five Jobs host tests, Clippy, two installed Timers
  subjects and four Jobs installed runs using original/O3/Os/Oz artifacts.
- Canic: 19 current CLI display tests and owned-build destination/refusal checks
  pass; earlier six record-admission tests retain their own snapshot boundary.
- Historical Auth: three executable tests plus graph/lock refusals and exact archive
  native/Wasm all-feature checks. This does not qualify current frozen Auth source.

## Next work and limits

Use current unfrozen consumer requirements for remaining Stage 3 work. Backup's
public digest value/format remains a consumer-contract decision; do not recreate
legacy wrapper APIs merely to remove that dependency. The borrowed-server DB case
needs no new wrapper; managed-server ownership, multi-canister partial snapshot
effects and remaining Platform gaps,
Profiler and other required adapters remain later work. Full watchdog/cohort
comparability, archive acceptance and native macOS Intel/ARM remain unqualified.

These slices assume trusted source/tools, cooperative process groups, exclusive
child reaping and caller exclusion of namespace/content writers. Identity checks
are not atomic compare-and-unlink against hostile same-UID writers. HTTP/installed
fixtures require loopback. No production consumer has migrated; no performance or
peak-memory improvement is claimed. Shared-target reuse still needs matched evidence.
