#!/usr/bin/env bash
set -euo pipefail

# Host selects the inputs; the shared checker owns formatting and preservation
# assertions. All Git changes occur in its disposable consumer copies.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture="$(tooling_fixture_create "$root" host-formatting)"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Host formatting input retained"' EXIT

# Keep the labelled first entry fixed while perturbing entries within its group.
perl "$root/scripts/ci/prepare-cargo-formatting-fixture.pl" \
    "$root/Cargo.toml" workspace.dependencies ic-host-fs ic-host-http > "$fixture/unsorted.toml"
bash "$root/scripts/ci/check-formatting-hooks.sh" "$root" \
    crates/ic-host-fs/src/durable/mod.rs Cargo.toml "$fixture/unsorted.toml" --rust-sources \
    make/consumer.mk ci/tool-versions.env ci/release-delivery.txt \
    scripts/ci/check-release-delivery.sh
fixture_complete=true
