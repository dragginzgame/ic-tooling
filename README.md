# IC Tooling New

A ground-up replacement of IC Tooling, governed by [design revision 2](docs/DESIGN.md).
The first milestone is one real build-to-owned-artifact workflow with newly
selected interfaces. Existing crates.io names may be reused with incremented
versions; retaining a name does not require retaining its old Rust API. The first
package is `ic-build` 0.87.0. Stage 2 adds `ic-host-artifacts` and `ic-host-fs`
0.80.0 for shared byte admission and held-directory publication. Toko Tooling is
pinned to 0.44.3 in both workspaces.

The production owner remains the read-only `../ic-tooling` reference. All new
packages remain unpublished; no production consumer has migrated.

Read [AGENTS.md](AGENTS.md), [TOKO-TOOLING.md](TOKO-TOOLING.md), and
[the current handoff](docs/status/current.md) before implementation.
The [first-slice contract](docs/build-owned-artifact.md) defines acceptance and
[the workload register](docs/workloads.md) records later migration obligations.
The [filesystem contract](docs/filesystem-custody.md) defines Stage 2. Run
`make test-custody` for its focused tests and `make test-backup-custody` for the
[pinned Backup migration](qualification/backup/README.md) in an isolated export.

Development uses the pinned Rust 1.100.0 compiler. Real Wasm qualification needs
its `wasm32-unknown-unknown` target, prepared explicitly with `rustup target add`.
The installed Toko tools remain the common setup/formatting/snapshot owner.
Run `make help` for current commands; previous copied-crate test targets are retired.

Historical implementation and evidence remain identifiable in Git at
`cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`. They describe the superseded transplant,
not acceptance of this replacement. No performance improvement is claimed.
