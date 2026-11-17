#!/usr/bin/env bash
# Real prepared server: child outcomes remain distinct from server teardown.
set -euo pipefail
: "${TESTKIT_SERVER_CLI:?select an absolute prepared ic-testkit-server executable}"
: "${TESTKIT_SERVER_DIRECTORY:?select the prepared Testkit server directory}"
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
scratch="$(mktemp -d "$repo/target/testkit-server-cli.XXXXXXXX")"
export SIMULATOR_TEST_EVIDENCE="$scratch"
trap 'printf "Testkit CLI evidence: %s\n" "$scratch"' EXIT
printf 'piped input\n' > "$scratch/input"
printf 'command-output\000\033[31m\n' > "$scratch/expected.stdout"
printf 'command-stderr\n' > "$scratch/expected.stderr"
for mode in success failure timeout signal evidence-failure; do
  status=0
  export SIMULATOR_TEST_MODE="$mode"
  # Expansion belongs to the child after the owner injects its endpoint.
  # shellcheck disable=SC2016
  "$TESTKIT_SERVER_CLI" run "$TESTKIT_SERVER_DIRECTORY" "$scratch/$mode" --ttl 2 -- \
    bash -c '
      set -eu
      endpoint="${IC_TESTKIT_POCKET_IC_URL%/}"
      printf "%s" "$endpoint" > "$SIMULATOR_TEST_EVIDENCE/endpoint"
      test -z "${POCKET_IC_BIN:-}"
      IFS= read -r input
      test "$input" = "piped input"
      printf "command-output\000\033[31m\n"
      printf "command-stderr\n" >&2
      curl --fail --silent --max-time 1 "$endpoint/instances" > "$SIMULATOR_TEST_EVIDENCE/instances"
      case "$SIMULATOR_TEST_MODE" in
        failure) exit 23 ;;
        timeout) sleep 10 ;;
        signal) kill -TERM "$PPID"; sleep 10 ;;
        evidence-failure)
          mkdir "$SIMULATOR_TEST_EVIDENCE/evidence-failure/command.txt" \
            "$SIMULATOR_TEST_EVIDENCE/evidence-failure/server.txt"
          exit 23 ;;
      esac
    ' < "$scratch/input" > "$scratch/$mode.stdout" 2> "$scratch/$mode.stderr" || status=$?
  case "$mode" in success) [[ $status == 0 ]] ;; failure|evidence-failure) [[ $status == 23 ]] ;; timeout|signal) [[ $status != 0 ]] ;; esac
  cmp "$scratch/expected.stdout" "$scratch/$mode.stdout"
  cmp "$scratch/expected.stdout" "$scratch/$mode/command.stdout"
  cmp "$scratch/expected.stderr" "$scratch/$mode/command.stderr"
  if [[ "$mode" != evidence-failure ]]; then
    grep -Fq 'cleanup: []' "$scratch/$mode/server.txt"
  fi
  if curl --silent --max-time 1 "$(cat "$scratch/endpoint")/instances" > /dev/null; then
    echo 'Server remained reachable after command completion' >&2; exit 1
  fi
done
status=0
"$TESTKIT_SERVER_CLI" run "$TESTKIT_SERVER_DIRECTORY" "$scratch/spawn" -- \
  "$scratch/no-such-command" > "$scratch/spawn.stdout" 2> "$scratch/spawn.stderr" || status=$?
[[ $status != 0 ]]
endpoint="$(cat "$scratch/spawn/endpoint.txt")"
if curl --silent --max-time 1 "${endpoint%/}/instances" > /dev/null; then exit 1; fi
printf 'CLI IO passthrough, success/failure/timeout/signal/spawn cleanup and original failure retention pass.\n'
