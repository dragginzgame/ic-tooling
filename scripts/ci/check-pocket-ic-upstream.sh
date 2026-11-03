#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd -P)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
source_root="${1:?reconstructed donor source required}"
[[ $# -eq 1 ]] || exit 2
mkdir -p "$root/target"
scratch="$(mktemp -d "$root/target/pocket-ic-upstream.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$scratch" "PocketIC source evidence retained"' EXIT
while read -r checksum file; do
    [[ "$file" == upstream/fallible-client.patch || "$file" == upstream/snapshot-client.patch ]] || exit 1
    bash "$root/scripts/ci/verify-file-checksum.sh" sha256 "$checksum" "$source_root/$file"
done < "$source_root/upstream/adaptations.sha256"
mkdir -p "$scratch/source/src/common"
cp "$source_root/src/"{lib.rs,nonblocking.rs} "$scratch/source/src/"
cp "$source_root/src/common/rest.rs" "$scratch/source/src/common/"
# Reverse only the exact reviewed client adaptation. Never refresh the original
# source/license fingerprints to admit an unrelated imported-body change.
patch --directory="$scratch/source" --strip=1 --reverse --fuzz=0 --batch \
    < "$source_root/upstream/snapshot-client.patch" > "$scratch/snapshot-patch.log"
patch --directory="$scratch/source" --strip=1 --reverse --fuzz=0 --batch \
    < "$source_root/upstream/fallible-client.patch" > "$scratch/patch.log"
while read -r checksum file; do
    [[ -n "$checksum" && -n "$file" ]] || exit 1
    checked="$source_root/$file"
    if [[ -f "$scratch/source/$file" ]]; then checked="$scratch/source/$file"; fi
    case "$file" in
        src/lib.rs|src/nonblocking.rs)
            # Reconstruct only the reviewed management dependency import rename
            # and rustfmt's corresponding import ordering. Original bodies stay exact.
            perl -0777 -pe '
                s{((?:pub )?use ic_management_canister_types::\{.*?\n\};\n)((?:(?:pub )?use ic_transport_types[^\n]+\n)+)}{
                    my ($management, $transport) = ($1, $2);
                    $management =~ s/ic_management_canister_types/pocket_ic_management_canister_types/;
                    $transport . $management;
                }se == 1 or die "management import adaptation changed\n";
            ' "$checked" > "$scratch/reconstructed"
            checked="$scratch/reconstructed" ;;
    esac
    bash "$root/scripts/ci/verify-file-checksum.sh" sha256 "$checksum" "$checked"
done < "$source_root/upstream/maintained-source.sha256"
echo 'Owned PocketIC client source and license fingerprints passed'

fixture_complete=true
