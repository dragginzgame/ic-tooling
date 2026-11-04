#!/usr/bin/env bash
# Shared companions: scripts/ci/run-validation-stage.pl scripts/release/observe-fixture-retention.pl
set -euo pipefail
[[ $# == 1 && $1 =~ ^(foundation|build|text|tools|testkit|native)$ ]] || exit 2
group=$1
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
environment=$(mktemp "$state/environment.XXXXXX")
capture=$(mktemp "$state/execution.XXXXXX")
jq -cnS --arg path "$PATH" --arg tmp "$root/$state/tmp" \
    '{PATH:$path,TMPDIR:$tmp,LC_ALL:"C"}' > "$environment"
perl scripts/ci/run-validation-stage.pl --state "$state" --stage fixture-retention \
    --contract tooling-fixture-retention/1 --source "$source" --group "$group" \
    --observe scripts/release/observe-fixture-retention.pl --environment "$environment" \
    --max-bytes 33554432 -- bash scripts/ci/test-host-fixture-retention.sh | tee "$capture"
admission=$(sed -n 's/^Validation stage fixture-retention: \(executed\|reused\); admission //p' "$capture")
case $admission in "$root/$state/attempts/fixture-retention-$group."*/admission.json) ;; *) exit 1 ;; esac
jq -e --arg source "$source" --arg group "$group" '
    .kind == "release-stage-admission" and .source == $source and .group == $group
    and .stage == "fixture-retention" and .contract == "tooling-fixture-retention/1"
    and (.mode == "executed" or .mode == "reused")' "$admission" >/dev/null
[[ ! -e "$destination/fixture-retention.json" && ! -L "$destination/fixture-retention.json" ]]
cp "$admission" "$destination/fixture-retention.json"
