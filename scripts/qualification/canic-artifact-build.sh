#!/usr/bin/env bash
set -euo pipefail
cli=$1
scratch=$2
args=(--json artifact build --manifest "$scratch/probe/Cargo.toml" --package owned-wasm-probe)
"$cli" "${args[@]}" > "$scratch/default.json"
hash=$(jq -er '.artifact.sha256 | select(test("^[0-9a-f]{64}$"))' "$scratch/default.json")
default="$CARGO_TARGET_DIR/canic-artifacts/$hash/owned_wasm_probe.wasm"
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
  jq -e '.error == "supplier" and (.message | contains("intentional owned-build diagnostic oracle"))' "$scratch/$name-refuse.stderr" >/dev/null
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
[[ -z $(find "$CARGO_TARGET_DIR/canic-artifacts" -name '.cargo-*' -print -quit) ]]
printf 'Default/explicit publication and symlink/hardlink/dangling failure preservation, exact identity, publication refusal and scratch cleanup passed.\n'
