#!/usr/bin/env bash
# Shared companions: scripts/ci/test-workspace-validation.sh
set -euo pipefail
selected_case=all
if [[ $# != 0 ]]; then
    [[ $# == 2 && $1 == --case && ( $2 == tooling-roster || $2 == release-routing ) ]] || {
        echo 'usage: test-make-snapshot.sh [--case tooling-roster|release-routing]' >&2; exit 2;
    }
    selected_case="$2"
fi

# Qualify Host's checkout-local routing using its actual Makefile and snapshot.
# Formatter, tool setup/check and release effects use disposable substitutes.
root="$0"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
unset SHARED_TOOLING_ROOT
unset TOOLING_STAGE_ADMISSIONS IC_TOOLING_RELEASE_GROUP TOOLING_RELEASE_GROUP
unset RELEASE_GROUPS RELEASE_PUBLISH_TARGET RELEASE_SETUP_TARGET RELEASE_BATCH_ACTIVE
unset RELEASE_SETUP_MODE PUBLICATION_RESULT
# Exercise every formatter pass while Shared owns input-cache qualification.
export FORMAT_RUST_ARGS=--no-cache
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
# shellcheck source=scripts/ci/make-adapter-probes.sh
source "$root/scripts/ci/make-adapter-probes.sh"
fixture="$(tooling_fixture_create "$root" host-make-snapshot)"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Host Make evidence retained"' EXIT
export TMPDIR="${fixture%/*}"
mkdir -p "$fixture/local"
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture/local" \
    Makefile make/consumer.mk scripts/ci/check-pipe-jobserver.pl scripts/release/editor.sh
for input in Makefile make/testkit.mk ci/tool-versions.env ci/release-delivery.txt scripts/ci/check-format-tools.sh \
    ci/tooling-checks.tsv scripts/ci/select-tooling-checks.sh; do
    parent=.
    [[ "$input" != */* ]] || parent="${input%/*}"
    mkdir -p "$fixture/local/$parent"
    cp "$root/$input" "$fixture/local/$input"
done
cargo metadata --manifest-path "$root/Cargo.toml" --locked --offline --no-deps \
    --format-version 1 > "$fixture/source-metadata.json"
while IFS= read -r manifest; do
    case "$manifest" in "$root"/*) relative="${manifest#"$root"/}" ;; *) exit 1 ;; esac
    mkdir -p "$fixture/local/${relative%/*}"
    cp "$manifest" "$fixture/local/$relative"
done < <(jq -r '.packages[].manifest_path' "$fixture/source-metadata.json")
export HOST_MAKE_METADATA="$fixture/formatting-metadata.json"
jq --arg source "$root" --arg destination "$fixture/local" '
    .workspace_root = $destination |
    .packages[].manifest_path |= ($destination + .[($source | length):])
' "$fixture/source-metadata.json" > "$HOST_MAKE_METADATA"
format_packages=()
while IFS= read -r package; do format_packages+=(--package "$package"); done \
    < <(jq -r '.packages[].name' "$HOST_MAKE_METADATA")
export HOST_MAKE_EVENTS="$fixture/events" HOST_MAKE_EXTERNAL="$fixture/external-events"
export HOST_MAKE_JOBSERVER_PROBE="$fixture/local/scripts/ci/check-pipe-jobserver.pl"
mkdir -p "$fixture/external/scripts/ci"
for helper in check-make-execution.sh check-format-tools.sh run-release.sh; do
    cat > "$fixture/external/scripts/ci/$helper" <<'STUB'
#!/usr/bin/env bash
printf '%s\n' "$0" >> "$HOST_MAKE_EXTERNAL"
exit 23
STUB
done
cat > "$fixture/local/cargo" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
if [[ "${HOST_MAKE_VERIFY_JOBSERVER:-}" == 1 ]]; then
    perl "$HOST_MAKE_JOBSERVER_PROBE"
fi
if [[ "${HOST_MAKE_ASSERT_POCKET_IC:-0}" == 1 && "$1" == test ]]; then
    [[ "${POCKET_IC_BIN:-}" == '/prepared/pocket ic' ]] || exit 91
fi
case "$*" in
    '--version') printf 'cargo 1.99.1 (fixture)\n' ;;
    'sort --version') printf 'cargo-sort %s\n' "$HOST_MAKE_SORT_VERSION" ;;
    'fmt --version') echo rustfmt ;;
    'metadata --locked --offline --no-deps --format-version 1') cat "$HOST_MAKE_METADATA" ;;
    *)
        printf '%s\n' "$*" >> "$HOST_MAKE_EVENTS"
        if [[ "$1" == fixture ]]; then exit "${HOST_MAKE_FIXTURE_STATUS:-0}"; fi
        if [[ "$1" == fetch ]]; then exit "${HOST_MAKE_FETCH_STATUS:-0}"; fi
        if [[ "$1" == clippy && "$*" == *--workspace* ]]; then
            exit "${HOST_MAKE_CLIPPY_STATUS:-0}"
        fi
        if [[ "${HOST_MAKE_FAIL:-}" == sort && "$1" == sort ]]; then
            echo 'formatter stdout evidence'
            echo 'formatter stderr evidence' >&2
            exit 23
        fi
        ;;
esac
STUB
cat > "$fixture/local/scripts/ci/run-release.sh" <<'STUB'
#!/usr/bin/env bash
if [[ "${HOST_MAKE_ASSERT_RELEASE_GROUP:-0}" == 1 ]]; then [[ "${RELEASE_GROUP:-}" == testkit ]] || exit 94; fi
printf '%s\n' "$*" >> "$HOST_MAKE_EVENTS"
STUB
# Qualify only the consumer's dispatch here; actual hooks and lock isolation
# have their own product fixture and installed release evidence.
cat > "$fixture/local/scripts/ci/run-release-batch.pl" <<'STUB'
use strict;
use warnings;
open my $out, '>>', $ENV{HOST_MAKE_EVENTS} or die $!;
print {$out} 'frozen ', join(' ', @ARGV), "\n" or die $!;
close $out or die $!;
STUB
mkdir -p "$fixture/local/scripts/dev"
for helper in scripts/dev/install-git-hooks.sh scripts/ci/verify-shared-tooling-snapshot.sh; do
    cat > "$fixture/local/$helper" <<'STUB'
#!/usr/bin/env bash
printf '%s %s\n' "${0##*/}" "$*" >> "$HOST_MAKE_EVENTS"
STUB
done
chmod +x "$fixture/local/cargo"
mkdir -p "$fixture/local/.tools/host/bin"
ln -s "$root/.tools/host/bin/yq" "$fixture/local/.tools/host/bin/yq"
# shellcheck source=/dev/null
. "$root/ci/tool-versions.env"
export HOST_MAKE_SORT_VERSION="$SHARED_TOOLING_CARGO_SORT_VERSION"
cd "$fixture/local"

prepare_tooling_stubs() {
    local helpers helper
    helpers="$(bash "$root/scripts/ci/select-tooling-checks.sh" make-stub)"
    while IFS= read -r helper; do
        mkdir -p "$fixture/local/${helper%/*}"
        printf '#!/usr/bin/env bash\nexec cargo fixture %s "$@"\n' "$helper" > "$fixture/local/$helper"
    done <<< "$helpers"
}

# Frozen membership/order independently checks the production discovery roster.
check_tooling_roster() {
    local status
    : > "$HOST_MAKE_EVENTS"
    PATH="$fixture/local:$PATH" make --no-print-directory tooling-command-check > "$fixture/tooling-order.log" 2>&1
    awk '{ print $1, $2 }' "$HOST_MAKE_EVENTS" > "$fixture/tooling-observed"
    cat > "$fixture/expected" <<'EVENTS'
fixture scripts/ci/test-qualification-record.sh
fixture scripts/ci/test-tools-layering.sh
fixture scripts/ci/test-product-ci-selection.sh
fixture scripts/ci/test-product-candid-boundary.sh
fixture scripts/ci/test-product-fixture-scratch.sh
fixture scripts/ci/test-product-evidence.sh
fixture scripts/ci/test-host-fixture-retention.sh
fixture scripts/release/test-stage-reuse.sh
fixture scripts/ci/test-make-snapshot.sh
fixture scripts/release/test-adapter.sh
fixture scripts/release/test-validation.sh
fixture scripts/publish/test-workspace.sh
fixture scripts/ci/check-release-commands.sh
EVENTS
    cmp "$fixture/expected" "$fixture/tooling-observed"
    : > "$HOST_MAKE_EVENTS"
    status=0
    PATH="$fixture/local:$PATH" HOST_MAKE_FIXTURE_STATUS=23 make --no-print-directory tooling-command-check \
        > "$fixture/tooling-failure.log" 2>&1 || status=$?
    [[ "$status" == 2 ]] || exit 1
    printf '%s\n' 'fixture scripts/ci/test-qualification-record.sh' > "$fixture/expected"
    cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
    # Each release checks its actual adapter once; full CI keeps all groups.
    all_groups=$(bash scripts/release/group.sh ci/cargo-release-groups.json --list)
    for group in $all_groups; do
        : > "$HOST_MAKE_EVENTS"
        PATH="$fixture/local:$PATH" IC_TOOLING_FULL_CI=0 make --no-print-directory tooling-command-check \
            "TOOLING_RELEASE_GROUP=$group" > "$fixture/tooling-$group.log" 2>&1
        grep -Fx "fixture scripts/release/test-adapter.sh --group $group" "$HOST_MAKE_EVENTS" >/dev/null
        [[ $(grep -c '^fixture scripts/release/test-adapter.sh' "$HOST_MAKE_EVENTS") == 1 ]]
    done
    : > "$HOST_MAKE_EVENTS"
    PATH="$fixture/local:$PATH" IC_TOOLING_FULL_CI=1 make --no-print-directory tooling-command-check \
        TOOLING_RELEASE_GROUP=testkit > "$fixture/tooling-full-ci.log" 2>&1
    grep -Fx 'fixture scripts/release/test-adapter.sh' "$HOST_MAKE_EVENTS" >/dev/null
    if grep -F -- '--group' "$HOST_MAKE_EVENTS" >/dev/null; then exit 1; fi
    # Release routing uses the same named stages that the gate seals. Product
    # effects remain substituted here; test-stage-reuse.sh owns actual reuse.
    cat > scripts/release/reuse-fixture.sh <<'STUB'
#!/usr/bin/env bash
exec cargo fixture scripts/release/reuse-fixture.sh "$@"
STUB
    : > "$HOST_MAKE_EVENTS"
    PATH="$fixture/local:$PATH" IC_TOOLING_FULL_CI=0 make --no-print-directory tooling-command-check \
        TOOLING_RELEASE_GROUP=testkit TOOLING_STAGE_ADMISSIONS=admissions > "$fixture/tooling-reuse.log" 2>&1
    for stage in fixture-evidence fixture-retention; do
        grep -Fx "fixture scripts/release/reuse-fixture.sh testkit $stage" "$HOST_MAKE_EVENTS" >/dev/null
    done
    if grep -E '^fixture scripts/ci/test-(product-evidence|host-fixture-retention).sh$' "$HOST_MAKE_EVENTS" >/dev/null; then exit 1; fi
    : > "$HOST_MAKE_EVENTS"
    PATH="$fixture/local:$PATH" IC_TOOLING_FULL_CI=1 make --no-print-directory tooling-command-check \
        TOOLING_RELEASE_GROUP=testkit TOOLING_STAGE_ADMISSIONS=admissions > "$fixture/tooling-no-reuse.log" 2>&1
    if grep -F 'fixture scripts/release/reuse-fixture.sh' "$HOST_MAKE_EVENTS" >/dev/null; then exit 1; fi
    grep -Fx 'fixture scripts/ci/test-product-evidence.sh' "$HOST_MAKE_EVENTS" >/dev/null
    grep -Fx 'fixture scripts/ci/test-host-fixture-retention.sh' "$HOST_MAKE_EVENTS" >/dev/null
}

if [[ $selected_case == tooling-roster ]]; then
    prepare_tooling_stubs
    check_tooling_roster
    fixture_complete=true
    echo 'Explicit tooling fixture roster and first-failure stopping passed'
    exit 0
fi
printf 'override RELEASE_REMOTE := recursive\n' > overrides.mk
recursive_make="$(command -v make) --no-print-directory -f Makefile -f overrides.mk"
routing_targets=(release-patch release-minor release-major release-resume release-batch-status release-batch-resume release-batch-reconcile release-batch-retire)
if [[ $selected_case == all ]]; then
    routing_targets=(help fmt-check "${routing_targets[@]}" install-hooks shared-tooling-check)
fi
for selection in ordinary environment command nested recursive; do
    unset SHARED_TOOLING_ROOT
    remote=review
    command=(make --no-print-directory -j2)
    if [[ "$selection" == environment ]]; then
        export SHARED_TOOLING_ROOT="$fixture/external"
    elif [[ "$selection" != ordinary ]]; then
        command+=("SHARED_TOOLING_ROOT=$fixture/external")
    fi
    if [[ "$selection" == nested || "$selection" == recursive ]]; then
        # Keep Make's recursive command and automatic variable literal.
        # shellcheck disable=SC2016
        printf '%%:\n\t+$(MAKE) -C "%s" $@\n' "$PWD" > "$fixture/parent.mk"
        command+=(-f "$fixture/parent.mk")
    fi
    if [[ "$selection" == recursive ]]; then
        command+=("MAKE=$recursive_make")
        remote=recursive
    fi
    for target in "${routing_targets[@]}"; do
        : > "$HOST_MAKE_EVENTS"
        "${command[@]}" "$target" "FORMAT_CARGO=$PWD/cargo" \
            VERSION=0.9.5 RELEASE_REMOTE=review RELEASE_BRANCH=release-review \
            RELEASE_GROUP=testkit HOST_MAKE_ASSERT_RELEASE_GROUP=1 BATCH=batch-_fixture \
            > "$fixture/$selection-$target.log" 2>&1
        case "$target" in
            help) : > "$fixture/expected" ;;
            fmt-check) printf 'sort --workspace --grouped --check\nfmt %s -- --check\n' "${format_packages[*]}" > "$fixture/expected" ;;
            release-resume) printf 'resume 0.9.5 %s release-review\n' "$remote" > "$fixture/expected" ;;
            release-patch|release-minor|release-major) printf 'frozen --root %s --groups testkit --kind %s --remote %s --branch release-review --delivery direct --publish-target publish --setup-target release-candidate-setup\n' "$PWD" "${target#release-}" "$remote" > "$fixture/expected" ;;
            release-batch-*) printf 'frozen --root %s --%s batch-_fixture --delivery direct\n' "$PWD" "${target#release-batch-}" > "$fixture/expected" ;;
            install-hooks) printf 'install-git-hooks.sh --consumer %s\n' "$PWD" > "$fixture/expected" ;;
            shared-tooling-check) printf 'verify-shared-tooling-snapshot.sh --consumer %s --manifest .shared-tooling.snapshot\n' "$PWD" > "$fixture/expected" ;;
        esac
        cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
        if [[ "$target" == fmt-check ]]; then
            grep -E '^Manifest sorting \[\.\]\.\.\. ok \([0-9.]+s\)$' "$fixture/$selection-$target.log" >/dev/null
            grep -E '^Rust formatting \[\.\]\.\.\. ok \([0-9.]+s\)$' "$fixture/$selection-$target.log" >/dev/null
        fi
        [[ ! -e "$HOST_MAKE_EXTERNAL" ]] || exit 1
    done
