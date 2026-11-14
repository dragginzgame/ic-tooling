#!/usr/bin/env bash
set -euo pipefail
# Consumer scope, whole-source custody and receipt tests; Cargo/Make effects substituted.
selection_only=false
collector_only=false
if [[ $# != 0 ]]; then
    [[ $# == 2 && $1 == --case && ( $2 == selection || $2 == collector ) ]] || exit 2
    if [[ $2 == selection ]]; then selection_only=true; else collector_only=true; fi
fi
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset TOOLING_STAGE_ADMISSIONS
unset GIT_DIR GIT_WORK_TREE GIT_INDEX_FILE GIT_COMMON_DIR GIT_OBJECT_DIRECTORY GIT_ALTERNATE_OBJECT_DIRECTORIES GIT_NAMESPACE
export GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
root="${BASH_SOURCE[0]}"
[[ $root == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture=$(tooling_fixture_create "$root" tooling-validation)
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Group validation fixtures retained"' EXIT
export TMPDIR="$fixture"
mkdir "$fixture/checkout" "$fixture/bin" "$fixture/home"
bash "$root/scripts/ci/prepare-workspace-fixture.sh" "$root" "$fixture/checkout"
rm "$fixture/checkout/source-metadata.json"
bash "$root/scripts/release/prepare-validation-fixture.sh" "$root" "$fixture/checkout"
# Preserve the known runtime caller surfaces for Text's reverse-consumer mapping.
for path in crates/ic-host-tools/src/icp_project/mod.rs qualification/ic-testkit-dev/src/bin/check_packages.rs; do
    mkdir -p "$fixture/checkout/${path%/*}"
    cp "$root/$path" "$fixture/checkout/$path"
done
# Keep the actual, explicitly qualified reverse-call surfaces for Tools/Build.
for path in qualification/ic-testkit-dev/tests/public_contracts.rs qualification/ic-testkit-dev/tests/canister_wire.rs \
    qualification/ic-testkit-dev/src/bin/check_packages.rs qualification/ic-testkit-dev/src/bin/tests/mod.rs; do
    mkdir -p "$fixture/checkout/${path%/*}"
    cp "$root/$path" "$fixture/checkout/$path"
done
export YQ="${YQ:-$root/.tools/host/bin/yq}"
YQ=$(command -v "$YQ")
VALIDATION_TEST_REAL_MAKE="$(command -v make)"
export VALIDATION_TEST_REAL_MAKE
export VALIDATION_TEST_STAGE_PATH="$PATH"
export VALIDATION_TEST_EVENTS="$fixture/events"
export CARGO_HOME="$fixture/home"
unset CARGO RUSTC RUSTDOC RUSTC_WRAPPER RUSTC_WORKSPACE_WRAPPER RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_ENCODED_RUSTDOCFLAGS RUSTDOCFLAGS RUSTC_BOOTSTRAP CARGO_TARGET_DIR
while IFS= read -r name; do
    case $name in CARGO_BUILD_*|CARGO_TARGET_*|CARGO_PROFILE_*|CARGO_FEATURE_*|CARGO_CFG_*|CC|CC_*|CXX|CXX_*|AR|AR_*|LD|LD_*|SDKROOT|MACOSX_DEPLOYMENT_TARGET) unset "$name" ;; esac
done < <(compgen -e)
cat > "$fixture/bin/make" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$*" == '--no-print-directory -f - MAKEFILES=' ]]; then exec "$VALIDATION_TEST_REAL_MAKE" "$@"; fi
last=''
while [[ $# -gt 0 ]]; do
    case "$1" in -C) cd "$2"; shift ;; esac
    last=$1
    shift
done
case $last in
    release-stage-*) exec bash scripts/release/validation-stage.sh "${last#release-stage-}" ;;
    *)
        printf '%s\n' "$last" >> "$VALIDATION_TEST_EVENTS"
        [[ $last != "${VALIDATION_TEST_FAIL:-}" ]] || exit 29
        if [[ $last == "${VALIDATION_TEST_DRIFT:-}" ]]; then
            printf '\nchanged during stage\n' >> crates/ic-host-text/src/lib.rs
        fi
        if [[ $last == tooling-command-check && -n ${TOOLING_STAGE_ADMISSIONS:-} ]]; then
            while IFS=$'\t' read -r stage _check; do
                PATH="$VALIDATION_TEST_STAGE_PATH" bash scripts/release/reuse-fixture.sh "$RELEASE_GROUP" "$stage"
            done < <(bash scripts/ci/select-tooling-checks.sh reusable)
        fi ;;
esac
STUB
chmod +x "$fixture/bin/make"
export PATH="$fixture/bin:$PATH"
# Metadata-only probes use an empty Cargo home; Rustup keeps its prepared compiler.
cd "$fixture/checkout"
git init -q -b main
git config user.name Fixture
git config user.email fixture@example.invalid
git config core.hooksPath /dev/null
git remote add origin https://example.invalid/fixture.git
git add .
git commit -qm baseline
baseline=$(git rev-parse HEAD)
export RELEASE_SOURCE RELEASE_GROUP
export BASELINE_SOURCE="$baseline"
node <<'JS'
const fs=require('fs');const source=process.env.BASELINE_SOURCE;
const names=['native (ubuntu-24.04, Linux, x86_64)','native (macos-15-intel, Darwin, x86_64)','native (macos-15, Darwin, arm64)'];
const steps=['Run native CI and publication qualification','Qualify focused release contracts','Retain release-validation comparison','Retain successful native qualification'];
const jobs=names.map(name=>({name,run_id:17,head_sha:source,status:'completed',conclusion:'success',steps:steps.map(name=>({name,conclusion:'success'}))}));
jobs.push({name:'msrv',status:'completed',conclusion:'success'});
fs.writeFileSync(process.env.TMPDIR+'/baseline.json',JSON.stringify({schema:1,source,run:17,jobs:{total_count:jobs.length,jobs}})+'\n');
JS
make_plan() {
    local label=$1
    directory=".git/release-state/$label"
    mkdir -p "$directory"
    cp "$fixture/baseline.json" "$directory/baseline.json"
    perl scripts/release/select-validation.pl "$RELEASE_SOURCE" "$RELEASE_GROUP" \
        "$directory/baseline.json" "$directory/plan.json" "$directory/context.json" > "$fixture/$label.selection.log"
    rustc -vV > "$directory/rustc.txt"
    cargo -V > "$directory/cargo.txt"
    uname -srm > "$directory/host.txt"
    pointer=".git/release-state/$label.pointer"
    jq -cnS --arg source "$RELEASE_SOURCE" --arg group "$RELEASE_GROUP" --arg directory "$directory" \
        '{schema:1,source:$source,group:$group,directory:$directory}' > "$pointer"
}
refuse() { if "$@"; then echo 'unexpected validation admission' >&2; exit 1; fi; }
absent() { local status=0; "$@" || status=$?; [[ $status == 1 ]]; }
check_focus_routing() {
    local real_bash selected stage
    real_bash=$(command -v bash)
    mkdir -p "$fixture/routing-bin" target/debug
    export ROUTING_REAL_BASH="$real_bash" ROUTING_EVENTS="$fixture/routing-events"
    cat > "$fixture/routing-bin/cargo" <<'STUB'
#!/bin/bash
set -euo pipefail
printf 'cargo %s\n' "$*" >> "$ROUTING_EVENTS"
STUB
    cat > "$fixture/routing-bin/make" <<'STUB'
#!/bin/bash
set -euo pipefail
printf 'make %s\n' "$*" >> "$ROUTING_EVENTS"
[[ ${ROUTING_FAIL:-} != profiles ]] || exit 23
STUB
    cat > "$fixture/routing-bin/bash" <<'STUB'
#!/bin/bash
set -euo pipefail
if [[ $1 == scripts/ci/check-tools-features.sh ]]; then
    printf 'graphs\n' >> "$ROUTING_EVENTS"
else exec "$ROUTING_REAL_BASH" "$@"; fi
STUB
    cat > target/debug/ic-testkit-server <<'STUB'
#!/bin/bash
[[ $* == check ]] || exit 2
printf '/fixture/admitted-pocket-ic\n'
STUB
    chmod +x "$fixture/routing-bin/"* target/debug/ic-testkit-server
    for selected in tools build foundation testkit testkit-wire; do
        : > "$ROUTING_EVENTS"
        for stage in tools lint graphs tests docs archives; do
            PATH="$fixture/routing-bin:$PATH" RELEASE_GROUP="$selected" \
                "$real_bash" "$root/scripts/release/validation-focus.sh" "$stage"
        done
        grep -Fx graphs "$ROUTING_EVENTS" >/dev/null
        absent grep -F -- --workspace "$ROUTING_EVENTS"
        case "$selected" in
            tools)
                grep -F -- "--package ic-host-tools" "$ROUTING_EVENTS" >/dev/null
                grep -F test-tools-ledger-evidence "$ROUTING_EVENTS" >/dev/null
                absent grep -F -- "--test canister_wire" "$ROUTING_EVENTS"
                : > "$ROUTING_EVENTS"
                refuse env PATH="$fixture/routing-bin:$PATH" RELEASE_GROUP=tools ROUTING_FAIL=profiles \
                    "$real_bash" "$root/scripts/release/validation-focus.sh" tests
                absent grep -F -- "--test public_contracts" "$ROUTING_EVENTS" ;;
            build)
                grep -Fx "make --no-print-directory tools-check" "$ROUTING_EVENTS" >/dev/null
                grep -F -- "--package ic-build" "$ROUTING_EVENTS" >/dev/null
                grep -F -- "--test canister_wire" "$ROUTING_EVENTS" >/dev/null
                absent grep -F test-tools-ledger-evidence "$ROUTING_EVENTS" ;;
            foundation)
                grep -Fx -- "cargo run --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline -- --package ic-host-artifacts --package ic-host-fs --package ic-host-http --package ic-host-process --package ic-host-wasm-inspect --package ic-build --package ic-host-tools --package ic-testkit --package ic-testkit-wire" "$ROUTING_EVENTS" >/dev/null
                grep -F -- "-p ic-build -p ic-host-tools -p ic-testkit -p ic-testkit-wire" "$ROUTING_EVENTS" >/dev/null
                grep -F test-fs-capability "$ROUTING_EVENTS" >/dev/null
                grep -F testkit-installed-check "$ROUTING_EVENTS" >/dev/null ;;
            testkit)
                grep -F -- "--package ic-testkit --package ic-testkit-wire" "$ROUTING_EVENTS" >/dev/null
                grep -F -- "test -p ic-testkit -p ic-testkit-wire" "$ROUTING_EVENTS" >/dev/null
                grep -F testkit-installed-check "$ROUTING_EVENTS" >/dev/null
                absent grep -F test-tools-ledger-evidence "$ROUTING_EVENTS" ;;
            testkit-wire)
                grep -F -- "--package ic-testkit-wire" "$ROUTING_EVENTS" >/dev/null
                absent grep -F -- "--package ic-testkit " "$ROUTING_EVENTS"
                grep -F -- "--test public_contracts --test canister_wire" "$ROUTING_EVENTS" >/dev/null
                absent grep -F test-fs-capability "$ROUTING_EVENTS"
                absent grep -F testkit-installed-check "$ROUTING_EVENTS" ;;
        esac
    done
}
check_new_scopes() {
    local selected package
    for selected in tools build foundation testkit testkit-wire; do
        git checkout -q "$baseline"
        package=$(jq -er --arg group "$selected" '.groups[] | select(.name == $group) | .primary' ci/cargo-release-groups.json)
        printf '\npub const QUALIFICATION_FIXTURE: u8 = 1;\n' >> "crates/$package/src/lib.rs"
        git add "crates/$package/src/lib.rs"
        git commit -qm "$selected-only"
        RELEASE_SOURCE=$(git rev-parse HEAD) RELEASE_GROUP=$selected
        make_plan "$selected-only"
        jq -e '.selection == "focused"' "$directory/context.json" >/dev/null
        # Keep each omission explained for this owner, rather than reusing
        # Text's exclusions that would omit the selected owner's contracts.
        jq -e 'all(.stages[]; .required or (.reason | type == "string" and length > 0))' "$directory/plan.json" >/dev/null
        : > "$VALIDATION_TEST_EVENTS"
        bash scripts/release/validation.sh run "$pointer" > "$fixture/$selected.run.log" 2>&1
        bash scripts/release/validation.sh receipt "$pointer" > "$fixture/$selected.receipt"
        grep -Fx release-scope-tests "$VALIDATION_TEST_EVENTS" >/dev/null
        absent grep -Fx test-workspace-installed "$VALIDATION_TEST_EVENTS"
        # Old qualification without the new mapping cannot enable the scope.
        cp "$fixture/baseline.json" "$fixture/qualified-baseline"
        printf 'null\n' > "$fixture/baseline.json"
        make_plan "$selected-no-baseline"
        jq -e '.selection == "full"' "$directory/context.json" >/dev/null
        mv "$fixture/qualified-baseline" "$fixture/baseline.json"
        printf '\npub fn unqualified_consumer() {}\n' >> qualification/ic-testkit-dev/src/lib.rs
        git add qualification/ic-testkit-dev/src/lib.rs
        git commit -qm other-owner
        RELEASE_SOURCE=$(git rev-parse HEAD)
        make_plan "$selected-mixed"
        jq -e '.selection == "full"' "$directory/context.json" >/dev/null
        git checkout -q "$baseline"
        # A new use in an unqualified consumer surface broadens even when the
        # baseline already contains that source and Cargo's parent is unchanged.
        printf 'use %s as supplier;\n' "${package//-/_}" >> qualification/ic-testkit-dev/src/lib.rs
        git add qualification/ic-testkit-dev/src/lib.rs
        git commit -qm new-consumer-surface
        RELEASE_SOURCE=$(git rev-parse HEAD)
        jq --arg source "$RELEASE_SOURCE" '.source = $source | .jobs.jobs |= map(if has("head_sha") then .head_sha = $source else . end)' \
            "$fixture/baseline.json" > "$fixture/consumer-baseline"
        cp "$fixture/baseline.json" "$fixture/qualified-baseline"
        cp "$fixture/consumer-baseline" "$fixture/baseline.json"
        make_plan "$selected-new-surface"
        if [[ $selected == foundation || $selected == testkit ]]; then
            jq -e '.selection == "focused"' "$directory/context.json" >/dev/null
        else
            jq -e '.selection == "full"' "$directory/context.json" >/dev/null
        fi
        mv "$fixture/qualified-baseline" "$fixture/baseline.json"
        git checkout -q "$baseline"
        jq '.exclusions[0].reason += " Revised mapping."' ci/release-validation-coverage.json > "$fixture/new-mapping"
        cp "$fixture/new-mapping" ci/release-validation-coverage.json
        git add ci/release-validation-coverage.json
        git commit -qm qualification-mapping-changed
        RELEASE_SOURCE=$(git rev-parse HEAD)
        make_plan "$selected-old-mapping"
        jq -e '.selection == "full"' "$directory/context.json" >/dev/null
        git checkout -q "$baseline"
        printf '\n[build-dependencies]\n%s = { workspace = true }\n' "$package" >> qualification/ic-testkit-dev/Cargo.toml
        git add qualification/ic-testkit-dev/Cargo.toml
        git commit -qm new-dependency-kind
        RELEASE_SOURCE=$(git rev-parse HEAD)
        jq --arg source "$RELEASE_SOURCE" '.source = $source | .jobs.jobs |= map(if has("head_sha") then .head_sha = $source else . end)' \
            "$fixture/baseline.json" > "$fixture/consumer-baseline"
        cp "$fixture/baseline.json" "$fixture/qualified-baseline"
        cp "$fixture/consumer-baseline" "$fixture/baseline.json"
        make_plan "$selected-new-kind"
        jq -e '.selection == "full"' "$directory/context.json" >/dev/null
        mv "$fixture/qualified-baseline" "$fixture/baseline.json"
    done
    git checkout -q "$baseline"
    jq '.exclusions[0].groups -= ["tools"]' ci/release-validation-coverage.json > "$fixture/incomplete-owner"
    cp "$fixture/incomplete-owner" ci/release-validation-coverage.json
    refuse bash scripts/ci/validation-roster.sh full > "$fixture/incomplete-owner.log" 2>&1
    git restore -- ci/release-validation-coverage.json
    jq 'del(.consumers.build)' ci/release-validation-coverage.json > "$fixture/incomplete-consumers"
    cp "$fixture/incomplete-consumers" ci/release-validation-coverage.json
    refuse perl scripts/release/select-validation.pl "$RELEASE_SOURCE" "$RELEASE_GROUP" \
        "$fixture/baseline.json" "$fixture/missing-consumer-plan" "$fixture/missing-consumer-context" \
        > "$fixture/incomplete-consumers.log" 2>&1
    git restore -- ci/release-validation-coverage.json
}
check_collector() {
    printf '\npub const COLLECTOR_FIXTURE: u8 = 1;\n' >> crates/ic-host-text/src/lib.rs
    git add crates/ic-host-text/src/lib.rs
    git commit -qm collector-fixture
    RELEASE_SOURCE=$(git rev-parse HEAD) RELEASE_GROUP=text
    make_plan collector-success
    : > "$VALIDATION_TEST_EVENTS"
    # Refuse only proof publication, after every actual substituted stage ran.
    mkdir "$directory/shared-proof.json"
    refuse bash scripts/release/validation.sh run "$pointer" > "$fixture/collector-run.log" 2>&1
    grep -Fx release-scope-archives "$VALIDATION_TEST_EVENTS" >/dev/null
    cp "$VALIDATION_TEST_EVENTS" "$fixture/executed-events"
    rmdir "$directory/shared-proof.json"
    bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector.log" 2>&1
    bash scripts/release/validation.sh receipt "$pointer" > "$fixture/collected.receipt"
    cmp "$fixture/executed-events" "$VALIDATION_TEST_EVENTS"
    cp "$directory/shared-proof.json" "$fixture/collected.proof"
    bash scripts/release/validation.sh collect "$pointer"
    cmp "$fixture/collected.proof" "$directory/shared-proof.json"
    run_directory=$(echo "$directory"/logs/validation.*)
    cp "$run_directory/0.log" "$fixture/collector-original.log"
    printf 'changed log\n' >> "$run_directory/0.log"
    refuse bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-changed.log" 2>&1
    cp "$fixture/collector-original.log" "$run_directory/0.log"
    # A missing proof must not turn failed, incomplete or changed execution into
    # successful evidence. Retain the original proof outside the gate meanwhile.
    mv "$directory/shared-proof.json" "$fixture/saved-collected.proof"
    cp "$run_directory/timings.tsv" "$fixture/collector-timings"
    : > "$run_directory/timings.tsv"
    refuse bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-incomplete.log" 2>&1
    cp "$fixture/collector-timings" "$run_directory/timings.tsv"
    cp "$directory/host.txt" "$fixture/collector-host"
    printf 'different host\n' > "$directory/host.txt"
    refuse bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-host.log" 2>&1
    cp "$fixture/collector-host" "$directory/host.txt"
    refuse env RELEASE_GROUP=build bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-group.log" 2>&1
    refuse env RUSTFLAGS=--cfg=unreviewed bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-environment.log" 2>&1
    printf '\nchanged input\n' >> crates/ic-host-text/src/lib.rs
    refuse bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-input.log" 2>&1
    git restore -- crates/ic-host-text/src/lib.rs
    git checkout -q "$baseline"
    refuse bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-source.log" 2>&1
    git checkout -q "$RELEASE_SOURCE"
    cmp "$fixture/executed-events" "$VALIDATION_TEST_EVENTS"
    make_plan collector-failed-stage
    refuse env VALIDATION_TEST_FAIL=release-scope-lint bash scripts/release/validation.sh run "$pointer" > "$fixture/collector-failed-run.log" 2>&1
    cp "$VALIDATION_TEST_EVENTS" "$fixture/failed-events"
    refuse bash scripts/release/validation.sh collect "$pointer" > "$fixture/collector-failed-collect.log" 2>&1
    cmp "$fixture/failed-events" "$VALIDATION_TEST_EVENTS"
    git checkout -q "$baseline"
}
if [[ $collector_only == true ]]; then
    check_collector
    fixture_complete=true
    echo 'Collector-only recovery re-admits original successful execution without reruns; drift and incomplete evidence refuse.'
    exit 0
