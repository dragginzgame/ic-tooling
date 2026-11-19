#!/usr/bin/env bash
# Capture current Canic and verify each selected dependency of canic-host.
supplier=$(CDPATH='' cd -- "${IC_TOOLING_CANDIDATE_ROOT:-$repo}" && pwd -P)
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-${capture_name}.XXXXXXXX")"
trap 'printf "Canic qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
cd "$scratch/source"
# Recheck source bytes when a captured file predates shared Cargo outputs.
find . -type f -exec touch {} +
[[ -f Cargo.lock && ! -L Cargo.lock ]]
cp Cargo.lock "$scratch/consumer.lock"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build"
export RUSTUP_AUTO_INSTALL=0
# Keep all candidate overrides on the Cargo command line. Only the exported
# lock may change. Canic owns its source, manifests, features and test modules.
supplier_inputs() {
  (cd "$supplier" && find Cargo.toml "${supplier_dirs[@]}" -type f -exec shasum -a 256 {} + | LC_ALL=C sort)
}
supplier_dirs=("${packages[@]/#/crates/}")
supplier_inputs > "$scratch/supplier-before.sha256"
patches=()
for package in "${packages[@]}"; do
  path=$(jq -Rn --arg path "$supplier/crates/$package" '$path')
  patches+=(--config "patch.crates-io.$package.path=$path")
done
cargo metadata "${patches[@]}" --format-version 1 > "$scratch/metadata.json"
jq -e --arg root "$supplier" --args '
  . as $m | [.packages[] | select(.name == "canic-host" and (.id as $id | $m.workspace_members | index($id)))] as $hosts |
  ($hosts | length) == 1 and
  all($ARGS.positional[]; . as $name |
    any($m.packages[]; .name == $name and .source == null and
      .manifest_path == ($root + "/crates/" + $name + "/Cargo.toml") and
      (.id as $id | any($m.resolve.nodes[] | select(.id == $hosts[0].id) | .deps[];
        .pkg == $id and any(.dep_kinds[]; .kind == null and .target == null)))))
' "${packages[@]}" < "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --quiet -- \
  verify-external-lock "$scratch/consumer.lock" Cargo.lock 1048576