done
unset SHARED_TOOLING_ROOT
# Unsupported release delivery refuses before adapter effects; unrelated goals
# remain available with the same environment selection.
: > "$HOST_MAKE_EVENTS"
status=0
make --no-print-directory release-version RELEASE_DELIVERY=pr \
    > "$fixture/unsupported-delivery.log" 2>&1 || status=$?
[[ "$status" == 2 && ! -s "$HOST_MAKE_EVENTS" ]] || exit 1
make --no-print-directory help RELEASE_DELIVERY=pr > "$fixture/unrelated-delivery.log" 2>&1
[[ ! -s "$HOST_MAKE_EVENTS" ]] || exit 1
if [[ $selected_case == release-routing ]]; then
    # A retired roster must refuse, rather than silently selecting one group.
    : > "$HOST_MAKE_EVENTS"
    if make --no-print-directory release-patch RELEASE_GROUP=testkit RELEASE_FROZEN_GROUPS=text,testkit \
        > "$fixture/retired-roster.log" 2>&1; then exit 1; fi
    grep -F 'Retired release settings: RELEASE_FROZEN_GROUPS' "$fixture/retired-roster.log" > /dev/null
    [[ ! -s $HOST_MAKE_EVENTS ]]
    # Clearing the roster cannot dispatch a selected group against live inputs.
    for kind in patch minor major; do
        : > "$HOST_MAKE_EVENTS"
        if make --no-print-directory "release-$kind" RELEASE_GROUP=testkit RELEASE_GROUPS= \
            > "$fixture/empty-roster-$kind.log" 2>&1; then exit 1; fi
        [[ ! -s $HOST_MAKE_EVENTS ]]
    done
    # The captured checkout must use the existing single-group state machine.
    cat > scripts/dev/install-host-tools.sh <<'STUB'
