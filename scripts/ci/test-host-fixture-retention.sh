#!/usr/bin/env bash
set -euo pipefail

# Inject failures into disposable copies of the real Host fixture entrypoints.
# Stop before setup, Cargo, Git or publication substitutes can run.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
# shellcheck source=vendor/toko-tooling/scripts/ci/fixture-retention-probes.sh
source "$root/vendor/toko-tooling/scripts/ci/fixture-retention-probes.sh"
fixture="$(tooling_fixture_create "$root" host-fixture-retention)"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Host fixture retention evidence retained"' EXIT
export TMPDIR="${fixture%/*}"
# Exercise actual product initialization/EXIT boundaries; generic lifecycle
# failure matrices belong to Toko Tooling. Use its prefix renderer directly,
# preserving current consumer paths rather than synthesizing the retired layout.
mkdir -p "$fixture/exit-source/scripts/ci"
mkdir "$fixture/relative-parent"
ln -s "$fixture/relative-parent" "$fixture/relative-alias"
inputs="$({ bash "$root/scripts/ci/select-tooling-checks.sh" retention;
    bash "$root/scripts/ci/select-tooling-checks.sh" relative; } | LC_ALL=C sort -u)"
while IFS= read -r input; do
    directory_variable=fixture
    [[ "$input" != scripts/publish/test-workspace.sh ]] || directory_variable=FIXTURE_ROOT
    for outcome in incomplete success failure; do
        expected=1
        exit_status=0
        completion=''
        if [[ $outcome == success ]]; then expected=0; completion='fixture_complete=true'; fi
        if [[ $outcome == failure ]]; then expected=23; exit_status=23; completion='fixture_complete=true'; fi
        name="relative-${input##*/}-$outcome"
        injection=$(cat <<'PROBE'
printf '%s\n' "$@DIRECTORY@" > "$RETENTION_EXIT_PATH"
printf 'retained evidence\n' > "$@DIRECTORY@/exit-evidence"
cd "$RETENTION_SOURCE_ROOT"
@COMPLETION@
exit "$RETENTION_STATUS"
PROBE
)
        injection=${injection//@DIRECTORY@/$directory_variable}
        injection=${injection//@COMPLETION@/$completion}
        fixture_probe_prefix "$root/$input" "$fixture/exit-source/scripts/ci/probe.sh" \
            root "$directory_variable" "$injection"
        status=0
        (cd "$fixture"; TMPDIR=relative-alias RETENTION_SOURCE_ROOT="$root" \
            RETENTION_EXIT_PATH="$fixture/$name.exit" RETENTION_STATUS="$exit_status" \
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
echo 'Host fixtures reject premature completion and retain failures across aliased scratch and cwd changes'
fixture_complete=true
