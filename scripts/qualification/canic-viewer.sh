#!/usr/bin/env bash
# Current retained browser-opener boundary; fixture only, never a real desktop opener.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-viewer.XXXXXXXX")"
trap 'printf "Canic viewer export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/evidence"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
cp "$repo/qualification/canic/viewer-launch-tests.rs" "$scratch/source/crates/canic-cli/src/view/launch_qualification.rs"
printf '\n#[cfg(test)]\nmod launch_qualification;\n' >> "$scratch/source/crates/canic-cli/src/view/mod.rs"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build" CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
export CANIC_VIEWER_EVIDENCE="$scratch/evidence"
manifest="$scratch/source/Cargo.toml"
cargo metadata --manifest-path "$manifest" --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e '. as $m |
  (.packages[] | select(.name == "canic-cli") | .id) as $cli |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.79.0" and (.source | startswith("registry+"))) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $cli) | .deps[]; .name == "ic_host_process" and .pkg == $process)
' "$scratch/metadata.json" >/dev/null
cargo clippy --manifest-path "$manifest" -p canic-cli --all-targets --locked --offline -- -D warnings
cargo test --manifest-path "$manifest" -p canic-cli --bin canic --locked --offline view::
printf 'Retained opener handoff and loopback viewer boundary passed; no replacement migration claimed.\n'
