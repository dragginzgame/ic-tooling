#!/usr/bin/env bash
# Archive runners own fresh candidates/evidence; Cargo owns disposable artifacts.
prepare_archive_compilation() {
  local repo="$1" candidate="$2" evidence="$3" kind="$4"
  case "$kind" in build|library) ;; *) return 2 ;; esac
  case "${ARCHIVE_QUALIFICATION_CACHE:-true}" in
    true) CARGO_TARGET_DIR="$repo/target/archive-qualification/$kind" ;;
    false) CARGO_TARGET_DIR="$evidence/compiled" ;;
    *) echo 'ARCHIVE_QUALIFICATION_CACHE must be true or false' >&2; return 2 ;;
  esac
  # Archive mtimes predate warm artifacts. Cargo can otherwise report changed
  # path packages as fresh even in a new candidate directory. Change timestamps
  # only; always recheck candidate source while retaining dependency artifacts.
  find "$candidate" -type f -exec touch {} + || return
  export CARGO_TARGET_DIR
  # Keep Cargo 1.100's intermediate build directory in the selected cache too.
  export CARGO_BUILD_BUILD_DIR="$CARGO_TARGET_DIR"
  printf '%s\n' "$CARGO_TARGET_DIR" > "$evidence/compilation-target.txt"
}
