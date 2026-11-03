#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd -P)"
# shellcheck source=scripts/ci/fixture-lifecycle.sh
source "$root/scripts/ci/fixture-lifecycle.sh"
source_root="${1:-$root/crates/ic-testkit}"
[[ $# -le 1 ]] || exit 2
mkdir -p "$root/target"
scratch="$(mktemp -d "$root/target/pocket-ic-source.XXXXXX")"
fixture_complete=false
trap 'fixture_finish "$?" "$scratch" "PocketIC source evidence retained"' EXIT
provenance="$source_root/pocket-ic-provenance"
read -r checksum file < "$provenance/module-transfer.sha256"
[[ "$file" == module-transfer.patch ]] || exit 1
bash "$root/scripts/ci/verify-file-checksum.sh" sha256 "$checksum" "$provenance/$file"
cp -R "$source_root/src/pocket_ic" "$scratch/src"
mv "$scratch/src/mod.rs" "$scratch/src/lib.rs"
cp -R "$provenance/upstream" "$scratch/upstream"
cp "$provenance/LICENSE" "$scratch/LICENSE"
# Only the reviewed module paths, import formatting and lint scaffolding may
# differ from Platform's qualified donor. All transferred bodies/tests are bound.
patch --directory="$scratch" --strip=1 --reverse --fuzz=0 --batch \
    < "$provenance/module-transfer.patch" > "$scratch/transfer.log"
while read -r checksum file; do
    bash "$root/scripts/ci/verify-file-checksum.sh" sha256 "$checksum" "$scratch/$file"
done < "$provenance/transfer-source.sha256"
bash "$root/scripts/ci/check-pocket-ic-upstream.sh" "$scratch"

fixture_complete=true