fi
check_focus_routing
check_new_scopes
if [[ $selection_only == true ]]; then
    fixture_complete=true
    echo 'Six-group selection, owner exclusions, missing evidence and consumer/mixed-source fallback passed (Make effects substituted).'
    exit 0
fi
check_collector
# Exercise read-only discovery itself, including green docs-only/incomplete runs.
cat > "$fixture/bin/gh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$TMPDIR/discovery-events"
case "$*" in
    *workflows*)
        event=workflow_dispatch branch=main path=.github/workflows/ci.yml repository=dragginzgame/ic-tooling
        case ${BASELINE_TEST_MODE:?} in
            dispatch) event=workflow_dispatch ;;
            push) event=push ;;
            pr) event=pull_request ;;
            dispatch-branch) event=workflow_dispatch branch=topic ;;
            dispatch-workflow) event=workflow_dispatch path=.github/workflows/other.yml ;;
            dispatch-repository) event=workflow_dispatch repository=other/tooling ;;
        esac
        jq -cn --arg source "$BASELINE_SOURCE" --arg event "$event" --arg branch "$branch" \
            --arg path "$path" --arg repository "$repository" \
            '{workflow_runs:[{id:17,head_sha:$source,status:"completed",conclusion:"success",
            head_branch:$branch,event:$event,path:$path,repository:{full_name:$repository}}]}' ;;
    *jobs*)
        case ${BASELINE_TEST_MODE:?} in
            valid|dispatch) jq '.jobs' "$TMPDIR/baseline.json" ;;
            docs) jq '.jobs | .jobs |= map(.steps = [])' "$TMPDIR/baseline.json" ;;
            partial) jq '.jobs | .jobs[1].conclusion = "failure"' "$TMPDIR/baseline.json" ;;
            truncated) jq '.jobs | .total_count += 1' "$TMPDIR/baseline.json" ;;
            source) jq '.jobs | .jobs[0].head_sha = "wrong-source"' "$TMPDIR/baseline.json" ;;
            compiler) jq '.jobs | .jobs[-1].conclusion = "failure"' "$TMPDIR/baseline.json" ;;
            unqualified) jq '.jobs | .jobs[0].steps |= map(select(.name != "Qualify focused release contracts"))' "$TMPDIR/baseline.json" ;;
            unretained) jq '.jobs | .jobs[0].steps |= map(select(.name != "Retain release-validation comparison"))' "$TMPDIR/baseline.json" ;;
            malformed) printf '{invalid json\n' ;;
            *) exit 13 ;;
        esac ;;
    *) exit 2 ;;
