#!/usr/bin/env bash
# Drive the current CLI through Cargo's pkgid, metadata and build commands.
set -euo pipefail
cli=$1
scratch=$2
export QUALIFICATION_REAL_CARGO
QUALIFICATION_REAL_CARGO=$(command -v cargo)
export QUALIFICATION_METADATA="$scratch/selected-metadata.json"
export QUALIFICATION_CALLS="$scratch/valid.calls"
probe="$scratch/probe/Cargo.toml"
output="$scratch/metadata.wasm"
"$QUALIFICATION_REAL_CARGO" metadata --locked --no-deps --format-version 1 --manifest-path "$probe" > "$scratch/probe-metadata.json"
mkdir "$scratch/metadata-tools"
cat > "$scratch/metadata-tools/cargo" <<'CARGO'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$1" >> "$QUALIFICATION_CALLS"
case "$1:${QUALIFICATION_CASE:-valid}" in
  pkgid:empty-id) exit 0 ;;
  pkgid:multiline-id) printf 'one\ntwo\n'; exit 0 ;;
  metadata:nonzero)
    cat "$QUALIFICATION_METADATA"
    printf 'metadata process oracle\033[31m\n' >&2
    exit 7 ;;
  metadata:stdout-over) head -c 8388609 /dev/zero; exit 0 ;;
  metadata:stderr-over) head -c 1048577 /dev/zero >&2; exit 0 ;;
  metadata:invalid-utf8) printf '\377'; exit 0 ;;
  metadata:*) cat "$QUALIFICATION_METADATA"; exit 0 ;;
esac
exec "$QUALIFICATION_REAL_CARGO" "$@"
CARGO
chmod 700 "$scratch/metadata-tools/cargo"
export PATH="$scratch/metadata-tools:$PATH"
args=(--json artifact build --manifest "$probe" --package owned-wasm-probe --output "$output")
cp "$scratch/probe-metadata.json" "$QUALIFICATION_METADATA"
: > "$QUALIFICATION_CALLS"
"$cli" "${args[@]}" > "$scratch/metadata-valid.json"
diff -u <(printf '%s\n' pkgid metadata build) "$QUALIFICATION_CALLS"
rg -a -q 'owned-wasm-probe!' "$output"
hash=$(sha256sum "$output" | cut -d ' ' -f1)
jq -e --arg hash "$hash" '.artifact.sha256 == $hash' "$scratch/metadata-valid.json" >/dev/null
cp "$output" "$scratch/retained.wasm"
check_refusal() {
  local name=$1 kind=$2 message=$3
  QUALIFICATION_CALLS="$scratch/$name.calls"
  shift 3
  : > "$QUALIFICATION_CALLS"
  if QUALIFICATION_CASE="$name" "$cli" "${args[@]}" > "$scratch/$name.stdout" 2> "$scratch/$name.stderr"; then
    echo "Expected refusal for $name" >&2; exit 1
  fi
  jq -e --arg kind "$kind" --arg message "$message" \
    '.error == $kind and (.message | contains($message)) and (.message | contains("\u001b") | not)' "$scratch/$name.stderr" >/dev/null
  diff -u <(printf '%s\n' "$@") "$QUALIFICATION_CALLS"
  cmp "$output" "$scratch/retained.wasm"
  [[ -z $(find "$CARGO_TARGET_DIR/canic-artifacts" -name '.cargo-*' -print -quit) ]]
  printf 'Passed refusal: %s\n' "$name"
}
for name in empty-id multiline-id; do
  check_refusal "$name" invalid 'Cargo package identity is not one line' pkgid
done
check_refusal nonzero supplier 'metadata process oracle' pkgid metadata
check_refusal stdout-over supplier 'Limit(Stdout)' pkgid metadata
check_refusal stderr-over supplier 'Limit(Stderr)' pkgid metadata
check_refusal invalid-utf8 supplier 'expected value' pkgid metadata
printf '{invalid json' > "$QUALIFICATION_METADATA"
check_refusal malformed-json supplier 'key must be a string' pkgid metadata
for field in target_directory build_directory packages; do
  jq --arg field "$field" 'del(.[$field])' "$scratch/probe-metadata.json" > "$QUALIFICATION_METADATA"
  check_refusal "missing-$field" supplier "missing field \`$field\`" pkgid metadata
done
for field in id name manifest_path metadata; do
  jq --arg field "$field" 'del(.packages[0][$field])' "$scratch/probe-metadata.json" > "$QUALIFICATION_METADATA"
  check_refusal "missing-package-$field" supplier "missing field \`$field\`" pkgid metadata
done
for key in endpoints schema lifecycle; do
  jq --arg key "$key" '.packages[0].metadata = {canic:{layout:{($key):"../outside"}}}' \
    "$scratch/probe-metadata.json" > "$QUALIFICATION_METADATA"
  check_refusal "layout-$key" invalid "layout.$key must name a path inside src" pkgid metadata
done
