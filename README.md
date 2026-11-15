# IC Tooling New

A ground-up replacement of IC Tooling, governed by [design revision 2](docs/DESIGN.md).
The first milestone is one real build-to-owned-artifact workflow with newly
selected interfaces. Existing crates.io names may be reused with incremented
versions; retaining a name does not require retaining its old Rust API. The first
package is `ic-build` 0.87.2. Stage 2 adds `ic-host-artifacts` and `ic-host-fs`
0.80.1 for shared byte admission and held-directory publication. Toko Tooling is
pinned to 0.44.3 in both workspaces.

`ic-cargo-qualification` 0.1.1 is a separate unpublished development executable
for external-lock admission; it adds no dependencies to the ordinary Build API.

`ic-host-http` 0.80.0 is a native [complete-body adapter](docs/http-body.md).
Consumers select Reqwest directly and own TLS/request policy.
`make test-http-body` and `make test-backup-http` check the adapter and current
Backup HTTP behavior using local loopback fixtures.

`ic-host-text` 0.77.1 supplies pure [escaping and table layouts](docs/text-presentation.md).
`make test-text` and `make test-query-text` qualify the leaf and current Query’s
presentation/output callers without network reports. `make test-db-text` exercises
the current local DB console against its installed records fixture.
`make test-canic-text` checks current Canic status, terminal and declaration output.

The [release ownership map](docs/release-ownership.md) declares foundation, build,
HTTP, Text and tools groups. [Changelogs](CHANGELOG.md) live under `releases/`;
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
`make test-scheduling-consumer` qualifies the [local Scheduling migration](qualification/scheduling/README.md)
through the owned build, Jobs capture/optimization and selected installed PocketIC
subjects; prepared server/Binaryen tools and loopback access are required.
The frozen `make test-auth-consumer` recipe reproduces historical [Auth's pinned package admission](qualification/auth/README.md)
and checks exact archives on native and Wasm targets.

`make test-canic-build` qualifies current Canic’s owned build and destination
publication/refusal path using a real Wasm probe. `make test-canic-app-build`
selects app recipe preflight, lock setup and serial partial-failure behavior.
`make test-canic-build-metadata` checks workspace/target selection and refusal
before compilation or output replacement.

`make test-canic-consumer` runs [Canic’s local record admission](qualification/canic/README.md)
and selected original retained-plan and signed-recovery tests.

`make test-db-consumer` qualifies [current DB rollback](qualification/db/README.md)
using an explicitly prepared external server and the upstream async client.
`make test-jobs-snapshots` checks current Scheduling controller recovery using the
same direct-client ownership boundary and prepared Scheduling tools.
It adds no simulator facade; see [ownership](docs/simulator-ownership.md).

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
