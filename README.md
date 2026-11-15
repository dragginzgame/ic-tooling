# IC Tooling New

Development implementation of the [replacement design](docs/design/DESIGN.md),
hosted at [dragginzgame/ic-tooling-new](https://github.com/dragginzgame/ic-tooling-new).
Build is version 0.86.1; Foundation remains 0.79.0. Crates.io publication is disabled.
The maintainer authorized implementation on 2026-11-15. The first slice retains
Foundation (Artifacts, FS, Process) and Build from the latest local `../ic-tooling`
and adds bounded streaming Cargo artifact selection. That sibling remains the
production owner; no consumer dependency or publication authority has moved.

Read [current progress](docs/status/current.md), [source provenance](docs/design/source-provenance.md)
and the [streaming selection guide](docs/build/streaming-selection.md).
The original [inventory](docs/design/inventory.tsv) and
[consumer evidence](docs/design/consumer-evidence.txt) remain historical inputs.

## Development

Use Rust 1.100.0 from `rust-toolchain.toml`. The real Cargo integration fixture
also requires its `wasm32-unknown-unknown` target. Prepare tools explicitly:

```sh
rustup target add --toolchain 1.100.0 wasm32-unknown-unknown
make install-tools
make tools-check
make install-hooks
```

Setup may download pinned tools. Normal checks are offline:

```sh
make check                 # Cargo tooling selection and installed bytes
make test-build-selection  # selector/protocol tests and real Wasm capture
make test-scheduling-consumer # real Scheduling host tests in isolated scratch
make fmt-check             # prepared shared formatting tools
make help                  # list developer commands and their effects
```

Dependency and LOC tools:

```sh
make deps-list
make deps-update
make deps-update DEPENDENCY_PACKAGE=toko-tooling DEPENDENCY_PRECISE=0.44.0
cargo tree --locked --offline
toko-cloc
toko-cloc -d 1 ic-build
make tools-report
make toko-tooling-status
```

`deps-list` lists local packages, not transitive dependencies or registry status.
`deps-update` runs Cargo updates sequentially for the product workspace and, once
present, the independent qualification workspace. It may access the network and
change lockfiles; it preserves manifest constraints. Use `CARGO_NET_OFFLINE=true`
for cached resolution. `DEPENDENCY_MANIFEST=path/Cargo.toml` adds an explicit
workspace. Inspect lock diffs after a failure because updates are not atomic.
An exact Toko version change requires editing its root pin before updating the
lock and running `make toko-tooling-preview` / `make toko-tooling-update`.

`toko-cloc` is a standalone command already available on this machine. On a new
machine, run `bash vendor/toko-tooling/scripts/dev/toko-cloc` directly, or install
that launcher onto your PATH. It selects the current repository's reviewed
reporter and prepared tools. No Make alias or separate LOC implementation is used.

`make test-build-selection` is a focused development check, not the complete
release gate. Product packages inherit root dependencies, metadata and lints.
All four packages have `publish = false`. [Scheduling qualification](qualification/scheduling/README.md)
contains a pinned consumer migration patch and uses that consumer's real workspace
and tests in isolated build scratch. There is no synthetic local qualification
package or manifest. Installed simulator/archive qualification and the remaining
product crates are subsequent design work.

## Engineering baseline

Read [TOKO-TOOLING.md](TOKO-TOOLING.md) and [AGENTS.md](AGENTS.md).
Toko Tooling is exactly pinned at 0.44.0 in Cargo and admitted from the registry
archive recorded in [Cargo.lock](Cargo.lock) and [.toko-tooling.snapshot](.toko-tooling.snapshot).
Shared commands live under `vendor/toko-tooling/`; local changes belong outside it.
The source checkout's lock selections were preserved for retained dependencies.

See [LICENSE.md](LICENSE.md) and [CONTRIBUTING.md](CONTRIBUTING.md).
