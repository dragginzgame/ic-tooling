#!/usr/bin/env bash
# Exercise the product command and asset admission, without networking.
set -euo pipefail
: "${TESTKIT_SERVER_CLI:?select an absolute prepared ic-testkit-server executable}"
[[ "$TESTKIT_SERVER_CLI" == /* && -x "$TESTKIT_SERVER_CLI" ]]
fixture="$(mktemp -d "${TMPDIR:-/tmp}/ic-setup-diagnostics.XXXXXXXX")"
fixture="$(cd "$fixture" && pwd -P)"
trap 'status=$?; if [[ $status == 0 ]]; then rm -rf "$fixture"; else printf "Setup regression evidence: %q\n" "$fixture" >&2; fi' EXIT
cat > "$fixture/curl" <<'CURL'
#!/usr/bin/env bash
set -eu
case "$SETUP_MODE" in
  failure)
    printf 'curl: (56) controlled failure\033[31m\n::error::injected\342\200\256\n' >&2
    exit 56 ;;
  overflow) head -c 200000 /dev/zero >&2; exit 0 ;;
  corrupt)
    while [[ "$1" != --output ]]; do shift; done
    printf corrupt > "$2" ;;
  success)
    while [[ "$1" != --output ]]; do shift; done
    cp "$TESTKIT_SERVER_ARCHIVE" "$2" ;;
esac
CURL
chmod +x "$fixture/curl"
"$TESTKIT_SERVER_CLI" --version | grep -Eq '^ic-testkit-server [0-9]+\.[0-9]+\.[0-9]+$'
status=0
"$TESTKIT_SERVER_CLI" run "$fixture/missing" "$fixture/invalid" --ttl 0 -- true > /dev/null 2>&1 || status=$?
[[ $status == 2 && ! -e "$fixture/invalid" ]]
if "$TESTKIT_SERVER_CLI" check "$fixture/missing" > /dev/null 2>&1; then exit 1; fi
for mode in failure overflow corrupt; do
  status=0
  SETUP_MODE="$mode" "$TESTKIT_SERVER_CLI" setup "$fixture/$mode-assets" \
    "$fixture/$mode" "$fixture/curl" /usr/bin/gzip > "$fixture/stdout" 2> "$fixture/stderr" || status=$?
  [[ $status != 0 && ! -s "$fixture/stdout" && ! -e "$fixture/$mode-assets/pocket-ic" ]]
  [[ -f "$fixture/$mode/download.stderr" && ! -L "$fixture/$mode/download.stderr" ]]
  [[ $(wc -c < "$fixture/$mode/download.stderr") -le 131072 ]]
  [[ $(wc -c < "$fixture/stderr") -lt 32768 ]]
  if [[ "$mode" == failure ]]; then
    [[ $status == 56 ]]
    grep -Fq 'curl\x3a (56) controlled failure\u{1b}[31m\n\x3a\x3aerror\x3a\x3ainjected\u{202e}' "$fixture/stderr"
    printf 'curl: (56) controlled failure\033[31m\n::error::injected\342\200\256\n' > "$fixture/expected"
    cmp "$fixture/expected" "$fixture/$mode/download.stderr"
  fi
done
# Refuse reused or linked evidence; never replace a retained failure's files.
cp "$fixture/failure/download.stderr" "$fixture/retained"
ln -s "$fixture/failure" "$fixture/linked"
for evidence in failure linked; do
  if SETUP_MODE=corrupt "$TESTKIT_SERVER_CLI" setup "$fixture/other-assets" \
    "$fixture/$evidence" "$fixture/curl" /usr/bin/gzip > /dev/null 2>&1; then exit 1; fi
done
cmp "$fixture/retained" "$fixture/failure/download.stderr"
if [[ -n "${TESTKIT_SERVER_ARCHIVE:-}" ]]; then
  cat > "$fixture/gzip" <<'GZIP'
#!/usr/bin/env bash
printf 'controlled gzip failure\033[31m\n' >&2
exit 43
GZIP
  chmod +x "$fixture/gzip"
  status=0
  SETUP_MODE=success "$TESTKIT_SERVER_CLI" setup "$fixture/gzip-assets" \
    "$fixture/gzip-evidence" "$fixture/curl" "$fixture/gzip" \
    > "$fixture/stdout" 2> "$fixture/stderr" || status=$?
  [[ $status == 43 && ! -s "$fixture/stdout" && ! -e "$fixture/gzip-assets/pocket-ic" ]]
  grep -Fq 'controlled gzip failure\u{1b}[31m\n' "$fixture/stderr"
  printf 'controlled gzip failure\033[31m\n' > "$fixture/expected"
  cmp "$fixture/expected" "$fixture/gzip-evidence/server.stderr"
  SETUP_MODE=success "$TESTKIT_SERVER_CLI" setup "$fixture/prepared" \
    "$fixture/success" "$fixture/curl" /usr/bin/gzip > "$fixture/stdout"
  [[ "$(cat "$fixture/stdout")" == "$("$TESTKIT_SERVER_CLI" check "$fixture/prepared")" ]]
fi
printf 'Setup failure status, bounded escaped diagnostics, raw evidence, checksum refusal and stale/link refusal pass.\n'
