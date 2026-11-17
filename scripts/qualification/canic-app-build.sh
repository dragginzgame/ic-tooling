#!/usr/bin/env bash
# App recipe composition over the exported current Canic CLI; no deployment.
set -euo pipefail
[[ $# == 2 ]] || exit 2
cli="$1"
scratch="$2"
config="$scratch/canic.json"
"$cli" --json app init fixture --fleet mainnet --network ic --config "$config" > "$scratch/init.json"
jq '.apps.fixture.canisters = {
  a: {wasm:"app/a.wasm", build:{manifest:"probe/Cargo.toml", package:"owned-wasm-probe"}},
  z: {wasm:"app/a.wasm", build:{manifest:"probe/Cargo.toml", package:"owned-wasm-probe"}}
}' "$config" > "$scratch/duplicate.json"
mv "$scratch/probe/Cargo.lock" "$scratch/probe.original.lock"
mkdir "$scratch/app"
printf 'keep a' > "$scratch/app/a.wasm"
printf 'keep z' > "$scratch/app/z.wasm"
# Duplicate destinations are rejected before lock initialization or build effects.
if "$cli" --json app build --config "$scratch/duplicate.json" > "$scratch/duplicate.stdout" 2> "$scratch/duplicate.stderr"; then
  echo 'Expected duplicate destination refusal' >&2; exit 1
fi
[[ ! -e "$scratch/probe/Cargo.lock" && "$(cat "$scratch/app/a.wasm")" == 'keep a' ]]
jq '.apps.fixture.canisters.z.wasm = "app/z.wasm"' "$scratch/duplicate.json" > "$config"
# A failed lock initializer must stop before compilation/publication, without retry.
mkdir "$scratch/lock-failure-tools"
cat > "$scratch/lock-failure-tools/cargo" <<'TOOL'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${1:-}" == generate-lockfile ]]; then
  printf 'attempt\n' >> "$CANIC_LOCK_CALLS"
  printf 'lock initialization oracle\033[31m\n' >&2
  exit 7
fi
exec "$CANIC_LOCK_REAL_CARGO" "$@"
TOOL
chmod 700 "$scratch/lock-failure-tools/cargo"
export CANIC_LOCK_CALLS="$scratch/lock-calls" CANIC_LOCK_REAL_CARGO
CANIC_LOCK_REAL_CARGO="$(command -v cargo)"
if PATH="$scratch/lock-failure-tools:$PATH" "$cli" --json app build --config "$config" > "$scratch/lock-failure.stdout" 2> "$scratch/lock-failure.stderr"; then
  echo 'Expected lock initialization refusal' >&2; exit 1
fi
jq -e '.error == "supplier" and (.message | contains("lock initialization oracle")) and (.message | contains("\u001b") | not)' "$scratch/lock-failure.stderr" >/dev/null
[[ "$(wc -l < "$CANIC_LOCK_CALLS")" == 1 && ! -e "$scratch/probe/Cargo.lock" ]]
[[ "$(cat "$scratch/app/a.wasm")" == 'keep a' && "$(cat "$scratch/app/z.wasm")" == 'keep z' ]]
# Recipes are relative to the config, not to the invoking directory.
(CDPATH='' cd -- "$scratch/runtime" && "$cli" --json app build --config "$config") > "$scratch/app.json"
[[ -s "$scratch/probe/Cargo.lock" ]]
sha256sum "$scratch/probe/Cargo.lock" > "$scratch/initialized-lock.sha256"
hash="$(sha256sum "$scratch/app/a.wasm" | cut -d ' ' -f1)"
size="$(wc -c < "$scratch/app/a.wasm")"
cmp "$scratch/app/a.wasm" "$scratch/app/z.wasm"
rg -a -q 'owned-wasm-probe!' "$scratch/app/a.wasm"
jq -e --arg root "$scratch" --arg hash "$hash" --argjson size "$size" '
  (.canisters | map(.canister)) == ["a","z"] and
  all(.canisters[]; .artifact.sha256 == $hash and .artifact.bytes == $size and
    .wasm == ($root + "/app/" + .canister + ".wasm"))
' "$scratch/app.json" >/dev/null
PATH="$scratch/lock-failure-tools:$PATH" "$cli" app build --config "$config" > "$scratch/app-human.txt"
[[ "$(wc -l < "$CANIC_LOCK_CALLS")" == 1 ]]
rg -Fq 'Nothing deployed.' "$scratch/app-human.txt"
sha256sum --check "$scratch/initialized-lock.sha256"
# Serial app builds are not a transaction: a later failure retains the first output.
jq '.apps.fixture.canisters.z.build.package = "missing-package"' "$config" > "$scratch/partial.json"
printf 'replace a' > "$scratch/app/a.wasm"
printf 'retain z' > "$scratch/app/z.wasm"
if "$cli" --json app build --config "$scratch/partial.json" > "$scratch/partial.stdout" 2> "$scratch/partial.stderr"; then
  echo 'Expected second recipe selection refusal' >&2; exit 1
fi
[[ "$(sha256sum "$scratch/app/a.wasm" | cut -d ' ' -f1)" == "$hash" ]]
[[ "$(cat "$scratch/app/z.wasm")" == 'retain z' ]]
sha256sum --check "$scratch/initialized-lock.sha256"
[[ -z "$(ls -A "$scratch/runtime")" ]]
printf 'App preflight, config-relative recipes, lock initialization/preservation, exact output and partial failure passed.\n'
