#!/usr/bin/env bash
set -euo pipefail

# Early selection admission only. Shared owns the complete group/graph policy.
[[ $# == 2 ]] || { echo 'usage: group.sh POLICY GROUP' >&2; exit 2; }
policy="$1" selected="$2"
fail() { printf 'select explicit RELEASE_GROUP from %s\n' "$policy" >&2; exit 1; }
[[ "$selected" =~ ^[A-Za-z0-9][A-Za-z0-9_-]*$ ]] || fail
[[ -f "$policy" && ! -L "$policy" ]] || fail
jq -e --arg group "$selected" '
    .schema == 1 and (.groups | type == "array") and
    ([.groups[] | select(.name == $group)] | length == 1)
' "$policy" > /dev/null || fail
