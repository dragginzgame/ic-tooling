#!/usr/bin/env bash
# Real current Canic CLI build/publication proof; no deployments or sibling edits.
set -euo pipefail
mode="${1:-artifact}"
foreground=false
lock_migration=false
parent_migration=false
if [[ "$mode" == parents ]]; then mode=locks; parent_migration=true; fi
if [[ "$mode" == locks ]]; then mode=filesystem; lock_migration=true; fi
case "$mode" in
  artifact|app|metadata|custody|config|records|starter|trees|filesystem) ;;
  process)
    [[ "$(uname -s)" == Linux ]] || { echo 'Combined foreground PTY qualification requires Linux; macOS remains separate.' >&2; exit 1; }
    mode=app
    foreground=true
    ;;
  *) exit 2 ;;
esac
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-build.XXXXXXXX")"
trap 'printf "Canic Build export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/probe" "$scratch/runtime"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
# Tar preserves sibling mtimes, which can predate a shared target's freshness
# observations even when a newly captured path package has different bytes.
# Refresh only export timestamps so Cargo rechecks these captured source inputs.
find "$scratch/source" -type f -exec touch {} +
if [[ "$mode" == records || "$mode" == filesystem ]]; then
  [[ ! -L "$scratch/source/crates/canic-host/src/store" ]]
  [[ ! -e "$scratch/source/crates/canic-host/src/store/publication_tests.rs" && ! -L "$scratch/source/crates/canic-host/src/store/publication_tests.rs" ]]
  mkdir -p "$scratch/source/crates/canic-host/src/store"
  cp "$repo/qualification/canic/record-publication-tests.rs" "$scratch/source/crates/canic-host/src/store/publication_tests.rs"
  printf '\n#[cfg(test)]\nmod publication_tests;\n' >> "$scratch/source/crates/canic-host/src/store.rs"
fi
if [[ "$mode" == filesystem ]]; then
  [[ ! -e "$scratch/source/crates/canic-host/src/store/read_tests.rs" && ! -L "$scratch/source/crates/canic-host/src/store/read_tests.rs" ]]
  cp "$repo/qualification/canic/record-read-tests.rs" "$scratch/source/crates/canic-host/src/store/read_tests.rs"
  printf '\n#[cfg(test)]\nmod read_tests;\n' >> "$scratch/source/crates/canic-host/src/store.rs"
fi
if [[ "$lock_migration" == true ]]; then
  [[ ! -e "$scratch/source/crates/canic-host/src/store/sidecar_tests.rs" && ! -L "$scratch/source/crates/canic-host/src/store/sidecar_tests.rs" ]]
  cp "$repo/qualification/canic/sidecar-lock-tests.rs" "$scratch/source/crates/canic-host/src/store/sidecar_tests.rs"
  printf '\n#[cfg(test)]\nmod sidecar_tests;\n' >> "$scratch/source/crates/canic-host/src/store.rs"
fi
if [[ "$parent_migration" == true ]]; then
  [[ ! -e "$scratch/source/crates/canic-host/src/store/parent_tests.rs" && ! -L "$scratch/source/crates/canic-host/src/store/parent_tests.rs" ]]
  cp "$repo/qualification/canic/parent-preparation-tests.rs" "$scratch/source/crates/canic-host/src/store/parent_tests.rs"
  printf '\n#[cfg(test)]\nmod parent_tests;\n' >> "$scratch/source/crates/canic-host/src/store.rs"
