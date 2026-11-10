#!/usr/bin/env bash
set -euo pipefail

# Copy Host workspace metadata and empty lib/bin entrypoints into an owned test
# directory. Callers keep publication policy, perturbations and effect stubs.
[[ $# == 2 && "$1" == /* && -d "$2" && ! -L "$2" ]] || {
    echo 'usage: prepare-workspace-fixture.sh ABSOLUTE_SOURCE OWNED_DIRECTORY' >&2
    exit 2
}
source_root="$1" destination="$2"
cp "$source_root/Cargo.toml" "$source_root/Cargo.lock" "$source_root/CHANGELOG.md" \
    "$source_root/README.md" "$source_root/LICENSE.md" "$destination/"
if [[ -f "$source_root/ci/cargo-release-groups.json" ]]; then
    mkdir -p "$destination/ci"
    cp "$source_root/ci/cargo-release-groups.json" "$destination/ci/"
    jq -er '[.groups[] | .changelog, (.metadata_files // [])[]] + (.shared_metadata_files // []) | unique[]' \
        "$source_root/ci/cargo-release-groups.json" |
        while IFS= read -r path; do
            case "/$path/" in //*|*/../*|*/./*|*//*) echo 'invalid fixture release path' >&2; exit 1 ;; esac
            if [[ "$path" == */* ]]; then mkdir -p "$destination/${path%/*}"; fi
            cp "$source_root/$path" "$destination/$path"
        done
fi
cargo metadata --manifest-path "$source_root/Cargo.toml" --no-deps \
    --format-version 1 --locked --offline > "$destination/source-metadata.json"
mkdir -p "$destination/qualification"
cp "$source_root/qualification/Cargo.toml" "$source_root/qualification/Cargo.lock" "$destination/qualification/"
cargo metadata --manifest-path "$source_root/qualification/Cargo.toml" --no-deps \
    --format-version 1 --locked --offline > "$destination/qualification/source-metadata.json"
jq -r '. as $m | .packages[] |
    select(.id as $id | $m.workspace_members | index($id)) |
    .manifest_path,
    (.targets[] | select(.kind | index("lib") or index("bin") or index("rlib") or index("cdylib")) | .src_path)' \
    "$destination/source-metadata.json" "$destination/qualification/source-metadata.json" |
    while IFS= read -r path; do
        case "$path" in
            "$source_root"/*) relative="${path#"$source_root"/}" ;;
            *) echo 'fixture input is outside the selected workspace' >&2; exit 1 ;;
        esac
        mkdir -p "$destination/${relative%/*}"
        if [[ "${relative##*/}" == Cargo.toml ]]; then
            [[ -f "$path" && ! -L "$path" ]] || {
                echo 'fixture requires a real regular manifest' >&2; exit 1;
            }
            cp "$path" "$destination/$relative"
        else
            : > "$destination/$relative"
        fi
    done
