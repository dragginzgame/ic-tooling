#!/usr/bin/env bash
# Qualify current local sibling inputs without modifying the sibling.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
source_checkout="${CANIC_SOURCE:-$repo/../canic}"
export PATH="$repo/.tools/host/bin:$PATH"
mkdir -p "$repo/target"
scratch="$(mktemp -d "$repo/target/canic-consumer.XXXXXXXX")"
trap 'printf "Canic qualification export: %s\n" "$scratch"' EXIT
mkdir "$scratch/source"
source "$repo/scripts/qualification/local-source.sh"
export_local_source
[[ ! -L "$scratch/source/crates/canic-host/src/store" ]]
[[ ! -e "$scratch/source/crates/canic-host/src/store/mod.rs" && ! -L "$scratch/source/crates/canic-host/src/store/mod.rs" ]]
mkdir -p "$scratch/source/crates/canic-host/src/store"
mv "$scratch/source/crates/canic-host/src/store.rs" "$scratch/source/crates/canic-host/src/store/mod.rs"
export GIT_CEILING_DIRECTORIES="$repo/target"
git -C "$scratch/source" apply --check "$repo/qualification/canic/record-admission.patch"
git -C "$scratch/source" apply "$repo/qualification/canic/record-admission.patch"
export CARGO_TARGET_DIR="$repo/target/canic-consumer-build"
export CARGO_NET_OFFLINE=true
export RUSTUP_AUTO_INSTALL=0
CDPATH='' cd -- "$scratch/source"
cargo metadata --format-version 1 --locked --offline > "$scratch/metadata.json"
jq -e --arg root "$repo" '
  . as $m |
  ($m.packages[] | select(.name == "canic-host") | .id) as $host |
  ($m.resolve.nodes[] | select(.id == $host)) as $node |
  all([{"alias":"ic_custody", "package":"ic-host-fs"},
       {"alias":"ic_bytes", "package":"ic-host-artifacts"}][];
    . as $expected |
    [$m.packages[] | select(.name == $expected.package and .version == "0.80.1")] as $matches |
    ($matches | length) == 1 and
    ($matches[0] as $package | $package.manifest_path == ($root + "/crates/" + $expected.package + "/Cargo.toml") and
    any($node.deps[]; .name == $expected.alias and .pkg == $package.id)))
' "$scratch/metadata.json" >/dev/null
cargo test -p canic-host --lib store::read_tests --locked --offline -- --list > "$scratch/read-tests.txt"
rg -q "^store::read_tests::exact_bytes_limits_and_original_missing_cause: test$" "$scratch/read-tests.txt"
rg -q "^store::read_tests::final_links_refuse_and_ancestor_links_remain_supported: test$" "$scratch/read-tests.txt"
cargo test -p canic-host --lib store::read_tests --locked --offline
for contract in \
  tests::plan_identity_admission_and_artifact_custody \
  tests::retained_evidence_never_overwrites_and_store_excludes_concurrent_writer \
  signed::tests::recovery_retains_exact_attempt_and_deadline_after_effect_expiry \
  signed::tests::expired_uncertain_attempt_stays_retained_without_a_new_signature; do
  cargo test -p canic-host --lib "$contract" --locked --offline -- --exact
done
