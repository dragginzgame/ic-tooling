#!/usr/bin/env bash
# The prepared managed-server caller owns the server; tests own and delete instances.
set -euo pipefail
manifest="$1"
scratch="$2"
: "${IC_TESTKIT_POCKET_IC_URL:?outer server owner supplies endpoint}"
endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
curl --fail --silent --show-error "$endpoint/instances" | jq -e 'length == 0' >/dev/null
for test in \
  held::tests::installed_modules_keep_replies_bound_to_slots_and_reject_failures \
  held::control::tests::installed_control_refuses_foreign_consumed_expired_and_automatic_releases; do
  log="$scratch/${test##*::}.log"
  cargo test --manifest-path "$manifest" -p ic-testkit --features held-reply-fixtures \
    --lib --locked --offline "$test" -- --ignored --exact --color never > "$log" 2>&1
  grep -Fq 'test result: ok. 1 passed; 0 failed' "$log"
done
curl --fail --silent --show-error "$endpoint/instances" > "$scratch/instances-after.json"
jq -e 'length == 3 and all(.[]; . == "Deleted")' "$scratch/instances-after.json" >/dev/null
printf 'Both installed fixture checks passed; all three instances are deleted.\n'
