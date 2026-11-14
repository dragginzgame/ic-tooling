# IC Tooling Agent Rules

Apply the installed [Toko Tooling baseline](TOKO-TOOLING.md), recorded in
[the snapshot](.toko-tooling.snapshot), for common engineering and action-authority
rules. Keep this overlay focused on local contracts, commands and exceptions.

Read [the current handoff](docs/status/current.md) before implementation.

## Local rules

- Keep the Host, Build and Testkit crate boundaries in [the extraction contract](docs/extraction.md).
  Do not add compatibility reexports for moved APIs or duplicate production owners.
- Consumers own product-tool pins, byte budgets, credentials, targets, retries, schemas,
  installation policy and paid-operation recovery. Libraries never claim lifecycle ownership.
- Tooling owns qualification of its Platform/Types/Runtime and optional Wire
  selections. Supplier publication never waits on this checkout or an older
  published Testkit client. See [consumer adoption](docs/hosts.md#platform-and-wire-adoption).
- Rust edition is 2024. Use ordinary directory modules and
  inherited workspace dependencies. Every product member uses the literal version of its
  group in `ci/cargo-release-groups.json`; each group's `primary` owns its version.
  Private qualification packages live in the independent `qualification/` workspace
  and retain their own versions outside product release groups.
  The root has no shared version. Group ledgers live
  under `releases/<group>/`, with the root changelog as their index.
  See [release ownership](docs/design/release-groups.md). Do not add Python tooling.
- Preserve dirty source and artifacts. Check for active Cargo/rustc processes
  using this repository's target directory before editing or compiling.
- Follow [the shared validation policy](TOKO-TOOLING.md#validation-and-evidence)
  using the local commands in [the host guide](docs/hosts.md).
  No implicit artifact cleanup. Explicit cleanup follows the Toko Tooling scope rules;
  plain Cargo cleanup retains its whole-workspace target semantics.
- Linux x86-64 and macOS 15 on Intel and Apple Silicon are required hosts.
  Report native macOS qualification separately from Linux evidence.
- Keep source provenance, design and handoff documents as evidence; follow-up
  belongs in the owning GitHub issues under the baseline.

## Upstream native PocketIC dependency

The maintainer explicitly approved returning to published `pocket-ic` 16.1 with
a breaking migration on 2026-11-10. On 2026-11-11 the maintainer approved the
native-only `ic_testkit::pocket_ic` re-export of that selected upstream crate, so
callers need not select the client separately. Preserve upstream type identity;
do not maintain a fork, restore fork-specific APIs or duplicate protocol DTOs here.
This is a scoped native IC dependency-boundary exception: upstream owns its
Candid/management identities, transport panics, polling and best-effort deletion.
Fork-specific typed transport recovery and bounded snapshot-transfer APIs retire.
Testkit owns server admission/custody and harness policy. Its helpers use the
upstream native `CanisterId`; portable helpers retain Platform Types Principal.
Keep all client dependencies native, use the caller's runtime, and do not add
blocking wrappers or a second transport to recreate removed fork guarantees.
Pool/reset recipes are retired. Snapshot callers retain stop/drain, explicit
funding and readiness policy. Qualify actual 16.1 behavior and report native
macOS evidence separately. See
[the migration](docs/testkit/client-migration.md).