esac
STUB
chmod +x "$fixture/bin/gh"
git remote set-url origin https://github.com/dragginzgame/ic-tooling.git
: > "$fixture/discovery-events"
BASELINE_TEST_MODE=valid bash scripts/release/validation-baseline.sh "$baseline" "$fixture/offline.json"
[[ ! -s "$fixture/discovery-events" ]]
jq -e '. == null' "$fixture/offline.json" >/dev/null
for mode in valid dispatch push pr dispatch-branch dispatch-workflow dispatch-repository \
    docs partial truncated source compiler unqualified unretained malformed failed; do
    CARGO_NET_OFFLINE=false BASELINE_TEST_MODE="$mode" bash scripts/release/validation-baseline.sh \
        "$baseline" "$fixture/discovery-$mode.json"
    if [[ $mode == valid || $mode == dispatch ]]; then
        jq -e --arg source "$baseline" '.source == $source and .run == 17' "$fixture/discovery-$mode.json" >/dev/null
    else jq -e '. == null' "$fixture/discovery-$mode.json" >/dev/null; fi
done
git remote set-url origin https://example.invalid/fixture.git
# Text-only changes exercise focused proof custody and exclude unrelated gates.
printf 'pub const FIXTURE: u8 = 1;\n' >> crates/ic-host-text/src/lib.rs
git add crates/ic-host-text/src/lib.rs
git commit -qm focused
RELEASE_SOURCE=$(git rev-parse HEAD) RELEASE_GROUP=text
make_plan focused-success
jq -e '.selection == "focused"' "$directory/context.json" >/dev/null
: > "$VALIDATION_TEST_EVENTS"
VALIDATION_TEST_FAIL=testkit-installed-check bash scripts/release/validation.sh run "$pointer" > "$fixture/focused.run.log" 2>&1
bash scripts/release/validation.sh receipt "$pointer" > "$fixture/focused.receipt"
jq -e --arg source "$RELEASE_SOURCE" '.kind == "group-validation-evidence" and
    .source == $source and .group == "text"' "$directory/shared-proof.json" >/dev/null
