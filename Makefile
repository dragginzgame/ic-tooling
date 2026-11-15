.DEFAULT_GOAL := check

FORMAT_RUST_ARGS := --extra-workspace qualification
include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help check test-owned-build lint-owned-build doc-owned-build
help:
	@printf '%s\n' \
	  'Development (offline; prepare tools and wasm32-unknown-unknown first):' \
	  '  make check             Admit Cargo tooling selection and installed snapshot' \
	  '  make test-testkit-catalog Reply catalog and generated module admission' \
	  '  make test-testkit-module Installed module test (externally owned server)' \
	  '  make test-testkit-control Installed controller refusals (externally owned server)' \
	  '  make test-text         Terminal escaping and Unicode table contracts' \
	  '  make test-canic-build-metadata Current Canic metadata admission/refusal' \
	  '  make test-canic-app-build Combined Canic Build/Text app and display contracts' \
	  '  make test-canic-build  Current Canic owned Wasm build and publication refusal' \
	  '  make test-canic-text   Current Canic CLI presentation qualification' \
	  '  make test-query-text   Current Query presentation and output qualification' \
	  '  make test-http-body    Bounded HTTP body fixtures (loopback required)' \
	  '  make test-backup-http  Current Backup HTTP policy and body qualification' \
	  '  make test-custody      Byte and publication/custody failure acceptance' \
	  '  make test-backup-custody Local Backup filesystem and metadata custody (loopback required)' \
	  '  make test-scheduling-held Focused rewrite held-reply baseline (prepared server)' \
	  '  make test-scheduling-consumer Scheduling rewrite source/graph admission and driver checks' \
	  '  make test-platform-agent Current Platform Agent ingress and identity boundary' \
	  '  make test-platform-consumer Current Platform raw_rand installed behavior' \
	  '  make test-db-text       Current DB installed console with new Text' \
	  '  make test-db-consumer   Current DB rollback and external-server ownership' \
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

.PHONY: test-db-consumer
test-db-consumer: toko-tooling-check
	bash scripts/qualification/db-consumer.sh


.PHONY: test-platform-consumer
test-platform-consumer: toko-tooling-check
	bash scripts/qualification/platform-consumer.sh

.PHONY: test-platform-agent
test-platform-agent: toko-tooling-check
	bash scripts/qualification/platform-consumer.sh agent

.PHONY: test-text test-query-text
test-text: toko-tooling-check
	cargo test -p ic-host-text --lib --locked --offline

test-query-text: toko-tooling-check
	bash scripts/qualification/query-consumer.sh

.PHONY: test-db-text
test-db-text: toko-tooling-check
	bash scripts/qualification/db-consumer.sh text

.PHONY: test-canic-text
test-canic-text: toko-tooling-check
	bash scripts/qualification/canic-text.sh

.PHONY: test-canic-build
test-canic-build: toko-tooling-check
	bash scripts/qualification/canic-build.sh

.PHONY: test-canic-app-build
test-canic-app-build: toko-tooling-check
	bash scripts/qualification/canic-build.sh app

.PHONY: test-canic-build-metadata
test-canic-build-metadata: toko-tooling-check
	bash scripts/qualification/canic-build.sh metadata

.PHONY: test-scheduling-held
test-scheduling-held: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh held

.PHONY: test-testkit-catalog
test-testkit-catalog: toko-tooling-check
	cargo test -p ic-testkit --features held-reply-fixtures --lib --locked --offline

.PHONY: test-testkit-module
test-testkit-module: toko-tooling-check
	@test -n "$${IC_TESTKIT_POCKET_IC_URL:-}" || { echo 'IC_TESTKIT_POCKET_IC_URL must select an externally owned server' >&2; exit 1; }
	cargo test -p ic-testkit --features held-reply-fixtures --lib --locked --offline held::tests::installed_modules_keep_replies_bound_to_slots_and_reject_failures -- --ignored --exact

.PHONY: test-testkit-control
test-testkit-control: toko-tooling-check
	@test -n "$${IC_TESTKIT_POCKET_IC_URL:-}" || { echo 'IC_TESTKIT_POCKET_IC_URL must select an externally owned server' >&2; exit 1; }
	cargo test -p ic-testkit --features held-reply-fixtures --lib --locked --offline held::control::tests::installed_control_refuses_foreign_consumed_expired_and_automatic_releases -- --ignored --exact
