# IC Tooling Agent Rules

Read [the current handoff](docs/status/current.md) and the installed
[baseline](DRAGGINZGAME.md). [.shared-tooling.snapshot](.shared-tooling.snapshot)
records the exact adopted files and revision.

## Local rules

- Keep the Host, Build and Testkit crate boundaries in [the extraction contract](docs/extraction.md).
  Do not add compatibility reexports for moved APIs or duplicate production owners.
- Consumers own product-tool pins, byte budgets, credentials, targets, retries, schemas,
  installation policy and paid-operation recovery. Libraries never claim lifecycle ownership.
- Rust edition is 2024. Use ordinary directory modules and
  inherited workspace dependencies. Every member uses the literal version of its
  group in `ci/cargo-release-groups.json`; each group's `primary` owns its version.
  The root has no shared version. Group ledgers live
  under `releases/<group>/`, with the root changelog as their index.
  See [release ownership](docs/design/release-groups.md). Do not add Python tooling.
- Preserve dirty source and artifacts. Check for active Cargo/rustc processes
  using this repository's target directory before editing or compiling.
- Follow [the shared validation policy](DRAGGINZGAME.md#validation-and-evidence)
  using the local commands in [the host guide](docs/hosts.md).
  No implicit artifact cleanup. Explicit cleanup follows the Shared scope rules;
  plain Cargo cleanup retains its whole-workspace target semantics.
- Linux x86-64 and macOS 15 on Intel and Apple Silicon are required hosts.
  Report native macOS qualification separately from Linux evidence.
- Follow shared changelog rules. GitHub issues are the sole follow-up tracker;
  source provenance, design and handoff documents are evidence, not issue queues.

## Maintained native PocketIC client

Tooling #324 transfers Platform’s qualified PocketIC 16.1 client into
`ic-testkit::pocket_ic`. Preserve imported Apache-2.0 licenses, protocol bytes,
external identities, fallible/no-replay behavior and bounded snapshot transfer.
The exact donor and upstream reconstruction guards own its source provenance.
The native module retains donor lint policy for unchanged imported bodies. Its
existing server convenience implementation alone retains direct flate2/tempfile
selection during this transfer; shared Host APIs remain the owner elsewhere.
This exception does not authorize new gzip/temporary-file implementations or
changes to hash, key or signature algorithms. Keep all client dependencies native.

### Management DTO identity

The transferred PocketIC 16.1 client retains the donor's exact
`ic-management-canister-types = "=0.11.0"` selection. Platform 0.20 exposes the same
management DTO identity, including snapshot metadata and status/settings fields.
Keep this pin until an intentional joint adapter upgrade qualifies client/facade
type identity and the installed management/snapshot contract. Run the focused
`client_management_identity` test and the `client_installed` test with an explicitly
admitted 16.1 server; native host and Wasm graph obligations remain separate.
