# Current handoff — 2026-11-10

## Checkout and delivery

Local main is 0c17900090f0c2d2ebf3182598b49e651223845e. The last completed
frozen deployment here published Foundation 0.76.2 (five packages), Build 0.81.1,
Tools 0.89.0 and Testkit 0.97.0; Text remains 0.74.3. Private qualification
packages remain 0.96.0. Batches batch-aL68C1ft and batch-9Fy1BGGp were reconciled.
Complete Linux release gates and registry verification belong to that deployment,
not the local changes below. Native macOS evidence is separate; no new native
macOS or full release qualification ran for the local cleanup batches.

Existing Cargo.lock selections remain local and preserved except required
resolution changes for the upstream PocketIC migration below. Earlier cleanup used
locked selections, including Platform 0.29.2; it performed no dependency updates.
Toko Tooling is 0.39.6, Wire 0.19 and the product Platform requirement remains 0.29.

## Local source and pending notes

All cleanup work remains uncommitted and unpublished. Product versions are
unchanged; the group ledgers select:

| Group | Pending | Completed local changes |
| --- | --- | --- |
| Foundation | 0.76.3 | Shared Linux stat parsing with distinct observer admission preserved. |
| Build | 0.81.2 | Streamed typed inventory evidence, Host FS scratch allocation, retained probe and failed bundle-cleanup evidence. |
| Tools | 0.89.1 | Derived borrowed response text; direct installation-status cases for duplicate precedence and invalid ignored metadata. |
| Testkit | 0.98.0 | Published upstream PocketIC, migrated harness/callers, diagnostic retention, semaphore-owned pool closure and consistent native gates. |

Testkit's upstream client/type/failure changes require caller migration, hence
the pending minor. The other drafts are compatible patches.
The [client migration](../testkit/client-migration.md) documents the hard cut.
[Publishing](../publishing.md) owns release gates and
frozen delivery; no release command ran for these cleanup changes.

## Upstream PocketIC hard cut

The maintainer approved replacing the maintained fork with published `pocket-ic`
16.1.0, without a compatibility module. Native harness helpers, the public example,
tests and private qualification callers now import the upstream async client and
DTOs directly. Portable Principal generation remains Types-owned; default/Wire
Testkit no longer select Platform. Only held fixtures select its codec/management
adapters. All client dependencies remain native and the Wasm graph stays Types-only.

Upstream panics and best-effort deletion replace typed transport recovery and
deletion receipts. Close/shutdown return `()`. Ordinary snapshot take/load replaces
the bounded transfer engine; timer/hook reset is caller-owned. Fork-only code,
provenance patches/notices, reconstruction scripts and tests retire together.
The external dependency keeps its own licence and notices.
The reviewed retired source archive remains at
`/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`; committed original
source remains in history at 0c17900090f0c2d2ebf3182598b49e651223845e.

Both lockfiles were resolved without broad updates. Upstream's exact constraint
requires thiserror/thiserror-impl 2.0.18 instead of the saved 2.0.21 selection.
Removing Testkit's direct Strum test dependency leaves upstream's 0.26 identity;
qualification adds upstream Base64 0.22 alongside its existing 0.23. Other shared
package versions were preserved.

Focused Linux logs are under `target/evidence/upstream-pocket-ic/`:
84 library cases passed (six explicit installed cases ignored); native installation,
management/status/metrics, install panic custody, six snapshot cases, diagnostics,
Wire calls, held/cycle replies, 17 baseline-pool cases and 11 concurrency cases
passed across the selected runs. The corrected killed-server assertion has its
own passing rerun. Installed private Wire state/caller/trap qualification passed.
Snapshot refusal also retains its structured harness cause. Product all-target
and private-driver warning-denied lint, doctests, native/Wasm graphs, Wasm
compilation, snapshot integrity and documentation links pass. No full release
gate or native macOS qualification ran for this migration.

