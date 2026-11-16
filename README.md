# IC Tooling

A ground-up replacement of IC Tooling, governed by [design revision 2](docs/DESIGN.md).
The first milestone is one real build-to-owned-artifact workflow with newly
selected interfaces. Existing crates.io names may be reused with incremented
versions; retaining a name does not require retaining its old Rust API. The first
package is `ic-build` 0.87.3. Stage 2 adds `ic-host-artifacts` and `ic-host-fs`
0.80.2 for shared byte admission and held-directory publication. Toko Tooling is
pinned to 0.44.5 in both workspaces.

The [first release](docs/release-ownership.md#first-production-cutover-scope)
contains Artifacts, FS, Process and Build. `make test-build-release-candidate`
checks extracted Cargo archives and a fresh current Canic build export. Linux,
Apple Silicon macOS and Intel macOS acceptance pass. Artifacts 0.80.2, FS 0.80.2,
Process 0.80.2 and Build 0.87.3 are published on crates.io. Production consumer
adoption remains separate; these APIs require explicit migration from legacy.
Foundation 0.80.2 adds [original retained-file publication](docs/filesystem-custody.md#reopening-an-original-publication-file)
after reopening, without rewriting recovery bytes. Its final versioned archives
pass native CI on all three hosts, and registry readback matches uploaded bytes.

The byte owner also supplies [complete producer commitments](docs/byte-commitments.md).
Independent tests pass; `make test-backup-commitments` compares the selected
consumer migration, currently blocked by failures in the unchanged Backup baseline.
`make test-backup-assembly` passes four current fixed-capture completion/refusal
tests with both suppliers and Clippy, without selecting the restore migration.

Filesystem [directory preparation](docs/directory-preparation.md) returns a held
leaf after explicit child creation, preserving partial failures. Independent Linux
tests and `make test-canic-publication` pass for current Canic artifact output;
other writers and native macOS consumer qualification remain open.
`make test-canic-config-publication` also passes six unchanged configuration tests
with both suppliers, retaining Canic's original sidecar-lock protocol.
`make test-canic-record-publication` compares create-only retained records and
signed-attempt recovery, including original authority and expiry behavior.
Filesystem [directory publication](docs/directory-publication.md) independently
passes no-replace and partial-failure tests. `make test-canic-directory-publication`
also passes 16 current starter/fleet tests with both suppliers.
`make test-canic-filesystem` composes retained reads and all these publication
adapters on one current source capture, checking both suppliers' test inventories.
Held-directory [sidecar locks](docs/sidecar-locks.md) pass independent tests.
`make test-canic-sidecar-lock` compares Store/configuration locking alongside
the retained-read/publication cases and released-helper interoperability.
`make test-canic-parent-preparation` also compares Store/plan writers using the
prepared directory directly for locking and publication.
That recipe also retains starter/fleet staging custody from allocation through
publication, with two additional collision and identity acceptance tests.
Starter and fleet staging member parents and writes also use held custody.
The composed recipe now includes recovery/release preparation and six publication
cases: 47 paired tests plus two custody tests. Live signing/submission remains
outside this qualification.
Artifact inspection adds two paired cases (49 total), preserving input aliases,
bounded regular-file admission and exact Wasm identity.
Private app-output comparison adds three paired alias/traversal/depth cases
(52 total), retaining refusal before locks or output creation.
Host's direct released-FS edge is test-only in this export, verified by library
compilation and metadata. The recipe also composes Host's Process adapters: nine
focused Process cases pass, and its native normal/build graph contains no legacy
FS or Process. `make test-canic-runtime` extends that same capture through real
CLI artifact/app builds, four viewer tests and 20 metadata/tool refusals.
Report file publication adds two paired cases through held FS custody and a
replacement-only parent-traversal refusal. Automatic report-directory allocation
still selects legacy FS; archive qualification remains open.

`ic-cargo-qualification` 0.1.3 is a separate unpublished development executable
for external-lock admission; it adds no dependencies to the ordinary Build API.

`ic-host-http` 0.80.0 is a native [complete-body adapter](docs/http-body.md).
Consumers select Reqwest directly and own TLS/request policy.
`make test-http-body` and `make test-backup-http` check the adapter and current
Backup HTTP behavior using local loopback fixtures.

`ic-host-process` 0.80.2 owns [bounded native execution](docs/process-execution.md).
Its [handoff/reaping API](docs/process-handoff.md) passes independent Linux tests
and current Canic viewer qualification, including reaping after observer drop.
Existing 0.80.1 consumer evidence remains tied to that version.
Build uses its group lifecycle and stream delivery. `make test-process` checks
the core and Build failures; `make test-process-terminal` runs the Linux PTY case.
`make test-canic-foreground` qualifies the current foreground client with a harmless
tool fixture, including terminal interaction and refusal without retry.
`make test-canic-signer` checks version/principal admission and bounded group
capture in a current Canic export, using harmless local signer fixtures.
`make test-canic-lock` checks Cargo lock initialization, diagnostic refusal and
existing-lock preservation with local Process 0.80.2 in a current Canic export.
On Linux, `make test-canic-process` combines Build, Text, lock initialization,
signer capture and the foreground client's real PTY fixture on one current export.
It includes the loopback viewer and opener reaping, requiring local socket access.
It also migrates metadata capture and checks subprocess, selection and prepared-tool
refusals before compilation; Host and foreground-client direct legacy Process edges are absent.
`make test-canic-viewer` checks the retained desktop-opener baseline
with a harmless child fixture and local loopback viewer tests; it launches no browser.
`make test-canic-viewer-replacement` selects local Process 0.80.2 and also checks
Linux reaping after the completion observer is dropped.

`ic-host-text` 0.77.2 supplies pure [escaping and table layouts](docs/text-presentation.md).
`make test-text` and `make test-query-text` qualify the leaf and current Query’s
presentation/output callers without network reports. `make test-db-text` exercises
the current local DB console against its installed records fixture.
`make test-canic-text` checks current Canic status, terminal and declaration output.

`ic-testkit` 0.107.1 starts the [shared Testkit contract](docs/testkit-held-replies.md)
with the original PocketIC type boundary, an owned bounded reply catalog and Wasm
generation. `make test-testkit-catalog` checks admission and module validity;
`make test-testkit-module` checks installed behavior on an externally owned server.
`make test-testkit-control` checks exclusive release and identity/progress refusals.
`make test-scheduling-replacement` qualifies a current Scheduling export against
the replacement; the sibling itself continues selecting the released supplier.
`make test-scheduling-pressure` selects watchdog admission/pressure and Jobs
pressure; `make test-scheduling-pressure-baseline` uses the released supplier.
`make test-scheduling-lifecycle` compares released and replacement suppliers on
one capture for watchdog traps/owner commands, refusal backoff and upgrade cases.
`make test-scheduling-executor` compares the synchronous executor trap/admission
cases and checks that its default Testkit graph excludes fixture dependencies.
`make test-scheduling-retention` compares retained callback custody and settlement
after public handles are dropped, using both suppliers on one source capture.
`make test-scheduling-jobs-lifecycle` compares calendar results, owner precedence,
upgrade reconstruction and downtime behavior on one current capture.
`make test-scheduling-jobs-settlement` compares retry budgets, explicit
reconciliation and completion-relative scheduling with both suppliers.
`make test-scheduling-uncertainty` compares transient lifetime custody and
unresolved-work reconstruction across actual upgrades.
`make test-scheduling-jobs-boundaries` compares initial and one-shot deadlines,
schedule/progress boundaries and bounded catch-up, including paused upgrades.
`make test-scheduling-completion-refusal` compares completion cadence, diagnostics
and low-cycle delivery refusal against the rewrite at its canonical path.
`make test-scheduling-aggregate` compares the complete Once installed roster,
including control/dispatch cost observations. Installed selections require explicit
`SCHEDULING_POCKET_IC_BIN` and `SCHEDULING_TESTKIT_CLI` executable paths.
`make test-scheduling-metrics` compares the existing metrics-disabled/enabled
probe with both Testkit suppliers, retaining exact Wasm bytes and observations.

The [release ownership map](docs/release-ownership.md) declares foundation, build,
HTTP, Process, Text, tools and Testkit groups. [Changelogs](CHANGELOG.md) live under `releases/`;
`make deps-list` reports each package’s group and version.

Read [AGENTS.md](AGENTS.md), [TOKO-TOOLING.md](TOKO-TOOLING.md), and
[the current handoff](docs/status/current.md) before implementation.
The [first-slice contract](docs/build-owned-artifact.md) defines acceptance and
[the workload register](docs/workloads.md) records later migration obligations.
The [filesystem contract](docs/filesystem-custody.md) defines Stage 2. Run
`make test-custody` for its focused tests and `make test-backup-custody` for the
[local Backup migration](qualification/backup/README.md) in an isolated export.
`make test-scheduling-consumer` admits the rewrite now at `../ic-scheduling`,
its retained Testkit dependency graph, and focused host-driver compilation.
See [current requirements](qualification/scheduling/README.md); this does not
claim installed-runtime or replacement Testkit acceptance.
`make test-scheduling-held` selects two installed held-reply scenarios with the
released supplier and an explicitly prepared server. The [Testkit contract](docs/testkit-held-replies.md)
records the shared implementation and its independent acceptance evidence.
The frozen `make test-auth-consumer` recipe reproduces historical [Auth's pinned package admission](qualification/auth/README.md)
and checks exact archives on native and Wasm targets.

`make test-canic-build` qualifies current Canic’s owned build and destination
publication/refusal path using a real Wasm probe. `make test-canic-app-build`
combines Build and Text for app recipe preflight, lock setup, serial partial
failure and display tests, plus Process 0.80.2 for lock initialization and signer
version/principal capture. Signer fixtures make no live calls and use no keys.
`make test-canic-build-metadata` checks workspace/target selection, prepared-tool
admission and compiler-wrapper refusal before compilation or output replacement.

`make test-canic-consumer` runs [Canic’s local record admission](qualification/canic/README.md)
and selected original retained-plan and signed-recovery tests.

`make test-db-consumer` qualifies [current DB rollback](qualification/db/README.md)
using an explicitly prepared external server and the upstream async client.
Scheduling’s earlier Jobs/Timers experiments are [historical](qualification/scheduling/historical.md).
The rewrite uses Testkit client types and held-reply fixtures; their replacement
passes independent fixture tests and focused current-consumer qualification.
See [simulator ownership](docs/simulator-ownership.md).

`make test-platform-consumer` qualifies [Platform’s current raw_rand fixture](qualification/platform/README.md)
with direct PocketIC and externally owned server lifetime.
`make test-platform-agent` selects Agent ingress, certificates and upstream type identity. Other Platform drivers
and Query remain open qualification work.

Active consumer runners snapshot current local sibling inputs without modifying
them; see [source selection](docs/qualification/local-sources.md). Old pinned runs
remain historical evidence. Backup’s current-source tests include local-loopback
metadata recovery fixtures.

Development uses the pinned Rust 1.100.0 compiler. Real Wasm qualification needs
its `wasm32-unknown-unknown` target, prepared explicitly with `rustup target add`.
The installed Toko tools remain the common setup/formatting/snapshot owner.
Run `make help` for current commands; previous copied-crate test targets are retired.

Historical implementation and evidence remain identifiable in Git at
`cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`. They describe the superseded transplant,
not acceptance of this replacement. No performance improvement is claimed.
