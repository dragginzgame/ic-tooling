#!/usr/bin/env bash
# Compose two real codec generations with one unchanged adapter implementation.
# Fixture manifests alone select the old generation; no production alias or fork.
set -euo pipefail
[[ $# == 0 ]] || { echo 'usage: test-wire-adapter-generations.sh' >&2; exit 2; }
root="$(CDPATH='' cd -P -- "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
parser="${YQ:-$root/.tools/host/bin/yq}"
# Setup/fetch is explicit and separate. This proof never installs or downloads.
server="$("$root/target/debug/ic-testkit-server" check)"
export POCKET_IC_BIN="$server" RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/target}"
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture="$(tooling_fixture_create "$root" wire-adapter-generations)"
fixture_complete=false
finish() {
    local status="$1"
    fixture_completion_status "$status" "$fixture_complete" || status=$?
    if [[ $status != 0 ]]; then fixture_replay_logs "$fixture/lock.log" "$fixture/test.log"; fi
    printf 'Wire generation composition evidence: %s\n' "$fixture" >&2
    exit "$status"
}
trap 'finish "$?"' EXIT
"$parser" -o=json '.' "$root/Cargo.toml" > "$fixture/product.json"
# Retain current package/lint policies and actual Testkit selection. Only the
# temporary adapter version and its direct Wire requirement vary between lines.
jq --arg root "$root" '.workspace | {
    package: (.package | .["license-file"] = ($root + "/LICENSE.md")), lints,
    dependencies: (.dependencies | with_entries(select(.key |
        IN("ic-testkit", "ic-wire", "serde", "tokio", "toko-tooling"))) |
        .["ic-testkit"].path = ($root + "/crates/ic-testkit"))
}' "$fixture/product.json" > "$fixture/common.json"
emit_workspace() {
    jq -r '
        def toml: if type == "object" then
            "{ " + (to_entries | map((.key | tojson) + " = " + (.value | toml)) | join(", ")) + " }"
            elif type == "array" then "[" + (map(toml) | join(", ")) + "]"
            else tojson end;
        def entries: to_entries[] | (.key | tojson) + " = " + (.value | toml);
        "[workspace]", "resolver = \"3\"",
        (select(has("members")) | {members, exclude} | entries),
        "[workspace.package]", (.package | entries),
        "[workspace.dependencies]", (.dependencies | entries),
        (.lints | to_entries[] | "[workspace.lints." + .key + "]", (.value | entries))
    ' "$1"
}
for generation in 21 22; do
    wire=0.21.5 version=0.0.0
    [[ $generation != 22 ]] || { wire=0.22.1; version=0.1.0; }
    adapter="$fixture/adapter-$generation"
    consumer="$fixture/consumer-$generation"
    mkdir -p "$adapter" "$consumer/tests"
    cp -R "$root/crates/ic-testkit-wire/src" "$adapter/src"
    cp "$root/crates/ic-testkit-wire/README.md" "$adapter/README.md"
    cp "$root/crates/ic-testkit-wire/tests/pocket_ic_calls.rs" "$consumer/tests/"
    sed "s/^version = .*/version = \"$version\"/" "$root/crates/ic-testkit-wire/Cargo.toml" > "$adapter/Cargo.toml"
    jq --arg wire "$wire" '.dependencies["ic-wire"] = ("=" + $wire)' \
        "$fixture/common.json" > "$adapter/workspace.json"
    emit_workspace "$adapter/workspace.json" >> "$adapter/Cargo.toml"
    cat > "$consumer/Cargo.toml" <<TOML
[package]
name = "wire-$generation-consumer"
version = "0.0.0"
edition.workspace = true
rust-version.workspace = true
publish = false
[dev-dependencies]
ic-testkit = { workspace = true, features = ["reply-fixtures"] }
ic-testkit-wire = { path = "../adapter-$generation", version = "=$version" }
ic-wire = "=$wire"
tokio = { workspace = true, features = ["macros", "rt-multi-thread"] }
[lints]
workspace = true
TOML
done
jq '{members: ["consumer-21", "consumer-22"], exclude: ["adapter-21", "adapter-22"],
    package: (.package | {edition, "rust-version": .["rust-version"]}), lints: .lints,
    dependencies: (.dependencies | with_entries(select(.key | IN("ic-testkit", "tokio"))))}
' "$fixture/common.json" > "$fixture/workspace.json"
emit_workspace "$fixture/workspace.json" > "$fixture/Cargo.toml"
cargo generate-lockfile --offline --manifest-path "$fixture/Cargo.toml" > "$fixture/lock.log" 2>&1
cargo metadata --locked --offline --format-version 1 --manifest-path "$fixture/Cargo.toml" \
    > "$fixture/metadata.json"
jq -e --arg core "$root/crates/ic-testkit/Cargo.toml" '
    . as $m | .packages as $p |
    ([$p[] | select(.name == "ic-wire") | .version] | sort) == ["0.21.5", "0.22.1"] and
    ([$p[] | select(.name == "ic-testkit")] | length == 1 and .[0].manifest_path == $core) and
    ([$p[] | select(.name == "pocket-ic")] | length == 1) and
    all(["21", "22"][]; . as $generation |
        ($p[] | select(.name == ("wire-" + $generation + "-consumer"))) as $consumer |
        ($m.resolve.nodes[] | select(.id == $consumer.id)) as $node |
        ($node.deps[] | select(.name == "ic_wire") | .pkg) as $wire |
        ($node.deps[] | select(.name == "ic_testkit_wire") | .pkg) as $adapter |
        ($m.resolve.nodes[] | select(.id == $adapter) | .deps[] |
            select(.name == "ic_wire") | .pkg) == $wire)
' "$fixture/metadata.json" > /dev/null
cargo test --locked --offline --manifest-path "$fixture/Cargo.toml" --workspace \
    > "$fixture/test.log" 2>&1
for generation in 21 22; do
    diff -r "$root/crates/ic-testkit-wire/src" "$fixture/adapter-$generation/src"
    cmp "$root/crates/ic-testkit-wire/tests/pocket_ic_calls.rs" \
        "$fixture/consumer-$generation/tests/pocket_ic_calls.rs"
done
cat "$fixture/test.log"
echo 'Wire 0.21/0.22 callers retain their own typed limits/errors over one Testkit/PocketIC'
fixture_complete=true
