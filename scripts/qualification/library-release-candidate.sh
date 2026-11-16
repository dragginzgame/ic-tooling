#!/usr/bin/env bash
# Explicit HTTP/Text/Testkit archive readiness; no publication or sibling edits.
set -euo pipefail
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
cd "$repo"
[[ $# -gt 0 ]] || { echo 'Select release groups: http text testkit' >&2; exit 2; }
packages=()
selection=()
features=()
testkit=false
seen=' '
for group in "$@"; do
  [[ "$seen" != *" $group "* ]] || { echo "Repeated group: $group" >&2; exit 2; }
  seen+="$group "
  case "$group" in
    http) package=ic-host-http ;;
    text) package=ic-host-text ;;
    testkit) package=ic-testkit; testkit=true; features=(--features ic-testkit/held-reply-fixtures) ;;
    *) echo "Unsupported library release group: $group" >&2; exit 2 ;;
  esac
  jq -e --arg group "$group" --arg package "$package" \
    '.groups[] | select(.name == $group) | .members == [$package]' ci/cargo-release-groups.json >/dev/null
  packages+=("$package")
  selection+=(-p "$package")
done
network=(--offline)
case "${LIBRARY_RELEASE_ONLINE:-false}" in
  true) network=() ;;
  false) ;;
  *) echo 'LIBRARY_RELEASE_ONLINE must be true or false' >&2; exit 2 ;;
