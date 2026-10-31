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
policy=(--consumer "$consumer" --require ic-platform=1 --require ic-platform-types=1 --require ic-metrics=1
    --forbid ic-build --forbid ic-host-text --forbid ic-wire-endpoint
    --forbid ic-agent --forbid candid_parser --forbid ic0 --forbid-pattern '^ic-cdk')
runtime_policy=("${policy[@]}")
policy+=(--forbid ic-platform-runtime-guard)

perl scripts/ci/check-cargo-graph.pl --consumer "$consumer" \
    --package ic-testkit-native --target "$host" \
    --require ic-testkit-native=1 --forbid-pattern '^(?!ic-testkit-native$)'
perl scripts/ci/check-cargo-graph.pl --consumer "$consumer" \
    --package ic-testkit-native --target "$host" --all-features \
    --require ic-platform=1 --require ic-stable-structures=1 \
    --forbid-pattern '^(?!ic-testkit-native$|ic-platform$|ic-stable-structures$|ic_principal$)'
perl scripts/ci/check-cargo-graph.pl "${policy[@]}" \
    --package ic-testkit --target "$host" --forbid clap --forbid ic-wire --require pocket-ic=1 \
    --forbid wat --forbid wast --forbid wasm-encoder
perl scripts/ci/check-cargo-graph.pl "${policy[@]}" \
    --package ic-testkit --target "$host" --features reply-fixtures \
    --require wasm-encoder=1 --forbid wat --forbid wast
perl scripts/ci/check-cargo-graph.pl "${policy[@]}" \
    --package ic-testkit --target "$host" --features held-reply-fixtures \
    --require wat=1 --require wast=1 --require wasm-encoder=1
testkit_native="$(perl scripts/ci/check-cargo-graph.pl "${policy[@]}" \
    --package ic-testkit --target "$host" --all-features \
    --require clap=1 --require ic-wire=1 --require pocket-ic=1 --json --observations)"
# Even profiles with an independent Wire caller keep Metrics arithmetic-only.
jq -e 'any(.observations[]; .name == "ic-metrics" and (.features | length == 0))' \
    <<< "$testkit_native" > /dev/null
perl scripts/ci/check-cargo-graph.pl --consumer "$consumer" \
    --package ic-testkit-dev --target "$host" --all-features \
    --require ic-build=1 --require ic-platform=1 --require pocket-ic=1 \
    --forbid ic-platform-runtime-guard \
    --require ic-host-artifacts=1 --require ic-host-fs=1 \
    --require ic-host-process=1 --require ic-host-text=1
perl scripts/ci/check-cargo-graph.pl --consumer "$consumer" \
    --package ic-testkit-native --target wasm32-unknown-unknown --all-features \
    --require ic-testkit-native=1 --forbid-pattern '^(?!ic-testkit-native$)'
testkit_wasm="$(perl scripts/ci/check-cargo-graph.pl "${policy[@]}" --package ic-testkit \
    --target wasm32-unknown-unknown --all-features --forbid-pattern '^ic-host-' \
    --forbid candid --forbid candid_derive --forbid pocket-ic --forbid ic-wire --forbid clap \
    --forbid wat --forbid wast --forbid wasm-encoder --forbid wasmparser --forbid ic-management-canister-types \
    --forbid ic-platform-macros --json --observations)"
jq -e 'any(.observations[]; .name == "ic-platform" and
    (.features | index("ic0") != null and index("executor") == null and index("runtime") == null))
    and any(.observations[]; .name == "ic-metrics" and (.features | length == 0))' \
    <<< "$testkit_wasm" > /dev/null
probe_wasm="$(perl scripts/ci/check-cargo-graph.pl "${runtime_policy[@]}" --package ic-testkit-perf-probe \
    --target wasm32-unknown-unknown --forbid-pattern '^ic-host-' \
    --forbid candid --forbid candid_derive --forbid pocket-ic --forbid clap \
    --require ic-wire=1 --require ic-platform-runtime-guard=1 --json --observations)"
jq -e 'any(.observations[]; .name == "ic-platform" and
    (.features | index("ic0") != null and index("executor") != null and index("runtime") != null))' \
    <<< "$probe_wasm" > /dev/null
