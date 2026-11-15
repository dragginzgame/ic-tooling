#!/usr/bin/env bash
# Qualify current local sibling inputs without modifying the sibling.
set -euo pipefail
mode="${1:-build}"
case "$mode" in build|snapshots) ;; *) exit 2 ;; esac
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${SCHEDULING_SOURCE:-$repo/../ic-scheduling}"
source_checkout="$(cd "$source_checkout" && pwd -P)"
: "${SCHEDULING_TESTKIT_CLI:?select the prepared ic-testkit-server 0.106.0 CLI}"
: "${SCHEDULING_SERVER_DIRECTORY:?select its prepared PocketIC asset directory}"
export PATH="$repo/.tools/host/bin:$PATH"
[[ "$("$SCHEDULING_TESTKIT_CLI" --version)" == 'ic-testkit-server 0.106.0' ]]
server="$("$SCHEDULING_TESTKIT_CLI" check --directory "$SCHEDULING_SERVER_DIRECTORY")"
export POCKET_IC_BIN="$server"
IC_OWNED_CARGO="$(rustup which --toolchain 1.100.0 cargo)"
IC_OWNED_RUSTC="$(rustup which --toolchain 1.100.0 rustc)"
export IC_OWNED_CARGO IC_OWNED_RUSTC
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/scheduling-consumer.XXXXXXXX")"
trap 'printf "Scheduling qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/runtime"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/scheduling/owned-build.patch"
git -C "$scratch/source" apply "$repo/qualification/scheduling/owned-build.patch"
git -C "$scratch/source" apply --check "$repo/qualification/scheduling/jobs-session.patch"
git -C "$scratch/source" apply "$repo/qualification/scheduling/jobs-session.patch"
# The original source-inventory check includes tracked and unignored files.
# This disposable fixture repository makes it inspect exactly this export.
git -C "$scratch/source" init -q
export TMPDIR="$scratch/runtime"
export CARGO_TARGET_DIR="$repo/target/scheduling-consumer-build"
export CARGO_NET_OFFLINE=true
export RUSTUP_AUTO_INSTALL=0
cd "$scratch/source"
manifest=qualification/Cargo.toml
cargo metadata --manifest-path "$manifest" --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '
  .packages as $packages |
  ([$packages[] | select(.name == "ic-build")] | length) == 1 and
  (
  ($packages[] | select(.name == "ic-timers-pocketic")) as $consumer |
  any(.resolve.nodes[] | select(.id == $consumer.id) | .deps[];
    .name == "ic_build" and
    (.pkg as $id | any($packages[]; .id == $id and .version == "0.87.1" and
      .manifest_path == ($root + "/crates/ic-build/Cargo.toml")))))
' "$scratch/metadata.json" >/dev/null
jq -e --arg root "$repo" '
  .packages as $packages |
  ($packages[] | select(.name == "ic-jobs-canister-tests")) as $consumer |
  (.resolve.nodes[] | select(.id == $consumer.id)) as $node |
  all($node.deps[]; .name != "ic_build") and
  any($node.deps[]; .name == "ic_custody" and
    (.pkg as $id | any($packages[]; .id == $id and .version == "0.80.1" and
      .manifest_path == ($root + "/crates/ic-host-fs/Cargo.toml"))))
' "$scratch/metadata.json" >/dev/null
if [[ "$mode" == build ]]; then
  cargo test --manifest-path "$manifest" -p ic-timers-pocketic --lib --bin ic-timers-pocketic --locked --offline
  cargo run --manifest-path "$manifest" -p ic-timers-pocketic --locked --offline -- focused \
    tests::lifecycle::shared_watchdog_survives_trap_and_upgrade \
    tests::ingress::typed_entrypoints_reject_invalid_wire_before_mutating_timer_state

  # Jobs owns external optimization and its retained evidence. Use the current
  # consumer's admission script to check already-prepared tools without installation.
  bash vendor/toko-tooling/scripts/dev/install-ic-tools.sh --consumer "$source_checkout" --tool wasm-opt --check
  optimizer="$source_checkout/.tools/ic/bin/wasm-opt"
  "$optimizer" --version > "$scratch/optimizer-version.txt"
  cargo test --manifest-path "$manifest" -p ic-jobs-canister-tests --bin jobs-capture-artifact --locked --offline
fi
cargo tree --manifest-path "$manifest" -p ic-jobs-canister-tests --features harness --edges normal,build --prefix none --locked --offline > "$scratch/jobs-graph.txt"
awk '
  $1 == "ic-testkit" { legacy = 1 }
  $1 == "pocket-ic" && $2 == "v16.1.0" { upstream = 1 }
  END { exit (legacy || !upstream) }
' "$scratch/jobs-graph.txt"
cargo build --manifest-path "$manifest" -p ic-jobs-test-consumer --release --target wasm32-unknown-unknown \
  --target-dir "$scratch/jobs-build" --message-format=json --locked --offline > "$scratch/jobs-build.jsonl"
input="$(jq -ser '
  [.[] | select(.reason == "compiler-artifact" and .target.name == "ic_jobs_test_consumer") |
   .filenames[] | select(endswith(".wasm"))] |
  if length == 1 then .[0] else error("expected one Jobs Wasm") end
' "$scratch/jobs-build.jsonl")"
mkdir "$scratch/jobs"
original="$scratch/jobs/original.wasm"
cargo run --manifest-path "$manifest" -p ic-jobs-canister-tests --bin jobs-capture-artifact --locked --offline -- "$input" "$original"
cmp "$input" "$original"
if [[ "$mode" == snapshots ]]; then
  JOBS_QUALIFICATION_EXPORT="$scratch" IC_JOBS_CONSUMER_WASM="$original" \
    "$SCHEDULING_TESTKIT_CLI" run --ttl 900 -- bash "$repo/scripts/qualification/jobs-session.sh"
  exit 0
fi
for variant in original O3 Os Oz; do
  artifact="$original"
  if [[ "$variant" != original ]]; then
    mkdir "$scratch/jobs/$variant"
    artifact="$scratch/jobs/$variant/consumer.wasm"
    cargo run --manifest-path "$manifest" -p ic-jobs-canister-tests --bin jobs-capture-artifact --locked --offline -- \
      "$original" "$artifact" --optimize "$optimizer" "-$variant"
    [[ ! -e "$scratch/jobs/$variant/producer.wasm" && -f "$scratch/jobs/$variant/optimizer.log" ]]
  fi
  IC_JOBS_CONSUMER_WASM="$artifact" "$SCHEDULING_TESTKIT_CLI" run --ttl 900 -- \
    cargo test --manifest-path "$manifest" -p ic-jobs-canister-tests --test recovery --features harness --locked --offline -- \
    --ignored --exact management_upgrade_and_real_watchdog_delivery --nocapture
done
