#!/usr/bin/env bash
set -euo pipefail
# Product contracts admitted only by a matching three-host comparison.
[[ $# == 1 ]] || exit 2
case ${RELEASE_GROUP:-} in
    native) package=ic-testkit-native ;;
    text) package=ic-host-text ;;
    tools) package=ic-host-tools ;;
    build) package=ic-build ;;
    *) echo 'focused release checks require native, text, tools or build' >&2; exit 2 ;;
esac
case "$1" in
    tools)
        if [[ $RELEASE_GROUP == build ]]; then
            make --no-print-directory tools-check
        else
            make --no-print-directory host-tools-check rust-tools-check release-editor-check
        fi ;;
    lint)
        packages=(-p "$package")
        if [[ $RELEASE_GROUP == text ]]; then packages+=(-p ic-host-tools); fi
        if [[ $RELEASE_GROUP != native ]]; then packages+=(-p ic-testkit-dev); fi
        cargo clippy "${packages[@]}" --all-targets --all-features --locked --offline -- -D warnings ;;
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
        fi
        if [[ $RELEASE_GROUP == tools ]]; then
            make --no-print-directory test-tools-default test-tools-interface \
                test-tools-candid test-tools-combined test-tools-agent test-tools-encoding \
                test-tools-registry test-tools-icp-project test-tools-icp-ledger test-tools-ledger-evidence
            cargo test -p ic-testkit-dev --test public_contracts --locked --offline
        fi
        if [[ $RELEASE_GROUP == build ]]; then
            cargo test -p ic-testkit-dev --lib --locked --offline
            cargo test -p ic-testkit-dev --bin check_packages --locked --offline
            cargo test -p ic-testkit-dev --test public_contracts --locked --offline
            server=$(target/debug/ic-testkit-server check)
            POCKET_IC_BIN="$server" cargo test -p ic-testkit-dev --test canister_benchmark --locked --offline
        fi ;;
    docs) RUSTDOCFLAGS='-D warnings' cargo doc -p "$package" --all-features --locked --offline --no-deps ;;
    archives) cargo run -p ic-testkit-dev --bin check_packages --locked --offline -- --package "$package" ;;
    *) exit 2 ;;
esac
