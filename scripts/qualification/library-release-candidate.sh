#!/usr/bin/env bash
# Explicit library/tool archive readiness; no publication or sibling edits.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
CDPATH='' cd -- "$repo"
[[ $# -gt 0 ]] || { echo 'Select release groups: http text testkit cargo-metadata sign' >&2; exit 2; }
packages=()
selection=()
features=()
testkit=false
sign=false
seen=' '
for group in "$@"; do
  [[ "$seen" != *" $group "* ]] || { echo "Repeated group: $group" >&2; exit 2; }
  seen+="$group "
  case "$group" in
    http) package=ic-host-http ;;
    text) package=ic-host-text ;;
    cargo-metadata) package=ic-cargo-metadata ;;
    testkit) package=ic-testkit; testkit=true; features+=(--features 'ic-testkit/held-reply-fixtures,ic-testkit/cli') ;;
    sign) package=ic-host-sign; sign=true; features+=(--features 'ic-host-sign/provider') ;;
    *) echo "Unsupported library release group: $group" >&2; exit 2 ;;
  esac
  jq -e --arg group "$group" --arg package "$package" \
    '.groups[] | select(.name == $group) | .members == [$package]' ci/cargo-release-groups.json >/dev/null
  packages+=("$package")
  selection+=(-p "$package")
