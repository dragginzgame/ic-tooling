#!/usr/bin/env bash
# Controlled metadata at the Canic adapter boundary; valid input runs real Cargo.
set -euo pipefail
cli="$1"
scratch="$2"
export QUALIFICATION_REAL_CARGO="$IC_OWNED_CARGO"
export QUALIFICATION_METADATA="$scratch/selected-metadata.json"
export QUALIFICATION_BUILD_MARKER="$scratch/build-invoked"
"$IC_OWNED_CARGO" metadata --no-deps --format-version 1 --locked --offline \
  --manifest-path "$scratch/probe/Cargo.toml" > "$scratch/probe-metadata.json"
cat > "$scratch/cargo-fixture" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == metadata ]]; then
  case "${QUALIFICATION_METADATA_MODE:-valid}" in
    nonzero)
      cat "$QUALIFICATION_METADATA"
      printf 'metadata process oracle\033[31m\n' >&2
      exit 7
      ;;
    stdout-over) head -c 8388609 /dev/zero; exit 0 ;;
    stderr-over) head -c 1048577 /dev/zero >&2; exit 0 ;;
    invalid-utf8) printf '\377'; exit 0 ;;
  esac
  cat "$QUALIFICATION_METADATA"
else
  touch "$QUALIFICATION_BUILD_MARKER"
  exec "$QUALIFICATION_REAL_CARGO" "$@"
fi
CARGO
chmod +x "$scratch/cargo-fixture"
export IC_OWNED_CARGO="$scratch/cargo-fixture"
args=(--json artifact build --manifest "$scratch/probe/Cargo.toml" --package owned-wasm-probe --output "$scratch/metadata.wasm")
cp "$scratch/probe-metadata.json" "$QUALIFICATION_METADATA"
"$cli" "${args[@]}" > "$scratch/metadata-valid.json"
[[ -f "$QUALIFICATION_BUILD_MARKER" ]]
rg -a -q 'owned-wasm-probe!' "$scratch/metadata.wasm"
hash="$(sha256sum "$scratch/metadata.wasm" | cut -d ' ' -f1)"
jq -e --arg hash "$hash" '.artifact.sha256 == $hash' "$scratch/metadata-valid.json" >/dev/null
rm "$QUALIFICATION_BUILD_MARKER"
# Supplier refusal must precede package selection and every build effect, even
# when a failing command emitted otherwise valid metadata.
for mode in nonzero stdout-over stderr-over invalid-utf8; do
  printf 'retained destination' > "$scratch/metadata.wasm"
  if QUALIFICATION_METADATA_MODE="$mode" "$cli" "${args[@]}" > "$scratch/metadata-process-$mode.stdout" 2> "$scratch/metadata-process-$mode.stderr"; then
    echo "Expected metadata process refusal: $mode" >&2; exit 1
  fi
  jq -e '.error == "supplier"' "$scratch/metadata-process-$mode.stderr" >/dev/null
  if [[ "$mode" == nonzero ]]; then
    jq -e '(.message | contains("metadata process oracle")) and (.message | contains("\u001b") | not)' "$scratch/metadata-process-$mode.stderr" >/dev/null
  fi
  [[ ! -e "$QUALIFICATION_BUILD_MARKER" ]]
  [[ "$(cat "$scratch/metadata.wasm")" == 'retained destination' ]]
  [[ -z "$(ls -A "$scratch/runtime")" ]]
