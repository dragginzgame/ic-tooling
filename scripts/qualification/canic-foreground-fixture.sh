#!/usr/bin/env bash
# Shared foreground fixture wiring for admitted exports; Linux PTY only.
prepare_foreground_fixture() {
  local source="$1" evidence="$2"
  mkdir -p "$evidence"
  cp "$repo/qualification/canic/foreground-tests.rs" "$source/apps/profile/canic-profile-client/src/foreground_process.rs"
  printf '\n#[cfg(test)]\nmod foreground_process;\n' >> "$source/apps/profile/canic-profile-client/src/main.rs"
  cp "$repo/qualification/canic/foreground-tool.sh" "$evidence/tool"
  chmod 700 "$evidence/tool"
}

test_foreground_fixture() (
  local source="$1" evidence="$2"
  [[ "$(uname -s)" == Linux ]] || { echo 'Foreground PTY qualification requires Linux script; macOS remains separate.' >&2; exit 1; }
  export CANIC_FOREGROUND_EVIDENCE="$evidence" CANIC_FOREGROUND_TOOL="$evidence/tool"
  export CANIC_FOREGROUND_MANIFEST="$source/Cargo.toml"
  cargo clippy --manifest-path "$CANIC_FOREGROUND_MANIFEST" -p canic-profile-client --all-targets --locked --offline -- -D warnings
  cargo test --manifest-path "$CANIC_FOREGROUND_MANIFEST" -p canic-profile-client --bin canic-profile --locked --offline foreground_process::
  printf 'fixture-answer\n' | script -q -e -c 'cargo test --manifest-path "$CANIC_FOREGROUND_MANIFEST" -p canic-profile-client --bin canic-profile --locked --offline foreground_process::inherited_terminal_input_and_stderr_remain_interactive -- --ignored --exact' /dev/null
)
