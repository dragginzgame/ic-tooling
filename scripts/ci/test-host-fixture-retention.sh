#!/usr/bin/env bash
set -euo pipefail

# Inject failures into disposable copies of the real Host fixture entrypoints.
# Stop before setup, Cargo, Git or publication substitutes can run.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
# shellcheck source=scripts/ci/fixture-retention-probes.sh
source "$root/scripts/ci/fixture-retention-probes.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/host-fixture-retention.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Host fixture retention evidence retained"' EXIT
export TMPDIR="${fixture%/*}"
# The self-probe sources this companion before installing its EXIT trap.
mkdir -p "$fixture/exit-source/scripts/ci"
cp "$root/scripts/ci/fixture-retention-probes.sh" "$fixture/exit-source/scripts/ci/"
inputs="$(bash "$root/scripts/ci/select-tooling-checks.sh" retention)"
while IFS= read -r input; do
    name="${input##*/}"
    directory_variable=fixture
    [[ "$input" != scripts/publish/test-workspace.sh ]] || directory_variable=FIXTURE_ROOT
    FIXTURE_PROBE_ROOT_VARIABLE=root FIXTURE_PROBE_DIRECTORY_VARIABLE="$directory_variable" \
        fixture_probe_exits "$root/$input" "$fixture" "$root" "$name" || exit 1
done <<< "$inputs"
# Exercise each real initialization/EXIT boundary with an aliased relative
# parent and a changed cwd. The fixture bodies own their separate product tests;
# path retention must not recursively run those entire suites again.
mkdir "$fixture/relative-parent"
ln -s "$fixture/relative-parent" "$fixture/relative-alias"
inputs="$(bash "$root/scripts/ci/select-tooling-checks.sh" relative)"
tooling="$(bash "$root/scripts/ci/select-tooling-checks.sh" tooling)"
while IFS= read -r input; do
    [[ $(printf '%s\n' "$tooling" | rg -Fxc -- "$input") == 1 ]] || {
        printf 'Relative-path fixture needs one explicit tooling entry: %s\n' "$input" >&2
        exit 1
    }
    directory_variable=fixture
    [[ "$input" != scripts/publish/test-workspace.sh ]] || directory_variable=FIXTURE_ROOT
    for expected in 0 23; do
        name="relative-${input##*/}-$expected"
        injection=$(cat <<'PROBE'
printf '%s\n' "$@DIRECTORY@" > "$RETENTION_EXIT_PATH"
printf 'retained evidence\n' > "$@DIRECTORY@/exit-evidence"
cd "$RETENTION_SOURCE_ROOT"
fixture_complete=true
exit "$RETENTION_STATUS"
PROBE
)
        injection=${injection//@DIRECTORY@/$directory_variable}
        fixture_probe_prefix "$root/$input" "$fixture/exit-source/scripts/ci/probe.sh" \
            root "$directory_variable" "$injection"
        status=0
        (cd "$fixture"; TMPDIR=relative-alias RETENTION_SOURCE_ROOT="$root" \
            RETENTION_EXIT_PATH="$fixture/$name.exit" RETENTION_STATUS="$expected" \
            bash "$fixture/exit-source/scripts/ci/probe.sh") > "$fixture/$name.log" 2>&1 || status=$?
        [[ "$status" == "$expected" && -f "$fixture/$name.exit" ]] || {
            fixture_replay_logs "$fixture/$name.log"; exit 1;
        }
        retained=$(cat "$fixture/$name.exit")
        [[ "$retained" == "$fixture/relative-parent/"* ]] || exit 1
        if [[ "$expected" == 0 ]]; then
            [[ ! -e "$retained" && ! -L "$retained" ]] || exit 1
        else
            [[ -d "$retained" && $(cat "$retained/exit-evidence") == 'retained evidence' ]] || exit 1
        fi
    done
done <<< "$inputs"
echo 'Host fixtures reject exit failures and premature completion, retaining evidence'
fixture_complete=true
