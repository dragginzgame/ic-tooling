#!/usr/bin/env bash
# Timers borrows the server; uploaded upstream snapshots preserve timer ownership.
set -euo pipefail
mode="${1:-timers}"
case "$mode" in timers|timers-native-snapshot) ;; *) exit 2 ;; esac
: "${IC_TESTKIT_POCKET_IC_URL:?external endpoint required}"
: "${TIMERS_QUALIFICATION_EXPORT:?captured source required}"
: "${TIMERS_PREPARED_SERVER:?prepared server binary for provenance required}"
endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
cd "$TIMERS_QUALIFICATION_EXPORT/source"
manifest=qualification/Cargo.toml
curl --fail --silent --show-error "$endpoint/instances" > "$TIMERS_QUALIFICATION_EXPORT/instances-before.json"
jq -e 'length == 0' "$TIMERS_QUALIFICATION_EXPORT/instances-before.json" >/dev/null
cargo test --manifest-path "$manifest" -p ic-timers-pocketic --test runtime --no-run --message-format=json --locked --offline > "$TIMERS_QUALIFICATION_EXPORT/runtime-build.jsonl"
runtime="$(jq -ser '[.[] | select(.reason == "compiler-artifact" and .target.name == "runtime" and .executable != null) | .executable] | if length == 1 then .[0] else error("expected one runtime executable") end' "$TIMERS_QUALIFICATION_EXPORT/runtime-build.jsonl")"
subject=tests::lifecycle::cached_baseline_restoration_preserves_timer_ownership_after_mutation
"$runtime" --list > "$TIMERS_QUALIFICATION_EXPORT/runtime-tests.txt"
rg -Fxq "$subject: test" "$TIMERS_QUALIFICATION_EXPORT/runtime-tests.txt"
if env -u IC_TESTKIT_POCKET_IC_URL POCKET_IC_BIN=/unavailable "$runtime" --exact "$subject" > "$TIMERS_QUALIFICATION_EXPORT/missing-endpoint.log" 2>&1; then
  echo 'Missing endpoint unexpectedly succeeded' >&2; exit 1
fi
rg -Fq 'external server owner must supply IC_TESTKIT_POCKET_IC_URL' "$TIMERS_QUALIFICATION_EXPORT/missing-endpoint.log"
curl --fail --silent --show-error "$endpoint/instances" | jq -e 'length == 0' >/dev/null
if [[ "$mode" == timers-native-snapshot ]]; then
  # This deliberately rejected migration must fail at the post-mutation timer rearm.
  if POCKET_IC_BIN="$TIMERS_PREPARED_SERVER" cargo run --manifest-path "$manifest" -p ic-timers-pocketic --locked --offline -- focused "$subject" > "$TIMERS_QUALIFICATION_EXPORT/native-snapshot.log" 2>&1; then
    echo 'Native snapshots now pass: reassess the retained workaround.' >&2; exit 1
  fi
  rg -Fq 'restore cycle 1 (mutated=true) must rearm without a competing-owner trap' "$TIMERS_QUALIFICATION_EXPORT/native-snapshot.log"
  rg -Fq 'ic-timers: competing global timer owner' "$TIMERS_QUALIFICATION_EXPORT/native-snapshot.log"
  curl --fail --silent --show-error "$endpoint/instances" > "$TIMERS_QUALIFICATION_EXPORT/instances-after.json"
  jq -e 'length == 1' "$TIMERS_QUALIFICATION_EXPORT/instances-after.json" >/dev/null
  printf 'Native snapshot migration rejected at mutated timer rearm; outer CLI owns failed-instance server teardown.\n'
  exit 0
fi
# The existing receipt hashes the prepared binary; the harness never spawns it.
POCKET_IC_BIN="$TIMERS_PREPARED_SERVER" cargo run --manifest-path "$manifest" -p ic-timers-pocketic --locked --offline -- focused \
  tests::lifecycle::runtime_composition_preserves_calls_cleanup_and_lifecycle \
  tests::lifecycle::shared_watchdog_survives_trap_and_upgrade \
  tests::ingress::typed_entrypoints_reject_invalid_wire_before_mutating_timer_state \
  "$subject"
curl --fail --silent --show-error "$endpoint/instances" > "$TIMERS_QUALIFICATION_EXPORT/instances-after.json"
jq -e 'length == 5 and all(.[]; . == "Deleted")' "$TIMERS_QUALIFICATION_EXPORT/instances-after.json" >/dev/null
printf 'Five Timers instances deleted; runtime composition and cached restores passed and external server remained reachable.\n'