#!/usr/bin/env bash
if [[ " $* " == *' --check '* ]]; then
    printf 'host-check\n' >> "$HOST_MAKE_EVENTS"
else
    printf 'host-bootstrap\n' >> "$HOST_MAKE_EVENTS"
fi
exit "${HOST_MAKE_BOOTSTRAP_STATUS:-0}"
STUB
    for kind in patch minor major; do
        : > "$HOST_MAKE_EVENTS"
        make --no-print-directory "release-$kind" RELEASE_GROUP=testkit RELEASE_BATCH_ACTIVE=1 \
            RELEASE_REMOTE=review RELEASE_BRANCH=release-review > "$fixture/captured-$kind.log" 2>&1
        printf '%s review release-review\n' "$kind" > "$fixture/expected"
        cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
    done
    for mode in prepare check; do
        : > "$HOST_MAKE_EVENTS"
        make --no-print-directory release-candidate-setup RELEASE_GROUP=testkit \
            "RELEASE_SETUP_MODE=$mode" > "$fixture/setup-$mode.log" 2>&1
        event=host-bootstrap
        [[ $mode != check ]] || event=host-check
        printf '%s\n' "$event" > "$fixture/expected"
        cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
    done
    : > "$HOST_MAKE_EVENTS"
    if HOST_MAKE_BOOTSTRAP_STATUS=23 make --no-print-directory release-candidate-setup \
        RELEASE_GROUP=testkit RELEASE_SETUP_MODE=prepare > "$fixture/bootstrap-failure.log" 2>&1; then exit 1; fi
    printf 'host-bootstrap\n' > "$fixture/expected"
    cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
    : > "$HOST_MAKE_EVENTS"
    for target in release-batch-status release-batch-resume release-batch-reconcile release-batch-retire; do
        if make --no-print-directory "$target" > "$fixture/missing-$target.log" 2>&1; then exit 1; fi
    done
    if make --no-print-directory release-candidate-setup RELEASE_SETUP_MODE=invalid \
        > "$fixture/invalid-setup.log" 2>&1; then exit 1; fi
    [[ ! -s $HOST_MAKE_EVENTS ]]
    make --no-print-directory release-batch-resume BATCH=batch-_fixture PUBLICATION_RESULT=published \
        > "$fixture/reconciled-resume.log" 2>&1
    printf 'frozen --root %s --resume batch-_fixture --delivery direct --publication-result published\n' "$PWD" > "$fixture/expected"
    cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
    fixture_complete=true
    echo 'Frozen entrypoint, captured group routing and delivery refusal passed (runner effects substituted)'
    exit 0
