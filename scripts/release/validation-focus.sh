#!/usr/bin/env bash
set -euo pipefail
# Owner/reverse-consumer contracts admitted by the existing three-host comparison.
[[ $# == 1 ]] || exit 2
package=$(jq -er --arg group "${RELEASE_GROUP:-}" \
    --slurpfile coverage ci/release-validation-coverage.json '
    .groups[] | select(.name == $group) |
    select($coverage[0].focused | index($group)) | .primary
' ci/cargo-release-groups.json)
members=()
while IFS= read -r member; do members+=("$member"); done \
    < <(bash scripts/release/group.sh ci/cargo-release-groups.json "$RELEASE_GROUP" members)
packages=()
archives=()
for member in "${members[@]}"; do
    packages+=(-p "$member")
    archives+=(--package "$member")
done
case "$RELEASE_GROUP" in
    foundation)
        packages+=(-p ic-build -p ic-host-tools -p ic-testkit -p ic-testkit-wire)
        archives+=(--package ic-build --package ic-host-tools --package ic-testkit --package ic-testkit-wire) ;;
    testkit) packages+=(-p ic-testkit-wire); archives+=(--package ic-testkit-wire) ;;
    text) packages+=(-p ic-host-tools) ;;
esac
case "$1" in
    tools)
        case "$RELEASE_GROUP" in
            foundation|build) make --no-print-directory tools-check ;;
            testkit|testkit-wire)
                make --no-print-directory host-tools-check rust-tools-check release-editor-check build-server-cli server-check ;;
            *) make --no-print-directory host-tools-check rust-tools-check release-editor-check ;;
        esac ;;
    lint)
        cargo clippy "${packages[@]}" --all-targets --all-features --locked --offline -- -D warnings
        cargo clippy --manifest-path qualification/Cargo.toml -p ic-testkit-dev --all-targets --all-features --locked --offline -- -D warnings ;;
    graphs)
        bash scripts/ci/check-tools-features.sh
        case "$RELEASE_GROUP" in
            foundation|testkit|testkit-wire)
                make --no-print-directory testkit-graph-check testkit-wasm-check ;;
        esac
        if [[ $RELEASE_GROUP == foundation ]]; then make --no-print-directory check-fs-non-unix; fi ;;
    tests)
        case "$RELEASE_GROUP" in
            foundation|testkit|testkit-wire)
                make --no-print-directory build-server-cli
                server=$(target/debug/ic-testkit-server check)
                POCKET_IC_BIN="$server" cargo test "${packages[@]}" --all-targets --all-features --locked --offline ;;
            *) cargo test -p "$package" --all-targets --all-features --locked --offline ;;
        esac
        if [[ $RELEASE_GROUP == text ]]; then
            cargo test -p "$package" --no-default-features --lib --locked --offline
            make --no-print-directory test-text-yaml test-tools-icp-project
            cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline
        fi
        if [[ $RELEASE_GROUP == tools || $RELEASE_GROUP == foundation ]]; then
            make --no-print-directory test-tools-default test-tools-interface \
                test-tools-candid test-tools-agent test-tools-encoding \
                test-tools-registry test-tools-icp-project test-tools-icp-ledger test-tools-ledger-evidence
        fi
        if [[ $RELEASE_GROUP == tools ]]; then
            cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --test public_contracts --locked --offline
        fi
        if [[ $RELEASE_GROUP == foundation ]]; then
            make --no-print-directory test-artifacts-minimal test-fs-minimal test-fs-capability \
                test-fs-temporary-file test-artifacts-wasm test-artifacts-text
            cargo test -p ic-host-http --no-default-features --features endpoint --lib --locked --offline endpoint::
            cargo check -p ic-host-http --no-default-features --features endpoint --lib --target wasm32-unknown-unknown --locked --offline
        fi
        case "$RELEASE_GROUP" in
            foundation|testkit)
                POCKET_IC_BIN="$server" cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --all-targets --all-features --locked --offline
                make --no-print-directory testkit-installed-check ;;
            testkit-wire)
                POCKET_IC_BIN="$server" cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --test public_contracts --test canister_wire --locked --offline ;;
            build)
                cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline
                cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --test public_contracts --locked --offline
                server=$(target/debug/ic-testkit-server check)
                POCKET_IC_BIN="$server" cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --test canister_wire --locked --offline ;;
        esac ;;
    docs) RUSTDOCFLAGS='-D warnings' cargo doc "${packages[@]}" --all-features --locked --offline --no-deps ;;
    archives)
        cargo run --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline -- "${archives[@]}" ;;
    *) exit 2 ;;
esac
