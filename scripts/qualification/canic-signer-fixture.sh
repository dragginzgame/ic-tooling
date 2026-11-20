#!/usr/bin/env bash
# Shared fixture wiring for already admitted Canic exports, never sibling sources.
prepare_signer_fixture() {
  local source="$1" evidence="$2" mode
  mkdir -p "$evidence"
  cp "$repo/qualification/canic/signer-tests.rs" "$source/crates/canic-host/src/signed/signer_process_tests.rs"
  printf '\n#[cfg(test)]\nmod signer_process_tests;\n' >> "$source/crates/canic-host/src/signed/mod.rs"
  [[ ! -e "$source/crates/canic-host/src/signed/process" && ! -L "$source/crates/canic-host/src/signed/process" ]] || return
  mkdir "$source/crates/canic-host/src/signed/process"
  mv "$source/crates/canic-host/src/signed/process.rs" "$source/crates/canic-host/src/signed/process/mod.rs" || return
  cp "$repo/qualification/canic/signer-capture-tests.rs" "$source/crates/canic-host/src/signed/process/signer_capture_tests.rs"
  printf '\n#[cfg(test)]\nmod signer_capture_tests;\n' >> "$source/crates/canic-host/src/signed/process/mod.rs"
  for mode in bad-version valid invalid utf8 nonzero stdout-over stderr-over literal; do
    cp "$repo/qualification/canic/signer-tool.sh" "$evidence/$mode"
    chmod 700 "$evidence/$mode"
  done
}

test_signer_fixture() {
  local source="$1" evidence="$2" scope
  for scope in signer_process_tests:: signer_capture_tests::; do
    CANIC_SIGNER_EVIDENCE="$evidence" cargo test --manifest-path "$source/Cargo.toml" -p canic-host --lib --locked "$scope" || return
  done
}
