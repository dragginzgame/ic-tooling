# IC Tooling New

A ground-up replacement of IC Tooling, governed by [design revision 2](docs/DESIGN.md).
The first milestone is one real build-to-owned-artifact workflow with newly
selected interfaces. Existing crates.io names may be reused with incremented
versions; retaining a name does not require retaining its old Rust API. The first
package is `ic-build` 0.87.3. Stage 2 adds `ic-host-artifacts` and `ic-host-fs`
0.80.1 for shared byte admission and held-directory publication. Toko Tooling is
pinned to 0.44.4 in both workspaces.

`ic-cargo-qualification` 0.1.2 is a separate unpublished development executable
for external-lock admission; it adds no dependencies to the ordinary Build API.

`ic-host-http` 0.80.0 is a native [complete-body adapter](docs/http-body.md).
Consumers select Reqwest directly and own TLS/request policy.
`make test-http-body` and `make test-backup-http` check the adapter and current
Backup HTTP behavior using local loopback fixtures.

`ic-host-process` 0.80.0 owns [bounded native execution](docs/process-execution.md).
Build uses its group lifecycle and stream delivery. `make test-process` checks
the core and Build failures; `make test-process-terminal` runs the Linux PTY case.
`make test-canic-foreground` qualifies the current foreground client with a harmless
tool fixture, including terminal interaction and refusal without retry.

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

The [release ownership map](docs/release-ownership.md) declares foundation, build,
HTTP, Process, Text, tools and Testkit groups. [Changelogs](CHANGELOG.md) live under `releases/`;
`make deps-list` reports each package’s group and version.

The production owner remains the read-only `../ic-tooling` reference. All new
packages remain unpublished; no production consumer has migrated.

Read [AGENTS.md](AGENTS.md), [TOKO-TOOLING.md](TOKO-TOOLING.md), and
[the current handoff](docs/status/current.md) before implementation.
The [first-slice contract](docs/build-owned-artifact.md) defines acceptance and
[the workload register](docs/workloads.md) records later migration obligations.
The [filesystem contract](docs/filesystem-custody.md) defines Stage 2. Run
`make test-custody` for its focused tests and `make test-backup-custody` for the
[local Backup migration](qualification/backup/README.md) in an isolated export.
`make test-scheduling-consumer` admits the current `../ic-scheduling-new` source,
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
failure and the fleet/placements display tests.
`make test-canic-build-metadata` checks workspace/target selection, prepared-tool
admission and compiler-wrapper refusal before compilation or output replacement.

`make test-canic-consumer` runs [Canic’s local record admission](qualification/canic/README.md)
and selected original retained-plan and signed-recovery tests.

`make test-db-consumer` qualifies [current DB rollback](qualification/db/README.md)
using an explicitly prepared external server and the upstream async client.
Scheduling’s earlier Jobs/Timers experiments are [historical](qualification/scheduling/historical.md).
The rewrite uses Testkit client types and held-reply fixtures; their replacement
replacement passes independent fixture tests and focused current-consumer qualification.
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
