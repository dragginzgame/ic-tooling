#!/usr/bin/env bash
# Real current Canic CLI build/publication proof; no deployments or sibling edits.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-build.XXXXXXXX")"
trap 'printf "Canic Build export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/probe" "$scratch/runtime"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/canic/owned-build.patch"
git -C "$scratch/source" apply "$repo/qualification/canic/owned-build.patch"
cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$scratch/probe/"
cp -R "$repo/qualification/wasm-probe" "$scratch/probe/"
IC_OWNED_CARGO="$(rustup which --toolchain 1.100.0 cargo)"
IC_OWNED_RUSTC="$(rustup which --toolchain 1.100.0 rustc)"
export IC_OWNED_CARGO IC_OWNED_RUSTC
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 TMPDIR="$scratch/runtime"
cargo metadata --manifest-path "$scratch/source/Cargo.toml" --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "canic-host") | .id) as $host |
  (.packages[] | select(.name == "ic-build" and .version == "0.87.1" and
    .manifest_path == ($root + "/crates/ic-build/Cargo.toml")) | .id) as $build |
  any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_build" and .pkg == $build)
' "$scratch/metadata.json" >/dev/null
CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo build --manifest-path "$scratch/source/Cargo.toml" -p canic-cli --bin canic --locked --offline
cli="$repo/target/canic-consumer-build/debug/canic"
# Deliberately occupied shared output: the build must capture its private output.
export CARGO_TARGET_DIR="$scratch/published"
default="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/owned_wasm_probe.wasm"
mkdir -p "$(dirname "$default")"
printf 'stale output' > "$default"
args=(artifact build --manifest "$scratch/probe/Cargo.toml" --package owned-wasm-probe)
"$cli" "${args[@]}" > "$scratch/default.json"
jq -e --arg path "$default" '.wasm == $path and .artifact.bytes > 8' "$scratch/default.json" >/dev/null
hash="$(sha256sum "$default" | cut -d ' ' -f1)"
jq -e --arg hash "$hash" '.artifact.sha256 == $hash' "$scratch/default.json" >/dev/null
rg -a -q 'owned-wasm-probe!' "$default"
# Explicit destination and returned identity refer to the independently published bytes.
"$cli" "${args[@]}" --output "$scratch/explicit.wasm" > "$scratch/explicit.json"
cmp "$default" "$scratch/explicit.wasm"
jq -e --arg hash "$hash" --arg path "$scratch/explicit.wasm" '.artifact.sha256 == $hash and .wasm == $path' "$scratch/explicit.json" >/dev/null
# Failed compilation must leave an existing destination unchanged.
if "$cli" "${args[@]}" --features refuse --output "$scratch/explicit.wasm" > "$scratch/refuse.stdout" 2> "$scratch/refuse.stderr"; then
  echo 'Expected compilation refusal' >&2; exit 1
fi
cmp "$default" "$scratch/explicit.wasm"
# Publication refusal retains an occupied directory and its unrelated bytes.
mkdir "$scratch/occupied"
printf 'retain me' > "$scratch/occupied/sentinel"
if "$cli" "${args[@]}" --output "$scratch/occupied" > "$scratch/publication.stdout" 2> "$scratch/publication.stderr"; then
  echo 'Expected publication refusal' >&2; exit 1
fi
[[ "$(cat "$scratch/occupied/sentinel")" == 'retain me' ]]
# Private target cleanup also applies after the failed Cargo invocation.
[[ -z "$(ls -A "$scratch/runtime")" ]]
printf 'Default and explicit publication, exact identity, compile/publication refusal and scratch cleanup passed.\n'
