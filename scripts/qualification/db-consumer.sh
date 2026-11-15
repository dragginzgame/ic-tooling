#!/usr/bin/env bash
# Current DB rollback oracle against an explicit externally owned simulator.
set -euo pipefail
mode="${1:-rollback}"
case "$mode" in rollback|text) ;; *) exit 2 ;; esac
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${DB_SOURCE:-$repo/../ic-db}"
source_checkout="$(cd "$source_checkout" && pwd -P)"
if [[ -z "${DB_TESTKIT_CLI:-}" ]]; then
  DB_TESTKIT_CLI="$(bash "$repo/vendor/toko-tooling/scripts/dev/install-rust-tools.sh" \
    --consumer "$repo/../ic-scheduling" --package ic-testkit --version 0.106.0 \
    --bin ic-testkit-server --profile release --features cli --check)"
fi
server_directory="${DB_SERVER_DIRECTORY:-$source_checkout/.tools/m0-pocketic}"
[[ "$("$DB_TESTKIT_CLI" --version)" == 'ic-testkit-server 0.106.0' ]]
server="$("$DB_TESTKIT_CLI" check --directory "$server_directory")"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/db-consumer.XXXXXXXX")"
trap 'printf "DB qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
sha256sum "$DB_TESTKIT_CLI" "$server" > "$scratch/tools.sha256"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/db/external-session.patch"
git -C "$scratch/source" apply "$repo/qualification/db/external-session.patch"
if [[ "$mode" == text ]]; then
  git -C "$scratch/source" apply --check "$repo/qualification/db/text.patch"
  git -C "$scratch/source" apply "$repo/qualification/db/text.patch"
fi
export CARGO_TARGET_DIR="$repo/target/db-consumer-build"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
cd "$scratch/source"
cargo metadata --manifest-path qualification/Cargo.toml --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e '
  all(.packages[]; .name != "ic-testkit") and
  ([.packages[] | select(.name == "pocket-ic" and .version == "16.1.0")] | length) == 1
' "$scratch/metadata.json" >/dev/null
package=ic-db-m0-rollback-canister
artifact=ic_db_m0_rollback_canister
if [[ "$mode" == text ]]; then
  package=ic-db-m1-records-canister
  artifact=ic_db_m1_records_canister
  jq -e --arg root "$repo" '[.packages[] | select(.name == "ic-host-text")] as $t |
    ($t | length == 1) and $t[0].version == "0.77.0" and
    $t[0].manifest_path == ($root + "/crates/ic-host-text/Cargo.toml")' "$scratch/metadata.json" >/dev/null
fi
CARGO_TARGET_DIR="$scratch/wasm-target" cargo build --manifest-path qualification/Cargo.toml -p "$package" --target wasm32-unknown-unknown --release --locked --offline --message-format=json-render-diagnostics > "$scratch/wasm.jsonl"
IC_DB_M0_WASM="$(jq -ser --arg artifact "$artifact" '
  [.[] | select(.reason == "compiler-artifact" and .target.name == $artifact) |
    .filenames[] | select(endswith(".wasm"))] |
  if length == 1 then .[0] else error("expected one selected DB Wasm") end
' "$scratch/wasm.jsonl")"
export IC_DB_M0_WASM
export IC_DB_M1_RECORDS_WASM="$IC_DB_M0_WASM"
sha256sum "$IC_DB_M0_WASM" > "$scratch/wasm.sha256"
env -u IC_TESTKIT_POCKET_IC_URL POCKET_IC_BIN=/unavailable cargo test --manifest-path qualification/Cargo.toml -p ic-db-m0-rollback-driver --lib tests::requires_external_endpoint --locked --offline -- --exact
export DB_QUALIFICATION_EXPORT="$scratch"
env -u IC_TESTKIT_POCKET_IC_URL -u POCKET_IC_BIN "$DB_TESTKIT_CLI" run \
  --directory "$server_directory" --ttl 300 \
  --server-stdout "$scratch/server.stdout" --server-stderr "$scratch/server.stderr" \
  -- bash "$repo/scripts/qualification/db-session.sh" "$mode"
sha256sum --check "$scratch/tools.sha256" "$scratch/wasm.sha256"
