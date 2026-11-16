#!/usr/bin/env bash
# Qualification owns setup reporting; the prepared launcher owns asset admission.
set -euo pipefail
if [[ $# != 3 ]]; then
  echo 'Usage: setup-pocketic.sh ABSOLUTE_LAUNCHER ABSOLUTE_ASSET_DIRECTORY ABSOLUTE_NEW_EVIDENCE_DIRECTORY' >&2
  exit 2
fi
launcher="$1" directory="${2%/}" evidence="${3%/}"
[[ "$launcher" == /* && -x "$launcher" && "$directory" == /* && "$evidence" == /* ]] || {
  echo 'Setup requires absolute paths and an executable prepared launcher' >&2; exit 2;
}
[[ "$("$launcher" --version)" == 'ic-testkit-server 0.106.0' ]] || {
  echo 'Setup requires the reviewed ic-testkit-server 0.106.0 bootstrap' >&2; exit 2;
}
# Refuse reused evidence so an older attempt cannot masquerade as this invocation.
mkdir "$evidence"
printf 'PocketIC setup logs: %q\n' "$evidence" >&2
report() {
  local file="$1"
  printf 'Evidence: %q\n' "$file" >&2
  if [[ ! -f "$file" || -L "$file" ]]; then
    echo 'Diagnostic unavailable (missing or not a regular non-symlink file)' >&2
    return
  fi
  # Display at most 4096 source bytes. Escape terminal controls, non-ASCII bytes,
  # backslashes and colons (including GitHub workflow-command syntax).
  printf 'First 4096 bytes (escaped; complete evidence retained): ' >&2
  head -c 4096 "$file" | LC_ALL=C perl -0777 -pe \
    's/([^\x20-\x7e]|[:\\])/sprintf("\\x%02x", ord($1))/ge' >&2
  printf '\n' >&2
}
if "$launcher" setup --directory "$directory" > "$evidence/setup.stdout" 2> "$evidence/setup.stderr"; then
  :
else
  status=$?
  printf 'PocketIC setup failed (exit %d); no retry performed.\n' "$status" >&2
  report "$evidence/setup.stderr" || true
  # Select only the attempt named by this invocation, never a directory glob or
  # the newest retained failure. The reviewed launcher emits this exact prefix.
  prefix="PocketIC setup evidence: $directory/.setup-v1-"
  while IFS= read -r line; do
    [[ "$line" == "$prefix"* ]] || continue
    suffix="${line#"$prefix"}"
    [[ "$suffix" =~ ^[0-9]+-[0-9]+$ ]] || continue
    attempt="$directory/.setup-v1-$suffix"
    report "$attempt/failure.txt" || true
    report "$attempt/download.stderr" || true
    break
  done < "$evidence/setup.stderr"
  exit "$status"
fi
# Setup success does not bypass the launcher's independent offline admission.
if "$launcher" check --directory "$directory" > "$evidence/check.stdout" 2> "$evidence/check.stderr"; then
  cat "$evidence/check.stdout"
else
  status=$?
  printf 'PocketIC offline admission failed (exit %d).\n' "$status" >&2
  report "$evidence/check.stderr" || true
  exit "$status"
fi
