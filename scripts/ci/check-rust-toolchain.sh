#!/usr/bin/env bash
set -euo pipefail
export RUSTUP_AUTO_INSTALL=0
[[ $# -le 2 ]] || { echo 'usage: check-rust-toolchain.sh [REPOSITORY [EVIDENCE-DIRECTORY]]' >&2; exit 2; }
root="${1:-${BASH_SOURCE[0]%/*}/../..}"
evidence="${2:-}"
fail() { printf 'compiler admission refused: %s\n' "$1" >&2; exit 1; }
version="$("${YQ:-yq}" -r -p toml -o yaml '.workspace.package."rust-version"' "$root/Cargo.toml")"
channel="$("${YQ:-yq}" -r -p toml -o yaml '.toolchain.channel' "$root/rust-toolchain.toml")"
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && "$channel" == "$version" ]] ||
    fail 'workspace minimum and pinned compiler must be the same exact stable version'
rustc_identity="$("${RUSTC:-rustc}" -vV)" || fail 'rustc identity observation failed'
cargo_identity="$("${CARGO:-cargo}" --version)" || fail 'Cargo identity observation failed'
grep -Fx "release: $version" <<< "$rustc_identity" > /dev/null || fail "rustc must be $version"
[[ "$cargo_identity" == "cargo $version "* ]] || fail "Cargo must be $version"
if [[ -n "$evidence" ]]; then
    [[ -d "$evidence" && ! -L "$evidence" ]] || fail 'expected a caller-owned evidence directory'
    printf '%s\n' "$rustc_identity" > "$evidence/rustc.txt"
    printf '%s\n' "$cargo_identity" > "$evidence/cargo.txt"
    printf '%s\n' "$version" > "$evidence/compiler-version"
    cp "$root/rust-toolchain.toml" "$evidence/rust-toolchain.toml"
fi
printf '%s\n' "$rustc_identity" "$cargo_identity" "Compiler policy passed: minimum and pin $version."
