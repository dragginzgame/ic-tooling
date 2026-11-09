#!/usr/bin/env bash
# Shared companions: scripts/ci/check-candid-boundary.pl ci/candid-boundary.json
set -euo pipefail
[[ $# == 0 ]] || exit 2
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
# One complete policy/inventory, including tests and independent fixture drivers.
# Never filter this selection by RELEASE_GROUP while retaining the full policy.
# The retired compiler fixture left no authored Rust under scripts. Keep that
# optional root covered whenever a source or a forbidden symlink is present.
sources=(crates docs)
[[ ! -e "$root/qualification" && ! -L "$root/qualification" ]] || sources+=(qualification)
# Tooling has no Canic fleet. Keep any future/fixture apps tree covered without
# requiring an empty directory after the native inspector's relocation.
[[ ! -e "$root/apps" && ! -L "$root/apps" ]] || sources+=(apps)
script_input=$(find "$root/scripts" -type d \( -name target -o -name .git -o -name .tools -o -name node_modules \) -prune \
    -o \( -name '*.rs' -o -type l \) -print -quit)
[[ -z "$script_input" ]] || sources+=(scripts)
exec perl "$root/scripts/ci/check-candid-boundary.pl" "$root" ci/candid-boundary.json "${sources[@]}"