esac
if [[ "$testkit" == true ]]; then
  : "${TESTKIT_RELEASE_LAUNCHER:?select a prepared ic-testkit-server 0.106.0 executable}"
  : "${POCKET_IC_BIN:?select a prepared PocketIC 16.1.0 executable}"
  for tool in "$TESTKIT_RELEASE_LAUNCHER" "$POCKET_IC_BIN"; do
    [[ "$tool" == /* && -x "$tool" ]] || { echo 'Prepared tools must be absolute executable paths' >&2; exit 2; }
  done
  [[ "$("$TESTKIT_RELEASE_LAUNCHER" --version)" == 'ic-testkit-server 0.106.0' ]]
  [[ "$("$POCKET_IC_BIN" --version)" == 'pocket-ic-server 16.1.0' ]]
fi
mkdir -p target
scratch="$(mktemp -d "$repo/target/library-release.XXXXXXXX")"
trap 'printf "Library release candidate evidence: %s\n" "$scratch"' EXIT
candidate="$scratch/candidate"
mkdir -p "$candidate/crates" "$scratch/archives"
inputs=(Cargo.toml Cargo.lock LICENSE.md ci/cargo-release-groups.json
  scripts/qualification/library-release-candidate.sh scripts/qualification/testkit-installed.sh)
for package in "${packages[@]}"; do inputs+=("crates/$package"); done
checkpoint() { find "${inputs[@]}" -type f -exec shasum -a 256 {} + | LC_ALL=C sort; }
checkpoint > "$scratch/source.sha256"
git rev-parse HEAD > "$scratch/source-head"
git diff --binary HEAD -- "${inputs[@]}" > "$scratch/source.patch"
tar -cf "$scratch/source.tar" "${inputs[@]}"
printf '%s\n' "$@" > "$scratch/groups.txt"
if [[ "$testkit" == true ]]; then
  shasum -a 256 "$TESTKIT_RELEASE_LAUNCHER" "$POCKET_IC_BIN" > "$scratch/tools.sha256"
fi
if [[ "${LIBRARY_RELEASE_ONLINE:-false}" == true ]]; then cargo fetch --locked > "$scratch/fetch.log" 2>&1; fi
# Lint before packaging or expensive runtime checks. The extracted src trees are
# compared below, so this evidence qualifies the exact archived implementation.
cargo clippy "${selection[@]}" --all-targets "${features[@]}" --locked --offline -- -D warnings > "$scratch/clippy.log" 2>&1
if [[ "$testkit" == true ]]; then
  cargo clippy -p ic-testkit --all-targets --no-default-features --locked --offline -- -D warnings > "$scratch/default-clippy.log" 2>&1
fi
cargo metadata --format-version 1 --no-deps --locked --offline > "$scratch/source-metadata.json"
for package in "${packages[@]}"; do
  version="$(jq -er --arg package "$package" '.packages[] | select(.name == $package) | .version' "$scratch/source-metadata.json")"
  cargo package -p "$package" --locked --allow-dirty "${network[@]}" --no-verify \
    --target-dir "$scratch/packaging" > "$scratch/$package-package.log" 2>&1
  archive="$scratch/packaging/package/$package-$version.crate"
  cp "$archive" "$scratch/archives/"
  tar -xzf "$archive" -C "$candidate/crates"
  mv "$candidate/crates/$package-$version" "$candidate/crates/$package"
  cmp LICENSE.md "$candidate/crates/$package/LICENSE.md"
  diff -r "crates/$package/src" "$candidate/crates/$package/src"
  tar -tzf "$archive" > "$scratch/$package-files.txt"
done
(cd "$scratch/archives" && shasum -a 256 ./*.crate) > "$scratch/archives.sha256"
{
  printf '[workspace]\nresolver = "3"\nmembers = [\n'
  for package in "${packages[@]}"; do printf '  "crates/%s",\n' "$package"; done
  printf ']\n'
} > "$candidate/Cargo.toml"
cp Cargo.lock "$candidate/Cargo.lock"
export CARGO_TARGET_DIR="$scratch/compiled"
cargo metadata --manifest-path "$candidate/Cargo.toml" --format-version 1 "${network[@]}" > "$scratch/candidate-metadata.json"
for package in "${packages[@]}"; do
  jq -e --arg package "$package" --arg root "$candidate" '
    [.packages[] | select(.name == $package)] as $p |
    ($p | length) == 1 and $p[0].source == null and
    $p[0].manifest_path == ($root + "/crates/" + $package + "/Cargo.toml") and
    $p[0].publish == ["crates-io"] and ($p[0].description | length > 0)
  ' "$scratch/candidate-metadata.json" >/dev/null
done
if [[ "$testkit" == true ]]; then
  cargo tree --manifest-path "$candidate/Cargo.toml" -p ic-testkit --no-default-features \
    -e normal,build --prefix none --locked --offline > "$scratch/default-tree.txt"
  if grep -Eq '^(wat|ic-platform|ic-platform-protocol|ic-platform-types) v' "$scratch/default-tree.txt"; then
    echo 'Fixture dependencies entered the default Testkit graph' >&2; exit 1
  fi
  cargo test --manifest-path "$candidate/Cargo.toml" -p ic-testkit --no-default-features \
    --locked --offline > "$scratch/default-tests.log" 2>&1
  RUSTDOCFLAGS='-D warnings' cargo doc --manifest-path "$candidate/Cargo.toml" -p ic-testkit \
    --no-default-features --no-deps --locked --offline > "$scratch/default-docs.log" 2>&1
fi
cargo test --manifest-path "$candidate/Cargo.toml" --workspace "${features[@]}" \
  --locked --offline > "$scratch/tests.log" 2>&1
RUSTDOCFLAGS='-D warnings' cargo doc --manifest-path "$candidate/Cargo.toml" --workspace \
  "${features[@]}" --no-deps --locked --offline > "$scratch/docs.log" 2>&1
if [[ "$testkit" == true ]]; then
  env -u IC_TESTKIT_POCKET_IC_URL "$TESTKIT_RELEASE_LAUNCHER" run --ttl 300 \
    --server-stdout "$scratch/server.stdout" --server-stderr "$scratch/server.stderr" -- \
    bash "$repo/scripts/qualification/testkit-installed.sh" "$candidate/Cargo.toml" "$scratch" \
    > "$scratch/installed.log" 2>&1
  shasum -a 256 -c "$scratch/tools.sha256"
fi
checkpoint > "$scratch/source-after.sha256"
cmp "$scratch/source.sha256" "$scratch/source-after.sha256"
printf '%s\n' "$(uname -sm)" > "$scratch/complete.txt"
printf 'Selected packaged libraries passed lint, graph, tests and documentation checks.\n'
