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
    grep -E "^Tooling fixture admission: $stage; mode=executed; elapsed_seconds=[0-9.e+-]+; reason=no-admissible-matching-proof$" \
        "$fixture/$stage-first.log" >/dev/null
    # These are deliberately unread by the invariant fixture contracts.
    printf '[workspace]\n' > Cargo.toml
    printf 'version = 4\n' > Cargo.lock
    run "$source_b" "$stage-second" build
    jq -e --arg source "$source_b" '.mode == "reused" and .source == $source and .group == "build"' \
        "admissions/$stage-second/$stage.json" >/dev/null
    first=$(jq -r .execution_proof "admissions/$stage-first/$stage.json")
    second=$(jq -r .execution_proof "admissions/$stage-second/$stage.json")
    [[ $first == "$second" ]]
    jq -nr --arg stage "$stage" --slurpfile cold "admissions/$stage-first/$stage.json" \
        --slurpfile warm "admissions/$stage-second/$stage.json" \
        '"Fixture admission timing: \($stage); executed_seconds=\($cold[0].elapsed_seconds); reused_seconds=\($warm[0].elapsed_seconds)"'
    grep -E "^Tooling fixture admission: $stage; mode=reused; elapsed_seconds=[0-9.e+-]+; reason=matching-proof-admitted$" \
        "$fixture/$stage-second.log" >/dev/null
    # An occupied destination must refuse before observing/executing again.
    cp "admissions/$stage-second/$stage.json" "$fixture/admission-before"
    if run "$source_b" "$stage-second" build; then exit 1; fi
    grep -F 'Occupied fixture admission:' "$fixture/$stage-second.log" >/dev/null
    if grep -F 'Validation stage evidence retained:' "$fixture/$stage-second.log" >/dev/null; then exit 1; fi
    cmp "$fixture/admission-before" "admissions/$stage-second/$stage.json"
    if [[ $stage == fixture-evidence ]]; then
        # A changed selected environment cannot inherit the warm proof.
        cp "$YQ" "$fixture/alternate-yq"
        YQ="$fixture/alternate-yq" run "$source_b" environment text
        jq -e '.mode == "executed"' "admissions/environment/$stage.json" >/dev/null
        # Lost original output must execute again, retaining the old proof and
        # its diagnostic instead of reporting a false reused admission.
        original_log=$(jq -r .logs.stdout.path "$first")
        mv "$original_log" "$fixture/original-output"
        run "$source_b" missing-log tools
        jq -e --arg first "$first" '.mode == "executed" and .execution_proof != $first' \
            "admissions/missing-log/$stage.json" >/dev/null
        grep -F 'Prior proof retained; executing ordinarily' "$fixture/missing-log.log" >/dev/null
        mv "$fixture/original-output" "$original_log"
    fi
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
