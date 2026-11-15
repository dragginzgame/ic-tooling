#!/usr/bin/env bash
# Current Canic CLI display contracts only; no deployments or legacy source.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-text.XXXXXXXX")"
trap 'printf "Canic Text export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/canic/text.patch"
git -C "$scratch/source" apply "$repo/qualification/canic/text.patch"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build"
cd "$scratch/source"
cargo metadata --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '
  . as $m | (.packages[] | select(.name == "canic-cli") | .id) as $cli |
  (.packages[] | select(.name == "ic-host-text" and .version == "0.77.2" and
    .manifest_path == ($root + "/crates/ic-host-text/Cargo.toml")) | .id) as $text |
  any($m.resolve.nodes[] | select(.id == $cli) | .deps[]; .name == "ic_host_text" and .pkg == $text)
' "$scratch/metadata.json" >/dev/null
cargo test -p canic-cli --bin canic --locked --offline -- --list > "$scratch/tests.txt"
for scope in status::tests:: fleet::terminal::tests:: placements::tests:: subnet_comparison::tests:: artifacts::tests::; do
  rg -q "^$scope.*: test$" "$scratch/tests.txt"
  cargo test -p canic-cli --bin canic --locked --offline "$scope"
done
for subject in \
  config_tests::canister_declarations_retain_roles_and_parents_without_effect_authority \
  config_tests::existing_canister_declaration_lists_exact_id_without_deployment_authority \
  config_tests::declaration_messages_escape_paths_while_json_retains_original_paths; do
  rg -Fxq "$subject: test" "$scratch/tests.txt"
  cargo test -p canic-cli --bin canic --locked --offline "$subject" -- --exact
done
