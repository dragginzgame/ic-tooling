#!/usr/bin/env bash
# Focused package admission, not Auth's complete release/MSRV feature matrix.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${AUTH_SOURCE:-$repo/../ic-auth}"
revision=e52a40be75dd9809d0528189d84b43fe96952ad9
export PATH="$repo/.tools/host/bin:$PATH" RUSTUP_AUTO_INSTALL=0
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/auth-consumer.XXXXXXXX")"
trap 'printf "Auth qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/runtime" "$scratch/consumer"
git -C "$source_checkout" archive "$revision" | tar -x -C "$scratch/source"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/auth/package-check.patch"
git -C "$scratch/source" apply "$repo/qualification/auth/package-check.patch"
git -C "$scratch/source" init -q
cargo build --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked \
  --message-format=json > "$scratch/tool.jsonl"
IC_CARGO_QUALIFICATION="$(jq -ser '
  [.[] | select(.reason == "compiler-artifact" and .target.name == "ic-cargo-qualification") |
   .executable | select(. != null)] |
  if length == 1 then .[0] else error("expected one qualification executable") end
' "$scratch/tool.jsonl")"
export IC_CARGO_QUALIFICATION TMPDIR="$scratch/runtime"
CDPATH='' cd -- "$scratch/source"
bash scripts/ci/test-package-consumer.sh
CARGO_TARGET_DIR="$scratch/packages" cargo package -p ic-auth -p ic-auth-protocol-types -p ic-auth-wire \
  --locked --allow-dirty --no-verify
for name in ic-auth-0.41.1 ic-auth-protocol-types-0.40.1 ic-auth-wire-0.40.1; do
  tar -xzf "$scratch/packages/package/$name.crate" -C "$scratch/consumer"
done
consumer="$scratch/consumer"
features="$(yq -p toml -o json '.' "$consumer/ic-auth-0.41.1/Cargo.toml" | jq -c '[.features | keys[] | "ic-auth/" + .] + ["ic-auth-wire/typescript"]')"
cat > "$consumer/Cargo.toml" <<TOML
[workspace]
exclude = ["ic-auth-0.41.1", "ic-auth-protocol-types-0.40.1", "ic-auth-wire-0.40.1"]
[package]
name = "ic-auth-msrv-consumer"
version = "0.0.0"
edition = "2024"
rust-version = "1.100.0"
publish = false
[dependencies]
ic-auth = { version = "=0.41.1", default-features = false }
ic-auth-protocol-types = "=0.40.1"
ic-auth-wire = "=0.40.1"
ic-platform-types = { version = "0.20.2", default-features = false }
[features]
all = $features
[patch.crates-io]
ic-auth = { path = "ic-auth-0.41.1" }
ic-auth-protocol-types = { path = "ic-auth-protocol-types-0.40.1" }
ic-auth-wire = { path = "ic-auth-wire-0.40.1" }
TOML
mkdir "$consumer/src"
cp "$repo/qualification/auth/consumer.rs" "$consumer/src/lib.rs"
cp Cargo.lock "$consumer/Cargo.lock"
export CARGO_TARGET_DIR="$repo/target/auth-consumer-build"
hash_inputs() {
  perl -0e 'print sort <STDIN>' | while IFS= read -r -d '' file; do
    digest="$(bash "$repo/vendor/toko-tooling/scripts/ci/verify-file-checksum.sh" --print sha256 "$file")"
    printf '%s\0%s\0' "$file" "$digest"
  done
}
archive_inputs() {
  for archive in "$scratch/packages/package/"*.crate; do printf '%s\0' "$archive"; done
  find "$consumer/ic-auth-0.41.1" "$consumer/ic-auth-protocol-types-0.40.1" "$consumer/ic-auth-wire-0.40.1" -type f -print0
}
archive_inputs | hash_inputs > "$scratch/archives-before"
cargo metadata --manifest-path "$consumer/Cargo.toml" --all-features --format-version 1 > "$consumer/metadata.json"
bash scripts/ci/check-package-consumer.sh "$PWD/Cargo.lock" "$consumer" 0.41.1 0.40.1
archive_inputs | hash_inputs > "$scratch/archives-resolved"
cmp "$scratch/archives-before" "$scratch/archives-resolved"
# Retain byte identities of exact archives and all unpacked inputs around checks.
inputs() {
  for archive in "$scratch/packages/package/"*.crate; do printf '%s\0' "$archive"; done
  find "$consumer" -type f -print0
}
snapshot() {
  inputs | hash_inputs
}
snapshot > "$scratch/before.sha256"
cargo check --manifest-path "$consumer/Cargo.toml" --all-features --locked
cargo check --manifest-path "$consumer/Cargo.toml" --all-features --target wasm32-unknown-unknown --locked
snapshot > "$scratch/after.sha256"
cmp "$scratch/before.sha256" "$scratch/after.sha256"
printf 'Exact Auth archives passed native/Wasm all-feature admission without input changes\n'
