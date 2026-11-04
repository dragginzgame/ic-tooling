#!/usr/bin/env bash
# Shared companions: scripts/ci/select-tooling-checks.sh scripts/release/group.sh scripts/release/reuse-fixture.sh
set -euo pipefail
[[ $# == 0 ]] || exit 2
group=${IC_TOOLING_RELEASE_GROUP:-}
[[ ${IC_TOOLING_FULL_CI:-0} != 1 ]] || group=''
if [[ -n $group ]]; then
    bash scripts/release/group.sh ci/cargo-release-groups.json "$group" members >/dev/null
fi
checks=$(bash scripts/ci/select-tooling-checks.sh tooling)
reusable=$(bash scripts/ci/select-tooling-checks.sh reusable)
while IFS= read -r check; do
    printf 'bash %s\n' "$check"
    if [[ $check == scripts/release/test-adapter.sh && -n $group ]]; then
        bash "$check" --group "$group"
    else
        stage=$(awk -F '\t' -v check="$check" '$2 == check {print $1}' <<< "$reusable")
        if [[ -n $stage && -n $group && -n ${TOOLING_STAGE_ADMISSIONS:-} ]]; then
            bash scripts/release/reuse-fixture.sh "$group" "$stage"
        else
            bash "$check"
        fi
    fi
done <<< "$checks"
