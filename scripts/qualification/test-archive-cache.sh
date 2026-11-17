#!/usr/bin/env bash
# Reuse completed real archive gates; mutate only separate disposable candidates.
set -euo pipefail
repo="$(CDPATH='' cd -- "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
[[ $# == 2 ]] || { echo 'Usage: test-archive-cache.sh BUILD_EVIDENCE TESTKIT_EVIDENCE' >&2; exit 2; }
mkdir -p "$repo/target/evidence/archive-cache"
scratch="$(mktemp -d "$repo/target/evidence/archive-cache/invalidation.XXXXXXXX")"
trap 'printf "Archive cache invalidation evidence: %s\n" "$scratch"' EXIT
source "$repo/scripts/qualification/archive-compilation.sh"
for kind in build library; do
  evidence="$1"
  shift
  [[ -f "$evidence/complete.txt" && -f "$evidence/compilation-target.txt" ]]
  CARGO_TARGET_DIR="$(cat "$evidence/compilation-target.txt")"
  export CARGO_TARGET_DIR
  [[ "$CARGO_TARGET_DIR" == "$repo/target/archive-qualification/$kind" ]]
  export CARGO_BUILD_BUILD_DIR="$CARGO_TARGET_DIR"
  features=()
  if [[ "$kind" == build ]]; then
    package=ic-host-artifacts
    module=src/producer/mod.rs
    test=producer::tests::completed_segments_match_fixed_sha256_vectors
  else
    package=ic-testkit
    module=src/held/mod.rs
    test=held::tests::owns_ordered_duplicate_slots_and_opaque_page_spanning_bytes
    features=(--features 'held-reply-fixtures,cli')
  fi
  for phase in changed recovered; do
    candidate="$scratch/$kind-$phase"
    cp -R "$evidence/candidate" "$candidate"
    source_file="$candidate/crates/$package/$module"
    if [[ "$phase" == changed ]]; then
      # Change one existing initializer, not generated Rust or a test oracle.
      if [[ "$kind" == build ]]; then
        perl -0pi -e 's/bytes: 0,/bytes: 1,/ == 1 or die "producer initializer changed\n"' "$source_file"
      else
        perl -0pi -e 's/let mut bytes = 0;/let mut bytes = 1;/ == 1 or die "catalog initializer changed\n"' "$source_file"
      fi
    fi
    # Cargo archives carry old mtimes. A new candidate with different bytes must
    # not inherit a passing executable merely because its timestamps are older.
    find "$candidate" -type f -exec touch -t 200001010000 {} +
    prepare_archive_compilation "$repo" "$candidate" "$scratch" "$kind"
    status=0
    cargo test --manifest-path "$candidate/Cargo.toml" -p "$package" --lib \
      "${features[@]}" --locked --offline --message-format=json "$test" \
      -- --exact --color never > "$scratch/$kind-$phase.log" 2> "$scratch/$kind-$phase.stderr" || status=$?
    if [[ "$phase" == changed ]]; then
      [[ $status == 101 ]]
      grep -Fq "test $test ... FAILED" "$scratch/$kind-$phase.log"
      jq -Rse --arg target "${package//-/_}" \
        '[split("\n")[] | fromjson? | select(.reason == "compiler-artifact" and .target.name == $target and .profile.test)] |
         length == 1 and .[0].fresh == false' "$scratch/$kind-$phase.log" >/dev/null
    else
      [[ $status == 0 ]]
      grep -Fq 'test result: ok. 1 passed; 0 failed' "$scratch/$kind-$phase.log"
    fi
    diff -qr "$evidence/candidate/crates/$package/src" "$candidate/crates/$package/src" \
      > "$scratch/$kind-$phase-source.diff" || [[ "$phase" == changed ]]
  done
done
printf 'Both warm caches reject changed, backdated source and accept fresh recovery candidates.\n'
