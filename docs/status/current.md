# Current implementation — 2026-11-15

## Accepted scope

The maintainer authorized starting the design using latest local `../ic-tooling`
as the source of truth, prioritizing technical debt, workflow and performance.
The maintainer authorized public GitHub delivery to `dragginzgame/ic-tooling-new`.
Canonical production ownership stays upstream; crates.io publication is excluded.

Build is prepared as 0.86.1 at the maintainer's request: the streaming selector is
a compatible addition, so the next patch applies. Foundation remains 0.79.0.
Every crate retains `publish = false`; no registry release is authorized.

## First completed implementation increment

- Retained Foundation and Build from clean source commit
  `a00369fc29fa837c6deb022cceb7e09aa447292c`, including existing tests and features.
- Added bounded streaming Wasm selection and shared its core with the existing
  selector. No existing public API was removed. Migrated the runnable Wasm
  example and real shared-target/capture fixture away from full-message buffers.
- Adopted the registry-admitted Toko 0.44.0 Make interface and formatting hook,
  root compiler/lints and inherited dependency selection. All packages disable
  crates.io publication. No release pipeline or sibling migration was added.
- Preserved provenance in [the source record](../design/source-provenance.md).
  Consumer migration and limits are in [the guide](../build/streaming-selection.md).

## Scheduling consumer increment

- Added `make test-scheduling-consumer` and a source-pinned migration patch in
  [qualification/scheduling](../../qualification/scheduling/README.md).
- The actual Scheduling callback now uses bounded streaming selection in an
  isolated exported workspace. Diagnostics, input checks, target-lock scope and
  capture lifetime are retained. The original sibling is unchanged.
- Cargo metadata proves one local identity for each replacement supplier across
  the full consumer graph. The copied lock changes only the four suppliers from
  registry to local paths; unrelated versions/checksums are unchanged.
- All seven existing build host tests and five report tests pass on Linux.
  Source drift, failure causes, process deadlines, publication and receipt
  identity are exercised. The migrated build callback is compiled; these tests
  do not run the complete installed simulator workflow.
- Runner syntax/ShellCheck, formatting and local documentation links pass.

## Focused Linux evidence

- Toko 0.44.0 adoption updates only the baseline within the selected snapshot;
  executable tooling and pins are unchanged. No local Python evidence-verifier
  imports require migration. The dependency update changes only Toko's lock entry.
  Snapshot admission, prepared-tools checks and formatting checks pass; the new
  crate is fetched into Cargo's cache for offline development.

- Cargo tooling declarations, lock checksum and 48 installed snapshot files agree.
- Added the shared dependency Make entrypoint. `make deps-list`, `make help`,
  `toko-cloc -d 0` and `make tools-report` pass. A targeted offline `deps-update`
  for the already selected Toko 0.44.0 passes with Cargo.lock byte-identical.
- 11 Build execution tests pass, covering exact selection, completion refusal,
  cancellation, mutation, decoder/sink failure and candidate bounds.
- The real shared-target collision test passes: two Cargo Wasm builds retain their
  independent captured bytes and required build-script evidence.
- 12 Artifacts stream/hash/error tests pass.
- FS publication/sync/foreign-stage evidence and Process full-duplex/drain/cleanup
  regressions pass (one selected test each).
- Prepared host/Rust tools, shared formatting checks and hook installation pass.
- Build library/tests and the migrated example pass Clippy with warnings denied;
  Build Rustdoc passes with warnings denied.

These are focused development checks. No complete suite, archive-based consumer
acceptance, native macOS result or measured performance gain is claimed.
The new selector bounds retained output paths; it does not bound Cargo's entire
memory use or make mutable output files immutable.

## Next design work

Complete Scheduling installed-runtime and archive acceptance beyond the host
checks above. Backup and Profiler acceptance remain necessary before calling
the Foundation migration complete.

Introduce independent local qualification packages when they have their own
tests to own; Scheduling uses its existing workspace. HTTP/Text/Testkit/Wire remain
outside this increment. Review
product qualification selection data for one roster consumed by Toko; no release
gate is narrowed here. Archive qualification, all required native hosts and
canonical publication decisions remain later design gates.
