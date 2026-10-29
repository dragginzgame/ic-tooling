#!/usr/bin/env bash
set -euo pipefail
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && printf '%s/.' "$PWD")"
root="${root%/.}"
cd "$root"

# Inspect all direct declarations (aliases, dev/build and target-specific edges)
# before inspecting the selected transitive graphs below.
bash scripts/ci/check-cargo-dependency-boundaries.sh --consumer "$root" --external-ic

[[ $# -le 1 ]] || { echo 'usage: check-tools-features.sh [TARGET]' >&2; exit 2; }
target="${1:-${CARGO_BUILD_TARGET:-$(rustc -vV | sed -n 's/^host: //p')}}"
case "$target" in
    x86_64-unknown-linux-gnu|x86_64-apple-darwin|aarch64-apple-darwin) ;;
    *) printf 'unsupported Host graph target: %s\n' "$target" >&2; exit 2 ;;
esac
# Host owns the profile matrix; inspect transitive normal/build dependencies for
# the explicit target, excluding dev fixtures. This is not artifact attestation.
# Observe command failures separately from dependency violations.
# Product families and higher Tooling layers never belong below Host or Build.
upper_layers='^(ic-wire(?:-|$)|ic-testkit(?:-|$)|ic-(auth|backup|blob-storage|jobs|metrics|persistence|query|relay|replica-watch|timers)(?:-|$)|canic(?:-|$)|icydb(?:-|$))'
perl scripts/ci/check-cargo-graph.pl --consumer "$root" --package ic-host-http --forbid ic-build --forbid-pattern "$upper_layers" \
    --target "$target" --require reqwest=1 --require ic-host-artifacts=1 \
    --forbid-pattern '^(ic-platform|candid|ic-agent|ic-host-(fs|process|tools)|rustls|native-tls|async-compression)$'
perl scripts/ci/check-cargo-graph.pl --consumer "$root" --package ic-host-http \
    --target wasm32-unknown-unknown --forbid-pattern '^(?!ic-host-http$)'
perl scripts/ci/check-cargo-graph.pl --consumer "$root" --package ic-build --forbid ic-platform --forbid-pattern "$upper_layers" \
    --target "$target" --forbid clap
perl scripts/ci/check-cargo-graph.pl --consumer "$root" --package ic-build --forbid ic-platform --forbid-pattern "$upper_layers" \
    --target "$target" --features cli --require clap=1
# Each profile selects its parser exclusions explicitly; labels carry no policy.
# Repeated dependency families remain local profile vocabulary.
codec='candid|candid_parser'
agent='ic-agent|reqwest|url'
serialization='serde_cbor|minicbor'
parser_free='^(serde_yaml|unsafe-libyaml|wat|wast)$'
yaml_allowed='^(wat|wast)$'
wat_allowed='^(serde_yaml|unsafe-libyaml)$'
check_graph() {
    local package="$1"
    shift
    local arguments=(--consumer "$root" --package "$package" --target "$target")
    arguments+=(--forbid ic-host-http --forbid ic-build --forbid-pattern "$upper_layers")
    # Tools explicitly owns selected Platform profiles; generic Host owners do not.
    [[ "$package" == ic-host-tools ]] || arguments+=(--forbid ic-platform)
    arguments+=(--forbid-pattern '^(ciborium|ciborium-io|ciborium-ll)$')
    arguments+=(--forbid-pattern '^(ic-cdk|ic-cdk-macros|ic-cdk-executor|ic-platform-macros|ic0)$')
    arguments+=(--forbid-pattern '^(serde_yaml_ng|serde_yml|serde-saphyr|granit-parser)$')
    perl "$root/scripts/ci/check-cargo-graph.pl" "${arguments[@]}" "$@" > /dev/null
}

check_graph ic-host-tools --forbid-pattern "$parser_free" \
    --forbid-pattern "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|${agent}|${serialization})$"
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features \
    --forbid-pattern "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|${agent}|${serialization})$"
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features encoding \
    --forbid-pattern "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$"
