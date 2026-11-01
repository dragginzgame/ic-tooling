#!/usr/bin/env bash
set -euo pipefail
# Manual qualification against caller-selected prepared tools and Platform vectors.
# No installation, publication, sibling edit or application profile change.
[[ $# == 5 && ! -e "$2" && -x "$3" && -x "$4" && -x "$5" ]] || {
    echo 'usage: qualify-wasm-backend.sh PLATFORM_SOURCE NEW_EVIDENCE_DIRECTORY WRAPPER WASM_OPT POCKET_IC' >&2
    exit 2
}
root="$(cd "$(dirname "$0")/../.." && pwd -P)"
platform="$(cd "$1" && pwd -P)"
mkdir -p "$2"
evidence="$(cd "$2" && pwd -P)"
host_target="${WASM_RECIPE_HOST_TARGET_DIR:-$evidence/host-target}"
wrapper="$(realpath "$3")"
optimizer="$(realpath "$4")"
server="$(realpath "$5")"
mkdir -p "$evidence/probe/src" "$evidence/driver/src" "$evidence/vectors/src"
# The crypto vector body and Wasm endpoint remain maintained by Platform.
cp "$platform/scripts/fixtures/wasm-size/lib.rs" "$evidence/probe/src/lib.rs"
cp "$platform/scripts/fixtures/ecdsa-benchmark/main.rs" "$evidence/vectors/src/main.rs"
cp "$root/scripts/fixtures/wasm-recipe/main.rs" "$evidence/driver/src/main.rs"
cp "$root/scripts/ci/qualify-wasm-backend.sh" "$evidence/qualify-wasm-backend.sh"
git -C "$platform" rev-parse HEAD > "$evidence/vector-source-head.txt"
git -C "$platform" diff --binary -- scripts/fixtures/wasm-size/lib.rs scripts/fixtures/ecdsa-benchmark/main.rs \
    > "$evidence/vector-source.patch"
shasum -a 256 "$evidence/probe/src/lib.rs" "$evidence/vectors/src/main.rs" \
    "$evidence/driver/src/main.rs" "$evidence/qualify-wasm-backend.sh" \
    "$wrapper" "$optimizer" "$server" > "$evidence/source-tools.sha256"
quote() { perl -MJSON::PP -e 'print encode_json($ARGV[0])' "$1"; }
cat > "$evidence/probe/Cargo.toml" <<'MANIFEST'
[package]
name = "tooling-ecdsa-recipe-probe"
version = "0.0.0"
edition = "2024"
[workspace]
[lib]
crate-type = ["cdylib"]
[features]
default = ["ecdsa-instructions"]
ecdsa = []
ecdsa-instructions = ["ecdsa"]
[dependencies]
ic-platform = { version = "=0.16.0", default-features = false, features = ["ic0", "ecdsa"] }
[profile.release]
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
strip = "symbols"
MANIFEST
cat > "$evidence/driver/Cargo.toml" <<MANIFEST
[package]
name = "tooling-qualify-wasm-recipe"
version = "0.0.0"
edition = "2024"
[workspace]
[dependencies]
clap = { version = "=4.6.7", features = ["derive"] }
ic-build = { path = $(quote "$root/crates/ic-build") }
ic-host-artifacts = { path = $(quote "$root/crates/ic-host-artifacts"), features = ["wasm"] }
ic-host-fs = { path = $(quote "$root/crates/ic-host-fs") }
ic-host-process = { path = $(quote "$root/crates/ic-host-process") }
serde_json = "1"
MANIFEST
cat > "$evidence/vectors/Cargo.toml" <<'MANIFEST'
[package]
name = "tooling-ecdsa-vectors"
version = "0.0.0"
edition = "2024"
[workspace]
[dependencies]
ic-platform = { version = "=0.16.0", default-features = false, features = ["ecdsa", "pocket-ic"] }
ic-testkit = "=0.76.1"
MANIFEST
export RUSTUP_AUTO_INSTALL=0 CARGO_NET_OFFLINE=true
for package in probe driver vectors; do
    cargo +1.91.0 generate-lockfile --offline --manifest-path "$evidence/$package/Cargo.toml" \
        > "$evidence/$package-lock.log" 2>&1
    cargo +1.91.0 metadata --frozen --format-version 1 --manifest-path "$evidence/$package/Cargo.toml" \
        > "$evidence/$package-metadata.json" 2> "$evidence/$package-metadata.log"
done
# Exact supplier admission belongs to this crypto-specific fixture, not Build.
jq -ne --slurpfile graph "$evidence/probe-metadata.json" \
    'all([["ic-platform", "0.16.0"], ["k256", "0.14.0"], ["crypto-bigint", "0.7.10"]][];
    . as $required | [$graph[0].packages[] | select(.name == $required[0]) | .version] == [$required[1]])' > /dev/null
CARGO_TARGET_DIR="$host_target" cargo +1.91.0 clippy --frozen \
    --manifest-path "$evidence/driver/Cargo.toml" -- -D warnings > "$evidence/driver-clippy.log" 2>&1
for package in driver vectors; do
    CARGO_TARGET_DIR="$host_target" cargo +1.91.0 build --frozen \
        --manifest-path "$evidence/$package/Cargo.toml" > "$evidence/$package-build.log" 2>&1
done
CARGO="$(rustup which --toolchain 1.91.0 cargo)" RUSTC="$(rustup which --toolchain 1.91.0 rustc)" \
    RUSTUP_TOOLCHAIN=1.91.0 "$host_target/debug/tooling-qualify-wasm-recipe" \
    --output "$evidence" --wrapper "$wrapper" --optimizer "$optimizer" > "$evidence/build-recipes.log" 2>&1
POCKET_IC_BIN="$server" "$host_target/debug/tooling-ecdsa-vectors" \
    64 "$evidence/cpubits-64.optimized.wasm" 32 "$evidence/cpubits-32.optimized.wasm" \
    > "$evidence/instructions.tsv" 2> "$evidence/runtime.log"
shasum -a 256 --check "$evidence/source-tools.sha256" > "$evidence/custody-check.log"
shasum -a 256 "$evidence"/*.wasm "$evidence"/*/Cargo.lock "$evidence/recipes.json" \
    "$evidence/instructions.tsv" > "$evidence/results.sha256"
printf 'Final Wasm, compiler/post-link recipes and installed vector evidence: %s\n' "$evidence"
