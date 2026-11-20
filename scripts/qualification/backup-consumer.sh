#!/usr/bin/env bash
# Qualify current local sibling inputs without modifying the sibling.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${BACKUP_SOURCE:-$repo/../ic-backup}"
scope="${1:-custody}"
case "$scope" in custody|http) ;; *) printf 'expected custody or http\n' >&2; exit 2;; esac
export PATH="$repo/.tools/host/bin:$PATH"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/backup-consumer.XXXXXXXX")"
trap 'printf "Backup qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source" "$scratch/runtime"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/backup/migration.patch"
git -C "$scratch/source" apply "$repo/qualification/backup/migration.patch"
export TMPDIR="$scratch/runtime"
export CARGO_TARGET_DIR="$repo/target/backup-consumer-build"
export RUSTUP_AUTO_INSTALL=0
CDPATH='' cd -- "$scratch/source"
cargo metadata --format-version 1 --locked > "$scratch/metadata.json"
jq -e --arg root "$repo" '
  . as $metadata |
  [.packages[] | select(.name == "ic-host-fs")] as $fs |
  ($fs | length) == 1 and $fs[0].version == "0.80.1" and
  $fs[0].manifest_path == ($root + "/crates/ic-host-fs/Cargo.toml") and
  ([$metadata.packages[] | select(.name == "ic-host-artifacts" and .version == "0.80.1" and
    .manifest_path == ($root + "/crates/ic-host-artifacts/Cargo.toml"))] | length) == 1 and
  any($metadata.packages[]; .name == "ic-backup" and
    (.id as $consumer | any($metadata.resolve.nodes[]; .id == $consumer and
      any(.deps[]; .name == "ic_byte_admission" and
        (.pkg as $supplier | any($metadata.packages[]; .id == $supplier and
          .name == "ic-host-artifacts" and .version == "0.80.1"))))))
' "$scratch/metadata.json" >/dev/null
cargo test -p ic-backup --lib --locked -- --list > "$scratch/tests.txt"
if [[ "$scope" == http ]]; then
  jq -e --arg root "$repo" '
    [.packages[] | select(.name == "ic-host-http")] as $http |
    ($http | length) == 1 and $http[0].version == "0.80.0" and
    $http[0].manifest_path == ($root + "/crates/ic-host-http/Cargo.toml")
  ' "$scratch/metadata.json" >/dev/null
  for name in \
    exact_retained_envelope_and_effective_target_produce_authenticated_evidence \
    redirects_throttling_acceptance_and_server_failures_never_retry \
    malformed_unproven_and_compressed_responses_remain_uncertain \
    declared_and_streamed_body_limits_refuse_before_decoding \
    wrong_binding_changed_envelope_and_expired_admission_send_nothing \
    disconnect_body_timeout_and_cancellation_never_retry \
    workflow_replay_preserves_settlement_or_pending_reservation_without_http \
    original_submit_deadline_and_ingress_expiry_bound_body_waits \
    chunked_certificate_authenticates_but_tampering_and_truncation_do_not \
    http_success_with_certified_rejection_is_a_rejected_result \
    signature_authentication_precedes_http_for_both_direct_key_profiles \
    shared_reader_preserves_exact_body_limit_and_empty_body_refusals \
    https_sends_tls_and_refuses_a_plaintext_peer_without_http_fallback; do
    contract="store::capture::advance::tests::http_tests::$name"
    rg -Fxq "$contract: test" "$scratch/tests.txt"
    cargo test -p ic-backup --lib "$contract" --locked -- --exact
  done
  exit 0
fi
cargo test -p ic-backup --lib store::tests --locked
# Metadata fixtures require local loopback; no live IC endpoint is used.
# Each of these exercises the migrated held-file update or closing admission path.
for contract in \
  store::capture::ledger::tests::closing_admission_refuses_changed_file_and_unsafe_name \
  store::capture::advance::tests::successful_status_retains_exact_evidence_and_cold_replay_has_no_effects \
  store::capture::advance::tests::partial_frames_and_uncertain_full_writes_invalidate_live_handle_and_preserve_bytes \
  store::capture::advance::tests::held_root_rename_keeps_append_in_original_store \
  store::capture::advance::tests::closing_custody_failure_never_returns_a_live_reservation \
  store::capture::advance::tests::http_tests::metadata_tests::exact_metadata_is_retained_and_replayed_without_live_signer_or_http \
  store::capture::advance::tests::http_tests::metadata_tests::complete_uncertain_metadata_frames_stay_spent_and_partial_frames_refuse \
  store::capture::advance::tests::http_tests::metadata_tests::metadata_needs_settled_capture_and_preserves_orphan_or_damaged_evidence \
  store::capture::advance::tests::http_tests::metadata_tests::authenticated_bad_metadata_stays_settled_without_repeated_remote_work \
  store::capture::advance::tests::http_tests::metadata_tests::schedule_tests::binding_authenticates_once_and_cold_replay_preserves_bytes_and_spending \
  store::capture::advance::tests::http_tests::metadata_tests::schedule_tests::binding_refuses_conflicting_schedule_and_uncertain_frames_preserve_custody; do
  rg -Fxq "$contract: test" "$scratch/tests.txt"
  cargo test -p ic-backup --lib "$contract" --locked -- --exact
done
