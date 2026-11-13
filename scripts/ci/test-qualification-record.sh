#!/usr/bin/env bash
# Shared companions: scripts/ci/record-qualification.sh Cargo.toml rust-toolchain.toml
set -euo pipefail
unset CARGO RUSTC
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture="$(tooling_fixture_create "$root" qualification-record)"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Qualification record evidence retained"' EXIT
mkdir "$fixture/repository" "$fixture/bin"
printf 'version = 4\n' > "$fixture/repository/Cargo.lock"
cp "$root/Cargo.toml" "$root/rust-toolchain.toml" "$fixture/repository/"
git -C "$fixture/repository" init -q
git -C "$fixture/repository" add Cargo.lock Cargo.toml rust-toolchain.toml
git -C "$fixture/repository" -c user.name=Fixture -c user.email=fixture@example.invalid -c commit.gpgsign=false commit -qm fixture
for tool in rustc cargo uname sw_vers; do
    cat > "$fixture/bin/$tool" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
case "${0##*/}" in
    rustc) printf 'rustc fixture\nrelease: %s\nhost: fixture-host\n' "${QUALIFICATION_COMPILER:-1.100.0}"; exit "${QUALIFICATION_COMPILER_STATUS:-0}" ;;
    cargo)
        printf 'cargo %s (fixture)\n' "${QUALIFICATION_CARGO_VERSION:-1.100.0}"
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
bash "$root/scripts/ci/record-qualification.sh" "$fixture/repository" "$fixture/native" native
cmp "$fixture/repository/Cargo.lock" "$fixture/native/Cargo.lock"
[[ "$(cat "$fixture/native/completion")" == complete ]]
[[ "$(cat "$fixture/native/source-revision")" == "$(git -C "$fixture/repository" rev-parse HEAD)" ]]
QUALIFICATION_OS=Darwin bash "$root/scripts/ci/record-qualification.sh" "$fixture/repository" "$fixture/apple" native
[[ "$(cat "$fixture/apple/os-version")" == 15.6 ]]
cp "$fixture/repository/rust-toolchain.toml" "$fixture/toolchain-original"
for invalid_pin in stable 1.99.1; do
    printf '[toolchain]\nchannel = "%s"\n' "$invalid_pin" > "$fixture/repository/rust-toolchain.toml"
    mkdir "$fixture/pin-$invalid_pin"
    if QUALIFICATION_ADD_SOURCE=true bash "$root/scripts/ci/check-rust-toolchain.sh" \
        "$fixture/repository" "$fixture/pin-$invalid_pin" > "$fixture/pin-$invalid_pin.log" 2>&1; then exit 1; fi
    [[ ! -e "$fixture/repository/unexpected-source" && ! -e "$fixture/pin-$invalid_pin/compiler-version" ]]
done
cp "$fixture/toolchain-original" "$fixture/repository/rust-toolchain.toml"
for refusal in observer floor cargo occupied new-source dirty; do
    output="$fixture/refuse-$refusal"
    case "$refusal" in
        observer) export QUALIFICATION_COMPILER_STATUS=23 ;;
        floor) export QUALIFICATION_COMPILER=1.99.1 ;;
        cargo) export QUALIFICATION_CARGO_VERSION=1.99.1 ;;
        occupied) mkdir "$output"; printf retained > "$output/retained" ;;
        new-source) export QUALIFICATION_ADD_SOURCE=true ;;
        dirty) printf changed >> "$fixture/repository/Cargo.lock" ;;
    esac
    if bash "$root/scripts/ci/record-qualification.sh" "$fixture/repository" "$output" native > "$fixture/$refusal.log" 2>&1; then exit 1; fi
    [[ ! -e "$output/completion" ]]
    unset QUALIFICATION_COMPILER_STATUS QUALIFICATION_COMPILER QUALIFICATION_CARGO_VERSION QUALIFICATION_ADD_SOURCE
    if [[ "$refusal" == occupied ]]; then [[ "$(cat "$output/retained")" == retained ]]; fi
    if [[ "$refusal" == new-source ]]; then
        [[ "$(cat "$fixture/repository/unexpected-source")" == 'new source' ]]
        rm -- "$fixture/repository/unexpected-source"
    fi
done
fixture_complete=true
echo 'Qualification records preserve source/lock/host evidence and refuse partial or mismatched results'
