#!/usr/bin/env bash
# Export only committed Backup source. Never read or modify its working tree.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${BACKUP_SOURCE:-$repo/../ic-backup}"
revision=d8955cf1e34f24934027f448ef16ee6841eafbb0
export PATH="$repo/.tools/host/bin:$PATH"
git -C "$source_checkout" cat-file -e "$revision^{commit}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/backup-consumer.XXXXXXXX")"
trap 'printf "Backup qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/runtime"
git -C "$source_checkout" archive "$revision" | tar -x -C "$scratch/source"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/backup/migration.patch"
git -C "$scratch/source" apply "$repo/qualification/backup/migration.patch"
export TMPDIR="$scratch/runtime"
export CARGO_TARGET_DIR="$repo/target/backup-consumer-build"
export CARGO_NET_OFFLINE=true
export RUSTUP_AUTO_INSTALL=0
cd "$scratch/source"
cargo metadata --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '
  [.packages[] | select(.name == "ic-host-fs")] as $fs |
  ($fs | length) == 1 and $fs[0].version == "0.80.0" and
  $fs[0].manifest_path == ($root + "/crates/ic-host-fs/Cargo.toml") and
  any(.packages[]; .name == "ic-host-artifacts" and .version == "0.80.0" and
    .manifest_path == ($root + "/crates/ic-host-artifacts/Cargo.toml"))
' "$scratch/metadata.json" >/dev/null
cargo test -p ic-backup --lib store::tests --locked --offline
# Each of these exercises the migrated held-file update or closing admission path.
for contract in \
  store::capture::ledger::tests::closing_admission_refuses_changed_file_and_unsafe_name \
  store::capture::advance::tests::successful_status_retains_exact_evidence_and_cold_replay_has_no_effects \
  store::capture::advance::tests::partial_frames_and_uncertain_full_writes_invalidate_live_handle_and_preserve_bytes \
  store::capture::advance::tests::held_root_rename_keeps_append_in_original_store \
  store::capture::advance::tests::closing_custody_failure_never_returns_a_live_reservation; do
  cargo test -p ic-backup --lib "$contract" --locked --offline -- --exact
done
