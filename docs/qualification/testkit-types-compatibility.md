# Testkit Types compatibility — Tooling #5

[Tooling #5](https://github.com/dragginzgame/ic-tooling/issues/5) is closed.
[Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2) subsequently
completed the consumer implementation and installed acceptance, publishing Timers
0.46.0 and Jobs 0.39.0 with Runtime/guard 0.23.14 and Testkit 0.107.2. Its owner
records Linux/PocketIC 16.0.0 evidence, distinct from the historical Tooling
captures below. The earlier 21-error build is no longer a current blocker.

The Testkit 0.107.2 correction changes its Types requirement from an exact patch
to `0.21.2` (`>=0.21.2, <0.22.0`). The lock still selects 0.21.2. No legacy Runtime,
second executor, SDK dependency or compatibility facade is added. Consumers can
resolve the published Runtime 0.23.13 cohort without substituting an unsupported
Types package. The removed caller obligation is coordinating a new Testkit release
for every otherwise-compatible Types patch merely because of Testkit's own pin.

Protocol remains 0.2.5 with only `http-request`. Testkit retains unspecified HTTP
pricing and its existing fixture API. PocketIC's client/types remain the original
upstream identities; tests now explicitly compare Principal bytes after canonical
text conversion at both module construction and controller binding.

## Qualification and delivery

The selected release group is only `testkit`, patch 0.107.2. The normal checkout
stays on `main`, targeting `dragginzgame/ic-tooling` and crates.io. Prior Toko
0.44.9 source-marker/snapshot maintenance supplies its admitted tooling. Other
product versions and unrelated pending work are preserved. The prepared external
launcher remains 0.106.0 with PocketIC 16.1.0; this library does not own a launcher.

Focused evidence is retained under `target/issue-5/`. The existing selected archive
gate checks default and fixture configurations, unit tests, warning-denied
Clippy/Rustdoc and both installed module/controller tests. These cover catalog,
slot/name/reply/module budgets, exact callback bytes, binding and pending/release
identity, foreign clients/tokens, consumed/expired requests and manual-progress
refusals. Server inventory independently observes instance deletion.

Archive run `target/library-release.rkypb3mu` passes all selected checks, including
six unit tests and both installed tests, with all three instances observed deleted.
Source and tool hashes remain stable. Cargo's verified publication dry run passes;
its archive is byte-identical to the qualified candidate. After committing, the
clean-provenance archive preserves the same normalized manifest, lock and source;
only VCS provenance changes. Hosted CI is asynchronous; no new native macOS
acceptance is inferred from local Linux execution.

Testkit **0.107.2 is published**, non-yanked, from commit
`74006cadd6501895959c6a7641a8f01f33cc47ff`, tag `testkit-v0.107.2`. Main and the
annotated tag were pushed atomically. The registry download is byte-identical to
the verified committed preflight/upload archive, SHA-256
`6d0ad701e379df81e149511b0d6bbb810648770ec777a4eb88fd7e87e4ee2883`.
Its original manifest/source match the committed files, and Cargo VCS provenance
identifies the release commit without a dirty marker. The unrelated root-manifest
comment edit was preserved during clean-input publication and restored afterward.

## Historical Scheduling boundary at publication

The read-only capture records HEAD `3d2b3fd3775252e779181c4175a8ecf661afb60b`
plus current tracked edits and non-ignored additions, source tar SHA-256
`1d182957e5462f2dd08236d71cedd9db1b76a50f108d234c0bdb57d6af201861`.
The original archive remains under `target/issue-5/scheduling-review/`. No sibling
source is edited. Only dependency selection and the existing held-reply selector
are adapted in its disposable export.

With local Testkit 0.107.2, the actual qualification workspace resolves one Types
0.21.2, Runtime 0.23.13 and guard 0.23.13. Both native drivers pass all-target
warning-denied Clippy. Building `once-probe` stops in `ic-timers` with 21 errors
at remaining old identity/system/task APIs; see `scheduling-local.log`. This is a
different current source capture from the earlier 17-error probe review.

After publication, a second export of the identical captured bytes changes only
the Testkit requirement to registry `=0.107.2` and applies the same private selector.
It resolves one registry Testkit 0.107.2, Types 0.21.2, Runtime 0.23.13 and guard
0.23.13 with no supplier path override. Both native drivers again pass all-target
warning-denied Clippy. The registry-only Wasm build stops at the same 21 Timers
errors; logs/metadata are `scheduling-registry-*` under `target/issue-5/`.

The held-flight/held-upgrade and watchdog-admission/Jobs-pressure cases require
that Wasm artifact. They remain blocked at their owning consumer; no passing
installed Scheduling result is claimed. Registry resolution and publication have
removed Tooling's supplier conflict but cannot establish the remaining acceptance
in [Tooling #5](https://github.com/dragginzgame/ic-tooling/issues/5).
[Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2) owns the
consumer implementation and migration evidence.
