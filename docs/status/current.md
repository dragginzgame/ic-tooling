# Current handoff — 2026-10-30

## Completed scope and source

The maintainer accepted all ten reviewed reductions and asked about common
Rustix guidance. The IC Tooling changes are complete locally and unstaged over
`86d816b3361dbeaf5bdad3fa43f47c19e59aad38` (Release 0.70.2). Admission contained
one unrelated Cargo.lock update; it remains byte-identical. Root Cargo.toml,
own package versions, HEAD and the real index are unchanged. Pending compatible
notes select 0.70.3; no production API or persisted-format migration is needed.
No Tooling commit, push, version transaction, tag, release or publication ran.

Evidence: `/tmp/ic-tooling-cleanup-0703.mBTxZ4/`. Admission source/patch/index/
manifest/lock, focused logs, graph argument captures, final source and preservation
records are retained there. Shared snapshot stays 0.27.1 at
`d08cbcafbaff4227e798a890232e2248bf167504`; no local snapshot-owned rule was edited.

## Converged mechanics

Grouping retains isolated trees only for the current group; sealed groups are
never searched. Conservative input order, exact feature equality and the 2n−1
observer-call bound remain. Wasm copies pair admitted lists directly, retaining
count refusal before effects. No resolver cache or package-selection mode is added.

Artifact manifests stream through the existing durable writer. The same format
counts its maximum read length using widest byte fields and a bounded sink,
and compares retained manifests. The intermediate document and separately
maintained row-width arithmetic are removed. Bytes, keys, bounded admission,
corruption refusal and typed publication/cleanup causes remain unchanged.

The launch gate uses existing Rustix pipe and owned-descriptor flag APIs before
spawning; only Process's native Rustix pipe capability is added. Pipe/descriptor
error variants, exact argv/stdin and the reviewed async-signal-safe post-fork
operations remain. Procfs details select named counters without owned maps,
retaining last-valid duplicates, missingness, units and optional PSS behavior.
Positive sysconf conversion has one owner; caches and fallbacks stay separate.

Opt-in FS testing owns the common exact-case libtest re-execution fixture used
by FS and Build. No compatibility aliases or production process owner are added.
Testkit graph profiles moved from Make shell fragments into
scripts/ci/check-testkit-graphs.sh. All ten checker argument vectors match the
admitted Make target exactly. Build README now describes the coordinated 0.70
family and current module paths; historical changelog entries are preserved.

## Validation and native boundaries

Strict all-target/all-feature warning-denied Clippy passes for Build, FS and
Process on prepared Rust 1.99.1. Actual Rust 1.91/Linux checks pass: 60 focused
top-level tests covering grouping, transactional manifests/output publication,
Wasm copy inventory, descriptor isolation, procfs and child/gate contracts.
Subprocess reruns of isolated cases are not counted as additional distinct tests.
The detail regression includes malformed/duplicate counters and unselected PSS.

Strict Rustdoc passes for those three packages on actual Rust 1.91. FS library
compilation with test support passes on the prepared Wasm target at Rust 1.91;
this does not promise executable subprocess support on Wasm. The actual Testkit
native/Wasm graph script, before/after argv equality, ShellCheck, shell syntax,
formatting, documentation links and diff checks pass.

No full workspace/release gate ran. No native macOS execution or hosted CI ran
for this local batch. Earlier Linux and both native macOS passes in CI run
40927061973 qualify delivered 0.70.2, not these subsequent local changes.

## Shared guidance proposal

Rustix declarations and native usage were inspected in Canic and IC Backup.
[Shared Tooling PR #323 — Prefer safe native interfaces behind existing Host owners](https://github.com/dragginzgame/shared-tooling/pull/323)
proposes Host owners first, then suitable std/Rustix APIs, with narrow native
features and documented ABI, unavailable-API and post-fork FFI exceptions.
Error/descriptor semantics and native-host qualification remain explicit.
It requires no Rustix dependency in unrelated/minimal/Wasm packages.

The PR uses the isolated `shared-tooling-rule/` clone under this evidence root,
from main `11d519c6`, with scoped commit `8acc0b95`. Its 500-reference doc-link
check, instruction review and diff checks pass. The normal Shared checkout's
dirty README/stack work was not copied or edited. The PR is open, unmerged and
unadopted here; no Shared version bump, release or fleet adoption ran.

## Preserved obligations and evidence

PR #265 merged through `01bbf850`; ordinary pull conflicts were resolved and the
maintainer completed merge `5123d971` and Release 0.70.2. Conflict and source
backups remain at `/tmp/ic-tooling-conflicts-0701.R6FwVw/` and
`/tmp/ic-tooling-merge-265.iHrIEC/`. Profiler #7 and Platform #95 retain their
separate supplier-source/adoption obligations. Issue #266 closed on delivered
0.70.2 with native qualification; earlier grouping/probe evidence remains at
`/tmp/ic-tooling-issues-263-267.9Ckcfb/` and
`/tmp/ic-tooling-266-closure-20261029.json`.

Snapshot #263 and Platform #267 closure evidence remains with those owning
issues. Canic's frozen four-case sequence passes on Testkit 0.69.13; no later
moving-root or production backup/restore qualification is inferred. Preserve
prior Platform, Wire and cleanup roots under /tmp, original failed package/
preparation attempts, migration/retirement archives, Host archive refs, stale
release-lock backups, release plans and stash evidence. No artifact cleanup or
sibling working-source edit was performed.
