#!/usr/bin/env bash
set -euo pipefail
runner_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)"
mode="${1:-raw-rand}"
case "$mode" in raw-rand|agent) ;; *) exit 2 ;; esac
: "${IC_TESTKIT_POCKET_IC_URL:?external endpoint required}"
: "${PLATFORM_QUALIFICATION_EXPORT:?captured source required}"
endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
cd "$PLATFORM_QUALIFICATION_EXPORT/source"
curl --fail --silent --show-error "$endpoint/instances" > "$PLATFORM_QUALIFICATION_EXPORT/instances-before.json"
jq -e 'length == 0' "$PLATFORM_QUALIFICATION_EXPORT/instances-before.json" >/dev/null
if [[ "$mode" == raw-rand ]]; then
  bash scripts/ci/check-platform-raw-rand.sh
  outputs=(target/evidence/raw-rand.*/consumer/Cargo.toml)
  [[ ${#outputs[@]} == 1 && -f "${outputs[0]}" ]]
  evidence="$(dirname "$(dirname "${outputs[0]}")")"
  package=platform-raw-rand-driver
  command=("$evidence/driver-bin" "$evidence/reference.wasm" "$evidence/narrow.wasm")
else
  bash scripts/ci/check-agent-installed.sh
  outputs=(target/agent-installed.*/Cargo.toml)
  [[ ${#outputs[@]} == 1 && -f "${outputs[0]}" ]]
  evidence="$(dirname "${outputs[0]}")"
  bash "$runner_dir/platform-principal.sh" "$PWD" "$PWD/$evidence"
  package=agent-installed-proof
  command=("$CARGO_TARGET_DIR/debug/agent-installed-proof")
  jq -e '.public_type_crossings | length == 1 and .[0].boundary == "PocketIC CanisterId -> Agent"' "$evidence/identities.json" >/dev/null
fi
if env -u IC_TESTKIT_POCKET_IC_URL POCKET_IC_BIN=/unavailable "${command[@]}" > "$PLATFORM_QUALIFICATION_EXPORT/missing-endpoint.log" 2>&1; then
  echo 'Missing endpoint unexpectedly succeeded' >&2
  exit 1
fi
rg -Fq 'external server owner must supply IC_TESTKIT_POCKET_IC_URL' "$PLATFORM_QUALIFICATION_EXPORT/missing-endpoint.log"
cargo tree --manifest-path "${outputs[0]}" -p "$package" --edges normal,build --prefix none --locked --offline > "$PLATFORM_QUALIFICATION_EXPORT/driver-graph.txt"
awk '$1 == "ic-testkit" { legacy = 1 } $1 == "pocket-ic" && $2 == "v16.1.0" { upstream = 1 } END { exit (legacy || !upstream) }' "$PLATFORM_QUALIFICATION_EXPORT/driver-graph.txt"
curl --fail --silent --show-error "$endpoint/instances" > "$PLATFORM_QUALIFICATION_EXPORT/instances-after.json"
jq -e 'length == 1 and .[0] == "Deleted"' "$PLATFORM_QUALIFICATION_EXPORT/instances-after.json" >/dev/null
printf 'Platform instance deleted; external server remained reachable.\n'