fi
# Exercise the actual Host aggregate under parallel Make. Each stage must run
# once in shared order, and a failure must prevent every later stage.
mkdir -p "$fixture/local/scripts/dev"
for tool in host ic rust; do
    cat > "$fixture/local/scripts/dev/install-$tool-tools.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
name="${0##*/}"
mode=install
for argument in "$@"; do
    if [[ "$argument" == --check ]]; then mode=check; fi
    if [[ "$argument" == --preflight ]]; then mode=preflight; fi
done
if [[ "$*" == *'--package ic-build'* ]]; then
    [[ "$*" == *'--bin ic-build-release-metadata --profile debug'* ]] || exit 1
    name=install-editor-tools.sh
fi
if [[ "$name" == install-ic-tools.sh ]]; then
    [[ "$*" == *'--tool wasm-opt'* && "$*" != *'--tool ic-wasm'* ]] || exit 93
    if [[ "${HOST_MAKE_OPTIMIZER_PATH:-}" != '' ]]; then printf '%s\n' "$HOST_MAKE_OPTIMIZER_PATH"; fi
fi
printf '%s %s\n' "$name" "$mode" >> "$HOST_MAKE_EVENTS"
if [[ "${HOST_MAKE_TOOL_FAIL:-}" == "$name" && "${HOST_MAKE_TOOL_FAIL_MODE:-}" == "$mode" ]]; then exit 23; fi
STUB
done
mkdir -p "$fixture/local/target/debug"
cat > "$fixture/local/target/debug/ic-testkit-server" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
mode=install
if [[ "$1" == check ]]; then mode=check; printf '%s\n' '/prepared/pocket ic'; fi
printf 'install-server-tools.sh %s\n' "$mode" >> "$HOST_MAKE_EVENTS"
if [[ "${HOST_MAKE_TOOL_FAIL:-}" == install-server-tools.sh && "${HOST_MAKE_TOOL_FAIL_MODE:-}" == "$mode" ]]; then exit 23; fi
STUB
chmod +x "$fixture/local/target/debug/ic-testkit-server"
for mode in install check; do
    target=install-tools
    [[ "$mode" != check ]] || target=tools-check
    for failed in none host rust editor server; do
        : > "$HOST_MAKE_EVENTS"
        status=0
        PATH="$fixture/local:$PATH" HOST_MAKE_TOOL_FAIL="install-$failed-tools.sh" HOST_MAKE_TOOL_FAIL_MODE="$mode" make --no-print-directory -j4 "$target" "MAKE=$(command -v make)" \
            "SHARED_TOOLING_ROOT=$fixture/external" > "$fixture/$target-$failed.log" 2>&1 || status=$?
        if [[ "$failed" == none ]]; then [[ "$status" == 0 ]] || exit 1;
        else [[ "$status" == 2 ]] || exit 1; fi
        : > "$fixture/expected"
        if [[ "$mode" == install ]]; then
            printf '%s\n' 'install-rust-tools.sh preflight' >> "$fixture/expected"
        fi
        for tool in host rust; do
            printf 'install-%s-tools.sh %s\n' "$tool" "$mode" >> "$fixture/expected"
            [[ "$tool" != "$failed" ]] || break
        done
        if [[ "$failed" == none || "$failed" == editor || "$failed" == server ]]; then
            printf 'install-editor-tools.sh %s\n' "$mode" >> "$fixture/expected"
        fi
        if [[ "$failed" == none || "$failed" == server ]]; then
            if [[ "$mode" == install ]]; then
                printf '%s\n' 'fetch --locked' >> "$fixture/expected"
                printf '%s\n' 'build -p ic-testkit --features cli --locked --offline --bin ic-testkit-server' >> "$fixture/expected"
            fi
            printf 'install-server-tools.sh %s\n' "$mode" >> "$fixture/expected"
        fi
        cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
        [[ ! -e "$HOST_MAKE_EXTERNAL" ]] || exit 1
    done
