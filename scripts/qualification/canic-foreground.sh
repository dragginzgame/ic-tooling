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
cp "$repo/qualification/canic/foreground-tests.rs" "$scratch/source/apps/profile/client/src/foreground_process.rs"
printf '\n#[cfg(test)]\nmod foreground_process;\n' >> "$scratch/source/apps/profile/client/src/main.rs"
cp "$repo/qualification/canic/foreground-tool.sh" "$scratch/tool"
chmod 700 "$scratch/tool"
export CANIC_FOREGROUND_EVIDENCE="$scratch/evidence" CANIC_FOREGROUND_TOOL="$scratch/tool"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build" CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
manifest="$scratch/source/Cargo.toml"
cargo metadata --manifest-path "$manifest" --format-version 1 --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "canic-profile-client") | .id) as $client |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.80.0" and .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name == "ic_host_process_next" and .pkg == $process) and
  all($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name != "ic_host_process")
' "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/original.lock" "$scratch/source/Cargo.lock" 1048576
cargo clippy --manifest-path "$manifest" -p canic-profile-client --all-targets --locked --offline -- -D warnings
cargo test --manifest-path "$manifest" -p canic-profile-client --bin canic-profile --locked --offline foreground_process::
# Command path is fixed; the environment carries paths without shell interpolation.
export CANIC_FOREGROUND_MANIFEST="$manifest"
printf 'fixture-answer\n' | script -q -e -c 'cargo test --manifest-path "$CANIC_FOREGROUND_MANIFEST" -p canic-profile-client --bin canic-profile --locked --offline foreground_process::inherited_terminal_input_and_stderr_remain_interactive -- --ignored --exact' /dev/null
printf 'Foreground consumer arguments, exact/over limits, reply refusal, no retry and terminal interaction passed.\n'
