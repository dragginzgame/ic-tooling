#!/usr/bin/env bash
# Focused source-integrity fixture; does not touch any real sibling.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/ic-local-source.XXXXXXXX")"
trap 'rm -rf "$fixture"' EXIT
source_checkout="$fixture/input"
scratch="$fixture/output"
mkdir -p "$source_checkout" "$scratch/source"
git -C "$source_checkout" init -q
printf 'original\n' > "$source_checkout/tracked"
printf 'removed\n' > "$source_checkout/deleted"
printf 'ignored\n' > "$source_checkout/.gitignore"
git -C "$source_checkout" add -- tracked deleted .gitignore
git -C "$source_checkout" -c user.name=Fixture -c user.email=fixture@example.invalid -c core.hooksPath=/dev/null commit -qm fixture
printf 'current\0bytes\n' > "$source_checkout/tracked"
chmod 755 "$source_checkout/tracked"
rm "$source_checkout/deleted"
printf 'new\n' > "$source_checkout/new file"
printf 'not source\n' > "$source_checkout/ignored"
ln -s tracked "$source_checkout/link"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
cmp "$source_checkout/tracked" "$scratch/source/tracked"
cmp "$source_checkout/new file" "$scratch/source/new file"
[[ -x "$scratch/source/tracked" && -L "$scratch/source/link" ]]
[[ "$(readlink "$scratch/source/link")" == tracked ]]
[[ ! -e "$scratch/source/deleted" && ! -e "$scratch/source/ignored" && ! -e "$scratch/source/.git" ]]
sha256sum --check "$scratch/source.sha256"
printf 'Current edits, additions, deletions, modes, links and ignored-file exclusion pass.\n'
# Relative source selection must use the caller's directory, even if CDPATH has
# a same-named directory elsewhere or would print the selected directory.
mkdir -p "$fixture/decoy/input"
for search in . "$fixture/decoy:."; do
  (
    CDPATH='' cd -- "$fixture"
    export CDPATH="$search"
    source_checkout=input
    scratch="$(mktemp -d "$fixture/relative.XXXXXXXX")"
    mkdir "$scratch/source"
    export_local_source
    [[ "$source_checkout" == "$fixture/input" && "$CDPATH" == "$search" ]]
    [[ "$(cat "$scratch/source-checkout")" == "$fixture/input" ]]
    cmp "$fixture/input/tracked" "$scratch/source/tracked"
  )
done
printf 'Relative source selection ignores CDPATH without changing its caller.\n'
# Change a selected file between the two captures; no export may be admitted.
scratch="$fixture/changing"
mkdir -p "$scratch/source"
tar() {
  command tar "$@" || return 1
  if [[ "${!#}" == "$scratch/source-before.tar" ]]; then
    printf 'changed during capture\n' > "$source_checkout/tracked"
  fi
}
if export_local_source > "$fixture/refusal.log" 2>&1; then
  printf 'Source drift was incorrectly admitted.\n' >&2
  exit 1
fi
rg -q 'Local source changed during snapshot' "$fixture/refusal.log"
[[ ! -e "$scratch/source/tracked" ]]
printf 'Concurrent source-change refusal passes.\n'
