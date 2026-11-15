.DEFAULT_GOAL := check

FORMAT_RUST_ARGS := --extra-workspace qualification
include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help check test-owned-build lint-owned-build doc-owned-build
help:
	@printf '%s\n' \
	  'Development (offline; prepare tools and wasm32-unknown-unknown first):' \
	  '  make check             Admit Cargo tooling selection and installed snapshot' \
	  '  make test-custody      Byte and publication/custody failure acceptance' \
	  '  make test-backup-custody Pinned real Backup filesystem migration' \
	  '  make test-owned-build  Stage 1 byte/custody/process/protocol and real Wasm acceptance' \
	  '  make lint-owned-build  Stage 1 library/tests with warnings denied' \
	  '  make doc-owned-build   Stage 1 public documentation with warnings denied' \
	  '  make fmt / fmt-check   Shared Rust/manifest formatting' \
	  '  make deps-list         Local package/dependency selections' \
	  '  make install-tools    Explicit common tool preparation' \
	  '  make tools-check      Prepared-tool admission'

check: toko-tooling-check

# Focused on the first workflow; not a complete product or release gate.
test-owned-build: toko-tooling-check
	cargo test -p ic-build --lib --locked --offline

lint-owned-build: toko-tooling-check
	cargo clippy -p ic-build --all-targets --locked --offline -- -D warnings

doc-owned-build: toko-tooling-check
	RUSTDOCFLAGS='-D warnings' cargo doc -p ic-build --no-deps --locked --offline

.PHONY: test-custody test-backup-custody
test-custody: toko-tooling-check
	cargo test -p ic-host-artifacts -p ic-host-fs --lib --locked --offline

test-backup-custody: toko-tooling-check
	bash scripts/qualification/backup-consumer.sh