done
# A fetch refusal stops explicit server setup before the offline build, even
# under parallel Make. The complete successful order is checked above.
: > "$HOST_MAKE_EVENTS"
status=0
PATH="$fixture/local:$PATH" HOST_MAKE_FETCH_STATUS=73 make --no-print-directory -j4 \
    install-server "MAKE=$(command -v make)" > "$fixture/server-fetch-failure.log" 2>&1 || status=$?
[[ "$status" == 2 ]] || exit 1
printf '%s\n' 'fetch --locked' > "$fixture/expected"
cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
# Exercise the actual Make shell/environment handoff, including a space in the admitted path.
PATH="$fixture/local:$PATH" HOST_MAKE_ASSERT_POCKET_IC=1 make --no-print-directory \
    "MAKE=$(command -v make)" testkit-installed-check > "$fixture/server-environment.log" 2>&1
# Exercise CI's final recipe with its selected CLI outside the default target path.
# All server/Cargo effects remain substituted; the command and quoting are real.
{
    printf 'ci-server-selection:\n'
    sed -n '/^[[:space:]][+]@server=.*cargo test --workspace/p' "$root/Makefile"
} > "$fixture/ci-server.mk"
mv "$fixture/local/target/debug/ic-testkit-server" "$fixture/server cli"
: > "$HOST_MAKE_EVENTS"
PATH="$fixture/local:$PATH" HOST_MAKE_ASSERT_POCKET_IC=1 make --no-print-directory \
    -f "$fixture/ci-server.mk" ci-server-selection "SERVER_CLI=$fixture/server cli" \
    > "$fixture/ci-server-selection.log" 2>&1
