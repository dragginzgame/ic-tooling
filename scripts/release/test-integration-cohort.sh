#!/usr/bin/env bash
# Shared companions: scripts/ci/check-rust-toolchain.sh Cargo.toml rust-toolchain.toml
set -euo pipefail

# Post-publication proof of the same public contracts required in private Dev.
[[ $# == 0 ]] || { echo 'usage: test-integration-cohort.sh' >&2; exit 2; }
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
cd "$root"
parser="${YQ:-$root/.tools/host/bin/yq}"
YQ="$parser" bash scripts/ci/check-rust-toolchain.sh "$root"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source scripts/ci/fixture-lifecycle.sh
mkdir -p "$root/target"
fixture="$(fixture_create "$root/target/published-integration.XXXXXX")"
fixture_complete=false
# Keep successful readback evidence as well as failures; never clean this fixture.
finish() {
    local status="$1"
    fixture_completion_status "$status" "$fixture_complete" || status=$?
    if [[ "$status" != 0 ]]; then
        fixture_replay_logs "$fixture/lock.log" "$fixture/fetch.log" "$fixture/consumer.log"
    fi
    printf 'Published integration evidence: %s\n' "$fixture" >&2
    exit "$status"
}
trap 'finish "$?"' EXIT
export RUSTUP_AUTO_INSTALL=0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/target}"
cargo metadata --manifest-path qualification/Cargo.toml --locked --offline --format-version 1 > "$fixture/source-metadata.json"
mkdir "$fixture/src"
cp qualification/ic-testkit-dev/tests/public_contracts.rs "$fixture/src/lib.rs"
cat > "$fixture/Cargo.toml" <<'TOML'
[package]
name = "integration-cohort-consumer"
version = "0.0.0"
edition = "2024"
rust-version = "1.100.0"
publish = false
[workspace]
[dependencies]
TOML
# Keep manifest generation and identity assertions on the same resolved inputs.
# Catalog requirements are compatible minimums, not exact selected patch versions.
# The facade may coexist with another version; canonical Types/Wire remain singleton.
jq -e '. as $source |
    ["ic-build", "ic-host-artifacts", "ic-host-process", "ic-host-tools",
     "ic-testkit", "ic-platform", "ic-platform-types", "ic-wire"] |
    map(. as $name |
        [$source.resolve.nodes[] |
            select(.id as $id | $source.workspace_members | index($id)) |
            .deps[] | select(.name == ($name | gsub("-"; "_"))) | .pkg] | unique as $ids |
        if ($ids | length) != 1 then error("expected one direct workspace selection for " + $name) else
        ($source.packages[] | select(.id == $ids[0])) as $package | {
        name: $name, version: $package.version, singleton: ($name != "ic-platform"),
        features: (if $name == "ic-host-tools" then ["registry"]
            elif $name == "ic-testkit" then ["wire"]
            elif $name == "ic-platform" then ["principal", "ic-protocol"]
            else [] end)
    } end)' "$fixture/source-metadata.json" > "$fixture/selection.json"
# These are new registry-only inputs, never edits to release metadata or root lock.
jq -er '.[] |
    if (.version | test("^[0-9]+\\.[0-9]+\\.[0-9]+$")) then
        (.name + " = { version = \"=" + .version + "\", default-features = false" +
        (if (.features | length) > 0 then ", features = " + (.features | tojson) else "" end) + " }")
    else error("expected a literal selected version for " + .name) end' \
    "$fixture/selection.json" >> "$fixture/Cargo.toml"
# Online resolution is explicit at invocation: CARGO_NET_OFFLINE=false bash ...
cargo generate-lockfile --manifest-path "$fixture/Cargo.toml" > "$fixture/lock.log" 2>&1
cargo fetch --locked --manifest-path "$fixture/Cargo.toml" > "$fixture/fetch.log" 2>&1
cargo metadata --locked --offline --format-version 1 --manifest-path "$fixture/Cargo.toml" \
    > "$fixture/metadata.json" 2> "$fixture/metadata.stderr"
jq -e --slurpfile selection "$fixture/selection.json" '
    .packages as $packages |
    all($selection[0][] | select(.singleton);
        . as $selected | [$packages[] | select(.name == $selected.name)] |
        length == 1 and all(.[]; .version == $selected.version and (.source | startswith("registry+"))))
    and all($packages[]; .name == "integration-cohort-consumer" or (.source | startswith("registry+")))
' "$fixture/metadata.json" > /dev/null
host="$(rustc -vV | sed -n 's/^host: //p')"
perl scripts/ci/check-cargo-graph.pl --consumer "$fixture" \
    --package integration-cohort-consumer --target "$host" \
    --require ic-platform-types=1 \
    --forbid ic-platform-runtime --forbid ic-platform-runtime-guard \
    --forbid pocket-ic --json --observations > "$fixture/native-graph.json"
cargo test --locked --offline --manifest-path "$fixture/Cargo.toml" \
    > "$fixture/consumer.log" 2>&1
cat "$fixture/consumer.log"
echo 'Published packages share the qualified public contracts'
fixture_complete=true
