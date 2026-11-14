#!/usr/bin/env bash
# Shared companions: scripts/release/select-validation.pl scripts/release/validation-baseline.sh scripts/release/validation-proof.pl scripts/release/validation-stage.sh scripts/release/validation-focus.sh ci/release-validation.json ci/release-validation-coverage.json make/release-validation-local.mk vendor/toko-tooling/scripts/ci/run-validation-targets.sh vendor/toko-tooling/scripts/ci/release-receipts.sh scripts/ci/validation-roster.sh scripts/ci/workspace-validation.sh
set -euo pipefail
[[ $# == 2 || ( $# == 3 && $1 == fork ) ]] || { echo 'usage: validation.sh plan|fork|run|collect|receipt|retained-receipt POINTER' >&2; exit 2; }
operation=$1 pointer=$2
root="${BASH_SOURCE[0]}"
[[ $root == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
source_sha=${RELEASE_SOURCE:?} group=${RELEASE_GROUP:?}
# shellcheck source=vendor/toko-tooling/scripts/ci/release-receipts.sh
source "$root/vendor/toko-tooling/scripts/ci/release-receipts.sh"
gate_directory() {
    local state
    state=$(git rev-parse --git-path release-state)
    # Shared checkpoints use checkout-relative paths. Linked worktrees keep their
    # gate artifacts under target rather than traversing an external Git dir.
    [[ $state == .git/release-state ]] || state=target/release-validation
    mkdir -p "$state"
    mktemp -d "$state/$group-gate.XXXXXX"
}
read_gate() {
    jq -er --arg source "$source_sha" --arg group "$group" \
        'select(.schema == 1 and .source == $source and .group == $group) | .directory' "$pointer"
}
create_gate() {
    perl "$root/scripts/release/select-validation.pl" "$source_sha" "$group" \
        "$directory/baseline.json" "$directory/plan.json" "$directory/context.json"
    rustc -vV > "$directory/rustc.txt"
    cargo -V > "$directory/cargo.txt"
    uname -srm > "$directory/host.txt"
    jq -cnS --arg source "$source_sha" --arg group "$group" --arg directory "$directory" \
        '{schema:1,source:$source,group:$group,directory:$directory}' > "$1"
}
admit_gate() {
    local ledger observed_source
    observed_source=$(git rev-parse HEAD) || return
    [[ $observed_source == "$source_sha" ]] || return
    ledger=$(jq -er --arg group "$group" '.groups[] | select(.name == $group) | .changelog' ci/cargo-release-groups.json) || return
    bash "$root/vendor/toko-tooling/scripts/ci/check-release-source.sh" --allow "$ledger" || return
    perl "$root/scripts/release/select-validation.pl" "$source_sha" "$group" \
        "$directory/baseline.json" "$directory/plan.json" "$directory/context.json" --check || return
    [[ $(rustc -vV) == "$(cat "$directory/rustc.txt")" && $(cargo -V) == "$(cat "$directory/cargo.txt")" &&
        $(uname -srm) == "$(cat "$directory/host.txt")" ]] || { echo 'gate host/compiler context changed' >&2; return 1; }
}
collect_proof() {
    admit_gate >&2 || return
    perl "$root/scripts/release/validation-proof.pl" verify "$source_sha" "$group" "$pointer" || return
    admit_gate >&2
}
collect_gate() {
    # Re-admit original execution, stage logs, input checkpoints and fixture
    # admissions through the existing verifier. This never executes a gate.
    release_receipt_write "$directory/shared-proof.json" collect_proof
}
case $operation in
    plan)
        [[ ! -e $pointer && ! -L $pointer ]]
        directory=$(gate_directory)
        bash "$root/scripts/release/validation-baseline.sh" "$source_sha" "$directory/baseline.json"
        create_gate "$pointer"
        ;;
    fork)
        destination=$3
        [[ ! -e $destination && ! -L $destination ]]
        previous=$(read_gate)
        perl "$root/scripts/release/select-validation.pl" "$source_sha" "$group" \
            "$previous/baseline.json" "$previous/plan.json" "$previous/context.json" --check
        directory=$(gate_directory)
        cp -p "$previous/baseline.json" "$directory/baseline.json"
        create_gate "$destination"
        ;;
    run)
        directory=$(read_gate)
        perl "$root/scripts/release/select-validation.pl" "$source_sha" "$group" \
            "$directory/baseline.json" "$directory/plan.json" "$directory/context.json" --check
        CARGO_NET_OFFLINE=true VALIDATION_REPOSITORY_ROOT="$PWD" VALIDATION_LOG_DIR="$directory/logs" \
            TOOLING_STAGE_ADMISSIONS="$directory/stage-admissions" \
                bash "$root/vendor/toko-tooling/scripts/ci/run-validation-targets.sh" --plan "$directory/plan.json"
        collect_gate
        ;;
    collect)
        directory=$(read_gate)
        # Collect only before the original proof has been sealed. Existing
        # evidence is checked, never silently overwritten with another inventory.
        if [[ -e "$directory/shared-proof.json" || -L "$directory/shared-proof.json" ]]; then
            admit_gate
            perl "$root/scripts/release/validation-proof.pl" check "$source_sha" "$group" "$pointer" > /dev/null
            admit_gate > /dev/null
        else collect_gate; fi
        ;;
    receipt|retained-receipt)
        if [[ "$operation" == receipt ]]; then
            perl "$root/scripts/release/validation-proof.pl" check "$source_sha" "$group" "$pointer" > /dev/null
        fi
        directory=$(read_gate)
        perl "$root/scripts/release/select-validation.pl" "$source_sha" "$group" \
            "$directory/baseline.json" "$directory/plan.json" "$directory/context.json" --check > /dev/null
        [[ $(rustc -vV) == "$(cat "$directory/rustc.txt")" && $(cargo -V) == "$(cat "$directory/cargo.txt")" &&
            $(uname -srm) == "$(cat "$directory/host.txt")" ]] || { echo 'gate host/compiler context changed' >&2; exit 1; }
        perl "$root/scripts/release/validation-proof.pl" receipt "$source_sha" "$group" "$pointer"
        ;;
    *) exit 2 ;;
esac
