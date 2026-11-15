#!/usr/bin/env bash
# Negative identity oracle for the selected Agent export; no build or server.
set -euo pipefail
[[ $# == 2 ]] || exit 2
source_root="$1"
evidence="$2"
jq '
  (.packages[] | select(.name == "ic_principal")) as $principal |
  (.packages[] | select(.name == "candid") | .id) as $candid |
  ($principal.id + "-qualification-other") as $foreign |
  .packages += [($principal | .id = $foreign)] |
  (.resolve.nodes[] | select(.id == $candid) | .deps[] |
    select(.pkg == $principal.id) | .pkg) = $foreign
' "$evidence/metadata.json" > "$evidence/mismatched-principal.json"
if bash "$source_root/scripts/ci/check-qualification-identities.sh" \
  "$evidence/mismatched-principal.json" "$source_root" agent-installed-proof \
  ic-platform,pocket-ic > "$evidence/mismatched-principal.log" 2>&1; then
  echo 'Mismatched Principal supplier unexpectedly admitted' >&2
  exit 1
fi
rg -Fq 'Agent/PocketIC CanisterId identity mismatch' "$evidence/mismatched-principal.log"
