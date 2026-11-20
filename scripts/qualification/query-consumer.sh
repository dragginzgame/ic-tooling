#!/usr/bin/env bash
# Current Query presentation only; no network reports or sibling edits.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${QUERY_SOURCE:-$repo/../ic-query}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/query-consumer.XXXXXXXX")"
trap 'printf "Query qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/query/text.patch"
git -C "$scratch/source" apply "$repo/qualification/query/text.patch"
export CARGO_TARGET_DIR="$repo/target/query-consumer-build"
CDPATH='' cd -- "$scratch/source"
cargo metadata --format-version 1 --locked > "$scratch/metadata.json"
jq -e --arg root "$repo" '
  [.packages[] | select(.name == "ic-host-text")] as $text |
  ($text | length == 1) and $text[0].version == "0.77.2" and
  $text[0].manifest_path == ($root + "/crates/ic-host-text/Cargo.toml")
' "$scratch/metadata.json" >/dev/null
cargo test -p ic-query-cli --bin icq --locked -- --list > "$scratch/tests.txt"
for scope in presentation topology; do
  rg -q "^$scope::tests::.*: test$" "$scratch/tests.txt"
  cargo test -p ic-query-cli --bin icq --locked "$scope::tests::"
done
