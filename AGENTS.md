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
