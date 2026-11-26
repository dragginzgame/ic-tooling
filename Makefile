.DEFAULT_GOAL := check

FORMAT_RUST_ARGS := --extra-workspace qualification
include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help check test-owned-build lint-owned-build doc-owned-build
help:
	@printf '%s\n' \
	  'Development (prepare tools and wasm32-unknown-unknown first; CARGO_NET_OFFLINE=true opts out of retrieval):' \
	  '  make check             Admit Cargo tooling selection and installed snapshot' \
	  '  make test-testkit-catalog Reply catalog and generated module admission' \
	  '  make test-process      Owned execution and affected Build failure checks' \
	  '  make test-process-terminal Linux foreground PTY fixture' \
	  '  make test-sign         Signer admission and cancellation tests (OpenSSL required)' \
	  '  make test-sign-keyring Real disposable keyring qualification (SIGNER_TEST_ICP required)' \
	  '  make test-canic-process Current Canic process capture, viewer and build checks' \
	  '  make test-testkit-module Installed module test (externally owned server)' \
	  '  make test-testkit-control Installed controller/clock refusals (externally owned server)' \
	  '  make test-testkit-server Offline assets and bounded startup refusal checks' \
	  '  make test-testkit-server-installed Prepared-archive lifecycle/gateway/held checks (explicit archive and gzip)' \
	  '  make prepare-testkit-server-cli Build the product server CLI (cached dependencies)' \
	  '  make test-pocketic-setup Failed-download diagnostics (TESTKIT_SERVER_CLI; no network)' \
	  '  make test-library-release-candidate RELEASE_GROUPS="testkit sign" Archive readiness (prepared server/ICP tools required)' \
	  '  make test-text         Terminal escaping and Unicode table contracts' \
	  '  make test-cargo-metadata Renderer lint and document/command boundary tests' \
	  '  make test-cargo-metadata-consumer Current Toko/Scheduling release transactions (prepared editor)' \
	  '  make test-canic-build-metadata Current Canic metadata admission/refusal' \
	  '  make test-canic-app-build Current Canic TOML recipes, lock setup and artifact outputs' \
	  '  make test-canic-build  Current Canic owned Wasm build and publication refusal' \
	  '  make test-canic-publication Canic artifact inspection and real CLI publication' \
	  '  make test-canic-config-publication Qualify Canic config writers' \
	  '  make test-canic-record-publication Qualify retained-record and recovery writers' \
	  '  make test-canic-starter-publication Qualify starter directory publication' \
	  '  make test-canic-directory-publication Qualify starter/fleet directory publication' \
	  '  make test-canic-filesystem Qualify retained reads and publication together' \
	  '  make test-canic-sidecar-lock Qualify Store/config locks and filesystem callers' \
	  '  make test-canic-parent-preparation Qualify held preparation, staging and recovery' \
	  '  make test-canic-runtime Qualify Host Foundation callers, CLI publication and viewer tests' \
	  '  make test-canic-foreground Harmless foreground client/PTY migration checks' \
	  '  make test-canic-text   Current Canic CLI presentation qualification' \
	  '  make test-query-text   Current Query presentation and output qualification' \
	  '  make test-http-body    Bounded HTTP body fixtures (loopback required)' \
	  '  make test-backup-http  Current Backup HTTP policy and body qualification' \
	  '  make test-custody      Byte and publication/custody failure acceptance' \
	  '  make test-backup-commitments Compare completed Backup byte commitments' \
	  '  make test-backup-assembly Compare current Backup assembly commitments' \
	  '  make test-backup-custody Local Backup filesystem and metadata custody (loopback required)' \
	  '  make test-scheduling-held Focused rewrite held-reply baseline (prepared server)' \
	  '  make test-scheduling-replacement Rewrite held replies with replacement Testkit' \
	  '  make test-scheduling-pressure Watchdog/Jobs held pressure with replacement Testkit' \
	  '  make test-scheduling-pressure-baseline Same selection with released Testkit' \
	  '  make test-scheduling-lifecycle Compare watchdog/lifecycle on one source capture' \
	  '  make test-scheduling-executor Compare default Testkit executor/trap behavior' \
	  '  make test-scheduling-retention Compare retained callbacks and dropped handles' \
	  '  make test-scheduling-jobs-lifecycle Compare Jobs calendar, upgrade and downtime' \
	  '  make test-scheduling-jobs-settlement Compare Jobs retry, reconciliation and completion' \
	  '  make test-scheduling-uncertainty Compare lifetime custody and unresolved upgrades' \
	  '  make test-scheduling-jobs-boundaries Compare initial deadlines and bounded catch-up' \
	  '  make test-scheduling-completion-refusal Compare completion, diagnostics and refusal' \
	  '  make test-scheduling-aggregate Compare the complete Once installed roster' \
	  '  make test-scheduling-metrics Compare paired metrics feature behavior and observations' \
	  '  make test-scheduling-consumer Scheduling rewrite source/graph admission and driver checks' \
	  '  make test-canic-consumer Local Canic retained-record admission' \
	  '  make test-auth-consumer Historical Auth recipe (on hold)' \
	  '  make test-owned-build  Stage 1 byte/custody/process/protocol and real Wasm acceptance' \
	  '  make test-build-release-candidate Package four release crates and qualify archives/current consumer' \
	  '  make lint-owned-build  Stage 1 library/tests with warnings denied' \
	  '  make doc-owned-build   Stage 1 public documentation with warnings denied' \
	  '  make fmt / fmt-check   Shared Rust/manifest formatting' \
	  '  make deps-list         Product package versions and release groups' \
	  '  make deps-list DEPENDENCY_MANIFEST=qualification/Cargo.toml  Include qualification packages' \
	  '  make install-tools    Explicit common tool preparation' \
	  '  make tools-check      Prepared-tool admission'

