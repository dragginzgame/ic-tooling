#!/usr/bin/env bash
# Real current Canic CLI build/publication proof; no deployments or sibling edits.
set -euo pipefail
mode="${1:-artifact}"
foreground=false
case "$mode" in
  artifact|app|metadata) ;;
  process)
    [[ "$(uname -s)" == Linux ]] || { echo 'Combined foreground PTY qualification requires Linux; macOS remains separate.' >&2; exit 1; }
    mode=app
    foreground=true
    ;;
  *) exit 2 ;;
esac
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
if [[ "$mode" == app ]]; then
  cp "$scratch/source/Cargo.lock" "$scratch/admitted-build.lock"
  git -C "$scratch/source" apply --check "$repo/qualification/canic/text.patch"
  git -C "$scratch/source" apply "$repo/qualification/canic/text.patch"
  sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/lock-process.patch" > "$scratch/lock-process.patch"
  git -C "$scratch/source" apply --check "$scratch/lock-process.patch"
  git -C "$scratch/source" apply "$scratch/lock-process.patch"
  # The lock migration already selects the identical local Process dependency.
  # Apply only signer implementation hunks; graph admission below proves selection.
  git -C "$scratch/source" apply --check --exclude=Cargo.toml --exclude=crates/canic-host/Cargo.toml "$repo/qualification/canic/signer-process.patch"
  git -C "$scratch/source" apply --exclude=Cargo.toml --exclude=crates/canic-host/Cargo.toml "$repo/qualification/canic/signer-process.patch"
  source "$repo/scripts/qualification/canic-signer-fixture.sh"
  prepare_signer_fixture "$scratch/source" "$scratch/signer-evidence"
  if [[ "$foreground" == true ]]; then
    # The existing local alias is shared; retain the client's dependency hunk.
    git -C "$scratch/source" apply --check --exclude=Cargo.toml "$repo/qualification/canic/foreground-process.patch"
    git -C "$scratch/source" apply --exclude=Cargo.toml "$repo/qualification/canic/foreground-process.patch"
    source "$repo/scripts/qualification/canic-foreground-fixture.sh"
    prepare_foreground_fixture "$scratch/source" "$scratch/foreground-evidence"
  fi
  cp "$repo/qualification/canic/lock-process-tests.rs" "$scratch/source/crates/canic-host/src/lock_process_tests.rs"
  printf '\n#[cfg(test)]\n#[path = "lock_process_tests.rs"]\nmod lock_process_tests;\n' >> "$scratch/source/crates/canic-host/src/app_build.rs"
  if [[ "$foreground" == true ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/metadata-process.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/metadata-process.patch"
  fi
  cargo metadata --manifest-path "$scratch/source/Cargo.toml" --format-version 1 --offline > "$scratch/composed-metadata.json"
  cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/admitted-build.lock" "$scratch/source/Cargo.lock" 1048576
fi
cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$scratch/probe/"
cp -R "$repo/qualification/owned-wasm-probe" "$scratch/probe/"
IC_OWNED_CARGO="$(rustup which --toolchain 1.100.0 cargo)"
IC_OWNED_RUSTC="$(rustup which --toolchain 1.100.0 rustc)"
export IC_OWNED_CARGO IC_OWNED_RUSTC
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 TMPDIR="$scratch/runtime"
cargo metadata --manifest-path "$scratch/source/Cargo.toml" --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "canic-host") | .id) as $host |
  (.packages[] | select(.name == "ic-build" and .version == "0.87.3" and
    .manifest_path == ($root + "/crates/ic-build/Cargo.toml")) | .id) as $build |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.80.1" and
    .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_build" and .pkg == $build) and
  any($m.resolve.nodes[] | select(.id == $build) | .deps[]; .name == "ic_host_process" and .pkg == $process)
