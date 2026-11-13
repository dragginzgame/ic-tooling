#!/usr/bin/env bash
# Shared companions: scripts/ci/run-validation-stage.pl scripts/release/observe-fixture-retention.pl scripts/ci/select-tooling-checks.sh ci/tooling-checks.tsv
set -euo pipefail
[[ $# -ge 1 && $# -le 2 ]] || exit 2
group=$1
bash scripts/release/group.sh ci/cargo-release-groups.json "$group" members >/dev/null
stage=${2:-fixture-retention}
check=$(bash scripts/ci/select-tooling-checks.sh reusable | awk -F '\t' -v stage="$stage" '$1 == stage {print $2}')
[[ -n $check && $check != *$'\n'* ]] || exit 2
root=$(pwd -P)
source=${RELEASE_SOURCE:?}
destination=${TOOLING_STAGE_ADMISSIONS:?}
[[ $source =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ &&
   $destination != /* && $destination != *..* && $destination != *$'\n'* &&
   $destination != *$'\r'* ]] || exit 2
state=target/release-stage-cache
for directory in "$state/tmp" "$destination"; do
    route=''
    IFS=/ read -r -a parts <<< "$directory"
    for part in "${parts[@]}"; do
        [[ -n $part && $part != . && $part != .. ]] || exit 2
        route=${route:+$route/}$part
        [[ ! -L $route ]] || { echo "redirected stage route: $route" >&2; exit 1; }
    done
done
mkdir -p "$state/tmp" "$destination"
[[ ! -e "$destination/$stage.json" && ! -L "$destination/$stage.json" ]] || {
    echo "Occupied fixture admission: $destination/$stage.json" >&2; exit 1;
}
environment=$(mktemp "$state/environment.XXXXXX")
capture=$(mktemp "$state/execution.XXXXXX")
jq -cnS --arg path "$PATH" --arg tmp "$root/$state/tmp" --arg stage "$stage" \
    --arg yq "${YQ:-$root/.tools/host/bin/yq}" \
    '{PATH:$path,TMPDIR:$tmp,LC_ALL:"C",TOOLING_FIXTURE_STAGE:$stage,YQ:$yq}' > "$environment"
perl scripts/ci/run-validation-stage.pl --state "$state" --stage "$stage" \
    --contract "tooling-$stage/1" --source "$source" --group "$group" \
    --observe scripts/release/observe-fixture-retention.pl --environment "$environment" \
    --max-bytes 33554432 -- bash "$check" | tee "$capture"
admission=$(sed -En "s/^Validation stage $stage: (executed|reused); admission //p" "$capture")
case $admission in "$root/$state/attempts/$stage-$group."*/admission.json) ;; *) exit 1 ;; esac
jq -e --arg source "$source" --arg group "$group" --arg stage "$stage" '
    .kind == "release-stage-admission" and .source == $source and .group == $group
    and .stage == $stage and .contract == ("tooling-" + $stage + "/1")
    and (.mode == "executed" or .mode == "reused")' "$admission" >/dev/null
[[ ! -e "$destination/$stage.json" && ! -L "$destination/$stage.json" ]]
cp "$admission" "$destination/$stage.json"
jq -r '"Tooling fixture admission: \(.stage); mode=\(.mode); elapsed_seconds=\(.elapsed_seconds); reason=" +
    (if .mode == "reused" then "matching-proof-admitted" else "no-admissible-matching-proof" end)' "$admission"
