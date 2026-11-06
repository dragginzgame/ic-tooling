#!/usr/bin/env bash
set -euo pipefail

# Early selection and read-only version discovery. Shared owns complete
# Cargo/graph admission during release preflight and metadata preparation.
[[ $# == 2 || ( $# == 3 && ( "$3" == members || "$3" == version || "$3" == batch ) ) ]] || {
    echo 'usage: group.sh POLICY GROUP [members|version|batch]' >&2; exit 2;
}
policy="$1" selected="$2"
fail() { printf 'select explicit RELEASE_GROUP from %s\n' "$policy" >&2; exit 1; }
[[ -f "$policy" && ! -L "$policy" ]] || fail
if [[ ${3:-} == batch ]]; then
    jq -e --arg selected "$selected" '
        . as $policy | ($selected | split(",")) as $groups |
        .schema == 1 and (.groups | type == "array") and
        ($groups | length > 0) and ($groups | length) == ($groups | unique | length) and
        all($groups[]; . as $name | test("^[A-Za-z0-9][A-Za-z0-9_-]*$") and
            ([$policy.groups[] | select(.name == $name)] | length == 1))
    ' "$policy" > /dev/null || fail
    if [[ $selected == *,* ]]; then
        metadata=$(RUSTUP_AUTO_INSTALL=0 cargo metadata --locked --offline --no-deps --format-version 1) || exit
        jq -e --arg selected "$selected" --slurpfile policy "$policy" '
            ($selected | split(",")) as $groups |
            (reduce $policy[0].groups[] as $g ({};
                reduce $g.members[] as $name (. ; .[$name] = $g.name))) as $owner |
            all(.packages[]; . as $p |
                all(.dependencies[] | select(.path != null); . as $d |
                    ($groups | index($owner[$p.name])) as $consumer |
                    ($groups | index($owner[$d.name])) as $supplier |
                    $consumer == null or $supplier == null or $supplier <= $consumer))
        ' <<< "$metadata" > /dev/null || {
            echo 'frozen release groups must be in workspace supplier-first order' >&2; exit 1;
        }
    fi
    exit 0
fi
[[ "$selected" =~ ^[A-Za-z0-9][A-Za-z0-9_-]*$ ]] || fail
selection="$(jq -ce --arg group "$selected" '
    if .schema == 1 and (.groups | type == "array") then
        [.groups[] | select(.name == $group)] |
        if length == 1 then .[0] else error("unknown release group") end
    else error("invalid release policy") end
' "$policy")" || fail
if [[ ${3:-} == members ]]; then
    jq -er '.members | if type == "array" and length > 0 and
        all(.[]; type == "string" and test("^[A-Za-z0-9][A-Za-z0-9_-]*$"))
        then .[] else error("invalid group members") end' <<< "$selection" || fail
fi
if [[ ${3:-} == version ]]; then
    primary=$(jq -er '.primary as $p | select($p | type == "string") |
        select(.members | index($p)) | $p' <<< "$selection") || fail
    # Fresh frozen checkouts have no local tool selections yet. Cargo parses
    # the manifests; jq projects its workspace metadata without installing tools.
    metadata=$(RUSTUP_AUTO_INSTALL=0 cargo metadata --locked --offline --no-deps --format-version 1) || exit
    jq -er --arg primary "$primary" '
        . as $m | [.packages[] | select(.name == $primary and
            (.id as $id | $m.workspace_members | index($id)))] |
        if length == 1 then .[0].version else error("group primary is not unique") end
    ' <<< "$metadata"
fi
