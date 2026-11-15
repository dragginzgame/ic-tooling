#!/usr/bin/env bash
set -euo pipefail
printf '%s\0' "$@" >> "$0.calls"
printf '\n' >> "$0.calls"
if [[ "${1:-}" == --version ]]; then
  if [[ "$0" == *bad-version ]]; then printf 'icp 0.0.0\n'; else printf 'icp 1.7.0\n'; fi
  exit 0
fi
[[ "$#" == 4 && "$1" == identity && "$2" == principal && "$3" == --identity ]]
case "$4" in
  valid) printf ' 2vxsx-fae\n' ;;
  invalid) printf 'not a principal\n' ;;
  utf8) printf '\377' ;;
  nonzero) printf '2vxsx-fae\n'; printf 'fixture diagnostic\n' >&2; exit 7 ;;
  stdout-over) head -c 1025 /dev/zero ;;
  stderr-over) head -c 4097 /dev/zero >&2 ;;
  '--literal;$(false)') printf '2vxsx-fae\n' ;;
  *) exit 91 ;;
esac