fi
writer_checks() {
  local selection
  if [[ "$parent_migration" == true ]]; then
    for selection in artifact_listing_preserves_unknown_outcomes_scope_and_corrupt_evidence immutable_release_plans_retain_deadlines_without_reserving_deployment_slots publication_rejects_conflicting_authority_and_retains_an_exact_read_contract publication_evidence_cannot_become_an_installation_receipt_or_change_the_manifest store_declarations_cannot_use_the_greeting_child_contract deployment_refuses_an_unpublished_release_before_reserving_the_child; do
      CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "publication::tests::$selection" -- --exact || return
    done
    for selection in private_parent_creation_preserves_existing_modes private_parent_refusals_precede_creation; do
      CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "store::parent_tests::$selection" -- --exact || return
    done
  fi
  if [[ "$lock_migration" == true ]]; then
    for selection in sidecar_preserves_inode_bytes_modes_and_legacy_contention unsafe_sidecars_refuse_without_replacing_foreign_entries; do
      CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "store::sidecar_tests::$selection" -- --exact || return
    done
  fi
  if [[ "$mode" == filesystem ]]; then
    for selection in exact_bytes_limits_and_original_missing_cause final_links_refuse_and_ancestor_links_remain_supported; do
      CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "store::read_tests::$selection" -- --exact || return
    done
  fi
  if [[ "$mode" == starter || "$mode" == trees || "$mode" == filesystem ]]; then
    for selection in starter_config_resolves_supplied_root_store_and_generated_app_without_effects invalid_names_or_source_are_refused_before_any_staging app_build_admits_all_recipes_before_creating_a_lock_or_output app_build_refuses_output_aliases_before_creating_a_lock_or_output source_alias_resolves_physically_and_destination_symlinks_are_not_replaced concurrent_initializers_publish_one_complete_project; do
      CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "starter::tests::$selection" -- --exact || return
    done
    if [[ "$mode" == trees || "$mode" == filesystem ]]; then
      for selection in stored_fleet_admits_full_manifests_and_keeps_retained_inline_delivery fleet_store_is_an_independent_output_binding_and_debit fleet_upgrade_requires_completion_and_never_reopens_or_substitutes_authority fleet_authority_is_retained_before_outputs_and_replanning_never_renews_it fleet_refuses_unsupported_trees_conflicts_and_tampered_artifacts expired_unsubmitted_fleet_cannot_start_root_or_renew_its_schedule cancellation_retains_evidence_and_permanently_revokes_only_unprepared_work existing_fleet_approval_keeps_its_original_digest two_child_approval_retains_slot_mapping_and_cancels_only_before_preparation retained_two_child_approval_preserves_digest_and_rejects_slot_reassignment; do
        CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "fleet_plan::tests::$selection" -- --exact || return
      done
    fi
    if [[ "$mode" != filesystem ]]; then return 0; fi
  fi
  for selection in nested_unknown_fields_refuse_edits_without_changing_config app_and_fleet_initialization_preserve_evidence_and_refuse_conflicts concurrent_first_app_initializers_cannot_replace_a_winner app_definition_bounds_apply_to_resolved_fleets_before_publication fleet_initialization_does_not_follow_config_symlinks initialization_is_local_bounded_and_preserves_existing_files; do
    CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "config::tests::$selection" -- --exact || return
  done
  if [[ "$mode" == records || "$mode" == filesystem ]]; then
    for selection in tests::plan_identity_admission_and_artifact_custody tests::retained_evidence_never_overwrites_and_store_excludes_concurrent_writer store::publication_tests::oversize_refuses_before_parent_creation store::publication_tests::record_creation_preserves_parent_mode_and_never_replaces_evidence; do
      CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "$selection" -- --exact || return
    done
    for selection in maintenance_observation_cannot_sign_another_root_or_effect recovery_retains_exact_attempt_and_deadline_after_effect_expiry exact_attempt_binding_rejects_changed_authority_argument_network_and_id status_authority_must_bind_the_same_request expired_uncertain_attempt_stays_retained_without_a_new_signature observation_authority_cannot_dispatch_effects_after_approval_expiry scoped_observation_rejects_sibling_and_legacy_payloads_before_signing publication_read_authority_survives_effect_expiry_without_renewing_mutations store_readiness_authority_cannot_change_target_or_renew_handover; do
      CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "signed::tests::$selection" -- --exact || return
    done
  fi
}
if [[ "$mode" == config || "$mode" == records || "$mode" == filesystem || "$mode" == starter || "$mode" == trees ]]; then
  writer_checks > "$scratch/$mode-baseline.log" 2>&1 || { tail -40 "$scratch/$mode-baseline.log"; exit 1; }
  if rg -q '^test result: ok\. 0 passed' "$scratch/$mode-baseline.log"; then
    echo 'Required writer test missing; migration was not applied.' >&2; exit 1
  fi
