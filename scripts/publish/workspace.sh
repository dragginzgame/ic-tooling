#!/usr/bin/env bash
# Shared companions: scripts/ci/cargo-publication-selection.pl scripts/ci/check-license.pl scripts/release/group.sh scripts/release/publication-lint.sh
set -euo pipefail

# Cargo workspace publication admission. Requires prepared Cargo 1.100, Git,
# jq and tar; Host admits archive/source proof, Shared owns attempt/Cargo/tee dispatch.
# Cargo owns packaging, dependency ordering, verification, upload
# and index waiting. The caller owns credentials and partial-upload recovery.
mode="${1:-}"
[[ ( $# == 1 && "$mode" == qualify ) ||
   ( ( $# == 1 || ( $# == 3 && "$2" == --packages ) ) && ( "$mode" == publish || "$mode" == check ) ) ]] || {
    echo 'usage: workspace.sh publish|check [--packages "NAME ..."] | qualify' >&2; exit 2;
}
purpose="$mode"
# Complete workspace qualification is always a dry run. Public publication and
# its selected-group dry run retain explicit group admission below.
[[ "$mode" != qualify ]] || mode=check
fail() { echo "publication refused: $1" >&2; exit 1; }
tooling_root="${BASH_SOURCE[0]}"
[[ "$tooling_root" == /* ]] || tooling_root="$PWD/$tooling_root"
tooling_root="$(cd -P "${tooling_root%/*}/../.." && printf '%s/.' "$PWD")"
tooling_root="${tooling_root%/.}"
# Publication and archive qualification share this consumer-owned selection.
# Shared companions: ci/public-packages.txt
public_packages=()
while IFS= read -r package || [[ -n "$package" ]]; do
    public_packages+=("$package")
done < "$tooling_root/ci/public-packages.txt"
packages=("${public_packages[@]}")
if [[ $# == 3 ]]; then
    case "$3" in *$'\r'*|*$'\n'*) fail 'package selections must not contain CR/LF' ;; esac
    IFS=$' \t' read -r -a packages <<< "$3"
    [[ ${#packages[@]} -gt 0 ]] || fail 'select at least one public package'
    seen='|'
    for package in "${packages[@]}"; do
        eligible=false
        for public_package in "${public_packages[@]}"; do
            [[ "$package" != "$public_package" ]] || eligible=true
        done
        "$eligible" || fail "unknown or private publication package: $package"
        [[ "$seen" != *"|$package|"* ]] || fail "duplicate publication package: $package"
        seen="$seen$package|"
    done
fi
package_path() {
    jq -er --arg package "$1" --arg prefix "$root/" '
        .packages[] | select(.name == $package) | .manifest_path
        | if type == "string" and startswith($prefix) and endswith("/Cargo.toml")
          then ltrimstr($prefix) | rtrimstr("/Cargo.toml")
          else error("package manifest is outside the workspace") end
        | if length > 0 and (test("[\u0000\r\n]") | not)
             and (split("/") | all(. != "" and . != "." and . != ".."))
          then . else error("invalid package directory") end
    ' "$attempt/metadata.json"
}
# Preserve path newlines, removing only Git's terminating record newline.
root="$(git rev-parse --show-toplevel && printf '/.')"
root="${root%$'\n/.'}"
case "$root" in *$'\r'*|*$'\n'*) fail 'directory paths must not contain CR/LF' ;; esac
cd -P -- "$root"
root="$PWD"
case "$root" in *$'\r'*|*$'\n'*) fail 'directory paths must not contain CR/LF' ;; esac
selected_group=''
if [[ "$purpose" != qualify ]]; then
    group_members="$(bash "$tooling_root/scripts/release/group.sh" "$root/ci/cargo-release-groups.json" "${RELEASE_GROUP:-}" members)"
    selected_group="$RELEASE_GROUP"
    # Early group admission owns membership selection; the independent public
    # allowlist and complete Cargo/source admission remain below.
    group_selection="$(jq -ner --arg members "$group_members" --arg public "${public_packages[*]}" '
        $members | split("\n") as $selected |
        $public | split(" ") | map(select(. as $name | $selected | index($name))) |
        if length == 0 then error("release group has no public packages") else join(" ") end
    ')" || fail 'release group has no public packages'
    if [[ $# == 1 ]]; then
        IFS=' ' read -r -a packages <<< "$group_selection"
    else
        for package in "${packages[@]}"; do
            case " $group_selection " in *" $package "*) ;; *) fail "package is outside the selected release group: $package" ;; esac
        done
    fi
fi
make --no-print-directory toko-tooling-check
source="$(git rev-parse --verify HEAD)" || exit $?
[[ "$source" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] || fail 'invalid source identity'
dirty="$(git status --porcelain --untracked-files=all)" || exit $?
[[ "$mode" != publish || -z "$dirty" ]] || fail 'commit workspace changes before publishing'
[[ -f LICENSE.md && ! -L LICENSE.md ]] || fail 'expected a regular root licence'
perl "$tooling_root/scripts/ci/check-license.pl" --file "$root/LICENSE.md"
[[ ! -L target && ! -L target/publish && ! -L target/publish/admission ]] || fail 'symlinked publication evidence directory'
umask 077
mkdir -p target/publish/admission
[[ ! -e target/publish/.cargo-publication.lock && ! -L target/publish/.cargo-publication.lock ]] ||
    fail 'Cargo attempt lock is occupied; reconcile retained evidence before removing a stale lock'
lock=target/publish/lock
mkdir "$lock" 2>/dev/null || fail 'publication lock is occupied; inspect its owner before removing a stale lock'
trap 'rm -f "$lock/owner"; rmdir "$lock"' EXIT
printf '%s\n' "$$" > "$lock/owner"
attempt="$(mktemp -d "target/publish/admission/$mode.XXXXXX")"
publication_attempt="$root/target/publish/${attempt##*/}"
printf '%s\n' "$publication_attempt" > "$attempt/publication-attempt"
printf '%s\n' "$selected_group" > "$attempt/release-group"
printf '%s\n' "$purpose" > "$attempt/purpose"
echo "Publication admission evidence: $root/$attempt"
printf '%s\n' "$dirty" > "$attempt/working-status"
cargo metadata --manifest-path "$root/Cargo.toml" --no-deps --format-version 1 \
    --locked --offline > "$attempt/metadata.json" 2> "$attempt/metadata.log" ||
    fail 'locked workspace metadata validation failed'
jq -e '
    . as $workspace
    | [.packages[] | select(.id as $id | $workspace.workspace_members | index($id))] as $packages
    | ($packages | map(select(.publish != [])) | map(.name) | sort) == ($ARGS.positional | sort)
      and ($packages | all(.publish == ["crates-io"]))
      and ($packages | all(.repository == "https://github.com/dragginzgame/ic-tooling"))
      and ($packages | all(.license == null and (.license_file | type) == "string"))
      and ($packages | any(.name == "ic-host-wasm-inspect" and
          any(.targets[]; .name == "ic-host-wasm-inspect" and (.kind | index("bin")))))
' "$attempt/metadata.json" --args "${public_packages[@]}" > /dev/null || fail 'expected the complete public crates.io roster and root licence'
perl "$tooling_root/scripts/ci/cargo-publication-selection.pl" "$attempt/metadata.json" \
    "${public_packages[*]}" "${packages[*]}" crates-io > "$attempt/selected-packages"
selectors=()
while IFS= read -r -d '' selector; do selectors+=("$selector"); done < "$attempt/selected-packages"
package_directories=()
for package in "${packages[@]}"; do
    directory="$(package_path "$package")" || fail "invalid package directory for $package"
    physical="$(cd -P -- "$root/$directory" && printf '%s/.' "$PWD")" || fail "missing package directory for $package"
    [[ "${physical%/.}" == "$root/$directory" ]] || fail "aliased package directory for $package"
    [[ -f "$directory/Cargo.toml" && ! -L "$directory/Cargo.toml" ]] || fail "expected a regular manifest for $package"
    licence="$(jq -er --arg package "$package" '.packages[] | select(.name == $package) | .license_file' "$attempt/metadata.json")"
    [[ "$licence" == /* ]] || licence="$root/$directory/$licence"
    licence_parent="$(cd -P -- "${licence%/*}" && printf '%s/.' "$PWD")" || fail "missing licence directory for $package"
    [[ "${licence_parent%/.}/${licence##*/}" == "$root/LICENSE.md" ]] || fail "expected the root licence for $package"
    package_directories+=("$directory")
done
repository="$(jq -r '.packages[0].repository' "$attempt/metadata.json")"
qualified_version=$(jq -er --arg package "${packages[0]}" \
    '.packages[] | select(.name == $package) | .version' "$attempt/metadata.json")
# Source admission precedes lint and registry effects. A committed release may
# reuse its exact full lint receipt; standalone, stale and focused-only attempts
# retain fresh workspace lint. Archive/source admission always stays fresh.
if [[ "$purpose" != qualify ]] &&
    bash "$tooling_root/scripts/release/publication-lint.sh" "$selected_group" "$qualified_version" "$source" \
        > "$attempt/lint-evidence.log" 2>&1; then
    cat "$attempt/lint-evidence.log"
else
    if [[ "$purpose" == qualify ]]; then
        printf 'Workspace lint: fresh; reason=publication-qualification\n'
    else
        cat "$attempt/lint-evidence.log"
        printf 'Workspace lint: fresh; reason=qualified-lint-not-admitted\n'
    fi
    make --no-print-directory clippy-workspace
fi
# Persist Host's admitted package/provenance selection before package preparation.
# Shared Tooling records the exact Cargo command in its separate fresh attempt.
printf '%s\n' publication-2 "$mode" "$source" crates-io \
    "${selectors[@]}" > "$attempt/intent"
if [[ "$mode" == publish ]]; then
    # Inspect Cargo's actual package provenance before upload. Package preparation
    # resolves dependencies but does not upload or build; publish still owns
    # compilation, dependency order and all registry effects.
    package_arguments=(package --manifest-path "$root/Cargo.toml")
    for selector in "${selectors[@]}"; do package_arguments+=(--package "$selector"); done
    package_arguments+=(--registry crates-io --all-features --locked --no-verify --target-dir "$root/target")
    printf '%s\0' "${package_arguments[@]}" > "$attempt/package-arguments"
    cargo "${package_arguments[@]}" > "$attempt/package.log" 2>&1 || fail 'package preparation failed; no upload attempted'
    provenance_arguments=()
    for index in "${!packages[@]}"; do
        package="${packages[$index]}"
        directory="${package_directories[$index]}"
        provenance_arguments+=(--package "${selectors[$index]}" "$directory")
    done
    bash "$tooling_root/scripts/ci/check-cargo-package-provenance.sh" \
        "$attempt/provenance" "$root/target/package" "$repository" "$source" "${provenance_arguments[@]}"
    for index in "${!packages[@]}"; do
        package="${packages[$index]}"
        version="${selectors[$index]#*@}"
        archive="$root/target/package/$package-$version.crate"
        perl "$tooling_root/scripts/ci/check-license.pl" --file "$root/LICENSE.md" \
            --archive "$archive" --member "$package-$version/LICENSE.md"
    done
fi
observed_source="$(git rev-parse --verify HEAD)" || exit $?
observed_dirty="$(git status --porcelain --untracked-files=all)" || exit $?
[[ "$observed_source" == "$source" && "$observed_dirty" == "$dirty" ]] || fail 'source changed before publication'
arguments=()
for selector in "${selectors[@]}"; do arguments+=(--package "$selector"); done
arguments+=(--manifest-path "$root/Cargo.toml" --target-dir "$root/target" --all-features)
[[ "$mode" != check ]] || arguments+=(--allow-dirty)
bash "$tooling_root/scripts/ci/run-cargo-publication.sh" "$mode" "$publication_attempt" \
    "$source" crates-io "${arguments[@]}"
