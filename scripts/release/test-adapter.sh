#!/usr/bin/env bash
set -euo pipefail

# Real consumer policy, Cargo metadata, Git, rendering and receipts. Tool setup
# and the complete CI/MSRV gates are substitutes; no release or delivery runs.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES RELEASE_COMMIT RELEASE_GROUP
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_NAMESPACE
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/tooling-release-groups.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Release adapter fixtures retained"' EXIT
# Shared retains failed group observations. Keep controlled failure probes
# beneath this owned fixture so a successful fixture owns their cleanup too.
export TMPDIR="$fixture"
export ADAPTER_REAL_CARGO ADAPTER_REAL_BASH ADAPTER_REAL_GIT ADAPTER_REAL_EDITOR ADAPTER_SELECTED_EDITOR
ADAPTER_REAL_CARGO="$(command -v cargo)"
ADAPTER_REAL_BASH="$(command -v bash)"
ADAPTER_REAL_GIT="$(command -v git)"
ADAPTER_REAL_EDITOR="$(bash "$root/scripts/release/editor.sh" check)"
ADAPTER_SELECTED_EDITOR="$fixture/editor"
tooling="$fixture/tooling"$'\n'
mkdir -p "$tooling" "$fixture/bin"
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$tooling" scripts/release/adapter.sh
adapter="$tooling/scripts/release/adapter.sh"
cat > "$ADAPTER_SELECTED_EDITOR" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${ADAPTER_CONTROL:-}" == render && "$3" == group-manifest ]]; then exit 31; fi
exec "$ADAPTER_REAL_EDITOR" "$@"
STUB
cat > "$fixture/bin/bash" <<'STUB'
#!/bin/bash
set -euo pipefail
if [[ "${1##*/}" == editor.sh && "${2:-}" == check ]]; then
    [[ "${ADAPTER_EDITOR_BUSY:-0}" == 0 ]] || exit 75
    [[ -f "$ADAPTER_EDITOR_READY" ]] || exit 1
    printf '%s\n' "$ADAPTER_SELECTED_EDITOR"
    exit 0
fi
exec "$ADAPTER_REAL_BASH" "$@"
STUB
cat > "$fixture/bin/cargo" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1" == fetch ]]; then
    printf 'cargo %s\n' "$*" >> "$ADAPTER_EVENTS"
    exit 0
fi
if [[ "$1" == metadata && "${ADAPTER_CONTROL:-}" == graph &&
      "$("$YQ" -p toml '.package.version' crates/ic-testkit/Cargo.toml)" == "$RELEASE_VERSION" ]]; then
    "$ADAPTER_REAL_CARGO" "$@"
    exit 37
fi
exec "$ADAPTER_REAL_CARGO" "$@"
STUB
cat > "$fixture/bin/make" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf 'make %s\n' "$*" >> "$ADAPTER_EVENTS"
case "$*" in
    '--no-print-directory install-tools') : > "$ADAPTER_EDITOR_READY" ;;
    '--no-print-directory tools-check'|'--no-print-directory dependency-pins-check') ;;
    '--no-print-directory ci') exit "${ADAPTER_GATE_RESULT:-0}" ;;
    '--no-print-directory msrv-qualification') exit "${ADAPTER_MSRV_RESULT:-0}" ;;
    *) echo "Unexpected Make operation: $*" >&2; exit 2 ;;
esac
STUB
cat > "$fixture/bin/git" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ ( "${ADAPTER_GIT_CONTROL:-}" == head && "$*" == 'rev-parse HEAD' ) ||
      ( "${ADAPTER_GIT_CONTROL:-}" == diff && "$*" == 'diff --binary HEAD' ) ]]; then
    "$ADAPTER_REAL_GIT" "$@"
    exit 23
