#!/usr/bin/env bash
# Real current Canic CLI build/publication proof; no deployments or sibling edits.
set -euo pipefail
mode="${1:-artifact}"
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
case "$mode" in
  custody|config|records|starter|trees|filesystem|locks|parents|runtime)
    exec bash "$repo/scripts/qualification/canic-consumer.sh" "$mode" ;;
  artifact|metadata|app|process) ;;
  *) exit 2 ;;
esac
capture_name=build
packages=(ic-host-fs ic-host-artifacts ic-host-process)
source "$repo/scripts/qualification/canic-host-source.sh"
source "$repo/scripts/qualification/canic-cases.sh"
mkdir "$scratch/probe" "$scratch/runtime"
cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$scratch/probe/"
cp -R "$repo/qualification/owned-wasm-probe" "$scratch/probe/"
CARGO_BUILD_BUILD_DIR="$CARGO_TARGET_DIR" cargo build "${patches[@]}" --locked \
  --target-dir "$scratch/cli" -p canic-cli --bin canic
cli="$scratch/cli/debug/canic"
if [[ $mode == app || $mode == process ]]; then
  run_cases canic-host --lib starter::tests::app_build_admits_all_recipes_before_creating_a_lock_or_output \
    starter::tests::app_build_refuses_output_aliases_before_creating_a_lock_or_output \
    starter::tests::app_build_preserves_config_and_cargo_inputs_including_unselected_recipes
fi
if [[ $mode == process ]]; then
  run_cases canic-host --lib process::tests:: artifact::tests:: artifact::store_source::tests::
  run_cases canic-cli --bin=canic view::tests:: fleet::terminal::tests:: placements::tests:: status::tests:: subnet_comparison::tests:: artifacts::tests::
fi
export CARGO_TARGET_DIR="$scratch/published" TMPDIR="$scratch/runtime"
case $mode in
  artifact) bash "$repo/scripts/qualification/canic-artifact-build.sh" "$cli" "$scratch" ;;
  metadata) bash "$repo/scripts/qualification/canic-build-metadata.sh" "$cli" "$scratch" ;;
  app|process)
    bash "$repo/scripts/qualification/canic-app-build.sh" "$cli" "$scratch"
    if [[ $mode == process ]]; then bash "$repo/scripts/qualification/canic-build-metadata.sh" "$cli" "$scratch"; fi ;;
esac
supplier_inputs > "$scratch/supplier-after.sha256"
cmp "$scratch/supplier-before.sha256" "$scratch/supplier-after.sha256"