fi
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/canic/owned-build.patch"
git -C "$scratch/source" apply "$repo/qualification/canic/owned-build.patch"
if [[ "$mode" == custody || "$mode" == config || "$mode" == records || "$mode" == filesystem || "$mode" == starter || "$mode" == trees ]]; then
  cp "$scratch/source/Cargo.lock" "$scratch/admitted-build.lock"
  sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/artifact-publication.patch" > "$scratch/publication.patch"
  git -C "$scratch/source" apply --check "$scratch/publication.patch"
  git -C "$scratch/source" apply "$scratch/publication.patch"
  [[ ! -e "$scratch/source/crates/canic-host/src/output_publication.rs" && ! -L "$scratch/source/crates/canic-host/src/output_publication.rs" ]]
  cp "$repo/qualification/canic/artifact-publication.rs" "$scratch/source/crates/canic-host/src/output_publication.rs"
  if [[ "$mode" == config || "$mode" == records || "$mode" == filesystem ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/config-publication.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/config-publication.patch"
  fi
  if [[ "$mode" == records || "$mode" == filesystem ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/record-publication.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/record-publication.patch"
  fi
  if [[ "$mode" == starter || "$mode" == trees || "$mode" == filesystem ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/starter-publication.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/starter-publication.patch"
  fi
  if [[ "$mode" == trees || "$mode" == filesystem ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/fleet-publication.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/fleet-publication.patch"
  fi
  if [[ "$mode" == filesystem ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/record-read.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/record-read.patch"
  fi
  if [[ "$lock_migration" == true ]]; then
    [[ ! -e "$scratch/source/crates/canic-host/src/sidecar_lock.rs" && ! -L "$scratch/source/crates/canic-host/src/sidecar_lock.rs" ]]
    cp "$repo/qualification/canic/sidecar-lock.rs" "$scratch/source/crates/canic-host/src/sidecar_lock.rs"
    git -C "$scratch/source" apply --check "$repo/qualification/canic/sidecar-lock.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/sidecar-lock.patch"
  fi
  if [[ "$parent_migration" == true ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/parent-preparation.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/parent-preparation.patch"
    [[ ! -e "$scratch/source/crates/canic-host/src/staging_directory.rs" && ! -L "$scratch/source/crates/canic-host/src/staging_directory.rs" ]]
    [[ ! -e "$scratch/source/crates/canic-host/src/staging_directory" && ! -L "$scratch/source/crates/canic-host/src/staging_directory" ]]
    cp "$repo/qualification/canic/staging-directory.rs" "$scratch/source/crates/canic-host/src/staging_directory.rs"
    mkdir "$scratch/source/crates/canic-host/src/staging_directory"
    cp "$repo/qualification/canic/staging-directory-tests.rs" "$scratch/source/crates/canic-host/src/staging_directory/tests.rs"
    git -C "$scratch/source" apply --check "$repo/qualification/canic/staging-directory.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/staging-directory.patch"
    git -C "$scratch/source" apply --check "$repo/qualification/canic/starter-members.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/starter-members.patch"
    git -C "$scratch/source" apply --check "$repo/qualification/canic/fleet-members.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/fleet-members.patch"
    git -C "$scratch/source" apply --check "$repo/qualification/canic/recovery-preparation.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/recovery-preparation.patch"
  fi
  cargo metadata --manifest-path "$scratch/source/Cargo.toml" --format-version 1 --offline > "$scratch/publication-metadata.json"
  jq -e --arg root "$repo" '. as $m |
    (.packages[] | select(.name == "canic-host") | .id) as $host |
    (.packages[] | select(.name == "ic-host-fs" and .version == "0.80.1" and .source == null and .manifest_path == ($root + "/crates/ic-host-fs/Cargo.toml")) | .id) as $fs |
    (.packages[] | select(.name == "ic-host-artifacts" and .version == "0.80.1" and .source == null and .manifest_path == ($root + "/crates/ic-host-artifacts/Cargo.toml")) | .id) as $bytes |
    any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_fs_next" and .pkg == $fs) and
    any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_bytes_next" and .pkg == $bytes)
  ' "$scratch/publication-metadata.json" >/dev/null
  cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/admitted-build.lock" "$scratch/source/Cargo.lock" 1048576
  CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo clippy --manifest-path "$scratch/source/Cargo.toml" -p canic-host --all-targets --locked --offline -- -D warnings
  if [[ "$mode" == config || "$mode" == records || "$mode" == filesystem || "$mode" == starter || "$mode" == trees ]]; then
    writer_checks > "$scratch/$mode-replacement.log" 2>&1
    for branch in baseline replacement; do
      if rg -q '^test result: ok\. 0 passed' "$scratch/$mode-$branch.log"; then
        echo 'Required writer test missing' >&2; exit 1
      fi
      rg '^test .* \.\.\. ok$' "$scratch/$mode-$branch.log" | LC_ALL=C sort > "$scratch/$mode-$branch-tests.txt"
    done
    cmp "$scratch/$mode-baseline-tests.txt" "$scratch/$mode-replacement-tests.txt"
    cat "$scratch/$mode-replacement-tests.txt"
    if [[ "$parent_migration" == true ]]; then
      for selection in collisions_preserve_foreign_entries_and_stop_at_budget held_allocation_survives_parent_rename_and_rejects_substitution; do
        CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline "staging_directory::tests::$selection" -- --exact > "$scratch/staging-$selection.log" 2>&1
        rg '^test result: ok\. 1 passed' "$scratch/staging-$selection.log"
      done
    fi
    exit 0
  fi
fi
if [[ "$mode" == app ]]; then
  cp "$scratch/source/Cargo.lock" "$scratch/admitted-build.lock"
  git -C "$scratch/source" apply --check "$repo/qualification/canic/text.patch"
  git -C "$scratch/source" apply "$repo/qualification/canic/text.patch"
  sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/lock-process.patch" > "$scratch/lock-process.patch"
  git -C "$scratch/source" apply --check "$scratch/lock-process.patch"
  git -C "$scratch/source" apply "$scratch/lock-process.patch"
  # The lock migration already selects the identical local Process dependency.
  # Apply only signer implementation hunks; graph admission below proves selection.
  git -C "$scratch/source" apply --check --exclude=Cargo.toml --exclude=crates/canic-host/Cargo.toml "$repo/qualification/canic/signer-process.patch"
  git -C "$scratch/source" apply --exclude=Cargo.toml --exclude=crates/canic-host/Cargo.toml "$repo/qualification/canic/signer-process.patch"
  source "$repo/scripts/qualification/canic-signer-fixture.sh"
  prepare_signer_fixture "$scratch/source" "$scratch/signer-evidence"
  if [[ "$foreground" == true ]]; then
    # The existing local alias is shared; retain the client's dependency hunk.
    git -C "$scratch/source" apply --check --exclude=Cargo.toml "$repo/qualification/canic/foreground-process.patch"
    git -C "$scratch/source" apply --exclude=Cargo.toml "$repo/qualification/canic/foreground-process.patch"
    source "$repo/scripts/qualification/canic-foreground-fixture.sh"
    prepare_foreground_fixture "$scratch/source" "$scratch/foreground-evidence"
  fi
  cp "$repo/qualification/canic/lock-process-tests.rs" "$scratch/source/crates/canic-host/src/lock_process_tests.rs"
  printf '\n#[cfg(test)]\n#[path = "lock_process_tests.rs"]\nmod lock_process_tests;\n' >> "$scratch/source/crates/canic-host/src/app_build.rs"
  if [[ "$foreground" == true ]]; then
    git -C "$scratch/source" apply --check "$repo/qualification/canic/metadata-process.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/metadata-process.patch"
    git -C "$scratch/source" apply --check "$repo/qualification/canic/store-source-process.patch"
    git -C "$scratch/source" apply "$repo/qualification/canic/store-source-process.patch"
    git -C "$scratch/source" apply --check --exclude=Cargo.toml "$repo/qualification/canic/viewer-process.patch"
    git -C "$scratch/source" apply --exclude=Cargo.toml "$repo/qualification/canic/viewer-process.patch"
    sed "s|@IC_TOOLING_NEW@|$repo|g" "$repo/qualification/canic/retire-process-declaration.patch" > "$scratch/retire-process.patch"
    git -C "$scratch/source" apply --check "$scratch/retire-process.patch"
    git -C "$scratch/source" apply "$scratch/retire-process.patch"
    cp "$repo/qualification/canic/viewer-launch-tests.rs" "$scratch/source/crates/canic-cli/src/view/launch_qualification.rs"
    printf '\n#[cfg(test)]\nmod launch_qualification;\n' >> "$scratch/source/crates/canic-cli/src/view/mod.rs"
    mkdir "$scratch/viewer-evidence"
  fi
  cargo metadata --manifest-path "$scratch/source/Cargo.toml" --format-version 1 --offline > "$scratch/composed-metadata.json"
  cargo run --manifest-path "$repo/Cargo.toml" -p ic-cargo-qualification --locked --offline -- verify-external-lock "$scratch/admitted-build.lock" "$scratch/source/Cargo.lock" 1048576
fi
cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$scratch/probe/"
cp -R "$repo/qualification/owned-wasm-probe" "$scratch/probe/"
IC_OWNED_CARGO="$(rustup which --toolchain 1.100.0 cargo)"
IC_OWNED_RUSTC="$(rustup which --toolchain 1.100.0 rustc)"
export IC_OWNED_CARGO IC_OWNED_RUSTC
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 TMPDIR="$scratch/runtime"
cargo metadata --manifest-path "$scratch/source/Cargo.toml" --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '. as $m |
  (.packages[] | select(.name == "canic-host") | .id) as $host |
  (.packages[] | select(.name == "ic-build" and .version == "0.87.3" and
    .manifest_path == ($root + "/crates/ic-build/Cargo.toml")) | .id) as $build |
  (.packages[] | select(.name == "ic-host-process" and .version == "0.80.2" and
    .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
  any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_build" and .pkg == $build) and
  any($m.resolve.nodes[] | select(.id == $build) | .deps[]; .name == "ic_host_process" and .pkg == $process)
' "$scratch/metadata.json" >/dev/null
if [[ "$mode" == app ]]; then
  if [[ "$foreground" == true ]]; then
    jq -e --arg root "$repo" '[.packages[] | select(.name == "ic-host-process")] |
      length == 1 and .[0].version == "0.80.2" and .[0].source == null and
      .[0].manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")
    ' "$scratch/metadata.json" >/dev/null
    jq -e '. as $m | all($m.workspace_members[]; . as $member |
      all($m.resolve.nodes[] | select(.id == $member) | .deps[]; .name != "ic_host_process" or (.pkg | startswith("path+"))))' "$scratch/metadata.json" >/dev/null
    jq -e '. as $m |
      (.packages[] | select(.name == "canic-host") | .id) as $host |
      all($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name != "ic_host_process")
    ' "$scratch/metadata.json" >/dev/null
    jq -e --arg root "$repo" '. as $m |
      (.packages[] | select(.name == "canic-profile-client") | .id) as $client |
      (.packages[] | select(.name == "ic-host-process" and .version == "0.80.2" and .manifest_path == ($root + "/crates/ic-host-process/Cargo.toml")) | .id) as $process |
      any($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name == "ic_host_process_next" and .pkg == $process) and
      all($m.resolve.nodes[] | select(.id == $client) | .deps[]; .name != "ic_host_process")
    ' "$scratch/metadata.json" >/dev/null
  fi
  jq -e '. as $m |
    (.packages[] | select(.name == "canic-host") | .id) as $host |
    (.packages[] | select(.name == "ic-host-process" and .version == "0.80.2") | .id) as $process |
    any($m.resolve.nodes[] | select(.id == $host) | .deps[]; .name == "ic_host_process_next" and .pkg == $process)
  ' "$scratch/metadata.json" >/dev/null
  jq -e --arg root "$repo" '. as $m |
    (.packages[] | select(.name == "canic-cli") | .id) as $cli |
    [.packages[] | select(.name == "ic-host-text")] as $text |
    ($text | length == 1) and $text[0].version == "0.77.2" and
    $text[0].manifest_path == ($root + "/crates/ic-host-text/Cargo.toml") and
    any($m.resolve.nodes[] | select(.id == $cli) | .deps[];
      .name == "ic_host_text" and .pkg == $text[0].id)
  ' "$scratch/metadata.json" >/dev/null
fi
CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo build --manifest-path "$scratch/source/Cargo.toml" -p canic-cli --bin canic --locked --offline
cli="$repo/target/canic-consumer-build/debug/canic"
# Deliberately occupied shared output: the build must capture its private output.
export CARGO_TARGET_DIR="$scratch/published"
if [[ "$mode" == metadata ]]; then
  bash "$repo/scripts/qualification/canic-build-metadata.sh" "$cli" "$scratch"
  exit 0
fi
if [[ "$mode" == app ]]; then
  CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo clippy --manifest-path "$scratch/source/Cargo.toml" -p canic-host -p canic-cli --all-targets --locked --offline -- -D warnings
  CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline lock_process_tests::
  CARGO_TARGET_DIR="$repo/target/canic-consumer-build" test_signer_fixture "$scratch/source" "$scratch/signer-evidence"
  if [[ "$foreground" == true ]]; then
    CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline artifact::tests::
    CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-host --lib --locked --offline artifact::store_source::tests::
    CARGO_TARGET_DIR="$repo/target/canic-consumer-build" test_foreground_fixture "$scratch/source" "$scratch/foreground-evidence"
  fi
  bash "$repo/scripts/qualification/canic-app-build.sh" "$cli" "$scratch"
  if [[ "$foreground" == true ]]; then
    # Qualify metadata execution without changing the app fixture's prepared tools.
    (bash "$repo/scripts/qualification/canic-build-metadata.sh" "$cli" "$scratch")
    bash "$repo/scripts/qualification/canic-store-source.sh" "$cli" "$scratch"
    CANIC_VIEWER_MODE=replacement CANIC_VIEWER_EVIDENCE="$scratch/viewer-evidence" CARGO_TARGET_DIR="$repo/target/canic-consumer-build" cargo test --manifest-path "$scratch/source/Cargo.toml" -p canic-cli --bin canic --locked --offline view::
  fi
  # Newly migrated display callers must work in the same Build + Text graph.
  (
    cd "$scratch/source"
    export CARGO_TARGET_DIR="$repo/target/canic-consumer-build"
    cargo test -p canic-cli --bin canic --locked --offline -- --list > "$scratch/display-tests.txt"
    for scope in fleet::terminal::tests:: placements::tests:: status::tests:: subnet_comparison::tests:: artifacts::tests::; do
      rg -q "^$scope.*: test$" "$scratch/display-tests.txt"
      cargo test -p canic-cli --bin canic --locked --offline "$scope"
    done
  )
  exit 0
fi
default="$CARGO_TARGET_DIR/wasm32-unknown-unknown/release/owned_wasm_probe.wasm"
mkdir -p "$(dirname "$default")"
printf 'stale output' > "$default"
args=(--json artifact build --manifest "$scratch/probe/Cargo.toml" --package owned-wasm-probe)
"$cli" "${args[@]}" > "$scratch/default.json"
jq -e --arg path "$default" '.wasm == $path and .artifact.bytes > 8' "$scratch/default.json" >/dev/null
hash="$(sha256sum "$default" | cut -d ' ' -f1)"
jq -e --arg hash "$hash" '.artifact.sha256 == $hash' "$scratch/default.json" >/dev/null
rg -a -q 'owned-wasm-probe!' "$default"
# Explicit destination and returned identity refer to the independently published bytes.
"$cli" "${args[@]}" --output "$scratch/explicit.wasm" > "$scratch/explicit.json"
cmp "$default" "$scratch/explicit.wasm"
jq -e --arg hash "$hash" --arg path "$scratch/explicit.wasm" '.artifact.sha256 == $hash and .wasm == $path' "$scratch/explicit.json" >/dev/null
if [[ "$mode" == custody ]]; then
  mkdir "$scratch/existing-parent"
  chmod 755 "$scratch/existing-parent"
  "$cli" "${args[@]}" --output "$scratch/existing-parent/new/deep/artifact.wasm" > "$scratch/nested.json"
  cmp "$default" "$scratch/existing-parent/new/deep/artifact.wasm"
  [[ -n "$(find "$scratch/existing-parent" -prune -perm 0755 -print)" ]]
  [[ -n "$(find "$scratch/existing-parent/new" -prune -perm 0700 -print)" ]]
  [[ -n "$(find "$scratch/existing-parent/new/deep/artifact.wasm" -prune -perm 0600 -print)" ]]
  ln -s existing-parent "$scratch/parent-alias"
  for output in "$scratch/parent-alias/forbidden/artifact.wasm" "$scratch/not-created/../escape.wasm"; do
    if "$cli" "${args[@]}" --output "$output" > "$scratch/parent-refuse.stdout" 2> "$scratch/parent-refuse.stderr"; then
      echo 'Expected parent authority refusal' >&2; exit 1
    fi
    if [[ "$output" == *parent-alias* ]]; then
      jq -e '.error == "supplier" and (.message | contains("filesystem link/type refused"))' "$scratch/parent-refuse.stderr" >/dev/null
    else
      jq -e '.error == "invalid" and (.message | contains("parent traversal"))' "$scratch/parent-refuse.stderr" >/dev/null
    fi
  done
  [[ ! -e "$scratch/existing-parent/forbidden" && ! -e "$scratch/not-created" && ! -e "$scratch/escape.wasm" ]]
fi
# Replace the selected entry without following a final symlink or mutating an alias.
printf 'foreign bytes' > "$scratch/foreign.wasm"
ln -s foreign.wasm "$scratch/symlink.wasm"
ln "$scratch/foreign.wasm" "$scratch/hardlink.wasm"
ln -s missing.wasm "$scratch/dangling.wasm"
for name in symlink hardlink dangling; do
  # A failed build must not unlink an alias or create its missing referent.
  if "$cli" "${args[@]}" --features refuse --output "$scratch/$name.wasm" > "$scratch/$name-refuse.stdout" 2> "$scratch/$name-refuse.stderr"; then
    echo "Expected compilation refusal for $name" >&2; exit 1
  fi
  jq -e '.error == "supplier" and (.message | contains("Cargo exited unsuccessfully"))' "$scratch/$name-refuse.stderr" >/dev/null
  case "$name" in
    symlink) [[ -L "$scratch/$name.wasm" && "$(readlink "$scratch/$name.wasm")" == foreign.wasm ]] ;;
    hardlink) [[ ! -L "$scratch/$name.wasm" && "$scratch/$name.wasm" -ef "$scratch/foreign.wasm" ]] ;;
    dangling) [[ -L "$scratch/$name.wasm" && "$(readlink "$scratch/$name.wasm")" == missing.wasm ]] ;;
  esac
  [[ "$(cat "$scratch/foreign.wasm")" == 'foreign bytes' ]]
  [[ ! -e "$scratch/missing.wasm" && ! -L "$scratch/missing.wasm" ]]
  [[ -z "$(ls -A "$scratch/runtime")" ]]
  "$cli" "${args[@]}" --output "$scratch/$name.wasm" > "$scratch/$name.json"
  [[ -f "$scratch/$name.wasm" && ! -L "$scratch/$name.wasm" ]]
  [[ "$(cat "$scratch/foreign.wasm")" == 'foreign bytes' ]]
  [[ ! "$scratch/$name.wasm" -ef "$scratch/foreign.wasm" ]]
  [[ ! -e "$scratch/missing.wasm" && ! -L "$scratch/missing.wasm" ]]
  cmp "$default" "$scratch/$name.wasm"
  jq -e --arg hash "$hash" --arg path "$scratch/$name.wasm" \
    '.artifact.sha256 == $hash and .wasm == $path' "$scratch/$name.json" >/dev/null
done
# Failed compilation must leave an existing destination unchanged.
if "$cli" "${args[@]}" --features refuse --output "$scratch/explicit.wasm" > "$scratch/refuse.stdout" 2> "$scratch/refuse.stderr"; then
  echo 'Expected compilation refusal' >&2; exit 1
fi
cmp "$default" "$scratch/explicit.wasm"
# Publication refusal retains an occupied directory and its unrelated bytes.
jq -e '.error == "supplier" and (.message | contains("intentional owned-build diagnostic oracle"))' "$scratch/refuse.stderr" >/dev/null
mkdir "$scratch/occupied"
printf 'retain me' > "$scratch/occupied/sentinel"
if "$cli" "${args[@]}" --output "$scratch/occupied" > "$scratch/publication.stdout" 2> "$scratch/publication.stderr"; then
  echo 'Expected publication refusal' >&2; exit 1
fi
[[ "$(cat "$scratch/occupied/sentinel")" == 'retain me' ]]
# Private target cleanup also applies after the failed Cargo invocation.
[[ -z "$(ls -A "$scratch/runtime")" ]]
printf 'Default/explicit publication and symlink/hardlink/dangling failure preservation, exact identity, publication refusal and scratch cleanup passed.\n'
if [[ "$mode" == custody ]]; then
  [[ -z "$(find "$scratch" -name '.canic-artifact-*' -print -quit)" ]]
  printf 'Held parent preparation, existing modes, private creation, traversal/link refusal and staging cleanup passed.\n'
fi