fi
exec "$ADAPTER_REAL_GIT" "$@"
STUB
chmod +x "$ADAPTER_SELECTED_EDITOR" "$fixture/bin/bash" "$fixture/bin/cargo" "$fixture/bin/make" "$fixture/bin/git"
export YQ="${YQ:-yq}"
# A selected parser may be a relative caller path; preserve its executable identity.
YQ="$(command -v "$YQ")"
export PATH="$fixture/bin:$PATH"
export RELEASE_REMOTE=origin RELEASE_BRANCH=main RELEASE_KIND=minor RELEASE_DATE=2026-10-31
invoke() { bash "$adapter" "$@"; }
refuse() {
    if "$@"; then echo 'Unexpected release adapter admission' >&2; exit 1; fi
}
for selected_group in integration foundation; do
    checkout="$fixture/$selected_group checkout"
    mkdir "$checkout"
    bash "$root/scripts/ci/prepare-workspace-fixture.sh" "$root" "$checkout"
    mv "$checkout/source-metadata.json" "$fixture/$selected_group.source-metadata.json"
    cd "$checkout"
    git init -q -b main
    git config user.name Fixture
    git config user.email fixture@example.invalid
    git config core.hooksPath /dev/null
    git remote add origin https://example.invalid/tooling-fixture.git
    export RELEASE_GROUP="$selected_group" ADAPTER_EVENTS="$fixture/$selected_group.events"
    export ADAPTER_EDITOR_READY="$fixture/$selected_group.editor-ready"
    export RELEASE_PREVIOUS RELEASE_VERSION RELEASE_SOURCE
    RELEASE_PREVIOUS="$(invoke version)"
    RELEASE_VERSION="$(bash "$root/scripts/ci/next-release-version.sh" "$RELEASE_PREVIOUS" minor)"
    bash "$root/scripts/ci/cargo-release-group.sh" plan "$selected_group" > "$fixture/$selected_group.before.json"
    ledger="$(jq -r .changelog "$fixture/$selected_group.before.json")"
    # Retain the consumer's real historical body, including imported undated
    # Testkit notes, while selecting this disposable attempt's next heading.
    awk -v version="$RELEASE_VERSION" '
        !selected && /^## \[[0-9]+\.[0-9]+\.[0-9]+\]/ { $0="## [" version "]"; selected=1 }
        { print }
        END { if (!selected) exit 1 }
    ' "$ledger" > "$fixture/$selected_group.notes"
    cp "$fixture/$selected_group.notes" "$ledger"
    git add -- Cargo.toml Cargo.lock CHANGELOG.md releases ci crates apps README.md LICENSE
    git commit -qm fixture
    RELEASE_SOURCE="$(git rev-parse HEAD)"
    state=".git/release-state/$RELEASE_GROUP-$RELEASE_VERSION.validation"
    : > "$ADAPTER_EVENTS"
    refuse env RELEASE_GROUP='' bash "$adapter" version > "$fixture/no-group.log" 2>&1
    refuse env RELEASE_GROUP=unknown bash "$adapter" version > "$fixture/unknown-group.log" 2>&1
    [[ ! -s "$ADAPTER_EVENTS" ]]
    invoke files > "$fixture/$selected_group.files"
    bash "$root/scripts/ci/cargo-release-group.sh" files "$selected_group" > "$fixture/$selected_group.expected-files"
    cmp "$fixture/$selected_group.files" "$fixture/$selected_group.expected-files"
    # Busy setup must not become an installation; refused source must precede it.
    status=0
    ADAPTER_EDITOR_BUSY=1 invoke preflight > "$fixture/$selected_group.busy.log" 2>&1 || status=$?
    [[ "$status" == 75 && ! -s "$ADAPTER_EVENTS" ]]
    printf '\nUnrelated dirty source\n' >> README.md
    refuse invoke preflight > "$fixture/$selected_group.dirty.log" 2>&1
    [[ ! -s "$ADAPTER_EVENTS" ]]
    git restore -- README.md
    invoke preflight > "$fixture/$selected_group.preflight.log" 2>&1
    [[ "$(grep -c '^make --no-print-directory install-tools$' "$ADAPTER_EVENTS")" == 1 ]]
    # The real consumer always asks for both complete gates. These stand-ins
    # are the sole qualification substitution, never the source/group checks.
    refuse env ADAPTER_GATE_RESULT=29 bash "$adapter" verify > "$fixture/$selected_group.gate.log" 2>&1
    [[ ! -e "$state" ]]
    refuse env ADAPTER_MSRV_RESULT=30 bash "$adapter" verify > "$fixture/$selected_group.msrv.log" 2>&1
    [[ ! -e "$state" ]]
    invoke verify > "$fixture/$selected_group.verify.log" 2>&1
    grep -Fx 'make --no-print-directory ci' "$ADAPTER_EVENTS" > /dev/null
    grep -Fx 'make --no-print-directory msrv-qualification' "$ADAPTER_EVENTS" > /dev/null
    # Valid-looking output from a failed Git observation grants no custody.
    ADAPTER_GIT_CONTROL='head' refuse invoke prepare > "$fixture/$selected_group.failed-head.log" 2>&1
    ADAPTER_GIT_CONTROL='diff' refuse invoke verify > "$fixture/$selected_group.failed-diff.log" 2>&1
    printf '\n# Unvalidated source edit\n' >> Cargo.toml
    refuse invoke prepare > "$fixture/$selected_group.source-before-prepare.log" 2>&1
    git restore -- Cargo.toml
    # Errors at either renderer or prepared Cargo observation leave all selected
    # and unselected metadata at the qualified source.
    ADAPTER_CONTROL=render refuse invoke prepare > "$fixture/$selected_group.render.log" 2>&1
    git diff --exit-code -- > /dev/null
    if [[ "$selected_group" == integration ]]; then
        ADAPTER_CONTROL=graph refuse invoke prepare > "$fixture/$selected_group.graph.log" 2>&1
        git diff --exit-code -- > /dev/null
    fi
    invoke prepare > "$fixture/$selected_group.prepare.log" 2>&1
    invoke check > "$fixture/$selected_group.check.log" 2>&1
    sed "/^## \\[$RELEASE_VERSION\\]/d" "$ledger" > "$fixture/$selected_group.prepared-body"
    git show "$RELEASE_SOURCE:$ledger" | sed "/^## \\[$RELEASE_VERSION\\]/d" > "$fixture/$selected_group.source-body"
    cmp "$fixture/$selected_group.source-body" "$fixture/$selected_group.prepared-body"
    # Every unselected member/ledger retains its committed bytes; shared root
    # edits are limited to selected supplier requirements and package identities.
    jq -r '.groups[] | select(.name != "'"$selected_group"'") | .changelog' ci/cargo-release-groups.json |
        while IFS= read -r other_ledger; do git diff --exit-code -- "$other_ledger"; done
    cargo metadata --locked --offline --no-deps --format-version 1 > "$fixture/$selected_group.metadata.json"
    git show "$RELEASE_SOURCE:Cargo.toml" | "$YQ" -p toml -o json '.' > "$fixture/$selected_group.catalog-before.json"
    "$YQ" -p toml -o json '.' Cargo.toml > "$fixture/$selected_group.catalog-after.json"
    jq -ne --slurpfile plan "$fixture/$selected_group.before.json" \
        --slurpfile before "$fixture/$selected_group.catalog-before.json" \
        --slurpfile after "$fixture/$selected_group.catalog-after.json" '
        def unchanged($root): $root.workspace.dependencies |
            delpaths($plan[0].dependencies | map([.alias,"version"]));
        unchanged($before[0]) == unchanged($after[0]) and
        all($plan[0].dependencies[]; . as $d |
            $after[0].workspace.dependencies[$d.alias].version == $d.next)
    ' > /dev/null
    jq -e --slurpfile plan "$fixture/$selected_group.before.json" --arg next "$RELEASE_VERSION" '
        . as $m | [.packages[] | select(.id as $id | $m.workspace_members | index($id))] |
        all(.[]; . as $p | if $plan[0].members | index($p.name) then .version == $next
            else .version == $plan[0].versions[$p.name] end)
    ' "$fixture/$selected_group.metadata.json" > /dev/null
    # Copying receipts to another group's namespace cannot authorize it even
    # with identical numeric versions and a real successful source receipt.
    if [[ "$selected_group" == integration ]]; then other_group=foundation; else other_group=integration; fi
    other_state=".git/release-state/$other_group-$RELEASE_VERSION.validation"
    cp "$state" "$other_state"
    cp "$state.notes" "$other_state.notes"
    cp "$state.payload" "$other_state.payload"
    refuse env RELEASE_GROUP="$other_group" bash "$adapter" check > "$fixture/$selected_group.wrong-group.log" 2>&1
    # Payload custody rejects a semantically valid manual note edit too.
    cp "$ledger" "$fixture/$selected_group.prepared-notes"
    printf '\nUnvalidated replacement\n' >> "$ledger"
    refuse invoke check > "$fixture/$selected_group.payload.log" 2>&1
    cp "$fixture/$selected_group.prepared-notes" "$ledger"
    # The ordinary commit checks admit only the declared files. Real Git
    # creates a disposable commit solely to test selected-source verification.
    git add -- Cargo.toml Cargo.lock "$ledger"
    jq -r '.manifests[].path | select(. != "Cargo.toml")' "$fixture/$selected_group.before.json" |
        while IFS= read -r path; do git add -- "$path"; done
    printf '\nUnrelated staging\n' >> README.md
    git add -- README.md
    refuse invoke commit-check > "$fixture/$selected_group.unrelated-staging.log" 2>&1
    git restore --staged --worktree -- README.md
    invoke commit-check > "$fixture/$selected_group.commit.log" 2>&1
    git commit -qm 'fixture prepared group'
    export RELEASE_COMMIT
    RELEASE_COMMIT="$(git rev-parse HEAD)"
    printf '\nUnrelated live notes\n' >> "$ledger"
    invoke committed-check > "$fixture/$selected_group.committed.log" 2>&1
    unset RELEASE_COMMIT
done
fixture_complete=true
echo 'Release adapter checks passed (both groups; setup and full qualification substituted).'
