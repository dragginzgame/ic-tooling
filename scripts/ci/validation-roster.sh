#!/usr/bin/env bash
# Shared companions: ci/release-validation.json ci/release-validation-coverage.json ci/cargo-release-groups.json vendor/toko-tooling/scripts/ci/validation-evidence.pl
set -euo pipefail
[[ $# == 1 && ( $1 == full || $1 == focused || $1 == groups ) ]] || exit 2
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
cd "$root"
mkdir -p target
scratch=$(mktemp -d target/validation-roster.XXXXXX)
complete=false
finish() {
    local status=$?
    if [[ $complete == true ]]; then rm -rf -- "$scratch"
    else printf 'Roster observations retained: %s\n' "$scratch" >&2; fi
    exit "$status"
}
trap finish EXIT
jq -cS '[.groups[].name]' ci/cargo-release-groups.json > "$scratch/groups.json"
perl vendor/toko-tooling/scripts/ci/validation-evidence.pl roster "$root" ci/release-validation.json \
    "$scratch/groups.json" > "$scratch/stages"
# These annotations own product coverage, not a second dispatch roster. Admit
# every classification before publishing any target to a CI/release caller.
jq -ner --arg mode "$1" --rawfile roster "$scratch/stages" \
    --slurpfile coverage ci/release-validation-coverage.json '
    def names: type == "array" and length > 0 and
        all(.[]; type == "string" and test("^[a-z][a-z0-9-]*$")) and
        (unique | length) == length;
    if ($coverage | length) != 1 then error("expected one product coverage document") else . end |
    $coverage[0] as $c |
    ($roster | split("\n") | map(select(length > 0))) as $stages |
    if all($stages[]; startswith("release-stage-")) then
        ($stages | map(ltrimstr("release-stage-"))) as $targets |
        if $c.schema != 1 or ($c.focused | names | not) or ($c.focused_stages | names | not) or ($c.common | names | not)
            or any($c.focused_stages[]; startswith("release-scope-") | not)
            or any($c.exclusions[]; (.targets | names | not) or (.groups | names | not) or
                any(.groups[]; . as $group | $c.focused | index($group) == null) or
                (.reason | type != "string" or (test("\\S") | not) or test("[\u0000-\u001f\u007f]")))
        then error("invalid product coverage") else
            [$targets[] | select(startswith("release-scope-"))] as $focused |
            [$targets[] | select(startswith("release-scope-") | not)] as $full |
            if $focused != $c.focused_stages or any($c.focused[];
                . as $group |
                ($c.common + [$c.exclusions[] | select(.groups | index($group)) | .targets[]]) as $covered |
                ($covered | unique | length) != ($covered | length) or
                ($full | sort) != ($covered | sort))
            then error("missing, duplicate or unknown stage coverage")
            else (if $mode == "full" then $full elif $mode == "groups" then $c.focused else $focused end)[] end
        end
    else error("invalid Tooling stage prefix") end' > "$scratch/selected"
cat "$scratch/selected"
complete=true
