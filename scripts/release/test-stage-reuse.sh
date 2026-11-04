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
        fixture_replay_logs "$fixture/first.log" "$fixture/second.log" "$fixture/changed.log" "$fixture/added.log"
        local admission
        for admission in "$fixture"/work/admissions/*/fixture-retention.json; do
            [[ -f $admission ]] || continue
            jq '{mode, source, group, execution_proof}' "$admission" >&2 || :
        done
    fi
    fixture_finish "$status" "$fixture" 'Stage reuse evidence retained'
}
trap finish EXIT
mkdir -p "$fixture/work/ci"
cp -R "$root/scripts" "$fixture/work/"
cp "$root/ci/tooling-checks.tsv" "$fixture/work/ci/"
cd "$fixture/work"
source_a=1111111111111111111111111111111111111111
source_b=2222222222222222222222222222222222222222
run() {
    RELEASE_SOURCE=$1 TOOLING_STAGE_ADMISSIONS="admissions/$2" \
        bash scripts/release/reuse-fixture.sh "$3" > "$fixture/$2.log" 2>&1
}
run "$source_a" first foundation
jq -e '.mode == "executed"' admissions/first/fixture-retention.json >/dev/null
# These are deliberately unread by the retention-prefix contract.
printf '[workspace]\n' > Cargo.toml
printf 'version = 4\n' > Cargo.lock
run "$source_b" second build
jq -e --arg source "$source_b" '.mode == "reused" and .source == $source and .group == "build"' \
    admissions/second/fixture-retention.json >/dev/null
first=$(jq -r .execution_proof admissions/first/fixture-retention.json)
second=$(jq -r .execution_proof admissions/second/fixture-retention.json)
[[ $first == "$second" ]]
jq -e --arg source "$source_a" '.origin.source == $source and .origin.group == "foundation"' "$first" >/dev/null
printf '\n# changed prefix input\n' >> scripts/ci/fixture-lifecycle.sh
run "$source_b" changed testkit
jq -e '.mode == "executed"' admissions/changed/fixture-retention.json >/dev/null
printf '# new inventory input\n' > scripts/ci/new-retention-input.sh
run "$source_b" added text
jq -e '.mode == "executed"' admissions/added/fixture-retention.json >/dev/null
grep -F 'Host fixtures reject' "$fixture/first.log" >/dev/null
if grep -F 'Host fixtures reject' "$fixture/second.log" >/dev/null; then exit 1; fi
echo 'Consumer retention stage executes once across group/version metadata, preserves its origin, and invalidates changed/new inputs'
fixture_complete=true
