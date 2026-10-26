#!/usr/bin/env bash
set -euo pipefail
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
fixture="$(fixture_create "${TMPDIR:-/tmp}/qualification-record.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Qualification record evidence retained"' EXIT
mkdir "$fixture/repository" "$fixture/bin"
printf 'version = 4\n' > "$fixture/repository/Cargo.lock"
git -C "$fixture/repository" init -q
git -C "$fixture/repository" add Cargo.lock
git -C "$fixture/repository" -c user.name=Fixture -c user.email=fixture@example.invalid -c commit.gpgsign=false commit -qm fixture
for tool in rustc cargo uname sw_vers; do
    cat > "$fixture/bin/$tool" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
case "${0##*/}" in
    rustc) printf 'rustc fixture\nrelease: %s\nhost: fixture-host\n' "${QUALIFICATION_COMPILER:-1.91.0}"; exit "${QUALIFICATION_COMPILER_STATUS:-0}" ;;
    cargo)
        printf 'cargo fixture\n'
        if [[ "${QUALIFICATION_ADD_SOURCE:-false}" == true ]]; then
            printf 'new source\n' > unexpected-source
        fi ;;
    uname)
        case "$1" in
            -s) printf '%s\n' "${QUALIFICATION_OS:-Linux}" ;;
            -m) printf 'fixture-architecture\n' ;;
            -r) printf 'fixture-kernel\n' ;;
            *) exit 2 ;;
        esac ;;
    sw_vers) printf '15.6\n' ;;
esac
STUB
    chmod +x "$fixture/bin/$tool"
done
export PATH="$fixture/bin:$PATH"
for profile in native msrv; do
    bash "$root/scripts/ci/record-qualification.sh" "$fixture/repository" "$fixture/$profile" "$profile"
    cmp "$fixture/repository/Cargo.lock" "$fixture/$profile/Cargo.lock"
    [[ "$(cat "$fixture/$profile/completion")" == complete ]]
    [[ "$(cat "$fixture/$profile/source-revision")" == "$(git -C "$fixture/repository" rev-parse HEAD)" ]]
done
QUALIFICATION_OS=Darwin bash "$root/scripts/ci/record-qualification.sh" "$fixture/repository" "$fixture/apple" native
[[ "$(cat "$fixture/apple/os-version")" == 15.6 ]]
for refusal in observer floor occupied new-source dirty; do
    output="$fixture/refuse-$refusal"
    case "$refusal" in
        observer) export QUALIFICATION_COMPILER_STATUS=23 ;;
        floor) export QUALIFICATION_COMPILER=1.99.1 ;;
        occupied) mkdir "$output"; printf retained > "$output/retained" ;;
        new-source) export QUALIFICATION_ADD_SOURCE=true ;;
        dirty) printf changed >> "$fixture/repository/Cargo.lock" ;;
    esac
    if bash "$root/scripts/ci/record-qualification.sh" "$fixture/repository" "$output" msrv > "$fixture/$refusal.log" 2>&1; then exit 1; fi
    [[ ! -e "$output/completion" ]]
    unset QUALIFICATION_COMPILER_STATUS QUALIFICATION_COMPILER QUALIFICATION_ADD_SOURCE
    if [[ "$refusal" == occupied ]]; then [[ "$(cat "$output/retained")" == retained ]]; fi
    if [[ "$refusal" == new-source ]]; then
        [[ "$(cat "$fixture/repository/unexpected-source")" == 'new source' ]]
        rm -- "$fixture/repository/unexpected-source"
    fi
done
fixture_complete=true
echo 'Qualification records preserve source/lock/host evidence and refuse partial or mismatched results'