check_graph ic-host-tools --forbid-pattern "$yaml_allowed" --no-default-features --features icp-project \
    --forbid-pattern "^(ic-platform|${codec}|ic-host-(artifacts|fs|process)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$" \
    --require ic-host-text=1 --require serde_yaml=1 --require unsafe-libyaml=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features ledger-evidence \
    --forbid-pattern '^(candid_parser|ic-host-(artifacts|fs|process|text)|wasmparser|ic-agent|reqwest|ic-stable-structures|icrc-ledger-types|icrc-cbor)$' \
    --require ic-platform=1 --require candid=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features icp-ledger \
    --forbid-pattern "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$" \
    --require prost=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features registry \
    --forbid-pattern "^(${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest|ic_principal|sha2)$" \
    --require ic-platform=1 --require prost=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features ic-interface \
    --forbid-pattern "^(ic-platform|${codec}|ic-host-(fs|process|text)|${serialization}|wasm-encoder)$" \
    --require ic-host-artifacts=1 --require wasmparser=1
check_graph ic-host-wasm-inspect --forbid-pattern "$parser_free" \
    --forbid-pattern "^(ic-host-process|ic-platform|candid_parser|${serialization}|wasm-encoder|cap-std|cap-fs-ext|tempfile)$" \
    --require ic-host-artifacts=1 --require ic-host-fs=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features certificate \
    --forbid-pattern "^(${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-agent|reqwest|minicbor)$" \
    --require ic-platform=1 --require serde_cbor=1 --require ic-certification=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features ic-interface,certificate \
    --forbid-pattern "^(${codec}|ic-host-(fs|process|text)|ic-agent|reqwest|minicbor)$" \
    --require ic-platform=1 --require serde_cbor=1 --require ic-certification=1 --require ic-host-artifacts=1 --require wasmparser=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features candid \
    --forbid-pattern "^(ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$" \
    --require ic-platform=1 --require candid=1 --require candid_parser=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features candid,ic-interface,certificate \
    --forbid-pattern '^(ic-host-(fs|process|text)|ic-agent|reqwest|minicbor)$' \
    --require ic-platform=1 --require candid_parser=1 --require serde_cbor=1 --require ic-certification=1 --require ic-host-artifacts=1 --require wasmparser=1
check_graph ic-host-tools --forbid-pattern "$yaml_allowed" --no-default-features --all-features \
    --forbid-pattern '^(ic-host-(fs|process)|ic-stable-structures|icrc-ledger-types|icrc-cbor)$' \
    --require ic-platform=1 --require candid_parser=1 --require serde_cbor=1 --require ic-certification=1 --require ic-host-artifacts=1 --require wasmparser=1 --require ic-agent=1 --require reqwest=1 --require ic-host-text=1 --require serde_yaml=1 --require unsafe-libyaml=1 --require prost=1
check_graph ic-host-text --forbid-pattern "$parser_free" \
    --forbid-pattern "^(ic-host-(artifacts|fs|process|tools)|ic-platform|candid_parser|ic-agent|ic-certification|${serialization}|serde)$" \
    --require unicode-width=1
check_graph ic-host-text --forbid-pattern "$yaml_allowed" --no-default-features --features yaml \
    --forbid-pattern "^(ic-host-(artifacts|fs|process|tools)|ic-platform|${codec}|ic-agent|ic-certification|${serialization})$" \
    --require unicode-width=1 --require serde=1 --require serde_yaml=1 --require unsafe-libyaml=1
check_graph ic-host-artifacts --forbid-pattern "$parser_free" \
    --forbid-pattern '^(ic-host-process|wasm-encoder|wasmparser|clap)$'
check_graph ic-host-artifacts --forbid-pattern "$parser_free" --no-default-features --features wasm \
    --forbid-pattern '^wasm-encoder$' \
    --require wasmparser=1
check_graph ic-host-artifacts --forbid-pattern "$parser_free" --no-default-features --features wasm-encoding \
    --forbid-pattern "^(ic-host-(fs|process|text|tools)|ic-platform|${codec}|wasmparser)$" \
    --require wasm-encoder=1
check_graph ic-host-artifacts --forbid-pattern "$wat_allowed" --no-default-features --features wasm-text \
    --forbid-pattern "^(ic-host-(fs|process|text|tools)|ic-platform|${codec}|wasmparser)$" \
    --require wat=1 --require wast=1 --require wasm-encoder=1
