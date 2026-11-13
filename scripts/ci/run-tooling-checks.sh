#!/usr/bin/env bash
# Shared companions: scripts/ci/select-tooling-checks.sh scripts/release/group.sh scripts/release/reuse-fixture.sh scripts/ci/product-fixture.sh
set -euo pipefail
[[ $# == 0 ]] || exit 2
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
cd "$root"
# Nested Shared probes inherit this product-owned selection too.
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
TMPDIR="$(tooling_fixture_parent "$root")"
export TMPDIR
group=${IC_TOOLING_RELEASE_GROUP:-}
fallback=no-release-group
[[ -z $group ]] || fallback=no-stage-admission-directory
[[ ${IC_TOOLING_FULL_CI:-0} != 1 ]] || fallback=full-ci
[[ ${IC_TOOLING_FULL_CI:-0} != 1 ]] || group=''
if [[ -n $group ]]; then
    bash scripts/release/group.sh ci/cargo-release-groups.json "$group" members >/dev/null
fi
checks=$(bash scripts/ci/select-tooling-checks.sh tooling)
reusable=$(bash scripts/ci/select-tooling-checks.sh reusable)
while IFS= read -r check; do
    printf 'bash %s\n' "$check"
    started=$SECONDS
    status=0
    route=fresh
    reason=fresh-fixture-contract
    stage=$(awk -F '\t' -v check="$check" '$2 == check {print $1}' <<< "$reusable")
    if [[ -n $stage ]]; then
        reason=$fallback
        if [[ -n $group && -n ${TOOLING_STAGE_ADMISSIONS:-} ]]; then
            route='stage-admission'
            reason=matching-proof-required
        fi
    fi
    printf 'Tooling fixture start: %s; route=%s; reason=%s\n' "$check" "$route" "$reason"
    if [[ $check == scripts/release/test-adapter.sh && -n $group ]]; then
        bash "$check" --group "$group" || status=$?
    else
        if [[ $route == stage-admission ]]; then
            bash scripts/release/reuse-fixture.sh "$group" "$stage" || status=$?
        else
            bash "$check" || status=$?
        fi
    fi
    printf 'Tooling fixture result: %s; status=%s; elapsed_seconds=%s; route=%s\n' \
        "$check" "$status" "$((SECONDS-started))" "$route"
    [[ $status == 0 ]] || exit "$status"
done <<< "$checks"
