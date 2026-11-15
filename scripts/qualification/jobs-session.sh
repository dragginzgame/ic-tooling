#!/usr/bin/env bash
# The external command owns the server; each test owns only its instance.
set -euo pipefail
: "${IC_TESTKIT_POCKET_IC_URL:?external server endpoint required}"
: "${JOBS_QUALIFICATION_EXPORT:?captured source required}"
endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
cd "$JOBS_QUALIFICATION_EXPORT/source"
curl --fail --silent --show-error "$endpoint/instances" > "$JOBS_QUALIFICATION_EXPORT/instances-before.json"
jq -e 'length == 0' "$JOBS_QUALIFICATION_EXPORT/instances-before.json" >/dev/null
args=(--manifest-path qualification/Cargo.toml -p ic-jobs-canister-tests --test recovery --features harness --locked --offline)
cargo test "${args[@]}" -- --list > "$JOBS_QUALIFICATION_EXPORT/jobs-tests.txt"
for subject in management_upgrade_and_real_watchdog_delivery synchronous_trap_rolls_back_storage_identity_and_timer_changes invalid_restore_refuses_upgrade_and_preserves_the_controller_snapshot; do
  rg -Fxq "$subject: test" "$JOBS_QUALIFICATION_EXPORT/jobs-tests.txt"
  cargo test "${args[@]}" -- --ignored --exact "$subject"
done
curl --fail --silent --show-error "$endpoint/instances" > "$JOBS_QUALIFICATION_EXPORT/instances-after.json"
jq -e 'length == 3 and all(.[]; . == "Deleted")' "$JOBS_QUALIFICATION_EXPORT/instances-after.json" >/dev/null
printf 'Three instances deleted; external server remained reachable.\n'
