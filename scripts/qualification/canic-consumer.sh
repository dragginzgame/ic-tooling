#!/usr/bin/env bash
# Qualify Canic-owned host tests against the selected Foundation supplier.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
mode=${1:-consumer}
case $mode in
  consumer|custody|records|filesystem|config|starter|trees|locks|parents|runtime) ;;
  *) echo "Unknown Canic foundation selection: $mode" >&2; exit 2 ;;
esac
supplier=$(CDPATH='' cd -- "${IC_TOOLING_CANDIDATE_ROOT:-$repo}" && pwd -P)
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-consumer.XXXXXXXX")"
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
  (cd "$supplier" && find Cargo.toml crates/ic-host-{fs,artifacts} -type f -exec shasum -a 256 {} + | LC_ALL=C sort)
}
supplier_inputs > "$scratch/supplier-before.sha256"
patches=()
for package in ic-host-fs ic-host-artifacts; do
  path=$(jq -Rn --arg path "$supplier/crates/$package" '$path')
  patches+=(--config "patch.crates-io.$package.path=$path")
done
cargo metadata "${patches[@]}" --format-version 1 > "$scratch/metadata.json"
jq -e --arg root "$supplier" '
  . as $m | [.packages[] | select(.name == "canic-host" and (.id as $id | $m.workspace_members | index($id)))] as $hosts |
  ($hosts | length) == 1 and
  all(["ic-host-fs", "ic-host-artifacts"][]; . as $name |
    any($m.packages[]; .name == $name and .source == null and
      .manifest_path == ($root + "/crates/" + $name + "/Cargo.toml") and
      (.id as $id | any($m.resolve.nodes[] | select(.id == $hosts[0].id) | .deps[];
        .pkg == $id and any(.dep_kinds[]; .kind == null and .target == null)))))
' "$scratch/metadata.json" >/dev/null
cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --quiet -- \
  verify-external-lock "$scratch/consumer.lock" Cargo.lock 1048576

read_cases=(
  store::tests::read::exact_bytes_limits_and_original_missing_cause
  store::tests::read::final_links_refuse_and_ancestor_links_remain_supported
)
record_cases=(
  tests::plan_identity_admission_and_artifact_custody
  tests::retained_evidence_never_overwrites_and_store_excludes_concurrent_writer
  signed::tests::recovery_retains_exact_attempt_and_deadline_after_effect_expiry
  signed::tests::expired_uncertain_attempt_stays_retained_without_a_new_signature
  store::tests::publication::oversize_refuses_before_parent_creation
  store::tests::publication::record_creation_preserves_parent_mode_and_never_replaces_evidence
  signed::tests::maintenance_observation_cannot_sign_another_root_or_effect
  signed::tests::exact_attempt_binding_rejects_changed_authority_argument_network_and_id
  signed::tests::status_authority_must_bind_the_same_request
  signed::tests::observation_authority_cannot_dispatch_effects_after_approval_expiry
  signed::tests::scoped_observation_rejects_sibling_and_legacy_payloads_before_signing
  signed::tests::publication_read_authority_survives_effect_expiry_without_renewing_mutations
  signed::tests::store_readiness_authority_cannot_change_target_or_renew_handover
)
config_cases=(
  config::tests::nested_unknown_fields_refuse_edits_without_changing_config
  config::tests::concurrent_first_app_initializers_cannot_replace_a_winner
  config::tests::fleet_initialization_does_not_follow_config_symlinks
  config::tests::app_and_fleet_initialization_preserve_evidence_and_refuse_conflicts
  config::tests::app_definition_bounds_apply_to_resolved_fleets_before_publication
  config::tests::initialization_is_local_bounded_and_preserves_existing_files
)
tree_cases=(
  starter::tests::concurrent_initializers_publish_one_complete_project
  starter::tests::source_alias_resolves_physically_and_destination_symlinks_are_not_replaced
  starter::tests::starter_config_resolves_supplied_root_store_and_generated_app_without_effects
  starter::tests::invalid_names_or_source_are_refused_before_any_staging
  starter::tests::app_build_admits_all_recipes_before_creating_a_lock_or_output
  starter::tests::app_build_refuses_output_aliases_before_creating_a_lock_or_output
)
fleet_cases=(
  fleet_plan::tests::stored_fleet_admits_full_manifests_and_keeps_retained_inline_delivery
  fleet_plan::tests::fleet_store_is_an_independent_output_binding_and_debit
  fleet_plan::tests::fleet_upgrade_requires_completion_and_never_reopens_or_substitutes_authority
  fleet_plan::tests::fleet_authority_is_retained_before_outputs_and_replanning_never_renews_it
  fleet_plan::tests::fleet_refuses_unsupported_trees_conflicts_and_tampered_artifacts
  fleet_plan::tests::expired_unsubmitted_fleet_cannot_start_root_or_renew_its_schedule
  fleet_plan::tests::cancellation_retains_evidence_and_permanently_revokes_only_unprepared_work
  fleet_plan::tests::existing_fleet_approval_keeps_its_original_digest
  fleet_plan::tests::two_child_approval_retains_slot_mapping_and_cancels_only_before_preparation
  fleet_plan::tests::retained_two_child_approval_preserves_digest_and_rejects_slot_reassignment
)
lock_cases=(
  store::tests::lock::sidecar_preserves_inode_bytes_modes_and_legacy_contention
  store::tests::lock::unsafe_sidecars_refuse_without_replacing_foreign_entries
)
parent_cases=(
  store::tests::parent::private_parent_creation_preserves_existing_modes
  store::tests::parent::private_parent_refusals_precede_creation
  staging_directory::tests::collisions_preserve_foreign_entries_and_stop_at_budget
  staging_directory::tests::held_allocation_survives_parent_rename_and_rejects_substitution
  output_path::tests::missing_outputs_and_dangling_aliases_share_physical_identity
  output_path::tests::invalid_ancestors_and_cycles_refuse_without_effects
  output_path::tests::dangling_depth_budget_counts_only_active_expansions
  publication::tests::artifact_listing_preserves_unknown_outcomes_scope_and_corrupt_evidence
  publication::tests::immutable_release_plans_retain_deadlines_without_reserving_deployment_slots
  publication::tests::publication_rejects_conflicting_authority_and_retains_an_exact_read_contract
  publication::tests::publication_evidence_cannot_become_an_installation_receipt_or_change_the_manifest
  publication::tests::store_declarations_cannot_use_the_greeting_child_contract
  publication::tests::deployment_refuses_an_unpublished_release_before_reserving_the_child
)
custody_cases=(
  artifact::tests::inspection::exact_wasm_identity_follows_file_and_parent_aliases
  artifact::tests::inspection::inspection_refuses_non_wasm_oversize_and_nonregular_inputs
)
case $mode in
  consumer) cases=("${read_cases[@]}" "${record_cases[@]}") ;;
  custody) cases=("${custody_cases[@]}") ;;
  config) cases=("${config_cases[@]}") ;;
  records) cases=("${config_cases[@]}" "${record_cases[@]}") ;;
  starter) cases=("${tree_cases[@]}") ;;
  trees) cases=("${tree_cases[@]}" "${fleet_cases[@]}") ;;
  *)
    cases=("${read_cases[@]}" "${record_cases[@]}" "${config_cases[@]}" "${tree_cases[@]}" "${fleet_cases[@]}" "${custody_cases[@]}")
    if [[ $mode != filesystem ]]; then cases+=("${lock_cases[@]}"); fi
    if [[ $mode == parents || $mode == runtime ]]; then cases+=("${parent_cases[@]}"); fi ;;