The published Toko 0.39.2 snapshot includes the native dependency-checker repair
from [PR #375](https://github.com/dragginzgame/toko-tooling/pull/375). The installed
checker now admits actual product and private qualification declarations. Both
exact pins, exceptions and lockfiles agree with the authenticated snapshot.
The subsequent 0.39.6 adoption changes only the tooling entries in both locks.
It replaces the earlier shared-document namespace plan with online references.
The exporter retired 79 unchanged managed files: reference docs/rules/audits,
task catalogs and unused fleet-maintenance entrypoints/inputs. The installed
snapshot now has 113 files; its only Markdown files are the root baseline,
licence and contribution policy. Native product guides remain local and their
shared-reference links select producer commit
`5806e6e4943f6be600cd53385498ce08a5f804c4` from the published crate's VCS record.
The required validation-evidence helper is now an explicit entrypoint instead
of arriving through a reference guide. The Make entrypoint still opts into shared
releases. Logs are `target/toko0396-*`; no other dependency selection changed.
Focused snapshot/pin checks, documentation links, validation-roster rendering and
the root-licence publication fixture pass with this reduced selection. All eleven
Rust members resolve the same canonical root licence. No full release gate or
native macOS qualification ran for this documentation/tooling adoption.

All eleven Rust packages inherit root `LICENSE.md` through `license-file`.
Licence references are also explicit in both private Node manifests; their
package metadata resolves to that same root file. The npm lock refresh changes
only the private package's own licence field, preserving dependency selections.
The extraction contract records the maintainer's confirmation that the six
Registry/Ledger protocol projection files are team-authored.
Canonical licence/contribution files are installer-owned. By explicit maintainer direction,
the added first-party NOTICE copies and their source/archive requirements are
removed. Cargo packages the single root licence automatically; no package-local
licence files are maintained. Fresh archives for all nine product packages contain
the canonical root licence and no NOTICE entries. Root-file source/archive refusal
checks, ShellCheck and documentation links pass. Current focused results are
under `target/root-licence-*`.
Tools, pin checks, snapshot
integrity, manifest ordering and documentation links pass. The Testkit release
adapter fixture passes with setup/full qualification substituted. Logs are
`target/toko0392-*`; no full release gate or new native macOS qualification ran.
No Toko release or Tooling delivery/publication was performed. Sibling consumer
source migration is separate scope.

## Focused Linux evidence

Earlier cleanup logs remain under target/. Fork-specific proofs below are
historical and superseded by the upstream migration; they do not qualify new APIs:

- audit-*: 27 client failure tests, 28 startup tests (six explicit ignored cases
  unrun), installed PocketIC management/snapshot qualification, Testkit lint and
  provenance guards.
- reduction-*: archive/source mutation, JSON ordering and flush-failure checks;
  process observations; bounded diagnostic logs;
  warning-denied lint for the four affected packages.
- status-simplification-*: 13 ICP tests and warning-denied Host Tools lint pass.
  The original Serde installation-status projection replaces the custom JSON
  visitors (78 net production lines removed). Direct duplicate-field/invalid
  metadata cases replace the implementation-comparison test. These supersede
  the earlier projection/oracle evidence; no measured memory need justified
  retaining the custom deserializer.
- response-derive-*: 12 response-decoding tests cover borrowed plain strings,
  owned decoded escapes, duplicate authority, malformed metadata and byte limits;
  warning-denied Host Tools lint passes. Serde's borrowed-string derive replaces
  the custom text visitor (28 net production lines removed).
- install-probe-*: 14 tools/install/probe/toolchain tests, the Git observation
  consumer, final cleanup-error assertion, warning-denied Build lint and API docs.
- discovery-reduction-*: three workspace discovery tests and warning-denied Build
  lint pass. Regular and symlinked markers now share file-kind admission; absent,
  dangling and invalid markers retain their existing failure behavior.
- snapshot-hash-*: eight snapshot failure/transfer tests and donor/upstream
  reconstruction with mutation guards. The cumulative adaptation was refreshed;
  original donor/upstream fingerprints and licenses are unchanged.
- traversal-boundary-tests.log: exclusion discovery across roots, earlier aliases
  into later generated targets and fresh observations after generated/source edits.
- snapshot-traversal-clippy.log: warning-denied Build/Testkit library/test lint
  with all package features; formatting and documentation-link checks also pass.
- pool-closure-*: eight FIFO/cancellation/shutdown unit tests and two installed
  PocketIC shutdown tests pass. Queued callers wake before held leases return;
  cancellation retains closed admission and does not replay deletion. The default
  integration profile emits an existing unused dead_code-expectation warning in
  shared test helpers; all-feature warning-denied lint passes.
- native-gates-wasm*: wasm32 library/all-feature CLI compilation and portable
  dependency graph pass. pool-gates-clippy.log records Testkit lint with all
  features. wasm64's compiler cfg is Wasm-family, but its standard library is not
  installed here; no wasm64 compilation or broader target support is claimed.

The traversal investigation retained the two passes: global exclusions must be
known before any root is hashed or a cached digest admitted. Reusing unchecked
listings would weaken mutation detection. A cross-root regression now records
that boundary. No traversal speed-up, heap measurement or Wasm-size saving is
claimed. The cache-engine merger and second invocation representation were also
rejected; their reasoning is preserved in the archive below.

The directory-sync descriptor shortcut was also rejected: Rust 1.99.1's
`File::sync_all` selects `F_FULLFSYNC` on Apple targets, so replacing it with plain
`fsync` would weaken that host's durability. Its implementation remains unchanged.
The scratch Linux tree-sync test run is not native macOS qualification.
Discovery's handwritten error implementation is also retained: deriving it
changed the boxed parser cause's downcast identity, which the focused test caught.

## Custody and history

Retain all completed/failed frozen batches, refs, plans, receipts and artifacts.
The failed batch-st9pUW60 produced no release/upload. Original deferred lock
selections remain under .git/release-state/deferred-platform029-20261109 and
preserve/* refs. Successful delivery did not authorize cleanup; none ran.
Deployment logs remain under target/deploy-wire019.

The complete pre-consolidation handoff, including all earlier local and delivered
observations, is preserved unchanged in the
[2026-11-10 archive](archive-2026-11-10.md). Its original SHA-256 is recorded there.
Historical pending-version statements are not current release instructions.
