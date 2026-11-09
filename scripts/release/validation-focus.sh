#!/usr/bin/env bash
set -euo pipefail
# Product contracts admitted only by a matching three-host comparison.
[[ $# == 1 ]] || exit 2
package=$(jq -er --arg group "${RELEASE_GROUP:-}" \
    --slurpfile coverage ci/release-validation-coverage.json '
    .groups[] | select(.name == $group) |
    select($coverage[0].focused | index($group)) | .primary
' ci/cargo-release-groups.json)
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
        cargo clippy "${packages[@]}" --all-targets --all-features --locked --offline -- -D warnings
        cargo clippy --manifest-path qualification/Cargo.toml -p ic-testkit-dev --all-targets --all-features --locked --offline -- -D warnings ;;
    graphs) bash scripts/ci/check-tools-features.sh ;;
    tests)
        cargo test -p "$package" --all-targets --all-features --locked --offline
        if [[ $RELEASE_GROUP == text ]]; then
            cargo test -p "$package" --no-default-features --lib --locked --offline
            make --no-print-directory test-text-yaml test-tools-icp-project
            cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline
        fi
        if [[ $RELEASE_GROUP == tools ]]; then
            make --no-print-directory test-tools-default test-tools-interface \
                test-tools-candid test-tools-agent test-tools-encoding \
                test-tools-registry test-tools-icp-project test-tools-icp-ledger test-tools-ledger-evidence
            cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --test public_contracts --locked --offline
        fi
        if [[ $RELEASE_GROUP == build ]]; then
            cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline
            cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --test public_contracts --locked --offline
            server=$(target/debug/ic-testkit-server check)
            POCKET_IC_BIN="$server" cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --test canister_wire --locked --offline
        fi ;;
    docs) RUSTDOCFLAGS='-D warnings' cargo doc -p "$package" --all-features --locked --offline --no-deps ;;
    archives) cargo run --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline -- --package "$package" ;;
    *) exit 2 ;;
esac
