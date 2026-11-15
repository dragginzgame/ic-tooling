#!/usr/bin/env bash
# Runs inside the released command-scoped server owner; never stops that server.
set -euo pipefail
mode="${1:-rollback}"
case "$mode" in rollback|text) ;; *) exit 2 ;; esac
subject=message_rollback
suite=rollback
if [[ "$mode" == text ]]; then
  subject=console_inspects_and_queries
  suite=records
fi
: "${IC_TESTKIT_POCKET_IC_URL:?external server owner must supply the endpoint}"
: "${DB_QUALIFICATION_EXPORT:?select the captured source export}"
endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
cd "$DB_QUALIFICATION_EXPORT/source"
curl --fail --silent --show-error "$endpoint/instances" > "$DB_QUALIFICATION_EXPORT/instances-before.json"
jq -e 'length == 0' "$DB_QUALIFICATION_EXPORT/instances-before.json" >/dev/null
cargo test --manifest-path qualification/Cargo.toml -p ic-db-m0-rollback-driver --test "$suite" --locked --offline -- --list > "$DB_QUALIFICATION_EXPORT/tests.txt"
rg -Fxq "$subject: test" "$DB_QUALIFICATION_EXPORT/tests.txt"
cargo test --manifest-path qualification/Cargo.toml -p ic-db-m0-rollback-driver --test "$suite" "$subject" --locked --offline -- --exact
# Independent observation of instance state and server reachability for this run.
curl --fail --silent --show-error "$endpoint/instances" > "$DB_QUALIFICATION_EXPORT/instances-after.json"
if [[ "$mode" == rollback ]]; then
  jq -e 'length == 1 and .[0] == "Deleted"' "$DB_QUALIFICATION_EXPORT/instances-after.json" >/dev/null
  printf 'Instance deleted; external server remained reachable after client close.\n'
else
  # Console success and rejection paths must all close their observed instances.
  # This does not promise cleanup after upstream panic or cancellation.
  jq -e 'length > 0 and all(.[]; . == "Deleted")' "$DB_QUALIFICATION_EXPORT/instances-after.json" >/dev/null
  jq '{instances: length, deleted: map(select(. == "Deleted")) | length}' "$DB_QUALIFICATION_EXPORT/instances-after.json"
fi
