# Current handoff — 2026-11-11

## Delivery

The cache reductions, Wire adoption and tooling refresh are committed and
published. Latest product release commit:
`6f4128ae2ecc7c4fe25814cd7c1ccc48c8cc17ec`. Both frozen batches were reconciled
into the ordinary main checkout.

| Group | Published | Release commit |
| --- | --- | --- |
| Foundation (five packages) | 0.76.4 | a1b50ffc |
| Build | 0.82.0 | ea58d4b9 |
| Text | 0.74.5 | f0352237 |
| Host Tools | 0.89.2 | fe337366 |
| Testkit | 0.99.0 | 6f4128ae |

All nine versions were verified present and not yanked in the crates.io index.
Private qualification packages remain 0.96.0. The deployment preserved selected
external dependencies; release preparation changed owned versions and their
product/qualification references together. The release uses Toko Tooling 0.39.9
and Wire 0.20; the product Platform requirement remains 0.29 (locked to 0.29.2).

## Local dependency adoption

Both workspaces now select published Platform 0.30.0, Types 0.19.2 and Wire
0.21.0. The private probe selects Runtime/macros/guard 0.21.1. Runtime now owns
the upstream IC0 1.2.0 dependency; qualification admits that direct owner before
requiring one raw provider, while native and portable Testkit keep forbidding it.
Default/Wire Testkit excludes Platform/runtime; portable Testkit stays Types-only.

Public supplier identities require pending Host Tools 0.90.0 and Testkit 0.100.0;
the previous Testkit 0.99.1 draft's cleanup is carried into 0.100.0. Platform's
removed textual DID service was already unused here. Wire's TypeScript naming
cut needs no local regeneration; these profiles select no TypeScript generator.
Binary bytes, budgets and runtime decoding acceptance are unchanged.

`make dependency-adoption-check`, three private public-contract tests, two
installed held-reply tests, selected Host Tools/Testkit warning-denied library
Clippy, and final graph/owner admission pass on Linux. No new native macOS or
full release qualification ran. Logs and both original lock/manifest snapshots
remain under `target/platform-wire-adoption.wBezlh/`. Only Platform/Wire changed
in the product lock relative to the saved dirty input; qualification changed the
requested family and added its IC0 leaf. Other selections and pending source
work are preserved. These changes remain local/uncommitted/unpublished; package
versions are unchanged until the authorized release workflow prepares them.

## Delivered reductions and adoption

Toko Tooling 0.39.9 is selected in both workspaces and installed through the
authenticated crates.io exporter, preserving the 113-file runtime selection.
That refresh changed only its lock entries; the pre-existing product
`rustls-pki-types` 1.16.0 selection remains intact. Tool catalogs are unchanged.
The 0.39.9 checker admits native PocketIC dev-dependencies for simulator tests.
Focused adoption evidence remains under `target/toko0399/` and `target/toko0398/`;
the complete release gates now qualify the delivered batch on Linux.

Wire 0.20 is adopted in both workspaces and published through Testkit 0.99.0.
The minor release includes public adapter identities and upstream Postcard
acceptance changes. Fourteen focused tests cover nine adapter
cases, live native replies, type identity, probe budgets and installed Wasm
requests/state/refusals. Graph policy, portable compilation, selected warning-denied
lint, formatting and snapshot/pin/doc admission pass. Focused logs remain under
`target/wire020/`; native macOS qualification is separate below.

