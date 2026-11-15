#!/usr/bin/env bash
# Current read-only Canic export; harmless tool fixtures, never live ICP calls.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
[[ "$(uname -s)" == Linux ]] || { echo 'This PTY recipe requires Linux script; macOS remains a separate gate.' >&2; exit 1; }
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-foreground.XXXXXXXX")"
trap 'printf "Canic foreground export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/evidence"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
cp "$scratch/source/Cargo.lock" "$scratch/original.lock"
sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/foreground-process.patch" > "$scratch/migration.patch"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$scratch/migration.patch"
git -C "$scratch/source" apply "$scratch/migration.patch"
source "$repo/scripts/qualification/canic-foreground-fixture.sh"
prepare_foreground_fixture "$scratch/source" "$scratch/evidence"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build" CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
manifest="$scratch/source/Cargo.toml"
cargo metadata --manifest-path "$manifest" --format-version 1 --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "canic-profile-client") | .id) as $client |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.80.1" and .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name == "ic_host_process_next" and .pkg == $process) and
  all($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name != "ic_host_process")
' "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/original.lock" "$scratch/source/Cargo.lock" 1048576
test_foreground_fixture "$scratch/source" "$scratch/evidence"
printf 'Foreground consumer arguments, exact/over limits, reply refusal, no retry and terminal interaction passed.\n'