absent grep -E 'testkit-|^tools-check$|^clippy-workspace$|^tooling-command-check$' "$VALIDATION_TEST_EVENTS"
grep -Fx release-scope-archives "$VALIDATION_TEST_EVENTS" >/dev/null
# Every omitted target still has explicit not-applicable evidence.
run_directory=$(echo "$directory"/logs/validation.*)
grep $'^release-stage-testkit-installed-check\tnot-applicable\t' "$run_directory/plan-events.tsv" >/dev/null
cp "$run_directory/0.log" "$fixture/original.log"
printf 'tampered\n' >> "$run_directory/0.log"
refuse bash scripts/release/validation.sh receipt "$pointer" > "$fixture/tampered.receipt" 2>&1
cp "$fixture/original.log" "$run_directory/0.log"
cp "$directory/shared-proof.json" "$fixture/focused.shared-proof"
jq -cS '.source = "0000000000000000000000000000000000000000"' \
    "$fixture/focused.shared-proof" > "$directory/shared-proof.json"
refuse bash scripts/release/validation.sh receipt "$pointer" > "$fixture/wrong-proof.log" 2>&1
mv "$fixture/focused.shared-proof" "$directory/shared-proof.json"
# Missing/failed/blocked execution cannot produce a successful receipt.
cp "$run_directory/timings.tsv" "$fixture/timings"
: > "$run_directory/timings.tsv"
refuse bash scripts/release/validation.sh receipt "$pointer" > "$fixture/missing.log" 2>&1
cp "$fixture/timings" "$run_directory/timings.tsv"
make_plan focused-failure
: > "$VALIDATION_TEST_EVENTS"
refuse env VALIDATION_TEST_FAIL=release-scope-lint bash scripts/release/validation.sh run "$pointer" > "$fixture/failure.log" 2>&1
[[ ! -e "$directory/shared-proof.json" ]]
absent grep -Fx release-scope-tests "$VALIDATION_TEST_EVENTS"
refuse bash scripts/release/validation.sh receipt "$pointer" > "$fixture/failure-receipt.log" 2>&1
make_plan focused-drift
refuse env VALIDATION_TEST_DRIFT=release-scope-lint bash scripts/release/validation.sh run "$pointer" > "$fixture/drift.log" 2>&1
git restore -- crates/ic-host-text/src/lib.rs
# Source/policy/group changes cannot re-admit a saved plan.
make_plan focused-stale
refuse env RELEASE_GROUP=build bash scripts/release/validation.sh run "$pointer" > "$fixture/group.log" 2>&1
jq '.stages |= .[1:]' "$directory/plan.json" > "$fixture/truncated-plan"
cp "$fixture/truncated-plan" "$directory/plan.json"
refuse bash scripts/release/validation.sh run "$pointer" > "$fixture/omission.log" 2>&1
# Recovery uses the same baseline admission rule as discovery, including hosts,
# retained qualification, source identities and the compiler lane.
make_plan focused-baseline
jq '.jobs.jobs[0].steps = []' "$directory/baseline.json" > "$fixture/incomplete-baseline"
cp "$fixture/incomplete-baseline" "$directory/baseline.json"
refuse bash scripts/release/validation.sh run "$pointer" > "$fixture/baseline-recovery.log" 2>&1
# Shared changes and mixed-group edits broaden all declared groups to the original roster.
printf '\n# shared input\n' >> Makefile
git add Makefile
git commit -qm shared
RELEASE_SOURCE=$(git rev-parse HEAD)
# Observe the real CI dispatcher with substituted Make effects, rather than
# parsing recipes or treating coverage annotations as a dispatch roster.
: > "$fixture/expected"
VALIDATION_TEST_EVENTS="$fixture/expected" bash scripts/ci/workspace-validation.sh ci \
    > "$fixture/full-ci.log" 2>&1
