#!/usr/bin/env bash
# Exercise real runner startup without capturing siblings or running builds.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
mkdir -p "$repo/target"
fixture="$(mktemp -d "$repo/target/cdpath.XXXXXXXX")"
trap 'rm -rf "$fixture"' EXIT
mkdir -p "$fixture/shadow tree/scripts/qualification"
cat > "$fixture/probe.sh" <<'PROBE'
# DEBUG runs after the real repo assignment and before any downstream effects.
unset repo
trap 'if [[ ${repo+x} ]]; then
  [[ "$repo" == "$EXPECTED_REPO" ]] || {
    printf "Incorrect repository: %q; expected %q\n" "$repo" "$EXPECTED_REPO" >&2
    exit 1
  }
  printf "Repository resolution passed.\n"
  exit 0
fi' DEBUG
PROBE
cat > "$fixture/editor" <<'EDITOR'
#!/usr/bin/env bash
printf 'ic-cargo-metadata 0.1.0\n'
EDITOR
chmod +x "$fixture/editor"
count=0
for script in "$repo"/scripts/qualification/*.sh; do
  [[ "$script" != "$repo/scripts/qualification/test-cdpath.sh" ]] || continue
  # Select entrypoints that compute their own repository root.
  if ! rg -q '^repo=' "$script"; then continue; fi
  name="${script##*/}"
  args=()
  if [[ "$name" == canic-store-source.sh ]]; then args=(/bin/true "$fixture"); fi
  for search in '' . "$fixture/shadow tree:."; do
    for prefix in scripts/qualification ./scripts/qualification; do
      (
        CDPATH='' cd -- "$repo"
        env CDPATH="$search" BASH_ENV="$fixture/probe.sh" EXPECTED_REPO="$repo" \
          CARGO_RELEASE_METADATA_EDITOR="$fixture/editor" \
          TESTKIT_SERVER_CLI=/bin/true TESTKIT_SERVER_DIRECTORY="$fixture" \
          bash "$prefix/$name" "${args[@]}"
      ) > "$fixture/result" 2>&1 || {
        printf 'Startup failed: %s (CDPATH=%q)\n' "$prefix/$name" "$search" >&2
        cat "$fixture/result" >&2
        exit 1
      }
      [[ "$(cat "$fixture/result")" == 'Repository resolution passed.' ]]
      count=$((count + 1))
    done
  done
done
[[ $count -gt 0 ]]
printf '%s relative runner invocations ignore CDPATH.\n' "$count"
