#!/usr/bin/env bash
# Shared companions: scripts/ci/select-tooling-checks.sh scripts/release/group.sh scripts/release/reuse-fixture.sh scripts/ci/product-fixture.sh scripts/ci/check-release-commands.sh
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
# Keep the final imported probe under the same admitted scratch environment.
RELEASE_GROUP=foundation bash scripts/ci/check-release-commands.sh "$root" \
    ci/tool-versions.env ci/release-delivery.txt make/consumer.mk make/tools.mk \
    make/testkit.mk make/rust-format.mk make/release.mk make/execution.mk \
    scripts/ci/check-make-execution.sh scripts/ci/check-release-delivery.sh \
    scripts/ci/run-formatting.sh scripts/ci/run-rust-formatting.pl scripts/ci/RustSource.pm
