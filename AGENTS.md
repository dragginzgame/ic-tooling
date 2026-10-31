# IC Tooling Agent Rules

Read docs/status/current.md first, then [DRAGGINZGAME.md](DRAGGINZGAME.md).
The baseline is Shared Tooling 0.27.19 revision 54fc49e12f439dbe18d232f622a6d78e61773353,
recorded with exact file digests in [.shared-tooling.snapshot](.shared-tooling.snapshot).

## Local rules

- Preserve sibling working checkouts. Task-relevant inspection is authorized;
  relevant cross-repository fixes use the standing
  [isolated PR workflow](rules/contributions.md#cross-repository-pull-requests),
  including its exclusions. No repeated per-step permission is needed.
- Follow [the contribution rules](rules/contributions.md). Ordinary fixes remain
  local; delegated delivery includes its necessary Git, release and publication
  steps. Preserve target restrictions, protections and tool-download authority.
- Keep the Host, Build and Testkit crate boundaries in [the extraction contract](docs/extraction.md).
  Do not add compatibility reexports for moved APIs or duplicate production owners.
- Consumers own product-tool pins, byte budgets, credentials, targets, retries, schemas,
  installation policy and paid-operation recovery. Libraries never claim lifecycle ownership.
- Rust edition is 2024, MSRV is 1.91.0. Use ordinary directory modules and
  inherited workspace dependencies. Every member uses the literal version of its
  group in `ci/cargo-release-groups.json`; `ic-build` and `ic-testkit` own their
  respective group versions. The root has no shared version. Group ledgers live
  under `releases/<group>/`, with the root changelog as their index.
  See [release ownership](docs/design/release-groups.md). Do not add Python tooling.
- Preserve dirty source and artifacts. Check for active Cargo/rustc processes
  using this repository's target directory before editing or compiling.
- Follow [the shared validation policy](DRAGGINZGAME.md#validation-and-evidence)
  using the local commands in [the host guide](docs/hosts.md).
  Release commands retain explicit release authorization; never clean consumer artifacts.
- Linux x86-64 and macOS 15 on Intel and Apple Silicon are required hosts.
  Report native macOS qualification separately from Linux evidence.
- Follow shared changelog rules. GitHub issues are the sole follow-up tracker;
  source provenance, design and handoff documents are evidence, not issue queues.
