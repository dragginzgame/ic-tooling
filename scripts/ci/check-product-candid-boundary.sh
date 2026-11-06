#!/usr/bin/env bash
# Shared companions: scripts/ci/check-candid-boundary.pl ci/candid-boundary.json
set -euo pipefail
[[ $# == 0 ]] || exit 2
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
# One complete policy/inventory, including tests and independent fixture drivers.
# Never filter this selection by RELEASE_GROUP while retaining the full policy.
exec perl "$root/scripts/ci/check-candid-boundary.pl" "$root" ci/candid-boundary.json crates apps scripts docs
