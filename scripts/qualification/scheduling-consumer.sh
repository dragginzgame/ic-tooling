#!/usr/bin/env bash
# Admit the current rewrite without applying old Scheduling migration patches.
set -euo pipefail
mode="${1:-admit}"
case "$mode" in admit|held) ;; *) echo "Expected admit or held; historical modes are retired." >&2; exit 2 ;; esac
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${SCHEDULING_SOURCE:-$repo/../ic-scheduling-new}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/scheduling-new.XXXXXXXX")"
trap 'printf "Scheduling rewrite export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
if [[ "$mode" == held ]]; then
  : "${SCHEDULING_POCKET_IC_BIN:?select an explicitly prepared local PocketIC server}"
  [[ "$SCHEDULING_POCKET_IC_BIN" == /* && -x "$SCHEDULING_POCKET_IC_BIN" ]]
  if [[ -z "${SCHEDULING_TESTKIT_CLI:-}" ]]; then
    SCHEDULING_TESTKIT_CLI="$(bash "$repo/vendor/toko-tooling/scripts/dev/install-rust-tools.sh" --consumer "$repo/../ic-scheduling" --package ic-testkit --version 0.106.0 --bin ic-testkit-server --profile release --features cli --check)"
  fi
  [[ "$("$SCHEDULING_TESTKIT_CLI" --version)" == 'ic-testkit-server 0.106.0' ]]
  export GIT_CEILING_DIRECTORIES="$repo/target"
  git -C "$scratch/source" apply --check "$repo/qualification/scheduling/held-selection.patch"
  git -C "$scratch/source" apply "$repo/qualification/scheduling/held-selection.patch"
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
if [[ "$mode" == held ]]; then
  cargo build --manifest-path "$manifest" -p once-probe --release --target wasm32-unknown-unknown --locked --offline
  cargo build --manifest-path "$manifest" -p once-driver --locked --offline
  env -u IC_TESTKIT_POCKET_IC_URL POCKET_IC_BIN="$SCHEDULING_POCKET_IC_BIN" "$SCHEDULING_TESTKIT_CLI" run \
    --ttl 300 --server-stdout "$scratch/server.stdout" --server-stderr "$scratch/server.stderr" -- \
    bash "$repo/scripts/qualification/scheduling-held.sh" "$scratch" "$CARGO_TARGET_DIR/debug/once-driver" \
    "$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/once_probe.wasm"
  sha256sum --check "$scratch/server.sha256"
  exit 0
fi
# Check the real retained API callers, without running the aggregate installed roster.
cargo check --manifest-path "$manifest" -p once-driver -p executor-driver --all-targets --locked --offline
printf 'Scheduling rewrite drivers admitted with retained Testkit types and held-reply fixtures; no replacement or installed-runtime acceptance claimed.\n'
