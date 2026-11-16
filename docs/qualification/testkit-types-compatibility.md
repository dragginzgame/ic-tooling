# Testkit Types compatibility — issue 5

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
its archive is byte-identical to the qualified candidate. Publication/readback
status and final archive identity are recorded below after verification. Hosted CI
is asynchronous; no new native macOS acceptance is inferred from local Linux execution.

## Current Scheduling boundary

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

The held-flight/held-upgrade and watchdog-admission/Jobs-pressure cases require
that Wasm artifact. They remain blocked at their owning consumer; no passing
installed Scheduling result is claimed. Registry resolution and publication can
remove Tooling's supplier conflict but cannot establish the remaining acceptance
in [Tooling #5](https://github.com/dragginzgame/ic-tooling/issues/5).
[Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2) owns the
consumer implementation and migration evidence.
