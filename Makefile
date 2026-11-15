.DEFAULT_GOAL := check

FORMAT_RUST_ARGS := --extra-workspace qualification
include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help check test-owned-build lint-owned-build doc-owned-build
help:
	@printf '%s\n' \
	  'Development (offline; prepare tools and wasm32-unknown-unknown first):' \
	  '  make check             Admit Cargo tooling selection and installed snapshot' \
	  '  make test-http-body    Bounded HTTP body fixtures (loopback required)' \
	  '  make test-backup-http  Current Backup HTTP policy and body qualification' \
	  '  make test-custody      Byte and publication/custody failure acceptance' \
	  '  make test-backup-custody Local Backup filesystem and metadata custody (loopback required)' \
	  '  make test-scheduling-consumer Local Scheduling owned build and installed tests' \
	  '  make test-canic-consumer Local Canic retained-record admission' \
	  '  make test-auth-consumer Historical Auth recipe (on hold)' \
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

.PHONY: test-scheduling-consumer
test-scheduling-consumer: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh

.PHONY: test-auth-consumer
test-auth-consumer: toko-tooling-check
	bash scripts/qualification/auth-consumer.sh

.PHONY: test-canic-consumer
test-canic-consumer: toko-tooling-check
	bash scripts/qualification/canic-consumer.sh

.PHONY: test-http-body test-backup-http
test-http-body: toko-tooling-check
	cargo test -p ic-host-http --lib --locked --offline

test-backup-http: toko-tooling-check
	bash scripts/qualification/backup-consumer.sh http