all_groups=$(bash scripts/release/group.sh ci/cargo-release-groups.json --list)
for RELEASE_GROUP in $all_groups; do
    make_plan "full-$RELEASE_GROUP"
    jq -e '.selection == "full"' "$directory/context.json" >/dev/null
    jq -r '.stages[] | select(.required) | .target | sub("^release-stage-";"")' "$directory/plan.json" > "$fixture/selected"
    cmp "$fixture/selected" "$fixture/expected"
done
: > "$VALIDATION_TEST_EVENTS"
refuse env VALIDATION_TEST_FAIL=testkit-installed-check bash scripts/release/validation.sh run "$pointer" > "$fixture/full-failure.log" 2>&1
make_plan full-success
bash scripts/release/validation.sh run "$pointer" > "$fixture/full-success.log" 2>&1
bash scripts/release/validation.sh receipt "$pointer" > "$fixture/full.receipt"
for stage in fixture-retention fixture-evidence; do
    admission="$directory/stage-admissions/$stage.json"
    cp "$admission" "$fixture/stage-admission"
    jq '.source = "wrong-source"' "$fixture/stage-admission" > "$admission"
    refuse bash scripts/release/validation.sh receipt "$pointer" > "$fixture/stage-identity.log" 2>&1
    cp "$fixture/stage-admission" "$admission"
    execution=$(jq -r .execution_proof "$admission")
    stage_log=$(jq -r .logs.stdout.path "$execution")
    cp "$stage_log" "$fixture/stage-log"
    printf 'tampered\n' >> "$stage_log"
    refuse bash scripts/release/validation.sh receipt "$pointer" > "$fixture/stage-log.log" 2>&1
    cp "$fixture/stage-log" "$stage_log"
