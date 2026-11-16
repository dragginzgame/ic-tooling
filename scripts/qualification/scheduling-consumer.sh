#!/usr/bin/env bash
# Admit the current rewrite without applying old Scheduling migration patches.
set -euo pipefail
mode="${1:-admit}"
selection="${2:-held-replies}"
case "$selection" in held-replies|held-pressure|lifecycle|executor|retention|jobs-lifecycle|jobs-settlement|uncertainty|jobs-boundaries|completion-refusal|aggregate|metrics-cost) ;; *) echo 'Unknown installed selection' >&2; exit 2 ;; esac
case "$mode" in admit|held|replacement|comparison) ;; *) echo "Expected admit, held, replacement or comparison; historical modes are retired." >&2; exit 2 ;; esac
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${SCHEDULING_SOURCE:-$repo/../ic-scheduling}"
# The rewrite moved to the canonical path; never substitute the legacy workspace.
git -C "$source_checkout" merge-base --is-ancestor f382bacac5461ea965e5813b56f15bd4757bf596 HEAD || {
  echo 'Scheduling source must descend from the qualified rewrite checkpoint' >&2
  exit 1
}
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/scheduling-new.XXXXXXXX")"
trap 'printf "Scheduling rewrite export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
if [[ "$mode" != admit ]]; then
  : "${SCHEDULING_POCKET_IC_BIN:?select an explicitly prepared local PocketIC server}"
  [[ "$SCHEDULING_POCKET_IC_BIN" == /* && -x "$SCHEDULING_POCKET_IC_BIN" ]] || {
    echo 'SCHEDULING_POCKET_IC_BIN must name an existing absolute executable' >&2; exit 1;
  }
  : "${SCHEDULING_TESTKIT_CLI:?select an explicitly prepared ic-testkit-server 0.106.0 launcher}"
  [[ "$SCHEDULING_TESTKIT_CLI" == /* && -x "$SCHEDULING_TESTKIT_CLI" ]] || {
    echo 'SCHEDULING_TESTKIT_CLI must name an existing absolute executable' >&2; exit 1;
  }
  [[ "$("$SCHEDULING_TESTKIT_CLI" --version)" == 'ic-testkit-server 0.106.0' ]]
  export GIT_CEILING_DIRECTORIES="$repo/target"
  selector_patch="$repo/qualification/scheduling/held-selection.patch"
  if [[ "$selection" == executor ]]; then
    selector_patch="$repo/qualification/scheduling/executor-endpoint.patch"
  fi
  git -C "$scratch/source" apply --check "$selector_patch"
  git -C "$scratch/source" apply "$selector_patch"
  "$SCHEDULING_POCKET_IC_BIN" --version > "$scratch/server-version.txt"
  sha256sum "$SCHEDULING_POCKET_IC_BIN" "$SCHEDULING_TESTKIT_CLI" > "$scratch/server.sha256"
fi
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
export CARGO_TARGET_DIR="$repo/target/scheduling-new-consumer-build"
manifest="$scratch/source/qualification/Cargo.toml"
cargo metadata --manifest-path "$manifest" --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e '. as $m |
  [.packages[] | select(.name == "ic-testkit" and .version == "0.106.0")] as $suppliers |
  $suppliers[0].id as $testkit |
  ($suppliers | length == 1) and
  all(["once-driver", "executor-driver"][]; . as $name |
    [$m.packages[] | select(.name == $name)] as $drivers |
    $drivers[0].id as $driver |
    ($drivers | length == 1) and
    any($m.resolve.nodes[] | select(.id == $driver) | .deps[]; .name == "ic_testkit" and .pkg == $testkit) and
    all($m.resolve.nodes[] | select(.id == $driver) | .deps[]; .name != "pocket_ic")) and
  any($m.resolve.nodes[] | select(.id == $testkit) | .features[]; . == "held-reply-fixtures")
' "$scratch/metadata.json" >/dev/null || {
  echo 'Expected Scheduling rewrite executor/Once drivers with shared Testkit and held-reply fixtures; source contract refused.' >&2
  exit 1
}
run_installed() {
  local output="$1"
  local driver=once-driver probe=once-probe wasm=once_probe.wasm
  if [[ "$selection" == executor ]]; then
    driver=executor-driver probe=executor-probe wasm=executor_probe.wasm
  fi
  mkdir -p "$output"
  cargo tree --manifest-path "$manifest" -p "$driver" -e normal,build --prefix none --locked --offline > "$output/driver-tree.txt"
  if [[ "$selection" == executor && ( "$mode" == replacement || "$output" == "$scratch/replacement" ) ]]; then
    rg -q '^ic-testkit v0.107.1 ' "$output/driver-tree.txt"
    if rg -q '^(wat|ic-platform|ic-platform-protocol|ic-platform-types) v' "$output/driver-tree.txt"; then
      echo 'Executor unexpectedly selects optional fixture dependencies' >&2; exit 1
    fi
  fi
  local wasm_path="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/$wasm"
  if [[ "$selection" == metrics-cost ]]; then
    cargo build --manifest-path "$manifest" -p metrics-probe --no-default-features --release --target wasm32-unknown-unknown --locked --offline
    cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/metrics_probe.wasm" "$output/metrics-off.wasm"
    cargo build --manifest-path "$manifest" -p metrics-probe --no-default-features --features metrics --release --target wasm32-unknown-unknown --locked --offline
    cp "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/metrics_probe.wasm" "$output/metrics-on.wasm"
    wasm_path="$output/metrics-off.wasm"
    sha256sum "$output/metrics-off.wasm" "$output/metrics-on.wasm" > "$output/metrics-wasm.sha256"
  else
    cargo build --manifest-path "$manifest" -p "$probe" --release --target wasm32-unknown-unknown --locked --offline
  fi
  cargo build --manifest-path "$manifest" -p "$driver" --locked --offline
  env -u IC_TESTKIT_POCKET_IC_URL POCKET_IC_BIN="$SCHEDULING_POCKET_IC_BIN" "$SCHEDULING_TESTKIT_CLI" run \
    --ttl 300 --server-stdout "$output/server.stdout" --server-stderr "$output/server.stderr" -- \
    bash "$repo/scripts/qualification/scheduling-held.sh" "$output" "$CARGO_TARGET_DIR/debug/$driver" \
    "$wasm_path" "$selection"
  if [[ "$selection" == metrics-cost ]]; then
    sha256sum --check "$output/metrics-wasm.sha256"
  fi
  sha256sum --check "$scratch/server.sha256"
}
if [[ "$mode" == comparison ]]; then
  run_installed "$scratch/baseline"
fi
if [[ "$mode" == replacement || "$mode" == comparison ]]; then
  cp "$scratch/metadata.json" "$scratch/released-metadata.json"
  cp "$scratch/source/qualification/Cargo.lock" "$scratch/released-Cargo.lock"
  sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/scheduling/replacement-testkit.patch" > "$scratch/replacement.patch"
  git -C "$scratch/source" apply --check "$scratch/replacement.patch"
  git -C "$scratch/source" apply "$scratch/replacement.patch"
  cargo metadata --manifest-path "$manifest" --format-version 1 --offline > "$scratch/metadata.json"
  jq -e --arg root "$repo" '. as $m |
    [.packages[] | select(.name == "ic-testkit")] as $suppliers |
    ($suppliers | length == 1) and $suppliers[0].version == "0.107.1" and
    $suppliers[0].manifest_path == ($root + "/crates/ic-testkit/Cargo.toml") and
    all(["once-driver", "executor-driver"][]; . as $name |
      ($m.packages[] | select(.name == $name) | .id) as $driver |
      any($m.resolve.nodes[] | select(.id == $driver) | .deps[];
        .name == "ic_testkit" and .pkg == $suppliers[0].id))
  ' "$scratch/metadata.json" >/dev/null
  cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- \
    verify-external-lock "$scratch/released-Cargo.lock" "$scratch/source/qualification/Cargo.lock" 1048576
  cargo clippy --manifest-path "$manifest" -p once-driver -p executor-driver --all-targets --locked --offline -- -D warnings
fi
if [[ "$mode" != admit ]]; then
  if [[ "$mode" == comparison ]]; then
    run_installed "$scratch/replacement"
  else
    run_installed "$scratch"
  fi
  exit 0
fi
# Check the real retained API callers, without running the aggregate installed roster.
cargo check --manifest-path "$manifest" -p once-driver -p executor-driver --all-targets --locked --offline
printf 'Scheduling rewrite drivers admitted with retained Testkit types and held-reply fixtures; no replacement or installed-runtime acceptance claimed.\n'
