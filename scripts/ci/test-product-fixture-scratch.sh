#!/usr/bin/env bash
set -euo pipefail
[[ $# == 0 ]] || exit 2
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture="$(tooling_fixture_create "$root" fixture-scratch)"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Product scratch fixture retained"' EXIT
checkout="$fixture/checkout"
mkdir "$checkout" "$fixture/override"
default=$(unset TMPDIR; tooling_fixture_create "$checkout" default)
[[ $default == "$checkout/target/tmp/"* ]]
explicit=$(TMPDIR="$fixture/override" tooling_fixture_create "$checkout" explicit)
[[ $explicit == "$fixture/override/"* ]]
ln -s "$fixture/override" "$fixture/alias"
aliased=$(cd "$fixture"; TMPDIR='alias' tooling_fixture_create "$checkout" aliased)
[[ $aliased == "$fixture/override/"* ]]
if TMPDIR="$fixture/invalid"$'\n' tooling_fixture_create "$checkout" newline > "$fixture/newline.log" 2>&1; then exit 1; fi
[[ ! -e "$fixture/invalid"$'\n' ]]
printf 'caller-owned\n' > "$fixture/file"
if TMPDIR="$fixture/file" tooling_fixture_create "$checkout" file > "$fixture/file.log" 2>&1; then exit 1; fi
[[ $(cat "$fixture/file") == caller-owned ]]

# Actual Make/driver routing; substitute only the unrelated leaf check.
bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$checkout" \
    Makefile vendor/toko-tooling/make/consumer.mk make/testkit.mk vendor/toko-tooling/ci/tool-versions.env
printf 'scripts/ci/probe-scratch.sh\ttooling\n' > "$checkout/ci/tooling-checks.tsv"
cat > "$checkout/scripts/ci/probe-scratch.sh" <<'PROBE'
#!/usr/bin/env bash
set -euo pipefail
[[ $TMPDIR == "$PWD/target/tmp" ]]
source vendor/toko-tooling/scripts/ci/fixture-lifecycle.sh
child=$(fixture_create "$TMPDIR/nested-shared.XXXXXX")
printf '%s\n' "$child" > nested-path
fixture_complete=true
fixture_finish 0 "$child" 'Nested scratch cleanup failed'
PROBE
(unset TMPDIR; make --no-print-directory -C "$checkout" tooling-command-check) > "$fixture/make.log" 2>&1
[[ ! -e $(cat "$checkout/nested-path") ]]
fixture_complete=true
echo 'Product scratch defaults, explicit/aliased paths, refusals and actual Make routing passed'