done
bash scripts/release/validation.sh receipt "$pointer" > "$fixture/full-restored.receipt"
cmp "$fixture/full.receipt" "$fixture/full-restored.receipt"
# New/omitted obligations refuse until their product coverage is explicit.
jq -cS '.stages += ["release-stage-new-obligation"]' ci/release-validation.json > "$fixture/new-stage"
cp "$fixture/new-stage" ci/release-validation.json
refuse perl scripts/release/select-validation.pl "$RELEASE_SOURCE" text \
    "$fixture/baseline.json" "$fixture/never-plan" "$fixture/never-context" > "$fixture/roster.log" 2>&1
git restore -- ci/release-validation.json
jq '.exclusions[0].targets = []' ci/release-validation-coverage.json > "$fixture/missing-coverage"
cp "$fixture/missing-coverage" ci/release-validation-coverage.json
refuse perl scripts/release/select-validation.pl "$RELEASE_SOURCE" text \
    "$fixture/baseline.json" "$fixture/never-plan" "$fixture/never-context" > "$fixture/missing-coverage.log" 2>&1
git restore -- ci/release-validation-coverage.json
# Unknown groups, absent full evidence and compiler/config overrides fail closed.
refuse env RELEASE_GROUP=unknown bash scripts/release/validation.sh run "$pointer" > "$fixture/unknown.log" 2>&1
# Use the original baseline plus a Text-only commit to check its reverse consumers.
git checkout -q "$baseline"
printf 'pub const FIXTURE: u8 = 2;\n' >> crates/ic-host-text/src/lib.rs
git add crates/ic-host-text/src/lib.rs
git commit -qm text
RELEASE_SOURCE=$(git rev-parse HEAD) RELEASE_GROUP=text
make_plan text-success
jq -e '.selection == "focused"' "$directory/context.json" >/dev/null
: > "$VALIDATION_TEST_EVENTS"
VALIDATION_TEST_FAIL=testkit-installed-check bash scripts/release/validation.sh run "$pointer" > "$fixture/text.run.log" 2>&1
bash scripts/release/validation.sh receipt "$pointer" > "$fixture/text.receipt"
grep -Fx release-scope-graphs "$VALIDATION_TEST_EVENTS" >/dev/null
grep -Fx release-scope-tests "$VALIDATION_TEST_EVENTS" >/dev/null
make_plan text-environment
# A changed environment cannot reinterpret already-selected evidence.
refuse env RUSTFLAGS=--cfg=unreviewed bash scripts/release/validation.sh run "$pointer" > "$fixture/environment.log" 2>&1
refuse env RUSTFLAGS=--cfg=unreviewed bash scripts/release/validation.sh fork "$pointer" "$fixture/changed.pointer" > "$fixture/fork-environment.log" 2>&1
# A release-owned version-only edit preserves scope; another owner's source does not.
fixture_version=$("$YQ" -p toml -o yaml '.package.version' crates/ic-host-text/Cargo.toml)
fixture_next_version=$(bash vendor/toko-tooling/scripts/ci/next-release-version.sh "$fixture_version" patch)
awk -v before="$fixture_version" -v after="$fixture_next_version" \
    '$0 == "version = \"" before "\"" { $0 = "version = \"" after "\"" } { print }' \
    crates/ic-host-text/Cargo.toml > "$fixture/version-manifest"