for profile in wasm wasm-validation wasm-encoding wasm-text wasm-encoding,wasm-validation wasm-text,wasm-validation; do
    feature_graph="$(cargo tree -p ic-host-artifacts --no-default-features --features "$profile" \
        --target "$target" --edges normal,build --prefix none --format '{p} features={f}' --locked --offline)" || exit
    status=0
    printf '%s\n' "$feature_graph" | rg '^wasmparser .+ features=([^, ]+,)*validate(,[^ ]*)?$' > /dev/null || status=$?
    case "$profile:$status" in
        wasm:1|wasm-encoding:1|wasm-text:1|wasm-validation:0|wasm-encoding,wasm-validation:0|wasm-text,wasm-validation:0) ;;
        *) printf '%s has an unexpected Wasm validator feature selection (status %s)\n' "$profile" "$status" >&2; exit 1 ;;
    esac
    case "$profile" in
        *wasm-text*)
            status=0
            printf '%s\n' "$feature_graph" | rg '^(wat|wast|wasm-encoder) .+ features=([^, ]+,)*component-model(,[^ ]*)?$' > /dev/null || status=$?
            case "$status" in
                1) ;;
                *) printf '%s has an unexpected WAT component feature selection (status %s)\n' "$profile" "$status" >&2; exit 1 ;;
            esac
            ;;
    esac
done
check_graph ic-host-fs --forbid-pattern "$parser_free" \
    --forbid-pattern "^(ic-host-(process|tools|text)|ic-platform|${codec}|ic-agent|ic-certification|reqwest|${serialization}|clap|cap-std|cap-fs-ext|tempfile)$" \
    --require ic-host-artifacts=1
check_graph ic-host-fs --forbid-pattern "$parser_free" --no-default-features --features capability \
    --forbid-pattern '^(tempfile|clap|ic-host-process|ic-platform|wasmparser|wasm-encoder)$' \
    --require ic-host-artifacts=1 --require cap-std=1 --require cap-fs-ext=1
check_graph ic-host-fs --forbid-pattern "$parser_free" --no-default-features --features temporary-file \
    --forbid-pattern '^(cap-std|cap-fs-ext|clap|ic-host-process|ic-platform|wasmparser|wasm-encoder)$' \
    --require ic-host-artifacts=1 --require tempfile=1
check_graph ic-host-fs --forbid-pattern "$parser_free" --no-default-features --features capability,temporary-file \
    --forbid-pattern '^(clap|ic-host-process|ic-platform|wasmparser|wasm-encoder)$' \
    --require ic-host-artifacts=1 --require cap-std=1 --require cap-fs-ext=1 --require tempfile=1
check_graph ic-host-fs --forbid-pattern "$parser_free" --no-default-features --features cli \
    --forbid-pattern '^(ic-host-process|ic-platform|wasmparser|wasm-encoder|cap-std|cap-fs-ext|tempfile)$' \
    --require clap=1 --require ic-host-artifacts=1
check_graph ic-host-process --forbid-pattern "$parser_free" \
    --forbid-pattern '^(clap|ic-platform|wasmparser|wasm-encoder|cap-std|cap-fs-ext|tempfile)$' \
    --require ic-host-fs=1
check_graph ic-host-fs --forbid-pattern "$parser_free" --no-default-features --features capability,temporary-file --target wasm32-unknown-unknown \
    --forbid-pattern '^(cap-std|cap-fs-ext|tempfile|rustix|ic-host-process|ic-platform)$' \
    --require ic-host-artifacts=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features agent \
    --forbid-pattern '^(candid_parser|ic-host-(artifacts|fs|process|text)|wasmparser)$' \
    --require ic-platform=1 --require candid=1 --require ic-agent=1 --require reqwest=1 --require url=1 --require serde_cbor=1
check_graph ic-host-tools --forbid-pattern "$parser_free" --no-default-features --features agent --target wasm32-unknown-unknown \
    --forbid-pattern "^(ic-host-(artifacts|fs|process|text)|wasmparser|${codec}|${agent}|${serialization})$" \
    --require ic-platform=1
printf 'tools/text feature dependency graphs passed for %s (normal/build)\n' "$target"
