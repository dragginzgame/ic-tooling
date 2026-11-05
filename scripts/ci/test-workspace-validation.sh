#!/usr/bin/env bash
# Shared companions: scripts/ci/fixture-lifecycle.sh scripts/ci/prepare-script-fixture.sh scripts/ci/check-pipe-jobserver.pl scripts/ci/workspace-validation.sh Makefile make/consumer.mk make/testkit.mk
set -euo pipefail
root="$(cd -P "${BASH_SOURCE[0]%/*}/../.." && pwd -P)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
mkdir -p "$root/target"
fixture=$(fixture_create "$root/target/workspace-validation-test.XXXXXX")
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Workspace validation fixture retained"' EXIT
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES SHARED_TOOLING_ROOT
mkdir "$fixture/checkout"
bash "$root/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture/checkout" \
    Makefile make/consumer.mk make/testkit.mk scripts/ci/check-pipe-jobserver.pl
cd "$fixture/checkout"
# Independent observation of the 34-stage gate after certificate owner extraction.
# Production dispatch has one owner: ci/release-validation.json.
cat > "$fixture/expected" <<'STAGES'
shared-tooling-check
check-no-symlinks
tools-check
compiler-check
dependency-pins-check
clippy-workspace
fmt-check
formatting-hook-check
check-doc-links
tooling-command-check
test-artifacts-minimal
test-fs-minimal
test-fs-capability
test-fs-temporary-file
test-artifacts-wasm
test-artifacts-encoding
test-artifacts-text
tools-features-check
isolated-features-check
testkit-graph-check
testkit-wasm-check
check-fs-non-unix
testkit-installed-check
testkit-package-check
test-tools-default
test-tools-interface
test-tools-candid
test-tools-combined
test-tools-agent
test-tools-encoding
test-tools-registry
test-wasm-inspection
docs-workspace
test-workspace-installed
STAGES
export WORKSPACE_TEST_EVENTS="$fixture/events"
cat >> Makefile <<'MAKE'

# Keep prerequisite routing, substituting its build/tool effects as well.
format-tools-check build-server-cli:
	+@perl scripts/ci/check-pipe-jobserver.pl
MAKE
while IFS= read -r target; do
    {
        printf '\n%s:\n' "$target"
        printf '\t+@perl scripts/ci/check-pipe-jobserver.pl\n'
        # shellcheck disable=SC2016 # Preserve recursive Make's exported variable.
        printf '\t+@printf '\''%%s\\n'\'' '\''%s'\'' >> "$$WORKSPACE_TEST_EVENTS"\n' "$target"
        # shellcheck disable=SC2016 # Preserve recursive Make's exported variable.
        printf '\t+@test "$${WORKSPACE_TEST_FAIL:-}" != '\''%s'\''\n' "$target"
    } >> Makefile
done < "$fixture/expected"
cat > scripts/publish/workspace.sh <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
[[ $# == 1 && $1 == qualify ]]
make --no-print-directory clippy-workspace
printf 'publication\n' >> "$WORKSPACE_TEST_EVENTS"
[[ ${WORKSPACE_TEST_FAIL:-} != publication ]]
STUB
options=(--no-print-directory -j4)
if make --help | rg -q -- --jobserver-style; then options+=(--jobserver-style=pipe); fi
for mode in ci ci-publication; do
    : > "$WORKSPACE_TEST_EVENTS"
    make "${options[@]}" "$mode" > "$fixture/$mode.log" 2>&1
    if [[ $mode == ci-publication ]]; then
        awk '{print} $0 == "clippy-workspace" {print "publication"}' "$fixture/expected" > "$fixture/publication-expected"
        cmp "$fixture/publication-expected" "$WORKSPACE_TEST_EVENTS"
    else cmp "$fixture/expected" "$WORKSPACE_TEST_EVENTS"; fi
    for failure in tools-check clippy-workspace fmt-check testkit-package-check; do
        : > "$WORKSPACE_TEST_EVENTS"
        if WORKSPACE_TEST_FAIL="$failure" make "${options[@]}" "$mode" > "$fixture/$mode-$failure.log" 2>&1; then exit 1; fi
        [[ $(tail -n 1 "$WORKSPACE_TEST_EVENTS") == "$failure" ]]
        lint_count=$(rg -c '^clippy-workspace$' "$WORKSPACE_TEST_EVENTS" || true)
        [[ ${lint_count:-0} -le 1 ]]
    done
done
: > "$WORKSPACE_TEST_EVENTS"
if WORKSPACE_TEST_FAIL=publication make "${options[@]}" ci-publication > "$fixture/publication-failed.log" 2>&1; then exit 1; fi
[[ $(tail -n 1 "$WORKSPACE_TEST_EVENTS") == publication ]]
# No partial dispatch when a new/omitted stage or focus classification is unmapped.
cp ci/release-validation.json "$fixture/policy"
cp ci/release-validation-coverage.json "$fixture/coverage"
for mutation in missing new; do
    if [[ $mutation == missing ]]; then
        jq -cS '.stages |= .[1:]' "$fixture/policy" > ci/release-validation.json
    else jq -cS '.stages += ["release-stage-unmapped"]' "$fixture/policy" > ci/release-validation.json; fi
    : > "$WORKSPACE_TEST_EVENTS"
    if make "${options[@]}" ci > "$fixture/$mutation.log" 2>&1; then exit 1; fi
    [[ ! -s $WORKSPACE_TEST_EVENTS ]]
done
cp "$fixture/policy" ci/release-validation.json
for mutation in focus multiple-documents; do
    if [[ $mutation == focus ]]; then
        jq '.focused_stages += ["release-scope-unknown"]' "$fixture/coverage" > ci/release-validation-coverage.json
    else cat "$fixture/coverage" "$fixture/coverage" > ci/release-validation-coverage.json; fi
    : > "$WORKSPACE_TEST_EVENTS"
    if make "${options[@]}" ci > "$fixture/$mutation.log" 2>&1; then exit 1; fi
    [[ ! -s $WORKSPACE_TEST_EVENTS ]]
done
fixture_complete=true
echo 'Both CI entrypoints preserve 34-stage coverage, one lint, fail-fast order and pipe jobserver access'
