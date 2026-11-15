#!/usr/bin/env bash
# Exercise the consumer's Store source policy with a tiny local Wasm fixture.
set -euo pipefail
cli="$1"
scratch="$2"
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
supplier="$scratch/supplier"
mkdir -p "$supplier/crates" "$scratch/store-tools" "$scratch/store-workspace"
cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$supplier/"
cp -R "$repo/qualification/owned-wasm-probe" "$supplier/crates/ic-wasm-store"
sed -i 's/owned-wasm-probe/crates\/ic-wasm-store/' "$supplier/Cargo.toml"
sed -i 's/owned-wasm-probe/ic-wasm-store/g' "$supplier/Cargo.lock" "$supplier/crates/ic-wasm-store/Cargo.toml"
cp "$repo/LICENSE.md" "$scratch/LICENSE.md"
printf '[workspace]\nmembers = []\n' > "$scratch/store-workspace/Cargo.toml"
# The resolver consumes recorded metadata; the fixture never downloads Store code.
revision=0123456789012345678901234567890123456789
jq -n --arg manifest "$supplier/crates/ic-wasm-store-protocol/Cargo.toml" --arg rev "$revision" '{packages:[{name:"ic-wasm-store-protocol",source:("git+https://github.com/dragginzgame/ic-wasm-store?rev="+$rev+"#"+$rev),manifest_path:$manifest}]}' > "$scratch/store-source.json"
cat > "$scratch/store-tools/cargo" <<'TOOL'
#!/usr/bin/env bash
set -euo pipefail
[[ "$1" == metadata && "$2" == --locked ]]
printf 'lookup\n' >> "$CANIC_STORE_LOOKUPS"
cat "$CANIC_STORE_METADATA"
TOOL
chmod 700 "$scratch/store-tools/cargo"
export CANIC_STORE_METADATA="$scratch/store-source.json" CANIC_STORE_LOOKUPS="$scratch/store-lookups"
sha256sum "$supplier/Cargo.lock" "$supplier/Cargo.toml" "$supplier/crates/ic-wasm-store/Cargo.toml" "$supplier/crates/ic-wasm-store/src/lib.rs" > "$scratch/store-source-before.sha256"
tar -cf "$scratch/supplier-before.tar" -C "$supplier" .
PATH="$scratch/store-tools:$PATH" "$cli" artifact build --manifest "$scratch/store-workspace/Cargo.toml" --package ic-wasm-store > "$scratch/store-build.json"
output="$scratch/store-workspace/target/store-build/wasm32-unknown-unknown/release/ic_wasm_store.wasm"
[[ -f "$output" && ! -e "$supplier/target" && "$(wc -l < "$CANIC_STORE_LOOKUPS")" == 1 ]]
rg -a -q 'owned-wasm-probe!' "$output"
jq -e --arg output "$output" '.wasm == $output and .artifact.bytes > 8' "$scratch/store-build.json" >/dev/null
sha256sum --check "$scratch/store-source-before.sha256"
tar -cf "$scratch/supplier-after.tar" -C "$supplier" .
cmp "$scratch/supplier-before.tar" "$scratch/supplier-after.tar"
printf 'Locked Store source selection and consumer-owned default output passed without supplier writes.\n'
