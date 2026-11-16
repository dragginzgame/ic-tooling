#!/usr/bin/env bash
# Failed-download regression through the actual prepared bootstrap; no network.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
: "${TESTKIT_RELEASE_LAUNCHER:?select the prepared ic-testkit-server 0.106.0 launcher}"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/ic-setup-diagnostics.XXXXXXXX")"
fixture="$(cd "$fixture" && pwd -P)"
trap 'status=$?; if [[ $status == 0 ]]; then rm -rf "$fixture"; else printf "Setup regression evidence: %q\n" "$fixture" >&2; fi' EXIT
mkdir "$fixture/bin"
cat > "$fixture/bin/curl" <<'CURL'
#!/usr/bin/env bash
printf 'called\n' >> "$SETUP_FIXTURE/curl-calls"
printf 'curl: (56) controlled receive failure\033[31m\n::error::injected\342\200\256\n' >&2
exit 56
CURL
chmod +x "$fixture/bin/curl"
export SETUP_FIXTURE="$fixture"
status=0
PATH="$fixture/bin:$PATH" bash "$repo/scripts/qualification/setup-pocketic.sh" \
  "$TESTKIT_RELEASE_LAUNCHER" "$fixture/assets" "$fixture/evidence" \
  > "$fixture/stdout" 2> "$fixture/stderr" || status=$?
[[ $status == 1 && ! -s "$fixture/stdout" ]]
[[ $(wc -l < "$fixture/curl-calls") -eq 1 ]]
grep -Fq 'curl\x3a (56) controlled receive failure\x1b[31m\x0a\x3a\x3aerror\x3a\x3ainjected\xe2\x80\xae' "$fixture/stderr"
grep -Fq 'PocketIC setup failed (exit 1)' "$fixture/stderr"
attempts=("$fixture"/assets/.setup-v1-*-*)
[[ ${#attempts[@]} == 1 && -f "${attempts[0]}/failure.txt" && -f "${attempts[0]}/download.stderr" ]]
printf 'curl: (56) controlled receive failure\033[31m\n::error::injected\342\200\256\n' > "$fixture/expected-stderr"
cmp "$fixture/expected-stderr" "${attempts[0]}/download.stderr"
grep -Fq "${attempts[0]}/download.stderr" "$fixture/stderr"
grep -Fq 'tool exited unsuccessfully' "$fixture/evidence/setup.stderr"
[[ ! -e "$fixture/evidence/check.stdout" ]]

# Exercise reporting bounds, stale evidence exclusion, and offline check dispatch.
cat > "$fixture/launcher" <<'LAUNCHER'
#!/usr/bin/env bash
set -eu
case "$1" in
  --version) echo 'ic-testkit-server 0.106.0' ;;
  setup)
    [[ ${SETUP_MODE:-failure} == success ]] && exit 0
    attempt="$3/.setup-v1-123-0"
    mkdir -p "$attempt" "$3/.setup-v1-999-0"
    echo 'stale-marker' > "$3/.setup-v1-999-0/download.stderr"
    echo "PocketIC setup evidence: $attempt" >&2
    echo 'original failure' > "$attempt/failure.txt"
    if [[ ${SETUP_MODE:-failure} != missing ]]; then
      printf '%4096s' '' | tr ' ' x > "$attempt/download.stderr"
      printf 'past-limit-marker\n' >> "$attempt/download.stderr"
    fi
    exit 56 ;;
  check)
    echo checked >> "$SETUP_FIXTURE/check-calls"
    if [[ ${CHECK_FAIL:-0} == 1 ]]; then echo 'checksum refusal' >&2; exit 23; fi
    echo '/prepared/pocket-ic' ;;
esac
LAUNCHER
chmod +x "$fixture/launcher"
status=0
bash "$repo/scripts/qualification/setup-pocketic.sh" "$fixture/launcher" \
  "$fixture/fake-assets" "$fixture/bounded-evidence" > "$fixture/stdout" 2> "$fixture/stderr" || status=$?
[[ $status == 56 && ! -e "$fixture/check-calls" ]]
if grep -Eq 'past-limit-marker|stale-marker' "$fixture/stderr"; then exit 1; fi
[[ $(wc -c < "$fixture/stderr") -lt 6000 ]]
grep -Fq 'past-limit-marker' "$fixture/fake-assets/.setup-v1-123-0/download.stderr"
status=0
SETUP_MODE=missing bash "$repo/scripts/qualification/setup-pocketic.sh" "$fixture/launcher" \
  "$fixture/missing-assets" "$fixture/missing-evidence" > "$fixture/stdout" 2> "$fixture/stderr" || status=$?
[[ $status == 56 ]]
grep -Fq 'Diagnostic unavailable' "$fixture/stderr"
SETUP_MODE=success bash "$repo/scripts/qualification/setup-pocketic.sh" "$fixture/launcher" \
  "$fixture/fake-assets" "$fixture/success-evidence" > "$fixture/stdout" 2> "$fixture/stderr"
[[ $(cat "$fixture/stdout") == /prepared/pocket-ic ]]
status=0
SETUP_MODE=success CHECK_FAIL=1 bash "$repo/scripts/qualification/setup-pocketic.sh" "$fixture/launcher" \
  "$fixture/fake-assets" "$fixture/check-evidence" > "$fixture/stdout" 2> "$fixture/stderr" || status=$?
[[ $status == 23 && ! -s "$fixture/stdout" ]]
grep -Fq 'checksum refusal' "$fixture/stderr"
[[ $(wc -l < "$fixture/check-calls") -eq 2 ]]
printf 'Setup diagnostics: actual failed download, escaping, evidence, bounds, missing evidence and offline admission pass.\n'
