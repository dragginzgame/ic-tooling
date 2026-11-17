#!/usr/bin/env bash
# Package and qualify only the first release's dependency-complete build stack.
# Does not publish, commit, mutate siblings, or claim native-host equivalence.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/build-release.XXXXXXXX")"
trap 'printf "Build release candidate evidence: %s\n" "$scratch"' EXIT
candidate="$scratch/candidate"
mkdir -p "$candidate/crates" "$scratch/archives"
packages=(ic-host-artifacts ic-host-fs ic-host-process ic-build)
network=(--offline)
case "${BUILD_RELEASE_ONLINE:-false}" in
  true) network=() ;;
  false) ;;
  *) echo 'BUILD_RELEASE_ONLINE must be true or false.' >&2; exit 2 ;;
esac
case "${BUILD_RELEASE_CONSUMER:-current}" in
  current|none) ;;
  *) echo 'BUILD_RELEASE_CONSUMER must be current or none.' >&2; exit 2 ;;
esac
patches=()
for package in "${packages[@]}"; do
  patches+=(--config "patch.crates-io.$package.path=\"$repo/crates/$package\"")
done
cargo metadata --manifest-path "$repo/Cargo.toml" --format-version 1 --no-deps --locked --offline > "$scratch/source-metadata.json"
git -C "$repo" rev-parse HEAD > "$scratch/source-head"
git -C "$repo" diff --binary HEAD -- Cargo.toml Cargo.lock crates/ic-build crates/ic-host-artifacts crates/ic-host-fs crates/ic-host-process > "$scratch/source.patch"
for package in "${packages[@]}"; do
  version="$(jq -er --arg package "$package" '.packages[] | select(.name == $package) | .version' "$scratch/source-metadata.json")"
  cargo package --manifest-path "$repo/Cargo.toml" -p "$package" --allow-dirty ${network[@]+"${network[@]}"} --no-verify \
    --target-dir "$scratch/packaging" "${patches[@]}" > "$scratch/$package-package.log" 2>&1
  archive="$scratch/packaging/package/$package-$version.crate"
  cp "$archive" "$scratch/archives/"
  tar -xzf "$archive" -C "$candidate/crates"
  mv "$candidate/crates/$package-$version" "$candidate/crates/$package"
  cmp "$repo/LICENSE.md" "$candidate/crates/$package/LICENSE.md"
done
(CDPATH='' cd -- "$scratch/archives" && shasum -a 256 ./*.crate) > "$scratch/archives.sha256"
# Normalized archive manifests retain registry requirements, not source paths.
# Overrides admit the as-yet unpublished suppliers only in this test workspace.
{
  printf '[workspace]\nresolver = "3"\nmembers = [\n'
  for package in "${packages[@]}"; do printf '  "crates/%s",\n' "$package"; done
  printf ']\n[patch.crates-io]\n'
  for package in "${packages[@]}"; do printf '%s = { path = "crates/%s" }\n' "$package" "$package"; done
} > "$candidate/Cargo.toml"
# Build's real-Wasm acceptance fixture belongs to qualification, not its archive.
mkdir "$candidate/qualification"
cp "$repo/qualification/"{Cargo.toml,Cargo.lock} "$candidate/qualification/"
cp -R "$repo/qualification/owned-wasm-probe" "$candidate/qualification/"
cp "$repo/Cargo.lock" "$candidate/Cargo.lock"
cp "$repo/LICENSE.md" "$candidate/LICENSE.md"
source "$repo/scripts/qualification/archive-compilation.sh"
prepare_archive_compilation "$repo" "$candidate" "$scratch" build
cargo metadata --manifest-path "$candidate/Cargo.toml" --format-version 1 ${network[@]+"${network[@]}"} > "$scratch/candidate-metadata.json"
if [[ "${BUILD_RELEASE_ONLINE:-false}" == true ]]; then
  cargo fetch --manifest-path "$candidate/qualification/Cargo.toml" --locked
fi
jq -e --arg root "$candidate" --argjson names '["ic-host-artifacts","ic-host-fs","ic-host-process","ic-build"]' '
  . as $m | all($names[]; . as $name |
    [$m.packages[] | select(.name == $name)] as $p |
    ($p | length) == 1 and $p[0].source == null and
    $p[0].manifest_path == ($root + "/crates/" + $name + "/Cargo.toml") and
    $p[0].publish == ["crates-io"] and ($p[0].description | length > 0))
' "$scratch/candidate-metadata.json" >/dev/null
cargo clippy --manifest-path "$candidate/Cargo.toml" --workspace --all-targets --locked --offline -- -D warnings > "$scratch/clippy.log" 2>&1
cargo test --manifest-path "$candidate/Cargo.toml" --workspace --lib --locked --offline > "$scratch/tests.log" 2>&1
# BSD script injects EOF for piped input; macOS uses a prompt-driven native PTY.
export BUILD_RELEASE_MANIFEST="$candidate/Cargo.toml"
export CARGO_TERM_COLOR=never
case "$(uname -s)" in
  Linux)
    # script's child shell expands the exported path.
    # shellcheck disable=SC2016
    printf 'fixture-answer\n' | script -q -e -c 'cargo test --manifest-path "$BUILD_RELEASE_MANIFEST" -p ic-host-process --lib --locked --offline tests::foreground_terminal_input_survives_direct_child_ownership -- --ignored --exact --color never' /dev/null > "$scratch/terminal.log" 2>&1
    ;;
  Darwin)
    expect "$repo/qualification/process-terminal.exp" "$candidate/Cargo.toml" > "$scratch/terminal.log" 2>&1
    ;;
  *) echo 'Native candidate qualification supports Linux and macOS.' >&2; exit 1 ;;
esac
grep -q 'test result: ok\. 1 passed; 0 failed' "$scratch/terminal.log"
RUSTDOCFLAGS='-D warnings' cargo doc --manifest-path "$candidate/Cargo.toml" --workspace --no-deps --locked --offline > "$scratch/docs.log" 2>&1
if [[ "${BUILD_RELEASE_CONSUMER:-current}" == current ]]; then
  IC_TOOLING_CANDIDATE_ROOT="$candidate" bash "$repo/scripts/qualification/canic-build.sh" metadata > "$scratch/consumer.log" 2>&1
else
  printf 'Current sibling consumer qualification not selected on this host.\n' > "$scratch/consumer.log"
fi
printf 'Four packaged candidates passed native unit/terminal tests, Clippy and Rustdoc.\n'
printf '%s\n' "$(uname -sm)" > "$scratch/complete.txt"
