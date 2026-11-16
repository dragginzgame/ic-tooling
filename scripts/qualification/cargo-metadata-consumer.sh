#!/usr/bin/env bash
# Current read-only consumer exports; Toko owns every transaction and rollback.
set -euo pipefail
: "${CARGO_RELEASE_METADATA_EDITOR:?select an absolute prepared ic-cargo-metadata executable}"
[[ "$CARGO_RELEASE_METADATA_EDITOR" == /* && -x "$CARGO_RELEASE_METADATA_EDITOR" ]]
[[ "$("$CARGO_RELEASE_METADATA_EDITOR" --version)" == 'ic-cargo-metadata 0.1.0' ]]
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/cargo-metadata-consumer.XXXXXXXX")"
trap 'printf "Metadata consumer evidence: %s\n" "$scratch"' EXIT
source "$repo/scripts/qualification/local-source.sh"
run_root="$scratch"
for owner in toko-tooling ic-scheduling; do
  scratch="$run_root/$owner"
  mkdir -p "$scratch/source"
  source_checkout="$repo/../$owner"
  export_local_source
  if [[ "$owner" == ic-scheduling ]]; then
    git -C "$source_checkout" merge-base --is-ancestor \
      f382bacac5461ea965e5813b56f15bd4757bf596 "$(cat "$scratch/source-head")" || {
      echo 'Scheduling source must descend from the qualified rewrite checkpoint' >&2
      exit 1
    }
  fi
done
scratch="$run_root"
shasum -a 256 "$CARGO_RELEASE_METADATA_EDITOR" > "$scratch/editor.sha256"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
export TMPDIR="$scratch/runtime"
mkdir "$TMPDIR"
shared="$scratch/toko-tooling/source/scripts/ci"
bash "$shared/test-cargo-release-metadata.sh" > "$scratch/coordinated.log" 2>&1
bash "$shared/test-cargo-qualification-release.sh" --case release > "$scratch/multi-lock.log" 2>&1
export GIT_CEILING_DIRECTORIES="$repo/target"
consumer="$scratch/ic-scheduling/source"
git -C "$consumer" apply --check "$repo/qualification/cargo-metadata/scheduling-editor.patch"
git -C "$consumer" apply "$repo/qualification/cargo-metadata/scheduling-editor.patch"
(cd "$consumer" && node qualification/release-check.mjs) > "$scratch/scheduling.log" 2>&1
shasum -a 256 -c "$scratch/editor.sha256"
printf 'Current Toko coordinated/multi-lock transactions and Scheduling release checks pass.\n'
