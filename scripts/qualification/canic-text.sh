#!/usr/bin/env bash
# Current Canic CLI display contracts only; no deployments or legacy source.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-text.XXXXXXXX")"
trap 'printf "Canic Text export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build"
CDPATH='' cd -- "$scratch/source"
find . -type f -exec touch {} +
[[ -f Cargo.lock && ! -L Cargo.lock ]]
cp Cargo.lock "$scratch/consumer.lock"
path=$(jq -Rn --arg path "$repo/crates/ic-host-text" '$path')
patches=(--config "patch.crates-io.ic-host-text.path=$path")
cargo metadata "${patches[@]}" --format-version 1 > "$scratch/metadata.json"
jq -e --arg manifest "$repo/crates/ic-host-text/Cargo.toml" '
  . as $m | any(.packages[]; .name == "canic-cli" and
    (.id as $cli | ($m.workspace_members | index($cli)) != null and
      any($m.resolve.nodes[] | select(.id == $cli) | .deps[];
        any(.dep_kinds[]; .kind == null and .target == null) and
        (.pkg as $id | any($m.packages[]; .id == $id and .name == "ic-host-text" and
          .source == null and .manifest_path == $manifest)))))
' "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --quiet --locked -- \
  verify-external-lock "$scratch/consumer.lock" Cargo.lock 1048576
source "$repo/scripts/qualification/canic-cases.sh"
run_cases canic-cli --bin=canic status::tests:: fleet::terminal::tests:: placements::tests:: \
  subnet_comparison::tests:: artifacts::tests:: \
  tests::config::canister_declarations_retain_roles_and_parents_without_effect_authority \
  tests::config::existing_canister_declaration_lists_exact_id_without_deployment_authority \
  tests::config::declaration_messages_escape_paths_while_json_retains_original_paths