esac

source "$repo/scripts/qualification/canic-cases.sh"
run_cases canic-host --lib "${cases[@]}"
if [[ $mode == runtime ]]; then
  run_cases canic-cli --bin=canic fleet::tests::report_escapes_untrusted_text_and_labels_partial_inventory \
    view::tests::opener_handoff_retains_child_without_opening_a_browser \
    view::tests::viewer_serves_only_the_report_over_loopback
fi
if [[ $mode == custody || $mode == runtime ]]; then
  mkdir "$scratch/workspace" "$scratch/output"
  chmod 755 "$scratch/output"
  cp -R "$repo/qualification/owned-wasm-probe" "$scratch/workspace/"
  cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$scratch/workspace/"
  build=(cargo run "${patches[@]}" --quiet --locked --target-dir "$CARGO_TARGET_DIR" -p canic-cli --bin canic --
    --json artifact build --manifest "$scratch/workspace/Cargo.toml" --package owned-wasm-probe)
  output="$scratch/output/new/artifact.wasm"
  CARGO_TARGET_DIR="$scratch/build" "${build[@]}" --output "$output" > "$scratch/artifact.json"
  hash=$(shasum -a 256 "$output" | cut -d ' ' -f 1)
  jq -e --arg path "$output" --arg hash "$hash" '.wasm == $path and .artifact.sha256 == $hash' "$scratch/artifact.json" >/dev/null
  [[ -n $(find "$scratch/output" -prune -perm 0755 -print) ]]
  [[ -n $(find "$scratch/output/new" -prune -perm 0700 -print) ]]
  [[ -n $(find "$output" -prune -perm 0600 -print) ]]
  ln -s output "$scratch/alias"
  if CARGO_TARGET_DIR="$scratch/build" "${build[@]}" --output "$scratch/alias/foreign" > "$scratch/refusal.stdout" 2> "$scratch/refusal.json"; then
    echo 'Canic unexpectedly wrote through a parent symlink' >&2; exit 1
  fi
  jq -e '.error == "supplier" and (.message | contains("filesystem link/type refused"))' "$scratch/refusal.json" >/dev/null
  [[ ! -e "$scratch/output/foreign" && ! -L "$scratch/output/foreign" ]]
fi
supplier_inputs > "$scratch/supplier-after.sha256"
cmp "$scratch/supplier-before.sha256" "$scratch/supplier-after.sha256"
