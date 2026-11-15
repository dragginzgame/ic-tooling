#!/usr/bin/env bash
# Qualify one current Platform fixture with a borrowed server.
set -euo pipefail
mode="${1:-raw-rand}"
case "$mode" in raw-rand|agent) ;; *) exit 2 ;; esac
repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="$(cd "${PLATFORM_SOURCE:-$repo/../ic-platform}" && pwd -P)"
# Reuse the same prepared Scheduling tools as the recorded qualification.
# The installed Toko checker selects/admit bytes; never search arbitrary caches.
if [[ -z "${PLATFORM_TESTKIT_CLI:-}" ]]; then
  if ! PLATFORM_TESTKIT_CLI="$(bash "$repo/vendor/toko-tooling/scripts/dev/install-rust-tools.sh" \
    --consumer "$repo/../ic-scheduling" --package ic-testkit --version 0.106.0 \
    --bin ic-testkit-server --profile release --features cli --check)"; then
    echo 'Set PLATFORM_TESTKIT_CLI to a prepared ic-testkit-server 0.106.0 executable.' >&2
    exit 1
  fi
fi
PLATFORM_SERVER_DIRECTORY="${PLATFORM_SERVER_DIRECTORY:-$repo/../ic-scheduling/.tools/ic-testkit-server}"
PLATFORM_SERVER_DIRECTORY="$(cd "$PLATFORM_SERVER_DIRECTORY" && pwd -P)"
[[ "$("$PLATFORM_TESTKIT_CLI" --version)" == 'ic-testkit-server 0.106.0' ]]
server="$("$PLATFORM_TESTKIT_CLI" check --directory "$PLATFORM_SERVER_DIRECTORY")"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/platform-consumer.XXXXXXXX")"
trap 'printf "Platform qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
sha256sum "$PLATFORM_TESTKIT_CLI" "$server" > "$scratch/tools.sha256"
if [[ "$mode" == raw-rand ]]; then
  sha256sum "$(command -v ic-wasm)" >> "$scratch/tools.sha256"
fi
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/platform/$mode-session.patch"
git -C "$scratch/source" apply "$repo/qualification/platform/$mode-session.patch"
# The consumer's existing fixture admits these prepared host tools read-only.
mkdir -p "$scratch/source/.tools"
ln -s "$source_checkout/.tools/host" "$scratch/source/.tools/host"
export CARGO_TARGET_DIR="$scratch/build"
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0
export PLATFORM_QUALIFICATION_EXPORT="$scratch"
env -u IC_TESTKIT_POCKET_IC_URL -u POCKET_IC_BIN "$PLATFORM_TESTKIT_CLI" run \
  --directory "$PLATFORM_SERVER_DIRECTORY" --ttl 900 \
  --server-stdout "$scratch/server.stdout" --server-stderr "$scratch/server.stderr" \
  -- bash "$repo/scripts/qualification/platform-session.sh" "$mode"
sha256sum --check "$scratch/tools.sha256"
