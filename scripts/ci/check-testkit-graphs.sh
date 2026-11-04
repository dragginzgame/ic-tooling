#!/usr/bin/env bash
set -euo pipefail

# Testkit owns these supported profiles; Shared owns graph observation mechanics.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
cd "$root"
consumer="$root/"
host="$(rustc -vV | sed -n 's/^host: //p')"
test -n "$host"
policy=(--consumer "$consumer" --require ic-platform-types=1 --forbid ic-metrics
    --forbid ic-wire-derive
    --forbid ic-build --forbid ic-host-text --forbid ic-wire-endpoint
    --forbid ic-agent --forbid candid_parser --forbid ic0 --forbid pocket-ic --forbid-pattern '^ic-cdk')
runtime_policy=("${policy[@]}")
policy+=(--forbid ic-platform-runtime-guard)
native_policy=("${policy[@]}" --require ic-platform=1 --forbid ic-platform-pocket-ic
    --forbid-feature ic-platform/pocket-ic --forbid ic-platform-runtime)

perl scripts/ci/check-cargo-graph.pl --consumer "$consumer" \
    --package ic-testkit-native --target "$host" --all-features \
    --require ic-testkit-native=1 --forbid-pattern '^(?!ic-testkit-native$)'
perl scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
    --package ic-testkit --target "$host" --forbid clap --forbid ic-wire \
    --forbid wat --forbid wast --forbid wasm-encoder
perl scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
    --package ic-testkit --target "$host" --features reply-fixtures \
    --require wasm-encoder=1 --forbid wat --forbid wast
perl scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
    --package ic-testkit --target "$host" --features held-reply-fixtures \
    --require wat=1 --require wast=1 --require wasm-encoder=1
perl scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
    --package ic-testkit --target "$host" --all-features \
    --require clap=1 --require ic-wire=1
perl scripts/ci/check-cargo-graph.pl --consumer "$consumer" \
    --package ic-testkit-dev --target "$host" --all-features \
    --require ic-build=1 --require ic-platform=1 --forbid ic-platform-pocket-ic \
    --forbid-feature ic-platform/pocket-ic \
    --forbid pocket-ic --forbid ic-metrics --forbid ic-platform-runtime --forbid ic-platform-runtime-guard \
    --require ic-host-artifacts=1 --require ic-host-fs=1 \
    --require ic-host-process=1 --require ic-host-text=1
perl scripts/ci/check-cargo-graph.pl --consumer "$consumer" \
    --package ic-testkit-native --target wasm32-unknown-unknown --all-features \
    --require ic-testkit-native=1 --forbid-pattern '^(?!ic-testkit-native$)'
testkit_wasm="$(perl scripts/ci/check-cargo-graph.pl "${policy[@]}" --package ic-testkit \
    --target wasm32-unknown-unknown --all-features --forbid-pattern '^ic-host-' \
    --forbid reqwest --forbid tokio --forbid tempfile --forbid flate2 --forbid ic-certification --forbid ic-transport-types \
    --forbid candid --forbid candid_derive --forbid ic-platform-pocket-ic --forbid ic-wire --forbid clap \
    --forbid wat --forbid wast --forbid wasm-encoder --forbid wasmparser --forbid ic-management-canister-types \
    --forbid ic-platform --forbid ic-platform-macros --require ic-platform-runtime=1 --json --observations)"
jq -e 'any(.observations[]; .name == "ic-platform-runtime" and
    (.features | index("ic0") != null and index("executor") == null and index("runtime") == null))' \
    <<< "$testkit_wasm" > /dev/null
probe_wasm="$(perl scripts/ci/check-cargo-graph.pl "${runtime_policy[@]}" --package ic-testkit-perf-probe \
    --target wasm32-unknown-unknown --forbid-pattern '^ic-host-' \
    --forbid candid --forbid candid_derive --forbid ic-platform-pocket-ic --forbid clap \
    --forbid ic-platform --require ic-wire=1 --require ic-platform-runtime=1 --require ic-platform-runtime-guard=1 --json --observations)"
jq -e 'any(.observations[]; .name == "ic-platform-runtime" and
    (.features | index("ic0") != null and index("executor") != null and index("runtime") != null))' \
    <<< "$probe_wasm" > /dev/null
