#!/usr/bin/env bash
# Shared companions: scripts/ci/check-rust-toolchain.sh
set -euo pipefail
[[ $# == 3 ]] || { echo 'usage: record-qualification.sh REPOSITORY NEW-DIRECTORY native' >&2; exit 2; }
repository="$1" output="$2" profile="$3"
tooling_root="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
case "$profile" in native) ;; *) exit 2 ;; esac
cd -- "$repository"
source="$(git rev-parse --verify HEAD)"
[[ "$source" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] || exit 1
git diff --quiet HEAD --
mkdir -- "$output"
printf '%s\n' "$source" > "$output/source-revision"
git status --porcelain --untracked-files=all > "$output/source-status"
[[ ! -s "$output/source-status" ]] || { echo 'qualification requires a clean source tree' >&2; exit 1; }
printf '%s\n' "$profile" > "$output/profile"
bash "$tooling_root/scripts/ci/check-rust-toolchain.sh" "$PWD" "$output"
uname -s > "$output/os"
uname -m > "$output/architecture"
case "$(cat "$output/os")" in
    Darwin) sw_vers -productVersion > "$output/os-version" ;;
    Linux) uname -r > "$output/os-version" ;;
    *) echo 'unsupported qualification host' >&2; exit 1 ;;
esac
cp Cargo.lock "$output/Cargo.lock"
printf '%s\n' "${GITHUB_RUN_ID:-local}" > "$output/run-id"
printf '%s\n' "${GITHUB_RUN_ATTEMPT:-local}" > "$output/run-attempt"
# Record only after every preceding job gate passed. Keep partial files on any
# observer failure, but never label them a completed qualification.
[[ "$(git rev-parse --verify HEAD)" == "$source" ]]
git status --porcelain --untracked-files=all > "$output/source-status-after"
cmp "$output/source-status" "$output/source-status-after"
cmp Cargo.lock "$output/Cargo.lock"
printf 'complete\n' > "$output/completion"
