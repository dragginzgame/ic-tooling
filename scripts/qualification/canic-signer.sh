#!/usr/bin/env bash
# Current read-only Canic export with harmless signer fixtures; no keys/network.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-signer.XXXXXXXX")"
trap 'printf "Canic signer export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/evidence"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
cp "$scratch/source/Cargo.lock" "$scratch/original.lock"
sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/signer-process.patch" > "$scratch/migration.patch"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$scratch/migration.patch"
git -C "$scratch/source" apply "$scratch/migration.patch"
source "$repo/scripts/qualification/canic-signer-fixture.sh"
prepare_signer_fixture "$scratch/source" "$scratch/evidence"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build" CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
manifest="$scratch/source/Cargo.toml"
cargo metadata --manifest-path "$manifest" --format-version 1 --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "canic-host") | .id) as $host |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.80.4" and .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_host_process_next" and .pkg == $process)
' "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/original.lock" "$scratch/source/Cargo.lock" 1048576
cargo clippy --manifest-path "$manifest" -p canic-host --all-targets --locked --offline -- -D warnings
test_signer_fixture "$scratch/source" "$scratch/evidence"
printf 'Signer version/principal admission, bounded group capture and no-retry fixtures passed.\n'
