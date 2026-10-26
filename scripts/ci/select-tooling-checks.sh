#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 ]] || exit 2
case "$1" in tooling|make-stub|retention|relative) ;; *) exit 2 ;; esac
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
while IFS=$'\t' read -r path roles; do
    case ",$roles," in *",$1,"*) printf '%s\n' "$path" ;; esac
done < "$root/ci/tooling-checks.tsv"