check: toko-tooling-check

.PHONY: test-sign test-sign-keyring
test-sign:
	cargo clippy -p ic-host-sign --all-targets --all-features --locked -- -D warnings
	cargo test -p ic-host-sign --lib --locked

test-sign-keyring:
	bash scripts/qualification/sign-keyring.sh

.PHONY: test-cargo-metadata test-cargo-metadata-consumer
test-cargo-metadata:
	cargo clippy -p ic-cargo-metadata --all-targets --locked -- -D warnings
	cargo test -p ic-cargo-metadata --locked

test-cargo-metadata-consumer:
	bash scripts/qualification/cargo-metadata-consumer.sh

.PHONY: test-pocketic-setup
test-pocketic-setup:
	bash scripts/qualification/test-pocketic-setup.sh

.PHONY: prepare-testkit-server-cli test-testkit-server-cli
prepare-testkit-server-cli:
	cargo build -p ic-testkit --features cli --bin ic-testkit-server --locked

test-testkit-server-cli:
	bash scripts/qualification/test-testkit-server-cli.sh

.PHONY: test-build-release-candidate
test-build-release-candidate: toko-tooling-check
	bash scripts/qualification/build-release-candidate.sh

.PHONY: test-library-release-candidate
# Explicit group scope; Testkit additionally requires prepared launcher/server paths.
test-library-release-candidate: toko-tooling-check
	bash scripts/qualification/library-release-candidate.sh $(RELEASE_GROUPS)

.PHONY: test-canic-signer
test-canic-signer: toko-tooling-check
	bash scripts/qualification/canic-signer.sh

.PHONY: test-canic-process
test-canic-process: toko-tooling-check
	bash scripts/qualification/canic-build.sh process

.PHONY: test-canic-viewer
test-canic-viewer: toko-tooling-check
	bash scripts/qualification/canic-viewer.sh

.PHONY: test-canic-viewer-replacement
test-canic-viewer-replacement: toko-tooling-check
	bash scripts/qualification/canic-viewer.sh replacement

.PHONY: test-canic-lock
test-canic-lock: toko-tooling-check
	bash scripts/qualification/canic-lock.sh

# Focused on the first workflow; not a complete product or release gate.
test-owned-build: toko-tooling-check
	cargo test -p ic-build --lib --locked

lint-owned-build: toko-tooling-check
	cargo clippy -p ic-build --all-targets --locked -- -D warnings

doc-owned-build: toko-tooling-check
	RUSTDOCFLAGS='-D warnings' cargo doc -p ic-build --no-deps --locked

.PHONY: test-custody test-backup-custody
.PHONY: test-backup-commitments
test-backup-commitments: toko-tooling-check
	bash scripts/qualification/backup-commitments.sh

.PHONY: test-backup-assembly
test-backup-assembly: toko-tooling-check
	bash scripts/qualification/backup-commitments.sh assembly

test-custody: toko-tooling-check
	cargo test -p ic-host-artifacts -p ic-host-fs --lib --locked

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
	cargo test -p ic-host-http --lib --locked

test-backup-http: toko-tooling-check
	bash scripts/qualification/backup-consumer.sh http

.PHONY: test-text test-query-text
test-text: toko-tooling-check
	cargo test -p ic-host-text --lib --locked

test-query-text: toko-tooling-check
	bash scripts/qualification/query-consumer.sh

.PHONY: test-canic-text
test-canic-text: toko-tooling-check
	bash scripts/qualification/canic-text.sh

.PHONY: test-canic-build
test-canic-build: toko-tooling-check
	bash scripts/qualification/canic-build.sh

.PHONY: test-canic-publication
test-canic-publication: toko-tooling-check
	bash scripts/qualification/canic-build.sh custody

.PHONY: test-canic-config-publication
test-canic-config-publication: toko-tooling-check
	bash scripts/qualification/canic-build.sh config

.PHONY: test-canic-record-publication
test-canic-record-publication: toko-tooling-check
	bash scripts/qualification/canic-build.sh records

.PHONY: test-canic-starter-publication
test-canic-starter-publication: toko-tooling-check
	bash scripts/qualification/canic-build.sh starter