cp "$fixture/version-manifest" crates/ic-host-text/Cargo.toml
git add crates/ic-host-text/Cargo.toml
git commit -qm version-only
RELEASE_SOURCE=$(git rev-parse HEAD)
make_plan text-version
jq -e '.selection == "focused"' "$directory/context.json" >/dev/null
perl scripts/release/select-validation.pl "$RELEASE_SOURCE" "$RELEASE_GROUP" \
    "$directory/baseline.json" "$directory/plan.json" "$directory/context.json" --check > "$fixture/version-readmission.log"
printf '\n# other owner source\n' >> crates/ic-build/src/lib.rs
git add crates/ic-build/src/lib.rs
git commit -qm mixed
RELEASE_SOURCE=$(git rev-parse HEAD)
make_plan mixed-owners
jq -e '.selection == "full"' "$directory/context.json" >/dev/null
printf 'null\n' > "$fixture/baseline.json"
make_plan no-baseline
jq -e '.selection == "full"' "$directory/context.json" >/dev/null
# Linked worktrees keep checkpoint inputs inside their checkout, while receipts
# may live in the external Git state directory. No source or ref is delivered.
git worktree add -q --detach "$fixture/linked checkout" HEAD
(
    cd "$fixture/linked checkout"
    bash scripts/release/validation.sh plan "$fixture/linked.pointer" > "$fixture/linked.plan.log" 2>&1
    jq -e '.directory | startswith("target/release-validation/")' "$fixture/linked.pointer" >/dev/null
    bash scripts/release/validation.sh run "$fixture/linked.pointer" > "$fixture/linked.run.log" 2>&1
    bash scripts/release/validation.sh receipt "$fixture/linked.pointer" > "$fixture/linked.receipt"
)
fixture_complete=true
echo 'Group plans, full-roster equivalence, stage failures, source drift and receipt custody passed (Make effects substituted).'
