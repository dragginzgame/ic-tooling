# Current implementation — 2026-11-15

## Accepted scope

The maintainer accepted [design revision 2](../DESIGN.md) and authorized its staged
implementation, including continuation after the Stage 1 architecture checkpoint.
[Build to owned bytes](../build-owned-artifact.md) and
[filesystem custody](../filesystem-custody.md) define the implemented slices.
The [workload register](../workloads.md) preserves later migration obligations.
Production consumer migration and publication remain separate delivery work.

## Reference and pending-work boundary

The production reference `../ic-tooling` is read-only, observed at
`a00369fc29fa837c6deb022cceb7e09aa447292c`.
The superseded transplant and design evidence remain in Git at
`cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`, including
`docs/design/inventory.tsv`, `consumer-evidence.txt`, `source-provenance.md`,
`docs/build/streaming-selection.md` and the previous status file.
Existing `ci/extraction-sources.json` describes historical extractions only.
The user's preexisting deletions of copied trees are preserved. No old product
source or test trees were imported. The maintainer authorized GitHub delivery of
this checkpoint on 2026-11-15. Registry publication remains disabled.

The requested Toko 0.44.3 refresh used the registry exporter and its ownership
proof. Both workspace locks select that exact version; the 48-file snapshot is
verified. Common tools remain owned by Toko, not local replacement scripts.

## Implemented ownership

- `ic-build` 0.87.0 owns spawn, bounded Cargo decoding, success checks, exact
  selection, owned byte capture and explicit process/private-target cleanup.
  Artifact bytes and SHA-256 outlive scratch without exposing a mutable path.
- `ic-host-artifacts` 0.80.0 owns bounded complete-stream byte admission and
  identity independently of native filesystem/process dependencies.
- `ic-host-fs` 0.80.0 owns held directories, explicit create/open, bounded complete
  enumeration, cooperative locks and consuming stage/write/seal/commit states.
  Publication distinguishes not published, committed and visible but unconfirmed.
  Retention is selected before effects; cleanup refuses observed foreign entries.
- Build delegates byte admission and descriptor access to those owners. Its
  temporary directory now explicitly requests mode 0700 rather than relying on
  tempfile's umask-dependent directory default.
- Backup's retained recovery-file writes remain separate from named publication.
  The migration removes old path-resolution wrappers and extra directory reopening.
  No compatibility facade, generic backend or release group has been introduced.
- All three packages remain unpublished. Existing registry names are retained with
  incompatible minor increments above observed 0.86.0 / 0.79.0 registry lines.

## Qualification and boundaries

[Backup qualification](../../qualification/backup/README.md) exports only commit
`d8955cf1e34f24934027f448ef16ee6841eafbb0`, applies the committed migration recipe,
and verifies Cargo selected the new filesystem supplier. Its dirty sibling tree
is neither exported nor modified. The patch preserves original persisted formats
and tests in the isolated export; it does not transplant them into product crates.
Legacy Backup byte/digest and HTTP suppliers remain explicitly unmigrated.

Latest focused Linux evidence:

- Build: 19 tests, including real offline Wasm, process/protocol refusal,
  cancellation, no-follow capture, scratch cleanup and compound failure evidence.
- Byte admission: 2 tests; filesystem: 9 tests covering held roots, permissions,
  aliases, locks, races, foreign staging, partial writes, retention and sync refusal.
- Backup: 54 store/payload/capture tests plus five selected ledger/advance tests.
- Warning-denied product Clippy and Rustdoc; portable byte-owner Wasm Clippy.
- Shared formatting and Toko snapshot/tool admission. Qualification has an
  independent workspace/lock; no product depends on its fixture package.

## Next work and remaining limits

Stage 3 next qualifies the installed Scheduling workflow against the build owner.
Refresh its committed source, establish the real required facts, and introduce
shared-target behavior only if that workflow demonstrates the need.

These slices assume trusted source/tool inputs, cooperative process-group members,
exclusive child reaping and caller exclusion of concurrent namespace/content
writers. Identity observations are not atomic compare-and-unlink against hostile
same-UID writers. Native macOS Intel/ARM, archive acceptance and full Backup
supplier cutover remain unqualified; Linux results do not establish them.
Canic/Auth/Storage, simulator/DB, Platform/Query gaps, Profiler and peripheral
adapters remain later work. No production consumer has migrated and no performance
or peak-memory improvement is claimed. Recheck registry versions before release.
