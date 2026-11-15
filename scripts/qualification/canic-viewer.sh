#!/usr/bin/env bash
# Current retained browser-opener boundary; fixture only, never a real desktop opener.
set -euo pipefail
mode="${1:-retained}"
case "$mode" in retained|replacement) ;; *) exit 2 ;; esac
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-viewer.XXXXXXXX")"
trap 'printf "Canic viewer export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/evidence"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
cp "$scratch/source/Cargo.lock" "$scratch/original.lock"
if [[ "$mode" == replacement ]]; then
  export GIT_CEILING_DIRECTORIES="$repo/target"
  sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/viewer-process.patch" > "$scratch/migration.patch"
  git -C "$scratch/source" apply --check "$scratch/migration.patch"
  git -C "$scratch/source" apply "$scratch/migration.patch"
fi
cp "$repo/qualification/canic/viewer-launch-tests.rs" "$scratch/source/crates/canic-cli/src/view/launch_qualification.rs"
printf '\n#[cfg(test)]\nmod launch_qualification;\n' >> "$scratch/source/crates/canic-cli/src/view/mod.rs"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build" CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
export CANIC_VIEWER_EVIDENCE="$scratch/evidence"
manifest="$scratch/source/Cargo.toml"
if [[ "$mode" == replacement ]]; then
  cargo metadata --manifest-path "$manifest" --format-version 1 --offline > "$scratch/metadata.json"
  cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/original.lock" "$scratch/source/Cargo.lock" 1048576
  jq -e --arg root "$repo" '. as $m |
    (.packages[] | select(.name == "canic-cli") | .id) as $cli |
    (.packages[] | select(.name == "ic-host-process" and .version == "0.80.2" and .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
    any($m.resolve.nodes[] | select(.id == $cli) | .deps[]; .name == "ic_host_process_next" and .pkg == $process) and
    all($m.resolve.nodes[] | select(.id == $cli) | .deps[]; .name != "ic_host_process")
  ' "$scratch/metadata.json" >/dev/null
else
cargo metadata --manifest-path "$manifest" --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e '. as $m |
  (.packages[] | select(.name == "canic-cli") | .id) as $cli |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.79.0" and (.source | startswith("registry+"))) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $cli) | .deps[]; .name == "ic_host_process" and .pkg == $process)
' "$scratch/metadata.json" >/dev/null
fi
cargo clippy --manifest-path "$manifest" -p canic-cli --all-targets --locked --offline -- -D warnings
cargo test --manifest-path "$manifest" -p canic-cli --bin canic --locked --offline view::
printf '%s opener handoff and loopback viewer boundary passed.\n' "$mode"
