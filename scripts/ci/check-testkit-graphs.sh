#!/usr/bin/env bash
set -euo pipefail

# Testkit owns these supported profiles; Shared owns graph observation mechanics.
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
cd "$root"
# Admit direct IC dependency owners in the independent qualification workspace
# before allowing Runtime's transitive IC0 provider in its Wasm graph.
bash vendor/toko-tooling/scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$root/qualification" --external-ic
consumer="$root/"
host="$(rustc -vV | sed -n 's/^host: //p')"
test -n "$host"
policy=(--consumer "$consumer" --require ic-platform-types=1 --forbid ic-metrics
    --forbid ic-wire-derive
    --forbid ic-build --forbid ic-host-text --forbid ic-wire-endpoint
    --forbid ic-agent --forbid candid_parser --forbid-pattern '^ic-cdk')
# Runtime owns the probe's qualified raw system API; Testkit never selects it.
runtime_policy=(--consumer "$root/qualification" "${policy[@]:2}" --require ic0=1)
policy+=(--forbid ic-platform-runtime-guard --forbid ic0)
native_policy=("${policy[@]}" --require pocket-ic=1 --require ic-management-canister-types=1
    --forbid ic-platform-pocket-ic --forbid-feature ic-platform/pocket-ic
    --forbid ic-platform-runtime --forbid ic-logging)

# Observe these roots separately: workspace/all-feature unification must not
# conceal the optional Wire boundary. Render the actual selected identities.
native_graph() {
    local profile=$1 observation
    shift
    observation=$(perl vendor/toko-tooling/scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
        --package ic-testkit --target "$host" --forbid clap \
        --forbid wat --forbid wast --forbid wasm-encoder --json --observations "$@")
    jq -r --arg profile "$profile" '.observations[] |
        select(.name | test("^ic-(testkit|platform(-types)?|wire)$")) |
        "Testkit \($profile): \(.identity)"' <<< "$observation"
}
native_graph default --forbid ic-wire --require ic-platform=1
native_graph wire --features wire --require ic-wire=1 --require ic-platform=1
perl vendor/toko-tooling/scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
    --package ic-testkit --target "$host" --features reply-fixtures \
    --require wasm-encoder=1 --forbid wat --forbid wast --require ic-platform=1
perl vendor/toko-tooling/scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
    --package ic-testkit --target "$host" --features held-reply-fixtures \
    --require wat=1 --require wast=1 --require wasm-encoder=1 --require ic-platform=1
perl vendor/toko-tooling/scripts/ci/check-cargo-graph.pl "${native_policy[@]}" \
    --package ic-testkit --target "$host" --all-features \
    --require clap=1 --require ic-wire=1
perl vendor/toko-tooling/scripts/ci/check-cargo-graph.pl --consumer "$root/qualification" \
    --package ic-testkit-dev --target "$host" --all-features \
    --require ic-build=1 --require ic-platform=1 --forbid ic-platform-pocket-ic \
    --forbid-feature ic-platform/pocket-ic \
    --require pocket-ic=1 --forbid ic-metrics --forbid ic-platform-runtime --forbid ic-platform-runtime-guard \
    --require ic-host-artifacts=1 --require ic-host-fs=1 \
    --require ic-host-process=1 --require ic-host-text=1
perl vendor/toko-tooling/scripts/ci/check-cargo-graph.pl "${policy[@]}" --package ic-testkit \
    --target wasm32-unknown-unknown --all-features --forbid-pattern '^ic-host-' \
    --forbid reqwest --forbid tokio --forbid tempfile --forbid flate2 --forbid ic-certification --forbid ic-transport-types \
    --forbid pocket-ic --forbid candid --forbid candid_derive --forbid ic-platform-pocket-ic --forbid ic-wire --forbid clap \
    --forbid wat --forbid wast --forbid wasm-encoder --forbid wasmparser --forbid ic-management-canister-types \
    --forbid ic-platform --forbid ic-platform-macros --forbid ic-platform-runtime
probe_wasm="$(perl vendor/toko-tooling/scripts/ci/check-cargo-graph.pl "${runtime_policy[@]}" --package ic-testkit-perf-probe \
    --target wasm32-unknown-unknown --forbid-pattern '^ic-host-' \
    --forbid ic-testkit --forbid pocket-ic \
    --forbid candid --forbid candid_derive --forbid ic-platform-pocket-ic --forbid clap \
    --forbid ic-platform --require ic-wire=1 --require ic-platform-runtime=1 --require ic-platform-runtime-guard=1 --json --observations)"
jq -e 'any(.observations[]; .name == "ic-platform-runtime" and
    (.features | index("ic0") != null and index("executor") != null and index("runtime") != null))' \
    <<< "$probe_wasm" > /dev/null
