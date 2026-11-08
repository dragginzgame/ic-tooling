#!/usr/bin/env bash
set -euo pipefail
root="${BASH_SOURCE[0]}"
[[ $root == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture=$(fixture_create "$root/target/release-stage-reuse.XXXXXX")
fixture_complete=false
finish() {
    local status=$?
    if [[ $status != 0 ]]; then
        fixture_replay_logs "$fixture"/*.log
        local admission
        for admission in "$fixture"/work/admissions/*/*.json; do
            [[ -f $admission ]] || continue
            jq '{mode, source, group, execution_proof}' "$admission" >&2 || :
        done
    fi
    fixture_finish "$status" "$fixture" 'Stage reuse evidence retained'
}
trap finish EXIT
mkdir -p "$fixture/work/ci"
bash "$root/scripts/release/prepare-validation-fixture.sh" "$root" "$fixture/work"
cp "$root/Cargo.toml" "$root/Cargo.lock" "$fixture/work/"
export YQ="${YQ:-$root/.tools/host/bin/yq}"
cd "$fixture/work"
source_a=1111111111111111111111111111111111111111
source_b=2222222222222222222222222222222222222222
run() {
    RELEASE_SOURCE=$1 TOOLING_STAGE_ADMISSIONS="admissions/$2" \
        bash scripts/release/reuse-fixture.sh "$3" "$stage" > "$fixture/$2.log" 2>&1
}
while IFS=$'\t' read -r stage _check; do
    run "$source_a" "$stage-first" foundation
    jq -e '.mode == "executed"' "admissions/$stage-first/$stage.json" >/dev/null
    # These are deliberately unread by the invariant fixture contracts.
    printf '[workspace]\n' > Cargo.toml
    printf 'version = 4\n' > Cargo.lock
    run "$source_b" "$stage-second" build
    jq -e --arg source "$source_b" '.mode == "reused" and .source == $source and .group == "build"' \
        "admissions/$stage-second/$stage.json" >/dev/null
    first=$(jq -r .execution_proof "admissions/$stage-first/$stage.json")
    second=$(jq -r .execution_proof "admissions/$stage-second/$stage.json")
    [[ $first == "$second" ]]
    jq -e --arg source "$source_a" '.origin.source == $source and .origin.group == "foundation"' "$first" >/dev/null
    printf '\n# changed prefix input\n' >> scripts/ci/fixture-lifecycle.sh
    run "$source_b" "$stage-changed" testkit
    jq -e '.mode == "executed"' "admissions/$stage-changed/$stage.json" >/dev/null
    printf '# new inventory input\n' > "scripts/ci/new-$stage-input.sh"
    run "$source_b" "$stage-added" text
    jq -e '.mode == "executed"' "admissions/$stage-added/$stage.json" >/dev/null
done < <(bash scripts/ci/select-tooling-checks.sh reusable)
echo 'Consumer fixture stages reuse unchanged inputs across groups, preserve execution origins and invalidate changed/new inputs'
fixture_complete=true
