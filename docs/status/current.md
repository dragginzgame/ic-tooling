# Current handoff — 2026-10-30

## Completed scope and source

The maintainer accepted the three remaining audit findings and requested upstream
issues for capability gaps. Local changes are unstaged over `b275fe7286115e4eb806e58d3fbbe6c22ab5ed2c`
(Release 0.70.4). Compatible notes select 0.70.5; own versions remain 0.70.4.
No commit, push, tag, release, publication or sibling working-source edit ran.

Shared Tooling 0.27.10 is exported from independently reviewed commit
`61a173990683597c1c6a25f0145a2c87200c3cee`, using an isolated source clone under
`/tmp/ic-tooling-cleanup-0705.Tfe0mO/shared-tooling/`. Impact review and independent
source verification pass for all 148 payloads. The exporter records the closure;
no snapshot-owned payload was patched. Existing tool pins are unchanged; the
catalog adds optional sccache without installing it or activating maintenance.

Artifact feature checks consume Shared's structured effective feature observations.
The existing six-profile validator/component policy remains local. Native clock
reads share a fallible Rustix query and nanosecond conversion; monotonic and
thread-CPU fallbacks remain. Linux thread-ID and affinity queries use Rustix,
retaining zero-PID current-thread selection and invalid-PID refusal. Time is
Unix-only and the additional thread capability is Linux-only. Darwin thread-ID,
libproc/Mach and reviewed post-fork FFI remain; signed sysconf retains its failure
sentinel because the Rustix parameter helpers return unsigned values.

The Make fixture isolates its minimal documentation corpus from newly exported
runtime companion docs. The actual checkout's complete docs are checked separately.

## Upstream publication dependency

[Shared Tooling #326 — Support explicit Cargo options in the opt-in publication adapter](https://github.com/dragginzgame/shared-tooling/issues/326)
is open with exact argument, source/evidence and refusal acceptance criteria.
The new adapter is documented for simple publishers and cannot yet carry this
workspace's feature/path options. The existing lower-level publisher remains
unchanged, including provenance, licenses, source admission, coordinated roster,
all-feature selection, target directory and check-only dirty admission. Its future
consolidation depends on that supplier capability and subsequent adoption.
Shared's Rustix guidance PR #323 is merged and included in the adopted snapshot.

## Validation and limitations

On Linux, strict all-target/all-feature warning-denied Process Clippy passes on
Rust 1.99.1. Eight focused Rust 1.91 tests pass, including clock/process facts,
current-thread affinity, invalid PID refusal and existing version observations.
The actual Linux Host/Tools graph policy passes. Seven feature-policy scenarios
pass using retained actual observations: baseline acceptance, missing/extra
validator, all three component sources and failed-observation status propagation.
Linux and both macOS target graphs confirm native time selection and Linux-only
thread selection; the Wasm graph excludes Rustix. Graphs are not native execution.

Snapshot/source verification, actual formatting-hook preservation and sorting
perturbation, consumer Make routing, release-adapter qualification, focused
ShellCheck, formatting, documentation links and diff checks pass. Common host/Cargo
tools and the release editor passed admission. The aggregate tool check initially
stopped at a missing local server CLI; its locked offline rebuild and subsequent
server admission pass against the existing PocketIC 16.1.0 bundle. No tool download
or server setup ran. Release/setup/Git effects in adapter fixtures were substituted.

An additional Process Wasm compile probe failed at the unchanged unconditional
sampling import of Unix-only `crate::tool`. Both conflicting declarations exist
at admission HEAD. It is an existing unsupported-target limitation, not passing
Wasm compilation evidence. No native macOS execution, hosted CI or full workspace/
release gate ran for this batch. Intel and Apple Silicon native qualification
remain separate from Linux and named-target graph evidence.

## Preservation and earlier evidence

Current evidence is `/tmp/ic-tooling-cleanup-0705.Tfe0mO/`: admission source archive
(including the preceding handoff), patch/index/HEAD/manifest/lock, impact and source
verification, focused logs, feature perturbations and final source/preservation
records. Cargo.lock's incoming unrelated edit, root Cargo.toml, HEAD and the real
index are unchanged. Initial failures and retained failed fixtures remain; no
consumer artifacts were cleaned. The failed Make fixture is
`/tmp/host-make-snapshot.OBblxU/`.

Deadline cleanup evidence remains at `/tmp/ic-tooling-deadlines-0704.y8lrZV/`;
the ten preceding reductions remain at `/tmp/ic-tooling-cleanup-0703.mBTxZ4/`.
PR #265 merge/conflict evidence remains at `/tmp/ic-tooling-conflicts-0701.R6FwVw/`
and `/tmp/ic-tooling-merge-265.iHrIEC/`. Profiler #7 and Platform #95 retain
separate supplier-source/adoption obligations. Issue #266's delivered 0.70.2
native matrix is CI run 40927061973, with evidence at
`/tmp/ic-tooling-issues-263-267.9Ckcfb/` and
`/tmp/ic-tooling-266-closure-20261029.json`; it does not qualify this batch.

Snapshot #263 and Platform #267 closure evidence remains with those owning issues.
Canic's frozen four-case sequence passes on Testkit 0.69.13; no later moving-root
or production backup/restore qualification is inferred. Preserve prior Platform,
Wire and cleanup roots, original failures, migration/retirement archives, Host
archive refs, stale release-lock backups, release plans and stash evidence.
