#!/usr/bin/env bash
# Shared companions: scripts/release/adapter.sh scripts/release/group.sh
set -euo pipefail
# Admit an existing full workspace lint through the normal committed-release
# receipt. Context comes from Tooling's preflight record, never Shared's journal.
# Missing, stale or narrower evidence returns failure: the publisher
# then runs its fresh lint. This helper grants no source or archive admission.
[[ $# == 3 && $1 =~ ^[A-Za-z0-9][A-Za-z0-9_-]*$ &&
   $2 =~ ^[0-9]+\.[0-9]+\.[0-9]+$ &&
   $3 =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] || exit 2
group=$1 version=$2 commit=$3
refuse() { printf 'Workspace lint reuse refused: %s\n' "$1" >&2; exit 1; }
root="${BASH_SOURCE[0]}"
[[ $root == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
state=$(git rev-parse --git-path release-state) || refuse 'release state observation failed'
[[ ! -L $state ]] || refuse 'redirected release state'
selection="$state/$group-$version.validation.selection"
[[ -f $selection && ! -L $selection ]] || refuse 'missing or redirected selection'
bash "$root/scripts/release/group.sh" ci/cargo-release-groups.json "$group" || refuse 'group admission failed'
[[ $(git rev-parse --verify HEAD) == "$commit" ]] || refuse 'release HEAD mismatch'
git diff --quiet "$commit" -- || refuse 'source observation failed or source changed'
context=$(jq -er --arg group "$group" --arg version "$version" '
    select(.schema == 1 and .group == $group and .release.version == $version) |
    [.release.kind,.release.previous,.release.date,.source,.release.remote,.release.branch] |
    if all(.[]; type == "string" and length > 0 and (test("[\u0000-\u0020\u007f\\\\]") | not))
    then @tsv else error("invalid release context") end' "$selection") || refuse 'invalid selection context'
IFS=$'\t' read -r RELEASE_KIND RELEASE_PREVIOUS RELEASE_DATE RELEASE_SOURCE RELEASE_REMOTE RELEASE_BRANCH <<< "$context"
export RELEASE_KIND RELEASE_PREVIOUS RELEASE_DATE RELEASE_SOURCE RELEASE_REMOTE RELEASE_BRANCH
export RELEASE_VERSION="$version"
export RELEASE_GROUP="$group" RELEASE_COMMIT="$commit" RELEASE_DELIVERY=direct
gate="$state/$group-$version.validation.gate"
[[ -f $gate && ! -L $gate ]] || refuse 'missing or redirected gate'
directory=$(jq -er --arg source "$RELEASE_SOURCE" --arg group "$group" \
    'select(.schema == 1 and .source == $source and .group == $group) | .directory' "$gate") || refuse 'gate identity mismatch'
case $directory in .git/release-state/*|target/release-validation/*) ;; *) refuse 'gate outside release evidence' ;; esac
jq -e 'any(.stages[]; .target == "release-stage-clippy-workspace" and .required == true)' \
    "$directory/plan.json" >/dev/null || refuse 'full workspace lint absent from plan'
# Validates frozen proof/log inventory, source/configuration/environment, host,
# compiler, exact prepared payload and unchanged code outside release files.
bash "$root/scripts/release/adapter.sh" committed-check || refuse 'committed release receipt admission failed'
printf 'Reused qualified workspace lint for %s at %s\n' "$group" "$commit"
