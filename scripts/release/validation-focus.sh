#!/usr/bin/env bash
set -euo pipefail
# Product contracts for the two proven independent scopes; no installation or cleanup.
[[ $# == 1 ]] || exit 2
case ${RELEASE_GROUP:-} in
    native) package=ic-testkit-native ;;
    text) package=ic-host-text ;;
    *) echo 'focused release checks require native or text' >&2; exit 2 ;;
esac
case "$1" in
    lint)
        if [[ $RELEASE_GROUP == native ]]; then
            cargo clippy -p "$package" --all-targets --all-features --locked --offline -- -D warnings
        else
            cargo clippy -p "$package" -p ic-host-tools -p ic-testkit-dev --all-targets --all-features --locked --offline -- -D warnings
        fi ;;
    graphs)
        if [[ $RELEASE_GROUP == native ]]; then
            host=$(rustc -vV | sed -n 's/^host: //p')
            [[ -n $host ]]
            for target in "$host" wasm32-unknown-unknown; do
                perl scripts/ci/check-cargo-graph.pl --consumer "$PWD/" \
                    --package "$package" --target "$target" --all-features \
                    --require ic-testkit-native=1 --forbid-pattern '^(?!ic-testkit-native$)'
            done
            cargo check -p "$package" --all-features --locked --offline --target wasm32-unknown-unknown
        else
            bash scripts/ci/check-tools-features.sh
        fi ;;
    tests)
        cargo test -p "$package" --all-targets --all-features --locked --offline
        if [[ $RELEASE_GROUP == text ]]; then
            cargo test -p "$package" --no-default-features --lib --locked --offline
            make --no-print-directory test-text-yaml test-tools-icp-project
            cargo test -p ic-testkit-dev --lib --locked --offline
            cargo test -p ic-testkit-dev --bin check_packages --locked --offline
        fi ;;
    docs) RUSTDOCFLAGS='-D warnings' cargo doc -p "$package" --all-features --locked --offline --no-deps ;;
    archives) cargo run -p ic-testkit-dev --bin check_packages --locked --offline -- --package "$package" ;;
    *) exit 2 ;;
esac