.PHONY: test-canic-directory-publication
test-canic-directory-publication: toko-tooling-check
	bash scripts/qualification/canic-build.sh trees

.PHONY: test-canic-filesystem
test-canic-filesystem: toko-tooling-check
	bash scripts/qualification/canic-build.sh filesystem

.PHONY: test-canic-sidecar-lock
test-canic-sidecar-lock: toko-tooling-check
	bash scripts/qualification/canic-build.sh locks

.PHONY: test-canic-parent-preparation
test-canic-parent-preparation: toko-tooling-check
	bash scripts/qualification/canic-build.sh parents

.PHONY: test-canic-runtime
test-canic-runtime: toko-tooling-check
	bash scripts/qualification/canic-build.sh runtime

.PHONY: test-canic-app-build
test-canic-app-build: toko-tooling-check
	bash scripts/qualification/canic-build.sh app

.PHONY: test-canic-build-metadata
test-canic-build-metadata: toko-tooling-check
	bash scripts/qualification/canic-build.sh metadata

.PHONY: test-scheduling-held
test-scheduling-held: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh held

.PHONY: test-scheduling-replacement
test-scheduling-replacement: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh replacement

.PHONY: test-scheduling-pressure test-scheduling-pressure-baseline
test-scheduling-pressure: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh replacement held-pressure

test-scheduling-pressure-baseline: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh held held-pressure

.PHONY: test-scheduling-lifecycle
test-scheduling-lifecycle: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison lifecycle

.PHONY: test-scheduling-executor
test-scheduling-executor: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison executor

.PHONY: test-scheduling-retention
test-scheduling-retention: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison retention

.PHONY: test-scheduling-jobs-lifecycle
test-scheduling-jobs-lifecycle: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison jobs-lifecycle

.PHONY: test-scheduling-jobs-settlement
test-scheduling-jobs-settlement: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison jobs-settlement

.PHONY: test-scheduling-uncertainty
test-scheduling-uncertainty: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison uncertainty

.PHONY: test-scheduling-jobs-boundaries
test-scheduling-jobs-boundaries: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison jobs-boundaries

.PHONY: test-scheduling-completion-refusal
test-scheduling-completion-refusal: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison completion-refusal

.PHONY: test-scheduling-aggregate
test-scheduling-aggregate: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison aggregate

.PHONY: test-scheduling-metrics
test-scheduling-metrics: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh comparison metrics-cost

.PHONY: test-testkit-catalog
.PHONY: test-process test-process-terminal
.PHONY: test-canic-foreground
test-canic-foreground: toko-tooling-check
	bash scripts/qualification/canic-foreground.sh

test-process: toko-tooling-check
	cargo test -p ic-host-process -p ic-build --lib --locked

test-process-terminal: toko-tooling-check
	@test "$$(uname -s)" = Linux || { echo 'This PTY recipe uses Linux script; native macOS qualification remains separate.' >&2; exit 1; }
	printf 'fixture-answer\n' | script -q -e -c 'cargo test -p ic-host-process --lib --locked tests::foreground_terminal_input_survives_direct_child_ownership -- --ignored --exact' /dev/null

test-testkit-catalog: toko-tooling-check
	cargo test -p ic-testkit --features held-reply-fixtures --lib --locked

.PHONY: test-testkit-module
test-testkit-module: toko-tooling-check
	@test -n "$${IC_TESTKIT_POCKET_IC_URL:-}" || { echo 'IC_TESTKIT_POCKET_IC_URL must select an externally owned server' >&2; exit 1; }
	cargo test -p ic-testkit --features held-reply-fixtures --lib --locked held::tests::installed_modules_keep_replies_bound_to_slots_and_reject_failures -- --ignored --exact

.PHONY: test-testkit-control
test-testkit-control: toko-tooling-check
	@test -n "$${IC_TESTKIT_POCKET_IC_URL:-}" || { echo 'IC_TESTKIT_POCKET_IC_URL must select an externally owned server' >&2; exit 1; }
	cargo test -p ic-testkit --features held-reply-fixtures --lib --locked held::control::tests::installed_control_refuses_foreign_consumed_expired_and_automatic_releases -- --ignored --exact
	cargo test -p ic-testkit --features held-reply-fixtures --lib --locked held::control::tests::clock::installed_clock_progression_preserves_live_replies_and_original_expiry -- --ignored --exact

.PHONY: test-testkit-server test-testkit-server-installed
test-testkit-server: toko-tooling-check
	cargo test -p ic-testkit --features managed-server --lib --locked server::tests

test-testkit-server-installed: toko-tooling-check
	@test -n "$${TESTKIT_SERVER_ARCHIVE:-}" -a -n "$${TESTKIT_GZIP:-}" || { echo 'Select TESTKIT_SERVER_ARCHIVE and absolute TESTKIT_GZIP explicitly' >&2; exit 1; }
	cargo test -p ic-testkit --features managed-server,held-reply-fixtures --lib --locked server::tests::installed_ -- --ignored
