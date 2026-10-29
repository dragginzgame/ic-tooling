# Current handoff — 2026-10-29

## Pending 0.69.14: Profiler native mechanics in Host

This isolated branch starts from `7a44d77f` (Release 0.69.13). The normal Tooling
and sibling checkouts remain untouched. Own package versions and dependencies
are unchanged. No release/publication is performed; pending notes are 0.69.14.

[#264 — Own raw process observations and Profiler workload mechanics in Host](https://github.com/dragginzgame/ic-tooling/issues/264)
supplies an additive Host Process interface and paired adoption. Raw procfs,
libproc/Mach and clock queries originate from Profiler `70907ff`; source provenance
is recorded in the extraction contract. Profiler retains its serialized DTOs,
process selection/creation-identity checks, cadence, retention and report policy.
Platform retains IC state/capture adapters and native Cargo/Bazel inputs.

Host owns base process facts, optional Linux I/O/PSS, typed child-list completeness
and native affinity/clock/thread observations. No workload or automatic resource
allowance is selected. The existing bounded RSS table sampler is unchanged.
Darwin child-list capacity comes from the caller and native buffer counts remain
checked against their ABI representation. Impossible Linux parent IDs refuse
instead of narrowing to another PID; real native observations retain their units.

Existing OwnedChild non-reaping observation preserves final sampling before
reaping. Explicit relinquish invalidates cleanup even if its nonblocking wait
fails, preserving a consumer's keep-running choice without claiming readiness.
The caller owns later lifetime/reaping. A separate explicit graceful operation
keeps the leader reserved until exit or caller-selected grace, then KILLs the
group and reaps synchronously. Existing bounded cleanup/handoff contracts remain.
The moved Unix gate preserves stdin, environment and exact argv; admission,
spawning and release remain caller-owned. Local FFI allowances retain imported
safety comments and remain confined to the native implementation modules.

## Qualification and consumer delivery

Evidence is retained at `/tmp/profiler-host-ownership-20261029`. Warning-denied
all-target/all-feature `make clippy PACKAGE=ic-host-process` passes before longer
tests. All 24 focused child lifecycle tests pass on Linux, including gate, explicit
keep-running, non-reaping final observation and TERM escalation. Declared Rust
1.91 passes those same tests plus procfs facts/identity/completeness regressions
and a native observation smoke test. Strict Rustdoc, formatting and local links
pass. Full workspace/release CI is separate; native macOS execution and Darwin
compilation are not qualified locally. No performance/Wasm-size saving is claimed.

Profiler adoption is tracked by
[Profiler #6 — Adopt Host-owned process observations and workload custody](https://github.com/dragginzgame/ic-profiler/issues/6).
Native supplying inputs belong to
[Platform #94 — Supply Host process dependencies for Profiler ownership adoption](https://github.com/dragginzgame/ic-platform/issues/94).
Their actual graph/native acceptance must be recorded separately. Their candidate
uses an immutable supplier revision until a supplying registry release exists;
no dependency-local patch or fabricated registry package is distributed.

## Preserved obligations

The released 0.69.13 cleanup and its validation remain documented in the release
notes. Its evidence is retained at `/tmp/ic-tooling-cleanup-06913.uEdXff/`, selected
by `/tmp/ic-tooling-cleanup-06913-current`; previous batches remain under
`/tmp/ic-tooling-cleanup-06912.cqgTkd/`, `/tmp/ic-tooling-cleanup-06911.Pznlfz/`
and `/tmp/ic-tooling-cleanup-06910.ZwEZDQ/`.

[#263](https://github.com/dragginzgame/ic-tooling/issues/263) retains consumer
acceptance scope at `/tmp/ic-tooling-263.WQ4ch3/`; no Canic adoption or four-case
sequence qualification is asserted. Earlier preparation race/package failures,
migration/retirement backups, source archives, stale-release lock evidence,
release plans and stashes remain preserved at their existing paths. No cleanup,
crypto change or sibling source migration is inferred from Host-local tests.
