# IC Build

IC Build owns Cargo execution and reported artifact selection, explicit input
inventories, invocation-owned artifact capture, package qualification and the
release-metadata editor. Consumers select recipes, tools, complete environments,
source/target custody, budgets, retained-data formats and publication policy.

The pending 0.85 API removes persistent Wasm caches and generic transaction
caches, their input resolver, batches, fingerprints, pruning and evidence wrappers.
Cargo retains its ordinary incremental compilation. Build outputs are consumed
under caller custody or copied into a `CapturedArtifact` for one invocation.
Existing on-disk caches remain untouched. See the
[migration and API guide](../../docs/build/artifacts.md).

| Owner | Supported surface |
| --- | --- |
| `cargo::execution` | `CargoInvocation`, `execute_cargo`, `CargoBuildFacts`, exact completed library/example Wasm selection |
| `cargo` | Metadata/tree observations, feature-safe grouping, native executable builds, Cargo diagnostics/exports, package qualification and lock projections |
| `artifacts` | Declared input evidence/depfiles, owned temporary captures, explicit output retention and native Candid extraction |
| `tools` / `provenance` | Caller-selected installation/toolchain and Git observations |

Build uses Host Artifacts, FS and Process directly. It has no PocketIC, Platform
or application-framework dependency. Testkit owns simulator lifecycle; consumers
own source leases, transforms, optimizer policy and release admission.

## Build and capture

Construct one `CargoInvocation`, including a nonempty `RUSTC` environment entry.
Admit `execution` and successful exit, then `messages`, then `facts`. Tool
observations do not prove source freshness or retain mutable Cargo output paths.
Pass the same recipe to `select_cdylib_wasm(messages, manifest, &invocation.recipe)`;
it requires one successful completion and distinguishes libraries from examples.
Hold target custody through reading/capture and keep captured owners alive through
consumption. Declare source inventories explicitly when the consumer needs them.

The [compiled Wasm example](examples/wasm_build.rs) builds once, holds a target
lock through capture and reads bounded bytes after releasing the target:

```text
cargo run -p ic-build --locked --offline --example wasm_build -- /checkout package
```

The [native example](examples/native_executable.rs) selects a binary/example from
Cargo's canonical metadata/messages and retains caller-owned execution policy.
[LTO evidence](../../docs/build/artifacts.md#deployable-wasm-lto) remains separate
from artifact custody and product optimization defaults.

## Release metadata

The optional `cli` feature builds `ic-build-release-metadata`. Library-only
consumers omit it and do not select Clap. Install a published editor with
`cargo install --locked --features cli ic-build`, or build this checkout with
`cargo build -p ic-build --features cli --bin ic-build-release-metadata`.

`group-manifest SOURCE PREVIOUS NEXT SELECTION_JSON` uses an explicit
`--max-input-bytes` allowance. Selection declares `version` (`workspace`, `package`
or `unchanged`) and exact dependency `alias`/`previous`/`next` projections. The
editor preserves unselected bytes and infers no group or release policy.

## Development

Rust 1.100.0 is required. Select `PACKAGE=ic-build` for ordinary Make checks.
The `test-support` feature supplies Cargo fixture scaffolding and retained
temporary directories; integration targets declare that requirement explicitly.
For Cargo execution/output contracts, run
`cargo test -p ic-build --features test-support --test cargo_execution --locked --offline`.
Default unit tests need no feature. Native optimizer qualification remains an
independent explicit workflow with real transforms and execution.

See [host qualification](../../docs/hosts.md), [publication](../../docs/publishing.md),
[ownership](../../docs/extraction.md) and [source provenance](../../docs/design/tooling-consolidation.md).

### Consumer-owned archive checks

On Unix, `cargo::packages::qualify_existing_package_archives` admits already-built
archives without repackaging them or selecting tests. It constructs the admitted
verification workspace, freezes external lock selections and invokes the caller's
checks with that workspace root. Consumers can create separate native/isolated-feature/Wasm
fixtures beneath the root. Extracted source bytes, admitted manifest/lock and
original source/archive identities must remain unchanged on success. The caller
owns check coverage, tools, budgets, retention and any subsequent release policy.

`qualify_package_archives` retains its existing packaging, all-target and doctest
convenience flow. A successful custom callback is followed by the same custody
verification; a callback failure retains its original cause and attempt directory.