printf '%s\n' 'install-server-tools.sh check' \
    'test --workspace --all-targets --all-features --locked --offline' > "$fixture/expected"
cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
mv "$fixture/server cli" "$fixture/local/target/debug/ic-testkit-server"
# Keep the actual Host LOC caller, argument selection and failure propagation
# qualified without replaying Shared's synthetic workspace/parser matrix.
cat > "$fixture/local/scripts/dev/cloc.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf 'cloc' >> "$HOST_MAKE_EVENTS"
printf ' <%s>' "$@" >> "$HOST_MAKE_EVENTS"
printf '\n' >> "$HOST_MAKE_EVENTS"
exit "${HOST_MAKE_CLOC_STATUS:-0}"
STUB
for selection in default manifest; do
    options=(--no-print-directory -j4 cloc)
    [[ "$selection" != manifest ]] || options+=(CLOC_MANIFEST='apps/selected app/Cargo.toml')
    for result in 0 23; do
        : > "$HOST_MAKE_EVENTS"
        status=0
        HOST_MAKE_CLOC_STATUS="$result" make "${options[@]}" \
            > "$fixture/cloc-$selection-$result.log" 2>&1 || status=$?
        if [[ "$result" == 0 ]]; then [[ "$status" == 0 ]] || exit 1;
        else [[ "$status" == 2 ]] || exit 1; fi
        if [[ "$selection" == default ]]; then
            printf 'cloc <%s>\n' "$PWD" > "$fixture/expected"
        else
            printf 'cloc <--manifest> <apps/selected app/Cargo.toml> <%s>\n' "$PWD" > "$fixture/expected"
        fi
        cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
    done
done
# Rust preflight must finish before either common installation stage may run.
: > "$HOST_MAKE_EVENTS"
status=0
HOST_MAKE_TOOL_FAIL=install-rust-tools.sh HOST_MAKE_TOOL_FAIL_MODE=preflight \
    make --no-print-directory -j4 install-tools > "$fixture/preflight-rust.log" 2>&1 || status=$?
[[ "$status" == 2 ]] || exit 1
printf '%s\n' 'install-rust-tools.sh preflight' > "$fixture/expected"
cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
[[ ! -e "$HOST_MAKE_EXTERNAL" ]] || exit 1
# Real pipe-descriptor admission in substituted Cargo, without compiling.
prepare_tooling_stubs
mkdir -p "$fixture/local/scripts/publish"
cat > "$fixture/local/scripts/publish/workspace.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == publish || "$1" == check || "$1" == qualify ]] || exit 1
printf '%s\0' "$@" > "$HOST_MAKE_EXTERNAL.publication-args"
mode="$1"
[[ "$mode" != qualify ]] || mode=check
if [[ "${HOST_MAKE_PUBLICATION_LINT:-0}" == 1 ]]; then
    make --no-print-directory clippy-workspace
    cargo "$mode"
    exit "${HOST_MAKE_PUBLICATION_STATUS:-0}"
fi
exec cargo "$mode"
STUB
# Public effects require an explicit group before any runner/helper is reached.
for target in release-patch release-minor release-major release-resume publish publish-check; do
    for invalid_group in '' unknown 'foundation integration'; do
        : > "$HOST_MAKE_EVENTS"
        rm -f "$HOST_MAKE_EXTERNAL.publication-args"
        if PATH="$fixture/local:$PATH" make --no-print-directory "$target" \
            "RELEASE_GROUP=$invalid_group" PACKAGE=ic-build VERSION=0.1.1 > "$fixture/invalid-group.log" 2>&1; then exit 1; fi
        [[ ! -s "$HOST_MAKE_EVENTS" && ! -e "$HOST_MAKE_EXTERNAL.publication-args" ]]
    done
    : > "$HOST_MAKE_EVENTS"
    if (unset RELEASE_GROUP; PATH="$fixture/local:$PATH" make --no-print-directory "$target" \
        PACKAGE=ic-build VERSION=0.1.1) > "$fixture/missing-group.log" 2>&1; then exit 1; fi
    [[ ! -s "$HOST_MAKE_EVENTS" && ! -e "$HOST_MAKE_EXTERNAL.publication-args" ]]
