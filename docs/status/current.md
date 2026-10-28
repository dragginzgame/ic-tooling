# Current handoff — 2026-10-28

## Source and authorized scope

The accepted ten-opportunity cleanup batch is local and unstaged over released
0.69.4, HEAD `fa3786c86a00e5e9f98a0c729d86bf45a1bd1a82`. The checkout was clean at
admission. Pending notes select 0.69.5 because the retained changes are compatible;
workspace versions remain 0.69.4. Cargo.lock, dependency declarations, the real
index and Shared Tooling snapshot are unchanged. No commit, push, release, tag,
publication, host-tool setup/download or artifact cleanup ran.

Evidence lives under `/tmp/ic-tooling-cleanup-0695.eglL1X/`: baseline source and
HEAD, incoming patch, saved benchmark binaries/source, raw timings, command logs
and final changed-source capture. The pointer is
`/tmp/ic-tooling-cleanup-0695-current`. Sibling working trees and artifacts remain
untouched. No unrelated active Cargo/rustc process was visible at admission.

## Retained changes

Seven candidates were retained. Benchmark provenance has a typed serialization
owner while keeping the existing JSON schema and admission before workers run.
Startup script fixtures share the ordinary spawn configuration with explicit
per-case timeouts; special TTL/output cases remain explicit. Tree observers use
a default no-op completion callback; durable writers keep explicit finalization.

Line framing borrows complete input frames and amortizes fragmented scratch
growth. Byte limits, rejected prefixes and terminal errors stay unchanged; the
retained-byte limit is not an allocator-capacity bound. Tables store each escaped
cell's scalar width once, adding one width value per cell while retaining output
bytes. Reusable digests are indexed by exact path with existing label, domain,
exclusion and traversal checks. Digest framing and cache identities are unchanged.

Host FS hashes borrowed Unix descriptors using canonical bounded stream hashing.
Tree checksums and Cargo source inventory avoid duplicate handles; shared offsets
still advance on success/refusal. Callers retain admission, custody and blocking
policy. Source modes still come from the same held descriptor. Durable writers
retain their existing File/sync behavior, including their explicit completion.

## Experiment decisions

`measurements.md` in the evidence directory records methodology and tradeoffs.
Five-round optimized Rust 1.91 Linux microbenchmarks show about 21% less time for
a fragmented 16 KiB frame, 15% for a large Unicode table, 78% for 512 warm digest
roots and 40% for 128-byte descriptor hashing. Whole-frame timing was neutral;
ASCII tables and large-file hashing improved only slightly, and eight digest
roots were about 1% slower. These are warm focused measurements, not whole
build/server speedups. Cache evidence exercises the actual production lookup;
its temporary instrumentation was removed before final checks.

Three experiments were declined: summary scratch reuse barely helped larger
samples; an artifact-target index slowed one/eight-target cases; borrowed Cargo
rendering would add another representation or require broader Spec migration.
The command-clone benchmark is only an upper bound, not evidence of a build-time
gain. Those three production paths retain their existing implementations.

## Validation and remaining qualification

Actual Rust 1.91 Linux tests pass for framing (6), Text (34), FS reads (40),
durable tree copying (5), input digests (12), Wasm caches (75), package inventory
and qualification (13), Cargo messages (2), execution (7), existing Cargo grouping
(7), benchmark driver/report behavior (21), and startup fixtures (28). Six live
startup cases remain explicitly ignored; no live PocketIC run is claimed here.
Startup fixtures used native Linux loopback access. The serialized-provenance
fixture passed again after removing duplicate/low-value assertions to satisfy
Clippy's test-length limit. A first inventory filter selected zero tests; the
corrected package filter produced the 13-test result above.

Warning-denied Clippy, Host FS Rustdoc, formatting and local documentation links
are recorded beside these tests. The lint warning concerned only the expanded
test helper, not runtime code. No full workspace/release gate ran. Native macOS
15 Intel and Apple Silicon qualification did not run for this batch; report that
separately from Linux evidence. Consumer migration and publication are not claimed.

## Preserved history and obligations

The [released 0.69.4 handoff](https://github.com/dragginzgame/ic-tooling/blob/fa3786c86a00e5e9f98a0c729d86bf45a1bd1a82/docs/status/current.md)
retains upstream adoption evidence under `/tmp/ic-tooling-deps-0694.GN7ALu/` and
links earlier cleanup records. Shared Tooling remains the exact 144-file snapshot
at `d08cbcafbaff4227e798a890232e2248bf167504`. This batch adds no advisory exemption
or crypto qualification; supplying [Platform #50](https://github.com/dragginzgame/ic-platform/issues/50)
and historical [Tooling #240](https://github.com/dragginzgame/ic-tooling/issues/240)
retain their existing evidence and obligations.

Migration/retirement archives remain under `/tmp/ic-tooling-migration/`. Original
Host refs, stale-lock evidence at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`,
release plans, failed attempts and earlier backup/stash evidence remain preserved.
