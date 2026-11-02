# IC Tooling

This workspace owns Host primitives (`ic-host-*`), build orchestration
(`ic-build`) and test support (`ic-testkit`, `ic-testkit-native`).
Build and Testkit use Host through explicit crate dependencies.
Private test drivers and the Wasm probe remain unpublished.

Foundation's five primitive/inspection packages, Build, Text, Host Tools and the
Testkit family have independent release versions in this workspace.
See [release ownership](docs/design/release-groups.md) and the
[Foundation](releases/foundation/CHANGELOG.md), [Build](releases/build/CHANGELOG.md),
[Text](releases/text/CHANGELOG.md), [Host Tools](releases/tools/CHANGELOG.md) and
[Testkit](releases/testkit/CHANGELOG.md) ledgers.

See [the consolidation inventory](docs/design/tooling-consolidation.md),
[Build documentation](docs/build/README.md) and
[Testkit documentation](docs/testkit/README.md).

## Host libraries

Host-side Rust libraries for artifact inspection, local filesystem safety,
admitted process execution and IC-specific format handling.

| Crate | Implemented source ownership |
| --- | --- |
| ic-host-artifacts | Bounded streams and hashing writers, exact byte comparison, SHA-256, gzip encoding/decoding, generic Wasm facts, validation and encoding |
| ic-host-fs | Regular/no-follow reads, missing-suffix path resolution, streamed durable publication, private files and observed descriptor locks |
| ic-host-http | Optional native bounded HTTP bodies with caller-selected partial failure evidence; see [usage](crates/ic-host-http/README.md) |
| ic-host-process | Executable resolution, digest/version admission, bounded capture, explicit child/group cleanup and Git observations |
| ic-host-text | Optional YAML decoding with byte admission, line normalization, decimal/UTC text, exact caller-selected quantity scales, terminal-safe rendering, Unicode tables and progress frames |
| ic-host-tools | Candid service facts, optional IC export facts, ICP command syntax, response decoding and certificate decoding |

The FS package's explicit `cli` feature supplies [native `ic-host-file` commands](docs/file-cli.md)
for bounded hashes and private create/replace operations, with caller-selected
batch limits and explicit executable preparation.

These run locally outside canisters. The ic-host prefix identifies their ecosystem;
the generic crates have no IC runtime dependency. Gzip and Wasm
support are optional features of ic-host-artifacts. Filesystem/process consumers
select the small default artifact dependency.

See the [Host API reference](docs/host-api-reference.md) for operation contracts,
resource bounds, failure evidence and caller responsibilities.

## Development

The published libraries support Linux x86-64 and macOS 15 on Intel and Apple Silicon,
with the consumer minimum declared in [Cargo.toml](Cargo.toml). The development
compiler is pinned separately in [rust-toolchain.toml](rust-toolchain.toml).
See [host qualification](docs/hosts.md) for
validation commands and native evidence requirements.

For development, prepare the toolchain selected in
[rust-toolchain.toml](rust-toolchain.toml) and the
[bootstrap prerequisites and shell PATH](docs/local-setup.md), then run:

```sh
make install-tools
make tools-check
make install-hooks
make test PACKAGE=ic-host-artifacts
```

Setup is explicit; `tools-check` verifies prepared tools offline. Hook installation
separately activates the repository-local formatter. Shared Tooling owns these
[common commands](docs/make-interface.md) and executable installation; this
workspace owns the Host, Build and Testkit libraries, package checks and publication policy.

Use `make check`, `make clippy`, `make docs-check`, `make test`
with `PACKAGE=<crate>` for focused work. Follow the
[shared validation policy](DRAGGINZGAME.md#validation-and-evidence) for when to
select those checks or full `make ci` validation. The
[host guide](docs/hosts.md) describes feature-specific and hook checks;
`make help` lists the available commands.

Git release and crates.io publication are separate operations; a tag does not
establish registry availability. Read the [release and publication guide](docs/publishing.md)
for prerequisites, explicit command effects and retained-evidence recovery.
Consumers adopt and qualify registry selections independently.

Read [the extraction contract](docs/extraction.md), [the handoff](docs/status/current.md)
and [agent rules](AGENTS.md). Human and agent contributions follow the
[PR contribution rules](rules/contributions.md).

### Bounded Wasm inspection CLI

```sh
cargo run -p ic-host-wasm-inspect --locked -- input.wasm 1048576 1000 1000 1000
```

The standalone application composes artifact/filesystem owners, retaining the five
library boundaries without depending on Host Tools or the Candid parser. It prints exact SHA-256
and raw/structural facts after complete admission; budgets and interpretation
stay caller-owned. It executes no Wasm and estimates no cycles or instructions.
Select the published package, version and binary through the standard
Shared Tooling Cargo installer; no extra feature selection is required.
Append `--expect-sha256 SHA256` for digest verification before reporting, or
`--details` for export/custom-section records after the TSV summary. These options
replace the retired `ic-host-fs` `inspect_artifact` example; default CLI output
is unchanged. See the [inspector guide](apps/ic-host-wasm-inspect/README.md).