done
# Only command-line PACKAGE requests a batch; values cross the shell literally.
for target in publish publish-check; do
    mode=publish
    [[ "$target" != publish-check ]] || mode=check
    PATH="$fixture/local:$PATH" make --no-print-directory "$target" RELEASE_GROUP=foundation > "$fixture/publication-default.log" 2>&1
    printf '%s\0' "$mode" > "$fixture/publication-expected"
    cmp "$fixture/publication-expected" "$HOST_MAKE_EXTERNAL.publication-args"
    PATH="$fixture/local:$PATH" RELEASE_GROUP=foundation PACKAGE=ic-testkit make --no-print-directory "$target" > "$fixture/publication-environment.log" 2>&1
    cmp "$fixture/publication-expected" "$HOST_MAKE_EXTERNAL.publication-args"
    for selection in 'ic-host-artifacts ic-host-fs' '' 'ic-host-artifacts; touch injected'; do
        PATH="$fixture/local:$PATH" make --no-print-directory "$target" RELEASE_GROUP=foundation "PACKAGE=$selection" > "$fixture/publication-selected.log" 2>&1
        printf '%s\0' "$mode" --packages "$selection" > "$fixture/publication-expected"
        cmp "$fixture/publication-expected" "$HOST_MAKE_EXTERNAL.publication-args"
        [[ ! -e injected ]] || exit 1
    done
done
check_tooling_roster
jobserver_options=(--no-print-directory -j4)
if make --help | grep -q -- --jobserver-style; then
    jobserver_options+=(--jobserver-style=pipe)
fi
cargo_targets=(check clippy clippy-workspace docs-check test test-artifacts-minimal check-fs-non-unix
    test-tools-default test-tools-interface
    test-wasm-inspection)
helper_targets=(publish publish-check dependency-pins-check
    release-adapter-check tooling-command-check publish-command-check)
# This fixture has substituted Cargo metadata and no product source tree.
# The Candid consumer control owns the real scan and rejection ordering;
# here retain its Make prerequisite while qualifying the metadata jobserver.
printf '#!/usr/bin/env bash\nexit 0\n' > scripts/ci/check-product-candid-boundary.sh
for target in "${cargo_targets[@]}" "${helper_targets[@]}"; do
    : > "$HOST_MAKE_EVENTS"
    PATH="$fixture/local:$PATH" HOST_MAKE_VERIFY_JOBSERVER=1 make "${jobserver_options[@]}" "MAKE=$(command -v make)" "$target" RELEASE_GROUP=foundation \
        > "$fixture/jobserver-$target.log" 2>&1
    [[ -s "$HOST_MAKE_EVENTS" ]] || exit 1
done
# The focused fixture owns actual CI dispatch, lint/publication order and jobserver checks.
bash "$root/scripts/ci/test-workspace-validation.sh"
# One pinned compiler covers native, isolated-feature and Wasm obligations.
: > "$HOST_MAKE_EVENTS"
PATH="$fixture/local:$PATH" make --no-print-directory compiler-check > "$fixture/compiler-check.log" 2>&1
# Execute the distinct graph/feature obligations formerly owned by the second compiler lane.
: > "$HOST_MAKE_EVENTS"
PATH="$fixture/local:$PATH" make --no-print-directory "MAKE=$(command -v make)" testkit-wasm-check check-fs-non-unix isolated-features-check > "$fixture/compiler-profiles.log" 2>&1
cat > "$fixture/expected" <<'EVENTS'
check -p ic-testkit --features ic-testkit/held-reply-fixtures --locked --offline --target wasm32-unknown-unknown
check -p ic-testkit-perf-probe --locked --offline --target wasm32-unknown-unknown
check -p ic-host-fs --lib --bins --test unsupported_lock --all-features --target wasm32-unknown-unknown --locked --offline
test -p ic-host-tools --no-default-features --features icp-project --all-targets --locked --offline icp_project::
test -p ic-host-tools --no-default-features --features icp-ledger --all-targets --locked --offline icp_ledger::
test -p ic-host-tools --no-default-features --features ledger-evidence --all-targets --locked --offline ledger_evidence::
test -p ic-host-text --no-default-features --features yaml --lib --locked --offline yaml::
EVENTS
cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
# Actual Host formatting stops after sorter failure and points at complete logs.
for target in fmt fmt-check; do
    : > "$HOST_MAKE_EVENTS"
    status=0
    HOST_MAKE_FAIL=sort RUNNER_TEMP="$fixture" make --no-print-directory "$target" \
        "FORMAT_CARGO=$PWD/cargo" > "$fixture/$target-failure.log" 2>&1 || status=$?
    [[ "$status" == 2 ]] || exit 1
    grep -E '^Manifest sorting \[\.\]\.\.\. FAILED \(exit 23, [0-9.]+s\)$' "$fixture/$target-failure.log" >/dev/null
    detail="$(sed -n 's/^Details: //p' "$fixture/$target-failure.log")"
    [[ -f "$detail" ]] || exit 1
    printf 'formatter stdout evidence\nformatter stderr evidence\n' > "$fixture/expected-log"
    cmp "$fixture/expected-log" "$detail"
    [[ "$(wc -l < "$HOST_MAKE_EVENTS")" -eq 1 ]] || exit 1