Build 0.82.0 makes shared Cargo targets the only compilation path and makes
shared-target maintenance explicit. The full input digest owns both cache
identity and mutation validation. The separate validation digest
and receipt field are removed. New fingerprint keys preserve old cache entries;
unrelated workspace manifest/lockfile edits can invalidate exact reuse. The
[Build guide](../build/artifacts.md#build-082-migration) records caller migration.
Focused Linux cache/batch/evidence and Cargo-input transaction tests, real Cargo
Wasm/examples/artifact-handoff/process-lock tests, 103 durable-filesystem tests,
warning-denied Build Clippy/rustdoc, formatting and document links pass. Logs and
before/after Rust counts remain under `target/loc-audit/`.

Build 0.82.0 also makes exact-cache pruning explicit.
Acquisitions no longer schedule pruning or attach nonfatal maintenance results,
timings or phases. Callers invoke the existing prune functions and retain their
records to protect live outputs; old exact-cache schedule markers remain untouched.
Focused cache, transaction/batch and pruning tests, cross-process handoff, real
Cargo Wasm acquisition, warning-denied Clippy/rustdoc and document links pass on
Linux. Evidence is under `target/loc-audit/explicit-pruning/`. This continuation
removed another 253 Rust lines. The two reduction passes removed 1,631 Rust lines
overall, leaving 68,768 across both workspaces: 25,609 library/application,
42,170 tests, 500 examples, 328 tooling and 161 benchmarks. These are source
counts, not measured executable or Wasm savings.

## Current contracts

Testkit 0.98.0 replaces the maintained fork with published `pocket-ic` 16.1.
Import its async client, builder and native DTOs directly. Harness helpers are
free `pic` functions; native IDs use upstream identities, while portable helpers
retain Platform Types Principal. There is no compatibility facade. Upstream
panics, rejection types and best-effort deletion replace fork transport errors;
close/shutdown return `()`. Ordinary snapshots replace bounded transfer, with
timer/hook reset owned by callers. See the [migration](../testkit/client-migration.md).
Sibling caller migration is separate from this repository's deployment.

Default/Wire Testkit excludes Platform; only held fixtures select its adapters.
Client dependencies remain native and the Wasm graph remains Types-only.
The delivered cleanup also shares Linux stat parsing, streams typed package
inventories, preserves probe/cleanup failures, simplifies response decoding and
uses semaphore-owned pool closure. Group ledgers own the release descriptions.

All eleven Rust packages inherit the canonical root `LICENSE.md`; the two
private Node manifests also resolve that root file. Source/archive publication
checks admit the canonical bytes. Release fixtures now declare the checker and
root-file inputs, fingerprint them for reuse and omit retired reference trees.
The 113-file Toko snapshot retains runtime inputs and canonical root files;
shared reference documentation is linked at its recorded upstream revision.

## Qualification

All five complete Linux release gates passed, including warning-denied lint in
both workspaces, installed PocketIC, extracted-package tests, native/Wasm graphs,
Wasm compilation, documentation and workspace tests. Cargo also verified each
published package. Native macOS 15 Intel and Apple Silicon qualification remains
separate from local Linux evidence. The manual [release-tag qualification run](https://github.com/dragginzgame/ic-tooling/actions/runs/42895053349)
passed at `6f4128ae2ecc7c4fe25814cd7c1ccc48c8cc17ec`, including native macOS 15
Intel and Apple Silicon and hosted Linux. The lint/security and MSRV statuses
also passed. These results do not qualify later dependency selections or edits.
Cancelled or superseded hosted runs are not passes.

The Wire/PocketIC migration guide now explicitly names the published Testkit
0.99.0/Wire 0.20 pairing. This documentation and its pending Testkit 0.100.0
ledger note are local and uncommitted. The existing ordinary-checkout Cargo.lock
edit was preserved; no dependencies were changed during that earlier review.

The same pending Testkit 0.100.0 batch removes the private pool shutdown's generic
error result/collection and its production `Infallible` adapter. Public shutdown
still returns `()` and awaits upstream best-effort deletion. All eight focused
pool tests pass on Linux, retaining lease waiting, closed-waiter wakeup,
cancellation recovery, lazy allocation and no deletion replay. This cleanup is
local and uncommitted. Warning-denied library Clippy, targeted formatting,
documentation links and diff checks also pass. The release-tag native results
above precede this change; no new native macOS qualification ran.

Testkit 0.100.0 also shares the nine server-provisioning tests' isolated fixture
setup and removes its redundant wrapper type, a net 75-line Rust source reduction.
All nine cases pass on Linux, retaining per-case authentication/corruption,
failure recovery, path custody and concurrent setup assertions. The subprocess
deadline, output bounds and panic-retaining fixture ownership are unchanged.
Targeted formatting, documentation links and diff checks pass. The current
ordinary lock selection was preserved byte-for-byte; locked native dependency
fetch prepared its previously uncached Types 0.19.2 before the offline test run.
This test-only cleanup is local/uncommitted; no new native macOS run was made.

The pending Build 0.83.0 hard cut now removes interval-limited shared-target
maintenance, persisted scheduling and scheduled outcome types. The existing
explicit maintenance function remains the sole operation; callers choose cadence
and failure policy. Source/cache overlap refusal, shared-target locking and
retained-artifact protection remain enforced. Malformed old schedule markers
survive unchanged, including explicit clearing of compiler state. This is a net
471-line reduction across Rust source/test files, not a compiled-size measurement.
See [the migration](../build/artifacts.md#build-083-migration) and
[Canic #762 — Migrate IC Build callers to explicit maintenance and pruning](https://github.com/dragginzgame/canic/issues/762).
No Canic source was changed or qualified.

Focused Linux evidence passes: five shared-target/boundary tests, maintenance
tool-failure recovery, the actual two-process shared-target build, warning-denied
Build Clippy across its library/tests/examples, rustdoc, formatting and document
links. The changes are local/uncommitted/unpublished; package versions are
unchanged. No new native macOS qualification ran. The proposed status-parser
merge was rejected: its differing duplicate-field and complete-metadata
validation contracts would require additional machinery or a separate behavior
change rather than a demonstrated simplification.

The same pending Build 0.83.0 now takes a positive duration directly for progress
and exposes heartbeat/batch records without single-variant wrappers. Zero-interval
admission, quiet/lock heartbeats, phase attribution, child custody and panic
cleanup remain covered. Export receipts read input digests from the verified
retained record; the initial fingerprint and all freshness checks remain.
Canic's progress caller migration is recorded on the existing owning issue above.

Pending Testkit 0.100.0 omits HTTP imports, callback tables, barrier data and reply
callback functions from cycle-only fixture Wasm. A representative one-slot,
one-cycle-cap fixture measures 554 to 319 raw bytes and 59 to 42 code-section
bytes; imports fall from eight to four and defined functions from four to two.
These are generated fixture measurements, not application-Wasm or runtime-cost
claims. The existing cycle-cap matrix owns the footprint assertions. Baseline
and final observations are retained in `target/cycle-footprint-before-1.log` and
`target/cycle-footprint-matrix-final.log`.

Twelve focused Build progress/evidence tests, three generator cases and six
installed PocketIC cycle/refund/held-request cases pass on Linux. Installed
evidence remains in `target/cycle-and-held-installed-final.log`. Warning-denied
Build and Testkit library/test lint, Build examples/rustdoc, targeted formatting
and documentation links pass. The retired 0.4 pool design now links its verified
immutable historical source, removing 514 documentation lines without losing
the record. This continuation remains local/uncommitted/unpublished, preserves
the ordinary lockfile bytes and ran no native macOS qualification.

The latest local follow-up removes owned labels from Build batch heartbeat
events; observers borrow `specs[event.index].label()`, while result entries retain
their labels. This extends the pending Build 0.83.0 progress migration above.
Testkit 0.100.0 now retains captured snapshots in a vector after the existing
ordered duplicate admission. Capture/restore/cleanup order and sender identity
are unchanged; no runtime or memory-size saving was measured. Standalone recipe
stop/start failures retain `StandaloneCanisterLifecycleError` as the typed cause
inside `io::Error`. Initial stop refusals no longer masquerade as snapshot
capture failures; original builder and snapshot errors remain intact.

Fourteen focused Linux tests pass: two-spec heartbeat routing; three snapshot/
recipe unit cases; six live snapshot admission/rollback/funding cases; and four
live standalone rejection, rebuild and transport-panic cases. The panic case
was explicitly run despite its default ignored status. Build library and
Testkit library/changed integration-target warning-denied Clippy pass with the
maintained held-fixture profile for the Testkit caller. A separate default-only
integration lint attempt hit the pre-existing broad `dead_code` expectation in
`tests/support/mod.rs` when that target uses all default helpers; that attempted
configuration is not a passing lint result. Logs remain under
`target/review-followups.ETVZfG/`. Both lockfiles are unchanged from this batch's
input. No new native macOS or full release qualification ran. This work remains
local/uncommitted/unpublished; Canic's renderer migration remains with #762.

The next test-only follow-up fixes that default-only lint limitation: the shared
integration-helper module deliberately allows `dead_code` with its exact
target/feature reason. Some independent targets use a subset, while default-only
concurrency uses all helpers, so a shared expectation cannot faithfully cover
the maintained selections. The allowance stays at this test-helper boundary;
production lint levels and `unfulfilled_lint_expectations` remain enabled.
The HTTP peer callback also drops its unused deadline argument. The worker
retains its actual deadline, cancellation and quiet-period replay checks.
All eight affected time, diagnostic and startup HTTP-fixture tests pass on
Linux. Warning-denied library/concurrency lint passes with both default and
held-fixture selections. Evidence remains in `target/fixture-cleanup.0T1Ssn/`;
both lockfiles are unchanged. No new native macOS or full release qualification
ran; the batch remains local/uncommitted/unpublished in Testkit 0.100.0.

Retain the snapshot cleanup callback's explicit future type. Both attempted
`AsyncFnMut` spellings failed the standalone recipe's `Send` lifetime contract
with E0477; those attempted replacements were removed. The established generic
callback and its existing rollback coverage remain unchanged.

Completed batches are `batch-4hupb2d0` (Foundation, Text and Tools patches) and
`batch-UEWDKToB` (Build and Testkit minors). Their source-bound plans, gate logs and upload
receipts remain under `.git/release-state/frozen/`; managed checkouts remain under
`target/release-frozen/`. Normal-checkout logs are `target/deploy-patches.log` and
`target/deploy-minors.log`; independent registry observations are under
`target/deploy-registry-20261110/`. Reconciliation logs are
`target/deploy-patch-reconcile.log` and `target/deploy-minor-reconcile.log`.

## Custody and retained decisions

Retain all completed/failed batches, refs, plans, receipts and artifacts; no
deployment cleanup was performed. Earlier completed batches `batch-HIlzE9l1`
and `batch-hip1eX2W` remain retained and reconciled, with their
`target/deployment-*` evidence. Failed batches `batch-Z5xKoNRN`,
`batch-N0QLIhim` and `batch-3v7LxlbU` produced no release/upload and were superseded
by the completed patch batch. Do not resume them against published versions.
The older failed `batch-st9pUW60` also produced no release/upload. Original
deferred lock selections remain under
`.git/release-state/deferred-platform029-20261109` and `preserve/*` refs.

The reviewed retired client archive remains at
`/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`; original committed
source remains at `0c17900090f0c2d2ebf3182598b49e651223845e`. The extraction
contract retains source provenance. The [earlier handoff archive](archive-2026-11-10.md)
remains unchanged; its historical pending versions are not release instructions.

Retain the two-pass generated-root discovery before hashing/cache admission.
Do not replace Darwin `File::sync_all` with plain `fsync`: Rust selects
`F_FULLFSYNC` there. Discovery's handwritten parser error also preserves the
cause's downcast identity, which the rejected derive changed. Earlier focused
Linux observations do not establish native macOS qualification.
