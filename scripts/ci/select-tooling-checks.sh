#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]] || exit 2
case "$1" in tooling|make-stub|retention|relative|reusable) ;; *) exit 2 ;; esac
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
while IFS=$'\t' read -r path roles stage; do
    if [[ $1 == reusable ]]; then
        [[ -z $stage ]] || printf '%s\t%s\n' "$stage" "$path"
        continue
    fi
    case ",$roles," in *",$1,"*) printf '%s\n' "$path" ;; esac
done < "$root/ci/tooling-checks.tsv"
