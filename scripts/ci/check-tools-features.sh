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
    local parser_exclusions="$1" package="$2" defaults="$3" features="$4" required="$5" forbidden="$6" owner
    local arguments=(--consumer "$root" --package "$package" --target "${7:-$target}")
    arguments+=(--forbid ic-host-http --forbid ic-build --forbid-pattern "$upper_layers")
    # Tools explicitly owns selected Platform profiles; generic Host owners do not.
    [[ "$package" == ic-host-tools ]] || arguments+=(--forbid ic-platform)
    local owners=()
    [[ "$defaults" != none ]] || arguments+=(--no-default-features)
    case "$features" in
        --all-features) arguments+=(--all-features) ;;
        '') ;;
        *) arguments+=(--features "$features") ;;
    esac
    arguments+=(--forbid-pattern "$forbidden" --forbid-pattern '^(ciborium|ciborium-io|ciborium-ll)$')
    # Platform's runtime is independently selectable; no Host profile owns it.
    arguments+=(--forbid-pattern '^(ic-cdk|ic-cdk-macros|ic-cdk-executor|ic-platform-macros|ic0)$')
    arguments+=(--forbid-pattern '^(serde_yaml_ng|serde_yml|serde-saphyr|granit-parser)$')
    arguments+=(--forbid-pattern "$parser_exclusions")
    if [[ -n "$required" ]]; then
        read -r -a owners <<< "$required"
        for owner in "${owners[@]}"; do arguments+=(--require "$owner=1"); done
    fi
    perl "$root/scripts/ci/check-cargo-graph.pl" "${arguments[@]}" > /dev/null
}

check_graph "$parser_free" ic-host-tools default '' '' \
    "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|${agent}|${serialization})$"
check_graph "$parser_free" ic-host-tools none '' '' \
    "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|${agent}|${serialization})$"
check_graph "$parser_free" ic-host-tools none encoding '' \
    "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$"
check_graph "$yaml_allowed" ic-host-tools none icp-project 'ic-host-text serde_yaml unsafe-libyaml' \
    "^(ic-platform|${codec}|ic-host-(artifacts|fs|process)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$"
check_graph "$parser_free" ic-host-tools none ledger-evidence 'ic-platform candid icrc-ledger-types' \
    '^(candid_parser|ic-host-(artifacts|fs|process|text)|wasmparser|ic-agent|reqwest|ic-stable-structures)$'
check_graph "$parser_free" ic-host-tools none icp-ledger prost \
    "^(ic-platform|${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$"
check_graph "$parser_free" ic-host-tools none registry 'ic-platform prost' \
    "^(${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest)$"
check_graph "$parser_free" ic-host-tools none ic-interface \
    'ic-host-artifacts wasmparser' "^(ic-platform|${codec}|ic-host-(fs|process|text)|${serialization}|wasm-encoder)$"
check_graph "$parser_free" ic-host-wasm-inspect default '' 'ic-host-artifacts ic-host-fs' \
    "^(ic-host-process|ic-platform|candid_parser|${serialization}|wasm-encoder|cap-std|cap-fs-ext|tempfile)$"
check_graph "$parser_free" ic-host-tools none certificate 'ic-platform serde_cbor ic-certification' \
    "^(${codec}|ic-host-(artifacts|fs|process|text)|wasmparser|ic-agent|reqwest|minicbor)$"
check_graph "$parser_free" ic-host-tools none ic-interface,certificate \
    'ic-platform serde_cbor ic-certification ic-host-artifacts wasmparser' \
    "^(${codec}|ic-host-(fs|process|text)|ic-agent|reqwest|minicbor)$"
check_graph "$parser_free" ic-host-tools none candid 'ic-platform candid candid_parser' \
    "^(ic-host-(artifacts|fs|process|text)|wasmparser|ic-certification|ic-agent|reqwest|${serialization})$"
