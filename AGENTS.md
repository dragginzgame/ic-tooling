# IC Tooling Agent Rules

Read [the current handoff](docs/status/current.md) and the installed
[baseline](TOKO-TOOLING.md). [.toko-tooling.snapshot](.toko-tooling.snapshot)
records the exact adopted files and revision.

## Local rules

- Keep the Host, Build and Testkit crate boundaries in [the extraction contract](docs/extraction.md).
  Do not add compatibility reexports for moved APIs or duplicate production owners.
- Consumers own product-tool pins, byte budgets, credentials, targets, retries, schemas,
  installation policy and paid-operation recovery. Libraries never claim lifecycle ownership.
- Tooling owns qualification of its Platform/Types/Runtime and optional Wire
  selections. Supplier publication never waits on this checkout or an older
  published Testkit client. See [consumer adoption](docs/hosts.md#platform-and-wire-adoption).
- Rust edition is 2024. Use ordinary directory modules and
  inherited workspace dependencies. Every member uses the literal version of its
  group in `ci/cargo-release-groups.json`; each group's `primary` owns its version.
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
- Follow shared changelog rules. GitHub issues are the sole follow-up tracker;
  source provenance, design and handoff documents are evidence, not issue queues.

## Maintained native PocketIC client

Tooling #324 transfers Platform’s qualified PocketIC 16.1 client into
`ic-testkit::pocket_ic`. Preserve imported Apache-2.0 licenses, protocol bytes, fallible/no-replay behavior and bounded snapshot transfer.
The exact donor and upstream reconstruction guards own its source provenance.
The native module retains donor lint policy for unchanged imported bodies.
The client requires an explicit server URL; `pic` owns server provisioning and
managed startup. Persistent instance directories are caller-owned paths; the
client creates no temporary state handles or clones. Changes to hash, key or
signature algorithms remain outside this maintenance scope. Keep all client
dependencies native.

### Management DTO identity

The native client uses Platform-owned management DTOs and canonical Types
Principal. No registry management dependency or external Principal alias remains.
Native install/create/log DTOs belong to Platform alongside snapshot DTOs;
keep them out of Wasm. Preserve external Candid/Serde bytes and admitted 16.1
management/snapshot behavior. The owned codec uses explicit work and allocation
budgets, not the retired upstream quotas. Run `client_management_identity` and
the installed `client_installed` test with an explicitly admitted 16.1 server;
native host and Wasm graph obligations remain separate.

### Async-only client contract

Testkit keeps one async PocketIC implementation. The caller owns
its Tokio runtime. Network and lifecycle helpers must not recreate blocking
wrappers or private runtimes. Explicit instance/baseline close and raw client
async deletion retain server custody; Rust Drop performs no client network I/O.
Keep cancellation/no-replay evidence and force cancelled pool preparations to
rebuild before reuse. Pure fixture generation and command-process execution stay
synchronous. See [the migration](docs/testkit/client-migration.md).
