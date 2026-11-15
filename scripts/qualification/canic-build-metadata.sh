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
  cat "$QUALIFICATION_METADATA"
else
  touch "$QUALIFICATION_BUILD_MARKER"
  exec "$QUALIFICATION_REAL_CARGO" "$@"
fi
CARGO
chmod +x "$scratch/cargo-fixture"
export IC_OWNED_CARGO="$scratch/cargo-fixture"
args=(artifact build --manifest "$scratch/probe/Cargo.toml" --package owned-wasm-probe --output "$scratch/metadata.wasm")
cp "$scratch/probe-metadata.json" "$QUALIFICATION_METADATA"
"$cli" "${args[@]}" > "$scratch/metadata-valid.json"
[[ -f "$QUALIFICATION_BUILD_MARKER" ]]
rg -a -q 'owned-wasm-probe!' "$scratch/metadata.wasm"
hash="$(sha256sum "$scratch/metadata.wasm" | cut -d ' ' -f1)"
jq -e --arg hash "$hash" '.artifact.sha256 == $hash' "$scratch/metadata-valid.json" >/dev/null
rm "$QUALIFICATION_BUILD_MARKER"
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
