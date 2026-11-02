#!/usr/bin/env bash
set -euo pipefail
# Product-owned source/configuration closure for metadata-only release fixtures.
[[ $# == 2 && $1 == /* && -d $2 && ! -L $2 ]] || exit 2
root=$1 destination=$2
cp -R "$root/ci" "$root/make" "$root/scripts" "$destination/"
cp "$root/Makefile" "$root/rust-toolchain.toml" "$root/.shared-tooling.snapshot" "$root/.gitignore" "$destination/"
