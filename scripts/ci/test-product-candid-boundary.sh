#!/usr/bin/env bash
set -euo pipefail
[[ $# == 0 ]] || exit 2
unset MAKEFLAGS MFLAGS MAKEOVERRIDES GNUMAKEFLAGS MAKEFILES
root="${BASH_SOURCE[0]}"
[[ "$root" == /* ]] || root="$PWD/$root"
root="$(cd -P "${root%/*}/../.." && pwd -P)"
# shellcheck source=scripts/ci/product-fixture.sh
source "$root/scripts/ci/product-fixture.sh"
fixture="$(tooling_fixture_create "$root" candid-consumer)"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "Candid consumer fixture retained"' EXIT
# Exercise the actual Make routing, product policy and exported checker. Only
# the unrelated metadata check is substituted; no Cargo, server or compiler runs.
bash "$root/vendor/toko-tooling/scripts/ci/prepare-script-fixture.sh" "$root" "$fixture" Makefile vendor/toko-tooling/make/consumer.mk make/testkit.mk vendor/toko-tooling/ci/tool-versions.env
while IFS= read -r path; do
    mkdir -p "$fixture/${path%/*}"
    cp "$root/$path" "$fixture/$path"
done < <({
    printf '%s\n' crates/ic-host-tools/src/ledger_evidence/mod.rs
    jq -r '.allowances[].path' "$root/ci/candid-boundary.json"
} | LC_ALL=C sort -u)
mkdir -p "$fixture/apps" "$fixture/docs" "$fixture/scripts/fixtures/native-consumer"
for path in apps/inventory.rs docs/inventory.rs; do
    printf '// Source-mask fixture data, never compiled.\n' > "$fixture/$path"
done
cat > "$fixture/vendor/toko-tooling/scripts/ci/check-dependency-pins.sh" <<'STUB'
#!/usr/bin/env bash
set -euo pipefail
[[ "$*" == --cargo-inheritance ]]
: > metadata-called
STUB
make --no-print-directory -C "$fixture" dependency-pins-check candid-boundary-check RELEASE_GROUP=text > "$fixture/clean.log" 2>&1
[[ -f "$fixture/metadata-called" ]]
[[ $(grep -c '^Candid source boundary passed (' "$fixture/clean.log") == 1 ]]
rm "$fixture/metadata-called"

refuse() {
    local label=$1 diagnostic=${3:-$2:} status=0
    make --no-print-directory -C "$fixture" dependency-pins-check RELEASE_GROUP=text > "$fixture/$label.log" 2>&1 || status=$?
    if [[ $status == 0 || -e "$fixture/metadata-called" ]]; then
        cat "$fixture/$label.log" >&2
        echo 'Candid source admission did not stop metadata validation' >&2
        exit 1
    fi
    grep -F "$diagnostic" "$fixture/$label.log" >/dev/null
}
# These are scanner input data, not executable Rust injection. The owned ledger
# must not regain a registry derive through Platform's retired re-export.
cat >> "$fixture/crates/ic-host-tools/src/ledger_evidence/mod.rs" <<'RUST'
#[derive(ic_platform::candid::contract::CandidType)]
pub struct InternalReport;
RUST
refuse owned-report crates/ic-host-tools/src/ledger_evidence/mod.rs
cp "$root/crates/ic-host-tools/src/ledger_evidence/mod.rs" "$fixture/crates/ic-host-tools/src/ledger_evidence/mod.rs"
cat > "$fixture/scripts/fixtures/native-consumer/main.rs" <<'RUST'
use ic_platform::candid::contract::CandidType as ExternalCodec;
#[cfg_attr(any(), derive(ExternalCodec))]
pub struct FixtureReport;
RUST
refuse independent-fixture scripts/fixtures/native-consumer/main.rs
rm "$fixture/scripts/fixtures/native-consumer/main.rs"
ln -s "$fixture/docs" "$fixture/scripts/fixtures/source-alias"
refuse script-symlink scripts/fixtures/source-alias 'symlink in selected sources:'
fixture_complete=true
echo 'Consumer Candid admission, empty script root, single Make execution and owned/fixture/symlink rejection passed'
