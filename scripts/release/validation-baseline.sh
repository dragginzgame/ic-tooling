#!/usr/bin/env bash
# Shared companions: ci/release-validation-baseline.jq
set -euo pipefail
# Read-only, bounded CI discovery. An absent qualified baseline selects the full gate.
[[ $# == 2 ]] || exit 2
source_sha=$1 output=$2
printf 'null\n' > "$output"
[[ ${CARGO_NET_OFFLINE:-false} != true ]] || exit 0
command -v gh >/dev/null 2>&1 || exit 0
remote=$(git remote get-url origin) || exit 0
case "$remote" in
    https://github.com/dragginzgame/ic-tooling|https://github.com/dragginzgame/ic-tooling.git|git@github.com:dragginzgame/ic-tooling.git) ;;
    *) exit 0 ;;
esac
scratch=$(mktemp -d "${output}.discovery.XXXXXX")
# Keep remote observations, including failures, with this attempt's evidence.
if ! gh api 'repos/dragginzgame/ic-tooling/actions/workflows/ci.yml/runs?branch=main&status=success&per_page=10' \
    > "$scratch/runs.json" 2> "$scratch/discovery.log"; then exit 0; fi
jq -er '.workflow_runs[] | select(.status == "completed" and .conclusion == "success" and
    .head_branch == "main" and (.event == "push" or .event == "workflow_dispatch") and
    .path == ".github/workflows/ci.yml" and
    .repository.full_name == "dragginzgame/ic-tooling") | [.id,.head_sha] | @tsv' \
    "$scratch/runs.json" > "$scratch/candidates.tsv" 2>> "$scratch/discovery.log" || exit 0
while IFS=$'\t' read -r run revision; do
    [[ $revision =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ && $run =~ ^[0-9]+$ ]] || continue
    git merge-base --is-ancestor "$revision" "$source_sha" 2>/dev/null || continue
    if ! gh api "repos/dragginzgame/ic-tooling/actions/runs/$run/jobs?per_page=100" \
        > "$scratch/$run.jobs.json" 2>> "$scratch/discovery.log"; then continue; fi
    if ! jq -cnS --arg source "$revision" --argjson run "$run" \
        --slurpfile jobs "$scratch/$run.jobs.json" \
        '{schema:1,source:$source,run:$run,jobs:$jobs[0]}' > "$scratch/$run.candidate.json" 2>> "$scratch/discovery.log"; then continue; fi
    jq -e -f ci/release-validation-baseline.jq "$scratch/$run.candidate.json" > /dev/null 2>> "$scratch/discovery.log" || continue
    cp "$scratch/$run.candidate.json" "$output"
    exit 0
done < "$scratch/candidates.tsv"
