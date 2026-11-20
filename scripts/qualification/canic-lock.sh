#!/usr/bin/env bash
# Current lock-initialization caller; other Canic Build suppliers stay selected.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-lock.XXXXXXXX")"
trap 'printf "Canic lock export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/probe" "$scratch/runtime"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
cp "$scratch/source/Cargo.lock" "$scratch/original.lock"
[[ ! -L "$scratch/source/crates/canic-host/src/app_build" ]]
[[ ! -e "$scratch/source/crates/canic-host/src/app_build/mod.rs" && ! -L "$scratch/source/crates/canic-host/src/app_build/mod.rs" ]]
mkdir -p "$scratch/source/crates/canic-host/src/app_build"
mv "$scratch/source/crates/canic-host/src/app_build.rs" "$scratch/source/crates/canic-host/src/app_build/mod.rs"
sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/lock-process.patch" > "$scratch/migration.patch"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$scratch/migration.patch"
git -C "$scratch/source" apply "$scratch/migration.patch"
cp "$repo/qualification/canic/lock-process-tests.rs" "$scratch/source/crates/canic-host/src/app_build/lock_process_tests.rs"
printf '\n#[cfg(test)]\nmod lock_process_tests;\n' >> "$scratch/source/crates/canic-host/src/app_build/mod.rs"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build" RUSTUP_AUTO_INSTALL=0
manifest="$scratch/source/Cargo.toml"
cargo metadata --manifest-path "$manifest" --format-version 1 > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "canic-host") | .id) as $host |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.80.4" and .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_host_process_next" and .pkg == $process)
' "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked -- verify-external-lock "$scratch/original.lock" "$scratch/source/Cargo.lock" 1048576
cargo clippy --manifest-path "$manifest" -p canic-host --all-targets --locked -- -D warnings
cargo test --manifest-path "$manifest" -p canic-host --lib --locked lock_process_tests::
cargo build --manifest-path "$manifest" -p canic-cli --bin canic --locked
cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$scratch/probe/"
cp -R "$repo/qualification/owned-wasm-probe" "$scratch/probe/"
export CARGO_TARGET_DIR="$scratch/published" TMPDIR="$scratch/runtime"
bash "$repo/scripts/qualification/canic-app-build.sh" "$repo/target/canic-consumer-build/debug/canic" "$scratch"
