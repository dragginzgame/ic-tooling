#!/usr/bin/env bash
set -euo pipefail

requested_group=''
if [[ $# != 0 ]]; then
    [[ $# == 2 && $1 == --group && -n $2 ]] || { echo 'usage: test-adapter.sh [--group GROUP]' >&2; exit 2; }
    requested_group=$2
fi

# Real group policy, Cargo metadata, Git, rendering and receipts. A compact
# fixture gate retains actual lint ordering, Shared proofs and stage admission;
# test-validation.sh separately owns complete production-roster coverage.
# Tool setup/product qualification are substituted; no delivery runs.
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES RELEASE_COMMIT RELEASE_GROUP
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_NAMESPACE
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
group_names=$(bash "$root/scripts/release/group.sh" "$root/ci/cargo-release-groups.json" --list)
groups=()
while IFS= read -r group; do groups+=("$group"); done <<< "$group_names"
if [[ -n $requested_group ]]; then
    bash "$root/scripts/release/group.sh" "$root/ci/cargo-release-groups.json" "$requested_group" members >/dev/null
    groups=("$requested_group")
fi
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture="$(tooling_fixture_create "$root" tooling-release-groups)"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Release adapter fixtures retained"' EXIT
# Shared retains failed group observations. Keep controlled failure probes
# beneath this owned fixture so a successful fixture owns their cleanup too.
export TMPDIR="$fixture"
ADAPTER_REAL_MAKE="$(command -v make)"
export ADAPTER_REAL_MAKE
export ADAPTER_REAL_CARGO ADAPTER_REAL_BASH ADAPTER_REAL_GIT ADAPTER_REAL_EDITOR ADAPTER_SELECTED_EDITOR
ADAPTER_REAL_CARGO="$(command -v cargo)"
export ADAPTER_REAL_STAGE_PATH="$PATH"
ADAPTER_REAL_BASH="$(command -v bash)"
ADAPTER_REAL_GIT="$(command -v git)"
# Focused receipt fixtures can select an already prepared pinned renderer while
# substituting setup admission. Production editor.sh always retains admission.
ADAPTER_REAL_EDITOR="${ADAPTER_REAL_EDITOR:-$(bash "$root/scripts/release/editor.sh" check)}"
[[ "$ADAPTER_REAL_EDITOR" == /* && -f "$ADAPTER_REAL_EDITOR" && -x "$ADAPTER_REAL_EDITOR" ]]
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
      "$("$YQ" -p toml '.package.version' "$ADAPTER_PRIMARY_MANIFEST")" == "$RELEASE_VERSION" ]]; then
    "$ADAPTER_REAL_CARGO" "$@"
    exit 37
fi
exec "$ADAPTER_REAL_CARGO" "$@"
STUB
cat > "$fixture/bin/make" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == '--no-print-directory -f - MAKEFILES=' ]]; then exec "$ADAPTER_REAL_MAKE" "$@"; fi
printf 'make %s\n' "$*" >> "$ADAPTER_EVENTS"
case "$*" in
    *'-- release-stage-tooling-command-check')
        [[ "${ADAPTER_GATE_RESULT:-0}" == 0 ]] || exit "$ADAPTER_GATE_RESULT"
        while IFS=$'\t' read -r stage _check; do
            PATH="$ADAPTER_REAL_STAGE_PATH" "$ADAPTER_REAL_BASH" scripts/release/reuse-fixture.sh "$RELEASE_GROUP" "$stage"
        done < <(bash scripts/ci/select-tooling-checks.sh reusable) ;;
    *'-- release-stage-'*) exit "${ADAPTER_GATE_RESULT:-0}" ;;
    '--no-print-directory install-tools') : > "$ADAPTER_EDITOR_READY" ;;
    '--no-print-directory tools-check'|'--no-print-directory compiler-check'|'--no-print-directory dependency-pins-check') ;;
    '--no-print-directory ci') exit "${ADAPTER_GATE_RESULT:-0}" ;;
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
common_probes=true
# Catalog identity is data, including newly declared groups. Qualification
# coverage remains independently explicit in the production selector.
jq '.groups += [(.groups[0] | .name = "additional-fixture-group")]' \
    "$root/ci/cargo-release-groups.json" > "$fixture/catalog.json"
{ printf '%s\n' "$group_names"; printf 'additional-fixture-group\n'; } > "$fixture/expected-groups"
bash "$root/scripts/release/group.sh" "$fixture/catalog.json" --list > "$fixture/actual-groups"
cmp "$fixture/expected-groups" "$fixture/actual-groups"
bash "$root/scripts/release/group.sh" "$fixture/catalog.json" additional-fixture-group members >/dev/null
for perturbation in '.groups += [.groups[0]]' '.groups[0].name = "../escape"' '.groups = []'; do
    jq "$perturbation" "$fixture/catalog.json" > "$fixture/invalid-catalog.json"
    refuse bash "$root/scripts/release/group.sh" "$fixture/invalid-catalog.json" --list > "$fixture/catalog-refusal.log" 2>&1
done
for selected_group in "${groups[@]}"; do
    checkout="$fixture/$selected_group checkout"
    mkdir "$checkout"
    bash "$root/scripts/ci/prepare-workspace-fixture.sh" "$root" "$checkout"
    bash "$root/scripts/release/prepare-validation-fixture.sh" "$root" "$checkout" --compact
    mv "$checkout/source-metadata.json" "$fixture/$selected_group.source-metadata.json"
    cd "$checkout"
    git init -q -b main
    git config user.name Fixture
    git config user.email fixture@example.invalid
    git config core.hooksPath /dev/null
    git remote add origin https://example.invalid/tooling-fixture.git
    ADAPTER_PRIMARY_MANIFEST="crates/$(jq -r --arg group "$selected_group" '.groups[] | select(.name == $group) | .primary' ci/cargo-release-groups.json)/Cargo.toml"
    export ADAPTER_PRIMARY_MANIFEST
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
    git add -- .
    git commit -qm fixture
    RELEASE_SOURCE="$(git rev-parse HEAD)"
    state=".git/release-state/$RELEASE_GROUP-$RELEASE_VERSION.validation"
    : > "$ADAPTER_EVENTS"
    if [[ $common_probes == true ]]; then
        refuse env RELEASE_GROUP='' bash "$adapter" version > "$fixture/no-group.log" 2>&1
        refuse env RELEASE_GROUP=unknown bash "$adapter" version > "$fixture/unknown-group.log" 2>&1
    fi
    [[ ! -s "$ADAPTER_EVENTS" ]]
    invoke files > "$fixture/$selected_group.files"
    bash "$root/scripts/ci/cargo-release-group.sh" files "$selected_group" > "$fixture/$selected_group.expected-files"
    cmp "$fixture/$selected_group.files" "$fixture/$selected_group.expected-files"
    # Busy setup must not become an installation; refused source must precede it.
    if [[ $common_probes == true ]]; then
        status=0
        ADAPTER_EDITOR_BUSY=1 invoke preflight > "$fixture/$selected_group.busy.log" 2>&1 || status=$?
        [[ "$status" == 75 && ! -s "$ADAPTER_EVENTS" ]]
        printf '\nUnrelated dirty source\n' >> README.md
        refuse invoke preflight > "$fixture/$selected_group.dirty.log" 2>&1
        [[ ! -s "$ADAPTER_EVENTS" ]]
        git restore -- README.md
    fi
    invoke preflight > "$fixture/$selected_group.preflight.log" 2>&1
    [[ "$(grep -c '^make --no-print-directory install-tools$' "$ADAPTER_EVENTS")" == 1 ]]
    # The complete native/Wasm gate is substituted, never source/group checks.
    if [[ $common_probes == true ]]; then
        refuse env ADAPTER_GATE_RESULT=29 bash "$adapter" verify > "$fixture/$selected_group.gate.log" 2>&1
        [[ ! -e "$state" ]]
    fi
    invoke verify > "$fixture/$selected_group.verify.log" 2>&1
    grep 'release-stage-clippy-workspace' "$ADAPTER_EVENTS" > /dev/null
    gate_pointer="$state.gate"
    gate_directory=$(jq -er .directory "$gate_pointer")
    shared_proof="$gate_directory/shared-proof.json"
    jq -e --arg source "$RELEASE_SOURCE" --arg group "$selected_group" \
        '.kind == "group-validation-evidence" and .source == $source and .group == $group' "$shared_proof" >/dev/null
    # Shared corruption/source failures are group-independent. Run them once;
    # a selected --group run still includes the entire common recovery suite.
    if [[ $common_probes == true ]]; then
        # A frozen receipt cannot admit a changed Shared proof or its retained log.
        cp "$shared_proof" "$fixture/$selected_group.shared-proof"
        printf '\n' >> "$shared_proof"
        refuse invoke prepare > "$fixture/$selected_group.changed-proof.log" 2>&1
        cp "$fixture/$selected_group.shared-proof" "$shared_proof"
        execution_log=$(jq -er '.run + "/0.log"' "$shared_proof")
        cp "$execution_log" "$fixture/$selected_group.execution-log"
        printf 'changed retained execution\n' >> "$execution_log"
        refuse invoke prepare > "$fixture/$selected_group.changed-log.log" 2>&1
        cp "$fixture/$selected_group.execution-log" "$execution_log"
        # Valid-looking output from a failed Git observation grants no custody.
        ADAPTER_GIT_CONTROL='head' refuse invoke prepare > "$fixture/$selected_group.failed-head.log" 2>&1
        ADAPTER_GIT_CONTROL='diff' refuse invoke verify > "$fixture/$selected_group.failed-diff.log" 2>&1
        printf '\n# Unvalidated source edit\n' >> Cargo.toml
        refuse invoke prepare > "$fixture/$selected_group.source-before-prepare.log" 2>&1
        git restore -- Cargo.toml
    fi
    # Errors at either renderer or prepared Cargo observation leave all selected
    # and unselected metadata at the qualified source.
    ADAPTER_CONTROL=render refuse invoke prepare > "$fixture/$selected_group.render.log" 2>&1
    git diff --exit-code -- > /dev/null
    ADAPTER_CONTROL=graph refuse invoke prepare > "$fixture/$selected_group.graph.log" 2>&1
    git diff --exit-code -- > /dev/null
    invoke prepare > "$fixture/$selected_group.prepare.log" 2>&1
    invoke check > "$fixture/$selected_group.check.log" 2>&1
    # Prepared versions differ from the original input checkpoint. Recovery
    # admits the already sealed inventory and exact payload, never a new proof.
    if [[ $common_probes == true ]]; then
        refuse bash "$tooling/scripts/release/validation.sh" receipt "$gate_pointer" \
            > "$fixture/$selected_group.prepared-live-proof.log" 2>&1
        cp "$state" "$fixture/$selected_group.receipt"
        sed '1s/tooling-release-validation-5/tooling-release-validation-4/' \
            "$fixture/$selected_group.receipt" > "$state"
        refuse invoke check > "$fixture/$selected_group.old-receipt.log" 2>&1
        cp "$fixture/$selected_group.receipt" "$state"
        invoke check > "$fixture/$selected_group.recovered.log" 2>&1
    fi
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
    case "$selected_group" in
        foundation) other_group=build ;;
        build) other_group=text ;;
        text) other_group=foundation ;;
        tools) other_group=testkit ;;
        testkit) other_group=native ;;
        native) other_group=testkit ;;
    esac
    other_state=".git/release-state/$other_group-$RELEASE_VERSION.validation"
    cp "$state" "$other_state"
    cp "$state.notes" "$other_state.notes"
    cp "$state.payload" "$other_state.payload"
    refuse env RELEASE_GROUP="$other_group" bash "$adapter" check > "$fixture/$selected_group.wrong-group.log" 2>&1
    # Payload custody rejects a semantically valid manual note edit too.
    if [[ $common_probes == true ]]; then
        cp "$ledger" "$fixture/$selected_group.prepared-notes"
        printf '\nUnvalidated replacement\n' >> "$ledger"
        refuse invoke check > "$fixture/$selected_group.payload.log" 2>&1
        cp "$fixture/$selected_group.prepared-notes" "$ledger"
    fi
    # The ordinary commit checks admit only the declared files. Real Git
    # creates a disposable commit solely to test selected-source verification.
    git add -- Cargo.toml Cargo.lock "$ledger"
    jq -r '.manifests[].path | select(. != "Cargo.toml")' "$fixture/$selected_group.before.json" |
        while IFS= read -r path; do git add -- "$path"; done
    if [[ $common_probes == true ]]; then
        printf '\nUnrelated staging\n' >> README.md
        git add -- README.md
        refuse invoke commit-check > "$fixture/$selected_group.unrelated-staging.log" 2>&1
        git restore --staged --worktree -- README.md
    fi
    invoke commit-check > "$fixture/$selected_group.commit.log" 2>&1
    git commit -qm 'fixture prepared group'
    export RELEASE_COMMIT
    RELEASE_COMMIT="$(git rev-parse HEAD)"
    printf '\nUnrelated live notes\n' >> "$ledger"
    invoke committed-check > "$fixture/$selected_group.committed.log" 2>&1
    if [[ $common_probes == true ]]; then
        # Publication reuses only the committed release's sealed full lint.
        # Its source/archive checks remain separate and fresh in workspace.sh.
        git restore --worktree -- "$ledger"
        lint_probe() {
            bash "$root/scripts/release/publication-lint.sh" "$selected_group" "$RELEASE_VERSION" "$RELEASE_COMMIT"
        }
        # Use the real preflight record; no hand-written Shared journal.
        cp "$state.selection" "$fixture/lint-selection.json"
        jq 'del(.release)' "$fixture/lint-selection.json" > "$state.selection"
        refuse lint_probe > "$fixture/lint-missing-context.log" 2>&1
        jq '.release.date = "2026-10-30"' "$fixture/lint-selection.json" > "$state.selection"
        refuse lint_probe > "$fixture/lint-changed-context.log" 2>&1
        cp "$fixture/lint-selection.json" "$state.selection"
        : > "$ADAPTER_EVENTS"
        lint_probe > "$fixture/lint-reused.log" 2>&1
        [[ ! -s "$ADAPTER_EVENTS" ]]
        grep 'Reused qualified workspace lint' "$fixture/lint-reused.log" >/dev/null
        RUSTFLAGS='--cfg changed_publication_context' refuse lint_probe > "$fixture/lint-environment.log" 2>&1
        refuse bash "$root/scripts/release/publication-lint.sh" "$selected_group" "$RELEASE_VERSION" "$RELEASE_SOURCE" \
            > "$fixture/lint-source.log" 2>&1
        printf '\nchanged source\n' >> README.md
        refuse lint_probe > "$fixture/lint-dirty.log" 2>&1
        git restore --worktree -- README.md
        cp "$gate_directory/plan.json" "$fixture/lint-full-plan.json"
        jq '.stages |= map(if .target == "release-stage-clippy-workspace" then .required = false else . end)' \
            "$fixture/lint-full-plan.json" > "$gate_directory/plan.json"
        refuse lint_probe > "$fixture/lint-narrow-plan.log" 2>&1
        cp "$fixture/lint-full-plan.json" "$gate_directory/plan.json"
        cp "$state" "$fixture/lint-receipt"
        printf 'changed receipt\n' >> "$state"
        refuse lint_probe > "$fixture/lint-corrupt-receipt.log" 2>&1
        cp "$fixture/lint-receipt" "$state"
        lint_probe > "$fixture/lint-recovered.log" 2>&1
    fi
    unset RELEASE_COMMIT
    common_probes=false
done
fixture_complete=true
echo "Release adapter checks passed (${groups[*]}; setup and full qualification substituted)."
