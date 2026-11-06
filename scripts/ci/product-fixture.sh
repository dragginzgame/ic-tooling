#!/usr/bin/env bash
# Shared companions: scripts/ci/fixture-lifecycle.sh
# Product defaults only; Shared retains creation, physical anchoring and cleanup.
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "${BASH_SOURCE[0]%/*}/fixture-lifecycle.sh"

tooling_fixture_parent() {
    [[ $# == 1 && "$1" == /* ]] || return 2
    local parent="${TMPDIR:-$1/target/tmp}"
    case "$parent" in *$'\r'*|*$'\n'*) return 2 ;; esac
    mkdir -p -- "$parent" || return
    (CDPATH='' cd -P -- "$parent" && pwd -P)
}

tooling_fixture_create() {
    [[ $# == 2 && "$2" =~ ^[a-z][a-z0-9-]*$ ]] || return 2
    local parent
    parent="$(tooling_fixture_parent "$1")" || return
    fixture_create "$parent/$2.XXXXXX"
}
