#!/usr/bin/env bash
# The released CLI owns this server; the selected driver owns its instance.
set -euo pipefail
scratch="$1"
driver="$2"
wasm="$3"
selection="${4:-held-replies}"
driver_args=("--$selection" "$wasm")
missing_endpoint='held-replies requires an externally owned server endpoint'
case "$selection" in
  held-replies) expected='held-replies: pressure and held-upgrade checks passed' ;;
  held-pressure) expected='held-pressure: watchdog admission, watchdog pressure and jobs pressure passed' ;;
  lifecycle) expected='lifecycle: watchdog traps, owner commands, refusal and upgrade checks passed' ;;
  retention) expected='retention: capture custody and dropped-handle checks passed' ;;
  executor)
    expected='executor probe: 3 lifecycle and 3 admission checks passed'
    missing_endpoint='executor requires an externally owned server endpoint'
    driver_args=("$wasm") ;;
  *) echo 'Unknown held-reply selection' >&2; exit 2 ;;
esac
: "${IC_TESTKIT_POCKET_IC_URL:?outer server owner supplies endpoint}"
endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
curl --fail --silent --show-error "$endpoint/instances" | jq -e 'length == 0' >/dev/null
if env -u IC_TESTKIT_POCKET_IC_URL POCKET_IC_BIN=/unavailable "$driver" "${driver_args[@]}" > "$scratch/missing-endpoint.log" 2>&1; then
  echo 'Missing external endpoint unexpectedly succeeded' >&2; exit 1
fi
rg -Fq "$missing_endpoint" "$scratch/missing-endpoint.log"
"$driver" "${driver_args[@]}" > "$scratch/$selection.log"
cat "$scratch/$selection.log"
rg -Fxq "$expected" "$scratch/$selection.log"
curl --fail --silent --show-error "$endpoint/instances" > "$scratch/instances-after.json"
jq -e 'length == 1 and .[0] == "Deleted"' "$scratch/instances-after.json" >/dev/null
