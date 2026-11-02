#!/usr/bin/env bash
# Shared companions: scripts/ci/check-release-source.sh ci/cargo-release-groups.json ci/release-validation.json scripts/release/validation-focus.sh make/release-validation-local.mk
set -euo pipefail
[[ $# == 1 ]] || exit 2
stage=$1
jq -e --arg target "release-stage-$stage" '.stages | index($target) != null' ci/release-validation.json >/dev/null
[[ $(git rev-parse HEAD) == "${RELEASE_SOURCE:?}" ]]
ledger=$(jq -er --arg group "${RELEASE_GROUP:?}" '.groups[] | select(.name == $group) | .changelog' ci/cargo-release-groups.json)
# Whole-source admission supplements Shared's plan/configuration checkpoints.
# Keep the one permitted pending ledger byte-bound in the runner inventory.
bash scripts/ci/check-release-source.sh --allow "$ledger"
status=0
make --no-print-directory -- "$stage" || status=$?
[[ $status == 0 ]] || exit "$status"
[[ $(git rev-parse HEAD) == "$RELEASE_SOURCE" ]]
bash scripts/ci/check-release-source.sh --allow "$ledger"
