#!/usr/bin/env bash
# Shared companions: scripts/ci/cargo-publication-selection.pl scripts/release/group.sh
set -euo pipefail

# Cargo workspace publication admission. Requires prepared Cargo 1.99, Git,
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
    bash "$tooling_root/scripts/release/group.sh" "$root/ci/cargo-release-groups.json" "${RELEASE_GROUP:-}"
    selected_group="$RELEASE_GROUP"
    group_selection="$(jq -er --arg group "$RELEASE_GROUP" --arg public "${public_packages[*]}" '
        [.groups[] | select(.name == $group)] |
        (if length != 1 then error("unknown release group") else .[0].members end) as $members |
        $public | split(" ") | map(select(. as $name | $members | index($name))) |
        if length == 0 then error("release group has no public packages") else join(" ") end
    ' ci/cargo-release-groups.json)" || fail 'invalid publication release group'
    if [[ $# == 1 ]]; then
        IFS=' ' read -r -a packages <<< "$group_selection"
    else
        for package in "${packages[@]}"; do
            case " $group_selection " in *" $package "*) ;; *) fail "package is outside the selected release group: $package" ;; esac
        done
    fi
fi
source="$(git rev-parse --verify HEAD)" || exit $?
[[ "$source" =~ ^([0-9a-f]{40}|[0-9a-f]{64})$ ]] || fail 'invalid source identity'
dirty="$(git status --porcelain --untracked-files=all)" || exit $?
[[ "$mode" != publish || -z "$dirty" ]] || fail 'commit workspace changes before publishing'
[[ -f LICENSE && ! -L LICENSE ]] || fail 'expected a regular root license notice'
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
      and ($packages | all(.publish == ["crates-io"] or (.publish == [] and (.name == "ic-testkit-dev" or .name == "ic-testkit-perf-probe"))))
      and ($packages | all(.repository == "https://github.com/dragginzgame/ic-tooling"))
      and ($packages | any(.name == "ic-host-wasm-inspect" and
          any(.targets[]; .name == "ic-host-wasm-inspect" and (.kind | index("bin")))))
' "$attempt/metadata.json" --args "${public_packages[@]}" > /dev/null || fail 'expected the complete public crates.io roster'
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
    notice="$directory/LICENSE"
    [[ -f "$notice" && ! -L "$notice" ]] || fail "expected a regular license notice for $package"
    cmp -s LICENSE "$notice" || fail "license notice differs from the root notice for $package"
    package_directories+=("$directory")
done
repository="$(jq -r '.packages[0].repository' "$attempt/metadata.json")"
# Source admission precedes the existing full lint gate. Refusal must stop
# packaging and registry effects; this path never prepares missing tools.
make --no-print-directory clippy-workspace
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
        directory="${package_directories[$index]}"
        perl "$tooling_root/scripts/ci/check-license.pl" --kind mit --file "$root/$directory/LICENSE" \
            --archive "$root/target/package/$package-$version.crate" --member "$package-$version/LICENSE"
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