check_graph "$parser_free" ic-host-tools none candid,ic-interface,certificate \
    'ic-platform candid_parser serde_cbor ic-certification ic-host-artifacts wasmparser' \
    '^(ic-host-(fs|process|text)|ic-agent|reqwest|minicbor)$'
check_graph "$yaml_allowed" ic-host-tools none --all-features \
    'ic-platform candid_parser serde_cbor ic-certification ic-host-artifacts wasmparser ic-agent reqwest icrc-ledger-types ic-host-text serde_yaml unsafe-libyaml prost' \
    '^(ic-host-(fs|process)|ic-stable-structures)$'
check_graph "$parser_free" ic-host-text default '' unicode-width \
    "^(ic-host-(artifacts|fs|process|tools)|ic-platform|candid_parser|ic-agent|ic-certification|${serialization}|serde)$"
check_graph "$yaml_allowed" ic-host-text none yaml 'unicode-width serde serde_yaml unsafe-libyaml' \
    "^(ic-host-(artifacts|fs|process|tools)|ic-platform|${codec}|ic-agent|ic-certification|${serialization})$"
check_graph "$parser_free" ic-host-artifacts default '' '' '^(ic-host-process|wasm-encoder|wasmparser|clap)$'
check_graph "$parser_free" ic-host-artifacts none wasm wasmparser '^wasm-encoder$'
check_graph "$parser_free" ic-host-artifacts none wasm-encoding wasm-encoder \
    "^(ic-host-(fs|process|text|tools)|ic-platform|${codec}|wasmparser)$"
check_graph "$wat_allowed" ic-host-artifacts none wasm-text 'wat wast wasm-encoder' \
    "^(ic-host-(fs|process|text|tools)|ic-platform|${codec}|wasmparser)$"
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
check_graph "$parser_free" ic-host-fs default '' ic-host-artifacts \
    "^(ic-host-(process|tools|text)|ic-platform|${codec}|ic-agent|ic-certification|reqwest|${serialization}|clap|cap-std|cap-fs-ext|tempfile)$"
check_graph "$parser_free" ic-host-fs none capability 'ic-host-artifacts cap-std cap-fs-ext' \
    '^(tempfile|clap|ic-host-process|ic-platform|wasmparser|wasm-encoder)$'
check_graph "$parser_free" ic-host-fs none temporary-file 'ic-host-artifacts tempfile' \
    '^(cap-std|cap-fs-ext|clap|ic-host-process|ic-platform|wasmparser|wasm-encoder)$'
check_graph "$parser_free" ic-host-fs none capability,temporary-file 'ic-host-artifacts cap-std cap-fs-ext tempfile' \
    '^(clap|ic-host-process|ic-platform|wasmparser|wasm-encoder)$'
check_graph "$parser_free" ic-host-fs none cli 'clap ic-host-artifacts' \
    '^(ic-host-process|ic-platform|wasmparser|wasm-encoder|cap-std|cap-fs-ext|tempfile)$'
check_graph "$parser_free" ic-host-process default '' 'ic-host-fs' '^(clap|ic-platform|wasmparser|wasm-encoder|cap-std|cap-fs-ext|tempfile)$'
check_graph "$parser_free" ic-host-fs none capability,temporary-file ic-host-artifacts \
    '^(cap-std|cap-fs-ext|tempfile|rustix|ic-host-process|ic-platform)$' wasm32-unknown-unknown
check_graph "$parser_free" ic-host-tools none agent 'ic-platform candid ic-agent reqwest url serde_cbor' \
    '^(candid_parser|ic-host-(artifacts|fs|process|text)|wasmparser)$'
check_graph "$parser_free" ic-host-tools none agent 'ic-platform' \
    "^(ic-host-(artifacts|fs|process|text)|wasmparser|${codec}|${agent}|${serialization})$" wasm32-unknown-unknown
printf 'tools/text feature dependency graphs passed for %s (normal/build)\n' "$target"
