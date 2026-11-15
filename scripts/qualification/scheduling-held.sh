#!/usr/bin/env bash
# The released CLI owns this server; the unchanged Testkit API serves both cases.
set -euo pipefail
scratch="$1"
driver="$2"
wasm="$3"
: "${IC_TESTKIT_POCKET_IC_URL:?outer server owner supplies endpoint}"
endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
curl --fail --silent --show-error "$endpoint/instances" | jq -e 'length == 0' >/dev/null
if env -u IC_TESTKIT_POCKET_IC_URL POCKET_IC_BIN=/unavailable "$driver" --held-replies "$wasm" > "$scratch/missing-endpoint.log" 2>&1; then
  echo 'Missing external endpoint unexpectedly succeeded' >&2; exit 1
fi
rg -Fq 'held-replies requires an externally owned server endpoint' "$scratch/missing-endpoint.log"
"$driver" --held-replies "$wasm" > "$scratch/held-replies.log"
cat "$scratch/held-replies.log"
rg -Fxq 'held-replies: pressure and held-upgrade checks passed' "$scratch/held-replies.log"
curl --fail --silent --show-error "$endpoint/instances" > "$scratch/instances-after.json"
jq -e 'length == 1 and .[0] == "Deleted"' "$scratch/instances-after.json" >/dev/null
