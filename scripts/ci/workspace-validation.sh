#!/usr/bin/env bash
# Shared companions: scripts/ci/validation-roster.sh scripts/ci/run-validation-targets.sh scripts/publish/workspace.sh
set -euo pipefail
[[ $# == 1 && ( $1 == ci || $1 == ci-publication ) ]] || exit 2
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
cd "$root"
roster=$(bash scripts/ci/validation-roster.sh full)
targets=() before=() after=()
lint=false
while IFS= read -r target; do
    targets+=("$target")
    if [[ $target == clippy-workspace ]]; then lint=true
    elif [[ $lint == false ]]; then before+=("$target")
    else after+=("$target"); fi
done <<< "$roster"
[[ $lint == true && ${#before[@]} -gt 0 && ${#after[@]} -gt 0 ]] || {
    echo 'Workspace qualification requires prerequisites, lint and product checks' >&2; exit 1;
}
if [[ $1 == ci ]]; then
    bash scripts/ci/run-validation-targets.sh --fail-fast "${targets[@]}"
else
    # Publication owns the one lint pass and dry run between these exact policy
    # partitions. It cannot skip source/package admission or upload in qualify mode.
    bash scripts/ci/run-validation-targets.sh --fail-fast "${before[@]}"
    bash scripts/publish/workspace.sh qualify
    CARGO_NET_OFFLINE=true bash scripts/ci/run-validation-targets.sh --fail-fast "${after[@]}"
fi
