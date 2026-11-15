#!/usr/bin/env bash
# Compare completed Backup commitment producers without migrating unrelated custody.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${BACKUP_SOURCE:-$repo/../ic-backup}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/backup-commitments.XXXXXXXX")"
trap 'printf "Backup commitment export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
manifest="$scratch/source/Cargo.toml"
export CARGO_TARGET_DIR="$repo/target/backup-consumer-build" CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
cp "$scratch/source/Cargo.lock" "$scratch/original.lock"
run_checks() {
  for selection in store::restore::tests store::capture::advance::tests::assembly_tests store::capture::advance::tests::completion_tests; do
    cargo test --manifest-path "$manifest" -p ic-backup --lib --locked --offline "$selection" || return
  done
}
run_checks > "$scratch/baseline.log" 2>&1 || {
  tail -40 "$scratch/baseline.log"
  echo 'Unchanged Backup baseline failed; commitment migration was not applied.' >&2
  exit 1
}
sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/backup/commitments.patch" > "$scratch/migration.patch"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$scratch/migration.patch"
git -C "$scratch/source" apply "$scratch/migration.patch"
cargo metadata --manifest-path "$manifest" --format-version 1 --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "ic-backup") | .id) as $consumer |
  (.packages[] | select(.name == "ic-host-artifacts" and .version == "0.80.1" and .source == null and .manifest_path == ($root + "/crates/ic-host-artifacts/Cargo.toml")) | .id) as $bytes |
  any($m.resolve.nodes[] | select(.id == $consumer) | .deps[]; .name == "ic_byte_admission" and .pkg == $bytes)
' "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/original.lock" "$scratch/source/Cargo.lock" 1048576
cargo clippy --manifest-path "$manifest" -p ic-backup --all-targets --locked --offline -- -D warnings
run_checks > "$scratch/replacement.log" 2>&1
for branch in baseline replacement; do
  if rg -q '^test result: ok\. 0 passed' "$scratch/$branch.log"; then
    echo 'A required Backup commitment test selection was empty' >&2; exit 1
  fi
  rg '^test .* \.\.\. ok$' "$scratch/$branch.log" | LC_ALL=C sort > "$scratch/$branch-tests.txt"
  [[ -s "$scratch/$branch-tests.txt" ]]
done
cmp "$scratch/baseline-tests.txt" "$scratch/replacement-tests.txt"
cat "$scratch/replacement-tests.txt"
