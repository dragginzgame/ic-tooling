#!/usr/bin/env bash
set -euo pipefail

# Early selection admission only. Shared owns complete Cargo/graph admission.
[[ $# == 2 || ( $# == 3 && "$3" == members ) ]] || {
    echo 'usage: group.sh POLICY GROUP [members]' >&2; exit 2;
}
policy="$1" selected="$2"
fail() { printf 'select explicit RELEASE_GROUP from %s\n' "$policy" >&2; exit 1; }
[[ "$selected" =~ ^[A-Za-z0-9][A-Za-z0-9_-]*$ ]] || fail
[[ -f "$policy" && ! -L "$policy" ]] || fail
selection="$(jq -ce --arg group "$selected" '
    if .schema == 1 and (.groups | type == "array") then
        [.groups[] | select(.name == $group)] |
        if length == 1 then .[0] else error("unknown release group") end
    else error("invalid release policy") end
' "$policy")" || fail
if [[ $# == 3 ]]; then
    jq -er '.members | if type == "array" and length > 0 and
        all(.[]; type == "string" and test("^[A-Za-z0-9][A-Za-z0-9_-]*$"))
        then .[] else error("invalid group members") end' <<< "$selection" || fail
fi