done
printf 'Four metadata subprocess/encoding refusals precede compilation and preserve destination.\n'
# Each mutation must be refused before any build and preserve destination bytes.
filters=(
  'del(.workspace_members)'
  '.workspace_members = ["foreign-package"]'
  '.workspace_members += [null]'
  '.packages = []'
  '.packages += [.packages[0]]'
  '.packages[0].targets = []'
  '.packages[0].targets += [.packages[0].targets[0]]'
  'del(.target_directory)'
  '.target_directory = "relative-target"'
)
for index in "${!filters[@]}"; do
  jq "${filters[$index]}" "$scratch/probe-metadata.json" > "$QUALIFICATION_METADATA"
  printf 'retained destination' > "$scratch/metadata.wasm"
  if "$cli" "${args[@]}" > "$scratch/metadata-$index.stdout" 2> "$scratch/metadata-$index.stderr"; then
    echo "Expected metadata refusal: ${filters[$index]}" >&2; exit 1
  fi
  jq -e '.error == "invalid" and (.message | contains("expected one workspace package and cdylib target"))' "$scratch/metadata-$index.stderr" >/dev/null
  [[ ! -e "$QUALIFICATION_BUILD_MARKER" ]]
  [[ "$(cat "$scratch/metadata.wasm")" == 'retained destination' ]]
  [[ -z "$(ls -A "$scratch/runtime")" ]]
done
printf '{invalid json' > "$QUALIFICATION_METADATA"
if "$cli" "${args[@]}" > "$scratch/metadata-json.stdout" 2> "$scratch/metadata-json.stderr"; then
  echo 'Expected malformed metadata refusal' >&2; exit 1
fi
jq -e '.error == "supplier"' "$scratch/metadata-json.stderr" >/dev/null
[[ ! -e "$QUALIFICATION_BUILD_MARKER" ]]
[[ "$(cat "$scratch/metadata.wasm")" == 'retained destination' ]]
[[ -z "$(ls -A "$scratch/runtime")" ]]
printf 'Valid metadata builds; ten metadata refusals precede compilation and preserve destination/scratch.\n'
# Prepared tools and compiler wrappers remain explicit at the consumer boundary.
cp "$scratch/probe-metadata.json" "$QUALIFICATION_METADATA"
export QUALIFICATION_WRAPPER_MARKER="$scratch/wrapper-invoked"
cat > "$scratch/compiler-wrapper" <<'WRAPPER'
#!/usr/bin/env bash
set -euo pipefail
touch "$QUALIFICATION_WRAPPER_MARKER"
exit 97
WRAPPER
chmod +x "$scratch/compiler-wrapper"
for key in IC_OWNED_CARGO IC_OWNED_RUSTC; do
  for state in missing unavailable; do
    selection=(-u "$key")
    expected=invalid
    if [[ "$state" == unavailable ]]; then
      selection=("$key=$scratch/unavailable-tool")
      expected=supplier
    fi
    if env "${selection[@]}" "$cli" "${args[@]}" > "$scratch/$key-$state.stdout" 2> "$scratch/$key-$state.stderr"; then
      echo "Expected prepared tool refusal: $key $state" >&2; exit 1
    fi
    jq -e --arg expected "$expected" '.error == $expected' "$scratch/$key-$state.stderr" >/dev/null
    [[ ! -e "$QUALIFICATION_BUILD_MARKER" ]]
    [[ "$(cat "$scratch/metadata.wasm")" == 'retained destination' ]]
    [[ -z "$(ls -A "$scratch/runtime")" ]]
  done
done
for key in RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER; do
  if env "$key=$scratch/compiler-wrapper" "$cli" "${args[@]}" > "$scratch/$key.stdout" 2> "$scratch/$key.stderr"; then
    echo "Expected compiler wrapper refusal: $key" >&2; exit 1
  fi
  jq -e '.error == "supplier" and (.message | contains("operation-owned environment entry"))' "$scratch/$key.stderr" >/dev/null
  [[ ! -e "$QUALIFICATION_BUILD_MARKER" && ! -e "$QUALIFICATION_WRAPPER_MARKER" ]]
  [[ "$(cat "$scratch/metadata.wasm")" == 'retained destination' ]]
  [[ -z "$(ls -A "$scratch/runtime")" ]]
done
printf 'Four prepared-tool and two wrapper refusals preserve destination/scratch without compilation or wrapper execution.\n'
