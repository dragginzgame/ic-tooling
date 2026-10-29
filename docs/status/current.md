# Current handoff — 2026-10-29

## PR #265 repair and source

[PR #265 — Own raw native observations and workload mechanics in Host](https://github.com/dragginzgame/ic-tooling/pull/265)
is repaired on its existing `feat/process-observations-for-profiler` branch in
`/tmp/profiler-host-ownership-20261029/tooling`. Admission was clean at `c20dbd11`.
The ordinary checkout is clean at `645ae21d` (Release 0.69.14) and is untouched.
Current main is merged into the PR branch with normal merge ancestry. Package
versions are inherited from main at 0.69.14; no new version bump is performed.
Compatible additive notes move to 0.69.15, preserving released 0.69.14 notes.
No release, publication, tag, merge to main or sibling source edit is performed.

Both native macOS jobs on original head `c20dbd11` failed during Clippy at the
same affinity observer. On non-Linux, its fallback could be const; Linux uses a
runtime syscall. A non-Linux-only `missing_const_for_fn` exception explains why
this cross-platform API remains a runtime function. Other lints stay denied.
No process mechanics or public signatures change in this repair.

## Observation and workload ownership

[#264 — Own raw process observations and Profiler workload mechanics in Host](https://github.com/dragginzgame/ic-tooling/issues/264)
supplies additive Host Process APIs. Raw procfs, libproc/Mach and clock queries
originate from Profiler `70907ff`; provenance is recorded in the extraction
contract. Profiler retains persisted DTOs, process selection/creation-identity
checks, cadence, aggregation, retention and diagnostics. Platform retains native
IC adapters and matching Cargo/Bazel inputs. The bounded RSS-only sampler stays
separate. No automatic limits, IC dependency or cryptographic change is added.

Explicit child relinquishment releases cleanup even if its nonblocking wait
fails; the caller owns later lifecycle/reaping. Graceful termination reserves the
leader until exit or selected grace, then kills the group and reaps synchronously.
Existing bounded cleanup and successful handoff retain their contracts. The Unix
gate preserves stdin, environment and exact argv; readiness and release are
caller-owned. Native FFI allowances retain scoped safety comments.

## Validation and delivery evidence

Repair evidence is retained at `/tmp/ic-tooling-265.xBrNN0/`, including original
native CI logs, admission source/handoffs, commands, final checks and patch.
Original CI passed native Linux, Rust 1.91, lint/security, optimizer, browser and
Testkit runtime jobs. Both macOS failures are retained as failures, not passes.
Focused repair validation passes strict all-target/all-feature Process Clippy,
seven Rust 1.91 observation/admission tests, strict Rustdoc, formatting, local
links and diff checks. Released root/detailed note history matches main exactly.
Updated native macOS qualification belongs to the new asynchronous CI run.
No full local workspace/release gate is inferred from this PR repair.

Original qualification remains at `/tmp/profiler-host-ownership-20261029`:
24 child tests on development Rust and Rust 1.91, three procfs tests and a native
observation smoke test. Paired Platform/Profiler adoption records real Cargo and
Bazel graphs, recorder/API/CLI checks and installed profiling scenarios against
immutable supplier `c20dbd11`. Those observations do not qualify this new Host
revision or native macOS. Consumers retain their own exact-source acceptance.
[Profiler #7 — Host process adoption](https://github.com/dragginzgame/ic-profiler/pull/7)
and [Platform #95 — Host process supplying inputs](https://github.com/dragginzgame/ic-platform/pull/95)
carry the paired work. No publication or automatic dependency retargeting occurs.

## Preserved obligations

[Testkit snapshot consumer acceptance (#263)](https://github.com/dragginzgame/ic-tooling/issues/263)
retains scope at `/tmp/ic-tooling-263.WQ4ch3/`; this work establishes neither
Canic adoption nor its retained four-case sequence qualification.

Released cleanup evidence remains at `/tmp/ic-tooling-cleanup-06914.hJOCPI/`,
`/tmp/ic-tooling-cleanup-06913.uEdXff/`, `/tmp/ic-tooling-cleanup-06912.cqgTkd/`,
`/tmp/ic-tooling-cleanup-06911.Pznlfz/` and `/tmp/ic-tooling-cleanup-06910.ZwEZDQ/`.
Preparation race/package failures, migration/retirement backups under
`/tmp/ic-tooling-migration/`, source archives, original Host archive refs,
`/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans and stash evidence
remain preserved. Earlier supplying-owner obligations keep their recorded scope.
