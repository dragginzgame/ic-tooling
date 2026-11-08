#!/usr/bin/env bash
# Shared companions: ci/release-delivery.txt ci/release-editor.env scripts/release/editor.sh scripts/release/group.sh scripts/release/validation.sh scripts/ci/check-release-delivery.sh scripts/ci/check-release-source.sh scripts/ci/next-release-version.sh scripts/ci/finalize-release-changelog.awk scripts/ci/release-receipts.sh scripts/ci/cargo-release-group.sh
set -euo pipefail
export RUSTUP_AUTO_INSTALL=0

# Shared owns group admission, literal rendering and transactional installation.
# This consumer owns source custody, complete qualification and release receipts.
operation="${1:-}"
[[ $# == 1 ]] || exit 2
tooling_root="${BASH_SOURCE[0]}"
[[ "$tooling_root" == /* ]] || tooling_root="$PWD/$tooling_root"
tooling_root="$(cd -P "${tooling_root%/*}/../.." && printf '%s/.' "$PWD")"
tooling_root="${tooling_root%/.}"
bash "$tooling_root/scripts/release/group.sh" ci/cargo-release-groups.json "${RELEASE_GROUP:-}"
bash "$tooling_root/scripts/ci/check-release-delivery.sh" \
    "$tooling_root/ci/release-delivery.txt" "${RELEASE_DELIVERY-direct}"
fail() { echo "release metadata refused: $1" >&2; exit 1; }
group() { bash "$tooling_root/scripts/ci/cargo-release-group.sh" "$1" "$RELEASE_GROUP" "${@:2}"; }
case "$operation" in
    version) exec bash "$tooling_root/scripts/release/group.sh" ci/cargo-release-groups.json "$RELEASE_GROUP" version ;;
    files) group "$operation"; exit ;;
    preflight|verify|prepare|check|commit-check|committed-check) ;;
    *) echo 'usage: adapter.sh version|files|preflight|verify|prepare|check|commit-check|committed-check' >&2; exit 2 ;;
esac
[[ "${RELEASE_SOURCE:?}" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] || fail 'invalid source identity'
selected_version="$(bash "$tooling_root/scripts/ci/next-release-version.sh" "${RELEASE_PREVIOUS:?}" "${RELEASE_KIND:?}")"
[[ "$selected_version" == "${RELEASE_VERSION:?}" ]] || fail 'conflicting release selection'
perl "$tooling_root/scripts/ci/validate-release-inputs.pl" date "${RELEASE_DATE:?}" > /dev/null
state="$(git rev-parse --git-path release-state)"
[[ ! -L "$state" ]] || fail 'symlinked release evidence directory'
identity="$RELEASE_GROUP-$RELEASE_VERSION"
receipt="$state/$identity.validation"
notes="$receipt.notes"
payload="$receipt.payload"
selection="$receipt.selection"
gate="$receipt.gate"
scratch="$(mktemp -d "${TMPDIR:-/tmp}/tooling-release-adapter.XXXXXX")"
finish() {
    local status=$?
    if [[ "$status" == 0 ]]; then rm -rf -- "$scratch"
    else printf 'Release adapter evidence retained: %s\n' "$scratch" >&2; fi
    exit "$status"
}
trap finish EXIT
group plan > "$scratch/plan.json"
changelog="$(jq -er '.changelog' "$scratch/plan.json")"
jq -jr '.files[] | . + "\u0000"' "$scratch/plan.json" > "$scratch/files"
files=()
while IFS= read -r -d '' file; do files+=("$file"); done < "$scratch/files"
# shellcheck source=scripts/ci/release-receipts.sh
source "$tooling_root/scripts/ci/release-receipts.sh"
select_editor() {
    # shellcheck source=ci/release-editor.env
    source "$tooling_root/ci/release-editor.env" || return
    CARGO_RELEASE_METADATA_EDITOR="$(bash "$tooling_root/scripts/release/editor.sh" check)" || return
    [[ "$CARGO_RELEASE_METADATA_EDITOR" == /* && -f "$CARGO_RELEASE_METADATA_EDITOR" && -x "$CARGO_RELEASE_METADATA_EDITOR" ]] ||
        fail 'release editor must be an admitted absolute executable'
    export CARGO_RELEASE_METADATA_EDITOR
    export CARGO_RELEASE_METADATA_MAX_BYTES="$HOST_RELEASE_EDITOR_MAX_BYTES"
}
admit_files() {
    local allowed=(--allow "$changelog") path
    if [[ "$operation" != preflight && "$operation" != verify && "$operation" != prepare ]]; then
        for path in "${files[@]}"; do allowed+=(--allow "$path"); done
    fi
    bash "$tooling_root/scripts/ci/check-release-source.sh" "${allowed[@]}" ||
        fail 'commit refused source paths before releasing'
}
receipt_header() {
    printf '%s\n' tooling-release-validation-5 "$RELEASE_KIND" || return
    release_receipt_identity || return
    bash "$tooling_root/scripts/release/validation.sh" "${1:-retained-receipt}" "$gate"
}
check_receipt() {
    [[ -f "$notes" && ! -L "$notes" ]] || fail 'source-bound validation notes are missing'
    release_receipt_check "$receipt" receipt_header || fail 'source-bound group validation evidence is missing or changed'
}
check_prepared() {
    check_receipt
    group check '' --manifest-order-grouped
    release_receipt_check "$payload" release_receipt_payload "${RELEASE_COMMIT:-}" "${files[@]}" ||
        fail 'prepared group payload differs from its saved receipt'
    if [[ -z "${RELEASE_COMMIT:-}" ]]; then admit_files; fi
}
unchanged_outside_release() {
    local path
    local pathspecs=(.)
    for path in "${files[@]}"; do pathspecs+=(":(literal,exclude)$path"); done
    git diff "$@" -- "${pathspecs[@]}" > /dev/null || fail 'changes outside the selected release files'
}
case "$operation" in
    preflight|verify)
        observed_source="$(git rev-parse HEAD)"
        observed_version="$(jq -er '.previous' "$scratch/plan.json")"
        [[ "$observed_source" == "$RELEASE_SOURCE" ]] || fail 'source changed'
        [[ "$observed_version" == "$RELEASE_PREVIOUS" ]] || fail 'selected group version changed'
        admit_files
        if [[ "$operation" == preflight ]]; then
            bash "$tooling_root/scripts/release/validation.sh" plan "$scratch/selection"
            current_selection="$scratch/selection"
        else
            current_selection="$selection"
            [[ -f "$current_selection" && ! -L "$current_selection" ]] || fail 'source-bound preflight selection is missing; run the saved source preflight'
        fi
        selected_context="$(jq -er '.directory + "/context.json"' "$current_selection")"
        scope="$(jq -er --arg source "$RELEASE_SOURCE" --arg group "$RELEASE_GROUP" \
            'select(.source == $source and .group == $group) | .selection' "$selected_context")"
        case "$scope" in focused|full) ;; *) fail 'invalid gate selection' ;; esac
        setup_tools() {
            if [[ "$scope" == focused && "$RELEASE_GROUP" != build ]]; then make --no-print-directory install-host-tools install-rust-tools install-release-editor
            else make --no-print-directory install-tools; fi
        }
        fresh_tool_setup=false
        if [[ "$operation" == preflight && ! -e "$state/$identity.plan" && ! -L "$state/$identity.plan" ]]; then
            fresh_tool_setup=true
        fi
        editor_setup=false
        if [[ "$fresh_tool_setup" == true ]]; then
            editor_status=0
            select_editor || editor_status=$?
            [[ "$editor_status" != 75 ]] || exit "$editor_status"
            if [[ "$editor_status" != 0 ]]; then
                setup_tools
                editor_setup=true
                select_editor
            fi
        else select_editor; fi
        awk -v version="$RELEASE_VERSION" -v previous="$RELEASE_PREVIOUS" -v date="$RELEASE_DATE" \
            -f "$tooling_root/scripts/ci/finalize-release-changelog.awk" "$changelog" > /dev/null
        if [[ "$fresh_tool_setup" == true && "$editor_setup" == false ]]; then setup_tools; fi
        CARGO_NET_OFFLINE=true make --no-print-directory compiler-check
        if [[ "$scope" == focused ]]; then
            CARGO_NET_OFFLINE=true make --no-print-directory release-scope-tools-check
        else CARGO_NET_OFFLINE=true make --no-print-directory tools-check; fi
        cargo sort --workspace --grouped --check
        CARGO_NET_OFFLINE=true make --no-print-directory dependency-pins-check
        if [[ "$fresh_tool_setup" == true ]]; then cargo fetch --locked
        else cargo fetch --locked --offline; fi
        if [[ "$operation" == preflight ]]; then
            # Tooling owns this preflight record. Carry its release context so
            # publication never interprets Shared's private journal format.
            # shellcheck disable=SC2016 # jq, not Bash, binds these fields.
            release_receipt_write "$selection" jq -cS \
                --arg kind "$RELEASE_KIND" --arg previous "$RELEASE_PREVIOUS" \
                --arg version "$RELEASE_VERSION" --arg date "$RELEASE_DATE" \
                --arg remote "$RELEASE_REMOTE" --arg branch "$RELEASE_BRANCH" \
                '.release = {kind:$kind,previous:$previous,version:$version,date:$date,remote:$remote,branch:$branch}' \
                "$current_selection"
        else
            mkdir -p "$state"
            cp -p "$changelog" "$scratch/notes"
            before="$(git diff --binary HEAD | git hash-object --stdin)"
            release_receipt_archive "$state/$identity.validation-attempt.XXXXXX" \
                "$receipt" "$notes" "$receipt.log" "$payload" "$gate"
            bash "$tooling_root/scripts/release/validation.sh" fork "$current_selection" "$scratch/gate"
            release_receipt_write "$gate" cat "$scratch/gate"
            {
                # The receipt is written only after every required plan stage passes.
                CARGO_NET_OFFLINE=true bash "$tooling_root/scripts/release/validation.sh" run "$gate"
            } 2>&1 | tee -a "$receipt.log"
            observed_source="$(git rev-parse HEAD)"
            after="$(git diff --binary HEAD | git hash-object --stdin)"
            [[ "$observed_source" == "$RELEASE_SOURCE" && "$after" == "$before" ]] || fail 'source changed during validation'
            admit_files
            cmp "$scratch/notes" "$changelog"
            release_receipt_write "$notes" cat "$scratch/notes"
            # Seal only evidence freshly re-admitted by Shared. Later payload
            # checks compare its frozen inventory after version preparation.
            release_receipt_write "$receipt" receipt_header receipt
        fi
        ;;
    prepare)
        observed_source="$(git rev-parse HEAD)"
        observed_version="$(jq -er '.previous' "$scratch/plan.json")"
        [[ "$observed_source" == "$RELEASE_SOURCE" ]] || fail 'preparation source changed'
        [[ "$observed_version" == "$RELEASE_PREVIOUS" ]] || fail 'preparation group version changed'
        check_receipt
        cmp "$notes" "$changelog"
        admit_files
        select_editor
        group prepare
        # An unsuccessful receipt write retains prepared metadata for explicit
        # recovery; no subsequent commit/tag/delivery can admit a missing receipt.
        release_receipt_write "$payload" release_receipt_payload '' "${files[@]}"
        ;;
    check) check_prepared ;;
    committed-check)
        [[ -n "${RELEASE_COMMIT:-}" ]] || fail 'missing committed release identity'
        check_prepared
        unchanged_outside_release --exit-code "$RELEASE_SOURCE" "$RELEASE_COMMIT"
        ;;
    commit-check)
        check_prepared
        git diff --quiet -- || fail 'unstaged release changes'
        unchanged_outside_release --cached --exit-code "$RELEASE_SOURCE"
        ;;
esac
