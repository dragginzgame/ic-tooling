#!/usr/bin/env bash
set -euo pipefail
# Product-owned source/configuration closure for metadata-only release fixtures.
[[ ( $# == 2 || ( $# == 3 && $3 == --compact ) ) && $1 == /* && -d $2 && ! -L $2 ]] || exit 2
root=$1 destination=$2
cp -R "$root/ci" "$root/make" "$root/scripts" "$destination/"
cp "$root/Makefile" "$root/rust-toolchain.toml" "$root/.toko-tooling.snapshot" "$root/.gitignore" "$destination/"
# The invariant workflow matrix observes its real workflow and admitted prose
# inventory even inside the disposable release-adapter fixture.
cp -R "$root/.github" "$root/docs" "$destination/"
cp "$root/TOKO-TOOLING.md" "$destination/"
if [[ ${3:-} == --compact ]]; then
    # Metadata/recovery fixtures retain the real selector, source proofs and
    # reusable-stage admissions without dispatching unrelated dummy products.
    jq -cS '.stages |= map(select(startswith("release-stage-release-scope-") or
        . == "release-stage-toko-tooling-check" or
        . == "release-stage-clippy-workspace" or
        . == "release-stage-tooling-command-check"))' \
        "$destination/ci/release-validation.json" > "$destination/ci/fixture-policy.json"
    mv "$destination/ci/fixture-policy.json" "$destination/ci/release-validation.json"
    jq -cS '.common = ["toko-tooling-check", "clippy-workspace", "tooling-command-check"] |
        .exclusions = []' "$destination/ci/release-validation-coverage.json" > "$destination/ci/fixture-coverage.json"
    mv "$destination/ci/fixture-coverage.json" "$destination/ci/release-validation-coverage.json"
fi