done
# Qualify every actual Host target; Shared owns flag aliases and combinations.
make_probe_target_admission "$(command -v make)" "$PWD/Makefile" "$fixture/local" \
    "$HOST_MAKE_EVENTS" fmt fmt-check release-patch release-minor release-major release-resume \
    "${cargo_targets[@]}" "${helper_targets[@]}"
[[ ! -e "$HOST_MAKE_EXTERNAL" ]] || exit 1
# Exercise the real documentation Make entrypoint with a complete minimal corpus.
# Runtime companion docs can link to other Shared guides outside this fixture's
# closure; the checkout's complete documentation has its own link check.
rm -rf docs
cp "$root/scripts/ci/check-product-doc-links.sh" "$root/scripts/ci/check-documentation-links.pl" scripts/ci/
git init -q
mkdir -p docs/build docs/testkit docs/design testing/browser crates/example target
printf 'target/\n' > .gitignore
printf '[ignored](missing)\n' > target/generated.md
for document in docs/build/guide.md docs/testkit/guide.md docs/design/tooling-consolidation.md \
    testing/browser/README.md crates/example/README.md; do
    printf '# Guide\n' > "$document"
done
make --no-print-directory check-doc-links > "$fixture/docs-pass.log" 2>&1
for document in docs/build/guide.md docs/testkit/guide.md docs/design/tooling-consolidation.md \
    testing/browser/README.md crates/example/README.md; do
    printf '[broken](missing-audit-target)\n' > "$document"
    if make --no-print-directory check-doc-links > "$fixture/docs-failure.log" 2>&1; then exit 1; fi
    grep -F "$document: missing local target missing-audit-target" "$fixture/docs-failure.log" >/dev/null
    printf '# Guide\n' > "$document"
done
# Qualify the optimizer's actual Make and workflow callers with no installations.
: > "$HOST_MAKE_EVENTS"
make --no-print-directory install-ic-tools ic-tools-check > "$fixture/optimizer-tools.log" 2>&1
printf 'install-ic-tools.sh install\ninstall-ic-tools.sh check\n' > "$fixture/expected"
cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
yq -o json '.' "$root/.github/workflows/build-optimizer.yml" | jq -er \
    '.jobs["wasm-optimizer"].steps[] | select(.name == "Qualify native artifact transforms") | .run' > "$fixture/optimizer-step"
cat > scripts/ci/qualify-wasm-opt.sh <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == "$HOST_MAKE_OPTIMIZER_PATH/wasm-opt" && "$2" == "$RUNNER_TEMP/wasm-qualification/optimizer" ]] || exit 94
printf 'qualify optimizer\n' >> "$HOST_MAKE_EVENTS"
STUB
: > "$HOST_MAKE_EVENTS"
HOST_MAKE_OPTIMIZER_PATH="$fixture/prepared optimizer" RUNNER_TEMP="$fixture" \
    bash -e -o pipefail "$fixture/optimizer-step"
printf 'install-ic-tools.sh check\nqualify optimizer\n' > "$fixture/expected"
cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
# Execute native CI's compiler setup: rustup's implicit-toolchain install reads
# rust-toolchain.toml and does not add the command-line target to that selection.
yq -o json '.' "$root/.github/workflows/ci.yml" | jq -er \
    '.jobs.native.steps[] | select(.name == "Prepare declared development compiler and Wasm target") | .run' > "$fixture/compiler-step"
cat > "$fixture/local/rustup" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> "$HOST_MAKE_EVENTS"
if [[ "$1" == toolchain ]]; then exit "${HOST_MAKE_COMPILER_STATUS:-0}"; fi
if [[ "$1" == target ]]; then exit "${HOST_MAKE_TARGET_STATUS:-0}"; fi
exit 98
STUB
chmod +x "$fixture/local/rustup"
for failed_step in none compiler target; do
    : > "$HOST_MAKE_EVENTS"
    compiler_status=0 target_status=0 expected_status=0
    if [[ "$failed_step" == compiler ]]; then compiler_status=37 expected_status=37; fi
    if [[ "$failed_step" == target ]]; then target_status=47 expected_status=47; fi
    status=0
    PATH="$fixture/local:$PATH" HOST_MAKE_COMPILER_STATUS="$compiler_status" \
        HOST_MAKE_TARGET_STATUS="$target_status" bash -e -o pipefail "$fixture/compiler-step" || status=$?
    [[ "$status" == "$expected_status" ]] || exit 1
    printf 'toolchain install --profile minimal --no-self-update\n' > "$fixture/expected"
    if [[ "$failed_step" != compiler ]]; then
        printf 'target add wasm32-unknown-unknown\n' >> "$fixture/expected"
    fi
    cmp "$fixture/expected" "$HOST_MAKE_EVENTS"
done
echo 'Host Make snapshot routing and execution admission passed (substitute effects)'
fixture_complete=true
