#!/usr/bin/env bash
# Harmless ICP-shaped fixture. No keys, networking, shell evaluation or submission.
set -euo pipefail
mode= identity=
args=("$@")
while (($#)); do
  case "$1" in
    --network) mode="$2"; shift 2 ;;
    --identity) identity="$2"; shift 2 ;;
    --) break ;;
    *) shift ;;
  esac
done
[[ -d "$identity" ]]
printf 'call\n' >> "$identity/calls"
printf '%s\0' "${args[@]}" > "$identity/arguments"
case "$mode" in
  valid) printf '  0x00ff10\n' ;;
  exact) head -c 16384 /dev/zero | tr '\000' '0' ;;
  overflow) head -c 16385 /dev/zero | tr '\000' '0' ;;
  nonzero) printf '00'; exit 7 ;;
  utf8) printf '\377' ;;
  odd) printf 'abc' ;;
  nonhex) printf 'zz' ;;
  terminal)
    [[ -t 0 && -t 2 ]]
    printf 'Fixture input: ' >&2
    IFS= read -r answer
    [[ "$answer" == fixture-answer ]]
    printf '00ff' ;;
  *) exit 90 ;;
esac