done
if [[ "$sign" == true ]]; then
  : "${SIGNER_TEST_ICP:?select the absolute prepared ICP 1.7.0 executable}"
  [[ "$SIGNER_TEST_ICP" == /* && -f "$SIGNER_TEST_ICP" && -x "$SIGNER_TEST_ICP" ]] || exit 2
fi
network=(--offline)
case "${LIBRARY_RELEASE_ONLINE:-false}" in
  true) network=() ;;
  false) ;;
  *) echo 'LIBRARY_RELEASE_ONLINE must be true or false' >&2; exit 2 ;;
esac
if [[ "$testkit" == true ]]; then
  : "${TESTKIT_SERVER_CLI:?select an absolute prepared ic-testkit-server executable}"
  : "${TESTKIT_SERVER_DIRECTORY:?select an absolute prepared Testkit server directory}"
  : "${TESTKIT_SERVER_ARCHIVE:?select the prepared PocketIC 16.1.0 gzip archive}"
  : "${TESTKIT_GZIP:?select an absolute prepared gzip executable}"
  [[ "$TESTKIT_SERVER_ARCHIVE" == /* && -f "$TESTKIT_SERVER_ARCHIVE" && ! -L "$TESTKIT_SERVER_ARCHIVE" ]] || {
    echo 'Prepared simulator archive must be an absolute regular non-symlink file' >&2; exit 2;
  }
  for tool in "$TESTKIT_SERVER_CLI" "$TESTKIT_GZIP"; do
    [[ "$tool" == /* && -x "$tool" ]] || { echo 'Prepared tools must be absolute executable paths' >&2; exit 2; }
  done
  server="$("$TESTKIT_SERVER_CLI" check "$TESTKIT_SERVER_DIRECTORY")"
fi
mkdir -p target
scratch="$(mktemp -d "$repo/target/library-release.XXXXXXXX")"
trap 'printf "Library release candidate evidence: %s\n" "$scratch"' EXIT
candidate="$scratch/candidate"
mkdir -p "$candidate/crates" "$scratch/archives"
inputs=(Cargo.toml Cargo.lock LICENSE.md ci/cargo-release-groups.json
  scripts/qualification/archive-compilation.sh
  scripts/qualification/library-release-candidate.sh scripts/qualification/testkit-installed.sh
  scripts/qualification/test-pocketic-setup.sh scripts/qualification/test-testkit-server-cli.sh
  scripts/qualification/sign-keyring.sh)
for package in "${packages[@]}"; do inputs+=("crates/$package"); done
checkpoint() { find "${inputs[@]}" -type f -exec shasum -a 256 {} + | LC_ALL=C sort; }
checkpoint > "$scratch/source.sha256"
git rev-parse HEAD > "$scratch/source-head"
git diff --binary HEAD -- "${inputs[@]}" > "$scratch/source.patch"
tar -cf "$scratch/source.tar" "${inputs[@]}"
printf '%s\n' "$@" > "$scratch/groups.txt"
if [[ "$testkit" == true ]]; then
  shasum -a 256 "$TESTKIT_SERVER_CLI" "$server" "$TESTKIT_SERVER_ARCHIVE" "$TESTKIT_GZIP" > "$scratch/tools.sha256"
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
  if [[ -d "crates/$package/tests" ]]; then diff -r "crates/$package/tests" "$candidate/crates/$package/tests"; fi
  tar -tzf "$archive" > "$scratch/$package-files.txt"
done
(CDPATH='' cd -- "$scratch/archives" && shasum -a 256 ./*.crate) > "$scratch/archives.sha256"
{
  printf '[workspace]\nresolver = "3"\nmembers = [\n'
  for package in "${packages[@]}"; do printf '  "crates/%s",\n' "$package"; done
  printf ']\n'
} > "$candidate/Cargo.toml"
cp Cargo.lock "$candidate/Cargo.lock"
source "$repo/scripts/qualification/archive-compilation.sh"
prepare_archive_compilation "$repo" "$candidate" "$scratch" library
cargo metadata --manifest-path "$candidate/Cargo.toml" --format-version 1 "${network[@]}" > "$scratch/candidate-metadata.json"
# Extracted archives resolve Host dependencies from the registry rather than root
# workspace paths. Fetch that graph explicitly before the offline feature checks.
if [[ "${LIBRARY_RELEASE_ONLINE:-false}" == true ]]; then
  cargo fetch --manifest-path "$candidate/Cargo.toml" --locked > "$scratch/candidate-fetch.log" 2>&1
fi
for package in "${packages[@]}"; do
  # A selected archive and a transitive registry dependency may share a name.
  # Admit the workspace member; preserve registry resolution for its dependencies.
  jq -e --arg package "$package" --arg root "$candidate" '
    .workspace_members as $members |
    [.packages[] | select(.name == $package and (.id as $id | $members | index($id)))] as $p |
    ($p | length) == 1 and $p[0].source == null and
    $p[0].manifest_path == ($root + "/crates/" + $package + "/Cargo.toml") and
    $p[0].publish == ["crates-io"] and ($p[0].description | length > 0)
  ' "$scratch/candidate-metadata.json" >/dev/null
done
if [[ "$testkit" == true ]]; then
  cargo tree --manifest-path "$candidate/Cargo.toml" -p ic-testkit --no-default-features \
    -e normal,build --prefix none --locked --offline > "$scratch/default-tree.txt"
  if grep -Eq '^(clap|ctrlc|ic-host-text|wat|ic-platform|ic-platform-protocol|ic-platform-types) v' "$scratch/default-tree.txt"; then
    echo 'CLI/fixture dependencies entered the default Testkit graph' >&2; exit 1
  fi
  for feature in managed-server held-reply-fixtures; do
    cargo tree --manifest-path "$candidate/Cargo.toml" -p ic-testkit --features "$feature" \
      -e normal,build --prefix none --locked --offline > "$scratch/$feature-tree.txt"
    if grep -Eq '^(clap|ctrlc|ic-host-text) v' "$scratch/$feature-tree.txt"; then
      echo "CLI dependencies entered the $feature graph" >&2; exit 1
    fi
  done
  cargo tree --manifest-path "$candidate/Cargo.toml" -p ic-testkit --features cli \
    -e normal,build --prefix none --locked --offline > "$scratch/cli-tree.txt"
  if grep -Eq '^(wat|ic-platform|ic-platform-protocol|ic-platform-types) v' "$scratch/cli-tree.txt"; then
    echo 'Fixture dependencies entered the CLI graph' >&2; exit 1
  fi
  cargo test --manifest-path "$candidate/Cargo.toml" -p ic-testkit --no-default-features \
    --locked --offline > "$scratch/default-tests.log" 2>&1
  RUSTDOCFLAGS='-D warnings' cargo doc --manifest-path "$candidate/Cargo.toml" -p ic-testkit \
    --no-default-features --no-deps --locked --offline > "$scratch/default-docs.log" 2>&1
fi
if [[ "$sign" == true ]]; then
  cargo tree --manifest-path "$candidate/Cargo.toml" -p ic-host-sign --no-default-features \
    -e normal,build --prefix none --locked --offline > "$scratch/sign-default-tree.txt"
  if grep -Eq '^(clap|keyring|secret-service|security-framework|zbus) v' "$scratch/sign-default-tree.txt"; then
    echo 'Credential provider dependencies entered the default Sign library graph' >&2; exit 1
  fi
fi
cargo test --manifest-path "$candidate/Cargo.toml" --workspace "${features[@]}" \
  --locked --offline > "$scratch/tests.log" 2>&1
RUSTDOCFLAGS='-D warnings' cargo doc --manifest-path "$candidate/Cargo.toml" --workspace \
  "${features[@]}" --no-deps --locked --offline > "$scratch/docs.log" 2>&1
if [[ "$testkit" == true ]]; then
  cargo install --path "$candidate/crates/ic-testkit" --features cli --debug \
    --bin ic-testkit-server --locked --offline --root "$scratch/cli-install" \
    > "$scratch/cli-install.log" 2>&1
  candidate_cli="$scratch/cli-install/bin/ic-testkit-server"
  "$candidate_cli" --version > "$scratch/cli-version.txt"
  TESTKIT_SERVER_CLI="$candidate_cli" bash "$repo/scripts/qualification/test-pocketic-setup.sh" \
    > "$scratch/cli-setup.log" 2>&1
  TESTKIT_SERVER_CLI="$candidate_cli" bash "$repo/scripts/qualification/test-testkit-server-cli.sh" \
    > "$scratch/cli-run.log" 2>&1
  env -u IC_TESTKIT_POCKET_IC_URL "$candidate_cli" run \
    "$TESTKIT_SERVER_DIRECTORY" "$scratch/server" --ttl 300 -- \
    bash "$repo/scripts/qualification/testkit-installed.sh" "$candidate/Cargo.toml" "$scratch" \
    > "$scratch/installed.log" 2>&1
  cargo test --manifest-path "$candidate/Cargo.toml" -p ic-testkit --features cli,held-reply-fixtures \
    --lib --locked --offline server::tests::installed_ -- --ignored --color never \
    > "$scratch/managed-installed.log" 2>&1
  grep -Fq 'test result: ok. 2 passed; 0 failed' "$scratch/managed-installed.log"
  shasum -a 256 -c "$scratch/tools.sha256"
fi
if [[ "$sign" == true ]]; then
  cargo install --path "$candidate/crates/ic-host-sign" --features provider --debug \
    --bin ic-sign --locked --offline --root "$scratch/sign-cli-install" > "$scratch/sign-cli-install.log" 2>&1
  "$scratch/sign-cli-install/bin/ic-sign" --version > "$scratch/sign-cli-version.txt"
  SIGNER_TEST_MANIFEST="$candidate/Cargo.toml" SIGNER_TEST_HELPER="$scratch/sign-cli-install/bin/ic-sign" \
    bash "$repo/scripts/qualification/sign-keyring.sh" > "$scratch/sign-provider.log" 2>&1
fi
checkpoint > "$scratch/source-after.sha256"
cmp "$scratch/source.sha256" "$scratch/source-after.sha256"
printf '%s\n' "$(uname -sm)" > "$scratch/complete.txt"
printf 'Selected packaged libraries passed lint, graph, tests and documentation checks.\n'