' "$scratch/metadata.json" >/dev/null
if [[ "$mode" == app ]]; then
  if [[ "$foreground" == true ]]; then
    jq -e '. as $m |
      (.packages[] | select(.name == "canic-host") | .id) as $host |
      all($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name != "ic_host_process")
    ' "$scratch/metadata.json" >/dev/null
    jq -e --arg root "$repo" '. as $m |
      (.packages[] | select(.name == "canic-profile-client") | .id) as $client |
      (.packages[] | select(.name == "ic-host-process" and .version == "0.80.1" and .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
      any($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name == "ic_host_process_next" and .pkg == $process) and
      all($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name != "ic_host_process")
    ' "$scratch/metadata.json" >/dev/null
  fi
  jq -e '. as $m |
    (.packages[] | select(.name == "canic-host") | .id) as $host |
    (.packages[] | select(.name == "ic-host-process" and .version == "0.80.1") | .id) as $process |
    any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_host_process_next" and .pkg == $process)
  ' "$scratch/metadata.json" >/dev/null
  jq -e --arg root "$repo" '. as $m |
    (.packages[] | select(.name == "canic-cli") | .id) as $cli |
    [.packages[] | select(.name == "ic-host-text")] as $text |
    ($text | length == 1) and $text[0].version == "0.77.2" and
    $text[0].manifest_path == ($root + "/crates/ic-host-text/Cargo.toml") and
    any($m.resolve.nodes[] | select(.id == $cli) | .deps[];
      .name == "ic_host_text" and .pkg == $text[0].id)
  ' "$scratch/metadata.json" >/dev/null
fi
CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo build --manifest-path "$scratch/source/Cargo.toml" -p canic-cli --bin canic --locked --offline
cli="$repo/target/canic-consumer-build/debug/canic"
# Deliberately occupied shared output: the build must capture its private output.
export CARGO_TARGET_DIR="$scratch/published"
if [[ "$mode" == metadata ]]; then
  bash "$repo/scripts/qualification/canic-build-metadata.sh" "$cli" "$scratch"
  exit 0
fi
if [[ "$mode" == app ]]; then
  CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo clippy --manifest-path "$scratch/source/Cargo.toml" -p canic-host -p canic-cli --all-targets --locked --offline -- -D warnings
  CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline lock_process_tests::
  CARGO_TARGET_DIR="$repo/target/canic-consumer-build" test_signer_fixture "$scratch/source" "$scratch/signer-evidence"
  if [[ "$foreground" == true ]]; then
    CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline artifact::tests::
    CARGO_TARGET_DIR="$repo/target/canic-consumer-build" test_foreground_fixture "$scratch/source" "$scratch/foreground-evidence"
  fi
  bash "$repo/scripts/qualification/canic-app-build.sh" "$cli" "$scratch"
  if [[ "$foreground" == true ]]; then
    # Qualify metadata execution without changing the app fixture's prepared tools.
    (bash "$repo/scripts/qualification/canic-build-metadata.sh" "$cli" "$scratch")
  fi
  # Newly migrated display callers must work in the same Build + Text graph.
  (
    cd "$scratch/source"
    export CARGO_TARGET_DIR="$repo/target/canic-consumer-build"
    cargo test -p canic-cli --bin canic --locked --offline -- --list > "$scratch/display-tests.txt"
    for scope in fleet::terminal::tests:: placements::tests:: status::tests:: subnet_comparison::tests:: artifacts::tests::; do
      rg -q "^$scope.*: test$" "$scratch/display-tests.txt"
      cargo test -p canic-cli --bin canic --locked --offline "$scope"
    done
  )
  exit 0
fi
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
# Replace the selected entry without following a final symlink or mutating an alias.
printf 'foreign bytes' > "$scratch/foreign.wasm"
ln -s foreign.wasm "$scratch/symlink.wasm"
ln "$scratch/foreign.wasm" "$scratch/hardlink.wasm"
ln -s missing.wasm "$scratch/dangling.wasm"
for name in symlink hardlink dangling; do
  # A failed build must not unlink an alias or create its missing referent.
  if "$cli" "${args[@]}" --features refuse --output "$scratch/$name.wasm" > "$scratch/$name-refuse.stdout" 2> "$scratch/$name-refuse.stderr"; then
    echo "Expected compilation refusal for $name" >&2; exit 1
  fi
  jq -e '.error == "supplier" and (.message | contains("Cargo exited unsuccessfully"))' "$scratch/$name-refuse.stderr" >/dev/null
  case "$name" in
    symlink) [[ -L "$scratch/$name.wasm" && "$(readlink "$scratch/$name.wasm")" == foreign.wasm ]] ;;
    hardlink) [[ ! -L "$scratch/$name.wasm" && "$scratch/$name.wasm" -ef "$scratch/foreign.wasm" ]] ;;
    dangling) [[ -L "$scratch/$name.wasm" && "$(readlink "$scratch/$name.wasm")" == missing.wasm ]] ;;
  esac
  [[ "$(cat "$scratch/foreign.wasm")" == 'foreign bytes' ]]
  [[ ! -e "$scratch/missing.wasm" && ! -L "$scratch/missing.wasm" ]]
  [[ -z "$(ls -A "$scratch/runtime")" ]]
  "$cli" "${args[@]}" --output "$scratch/$name.wasm" > "$scratch/$name.json"
  [[ -f "$scratch/$name.wasm" && ! -L "$scratch/$name.wasm" ]]
  [[ "$(cat "$scratch/foreign.wasm")" == 'foreign bytes' ]]
  [[ ! "$scratch/$name.wasm" -ef "$scratch/foreign.wasm" ]]
  [[ ! -e "$scratch/missing.wasm" && ! -L "$scratch/missing.wasm" ]]
  cmp "$default" "$scratch/$name.wasm"
  jq -e --arg hash "$hash" --arg path "$scratch/$name.wasm" \
    '.artifact.sha256 == $hash and .wasm == $path' "$scratch/$name.json" >/dev/null
done
# Failed compilation must leave an existing destination unchanged.
if "$cli" "${args[@]}" --features refuse --output "$scratch/explicit.wasm" > "$scratch/refuse.stdout" 2> "$scratch/refuse.stderr"; then
  echo 'Expected compilation refusal' >&2; exit 1
fi
cmp "$default" "$scratch/explicit.wasm"
# Publication refusal retains an occupied directory and its unrelated bytes.
jq -e '.error == "supplier" and (.message | contains("intentional owned-build diagnostic oracle"))' "$scratch/refuse.stderr" >/dev/null
mkdir "$scratch/occupied"
printf 'retain me' > "$scratch/occupied/sentinel"
if "$cli" "${args[@]}" --output "$scratch/occupied" > "$scratch/publication.stdout" 2> "$scratch/publication.stderr"; then
  echo 'Expected publication refusal' >&2; exit 1
fi
[[ "$(cat "$scratch/occupied/sentinel")" == 'retain me' ]]
# Private target cleanup also applies after the failed Cargo invocation.
[[ -z "$(ls -A "$scratch/runtime")" ]]
printf 'Default/explicit publication and symlink/hardlink/dangling failure preservation, exact identity, publication refusal and scratch cleanup passed.\n'
