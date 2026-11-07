#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd -P)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
mkdir -p "$root/target"
fixture="$(mktemp -d "$root/target/pocket-ic-source-test.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$fixture" "PocketIC source evidence retained"' EXIT
bash "$root/scripts/ci/check-pocket-ic-source.sh"
for mutation in import body license adaptation snapshot-adaptation module-adaptation tooling-adaptation protocol-adaptation surface-adaptation logging-adaptation management-adaptation loc-adaptation http-body; do
    cp -R "$root/crates/ic-testkit" "$fixture/$mutation"
    case "$mutation" in
        loc-adaptation) printf '\nchanged LOC cut patch\n' >> "$fixture/$mutation/pocket-ic-provenance/loc-surface-cut.patch" ;;
        management-adaptation) printf '\nchanged management patch\n' >> "$fixture/$mutation/pocket-ic-provenance/management-routing.patch" ;;
        logging-adaptation) printf '\nchanged logging patch\n' >> "$fixture/$mutation/pocket-ic-provenance/logging-owner.patch" ;;
        surface-adaptation) printf '\nchanged surface patch\n' >> "$fixture/$mutation/pocket-ic-provenance/surface-cut.patch" ;;
        tooling-adaptation) printf '\nchanged maintenance patch\n' >> "$fixture/$mutation/pocket-ic-provenance/tooling-maintenance.patch" ;;
        protocol-adaptation) printf '\nchanged protocol patch\n' >> "$fixture/$mutation/pocket-ic-provenance/protocol-codec.patch" ;;
        module-adaptation) printf '\nchanged module patch\n' >> "$fixture/$mutation/pocket-ic-provenance/module-transfer.patch" ;;
        http-body) printf '\nfn changed_http_body() {}\n' >> "$fixture/$mutation/src/pocket_ic/http.rs" ;;
        import) perl -0777 -i -pe 's/ic_management_canister_types::/changed_management_types::/ == 1 or die "missing import\n"' "$fixture/$mutation/src/pocket_ic/mod.rs" ;;
        body) printf '\nfn altered_client_body() {}\n' >> "$fixture/$mutation/src/pocket_ic/nonblocking.rs" ;;
        license) printf '\nchanged license\n' >> "$fixture/$mutation/pocket-ic-provenance/LICENSE" ;;
        adaptation) printf '\nchanged patch\n' >> "$fixture/$mutation/pocket-ic-provenance/upstream/fallible-client.patch" ;;
        snapshot-adaptation) printf '\nchanged patch\n' >> "$fixture/$mutation/pocket-ic-provenance/upstream/snapshot-client.patch" ;;
    esac
    if bash "$root/scripts/ci/check-pocket-ic-source.sh" "$fixture/$mutation" > "$fixture/refusal.log" 2>&1; then
        echo "Unexpectedly admitted PocketIC mutation: $mutation" >&2; exit 1
    fi
done
echo 'PocketIC adaptations pass; changed imports, bodies, patches and licenses refuse.'

fixture_complete=true
