# Testkit owns simulator setup and native/Wasm qualification.
SERVER_CLI := $(CURDIR)/target/debug/ic-testkit-server
.PHONY: build-server-cli install-server server-check testkit-graph-check testkit-wasm-check testkit-check testkit-browser-check testkit-package-check testkit-installed-check
build-server-cli:
	+cargo build -p ic-testkit --features cli --locked --offline --bin ic-testkit-server
install-server:
	+cargo fetch --locked
	+$(MAKE) --no-print-directory build-server-cli
	"$(SERVER_CLI)" setup
server-check:
	@test -x "$(SERVER_CLI)" || { echo 'Run make install-server during explicit setup.' >&2; exit 1; }
	"$(SERVER_CLI)" check
testkit-wasm-check:
	+cargo check -p ic-testkit --features ic-testkit/held-reply-fixtures --locked --offline --target wasm32-unknown-unknown
	+cargo check -p ic-testkit-perf-probe --locked --offline --target wasm32-unknown-unknown
testkit-browser-check:
	node --test testing/browser/session.test.mjs testing/browser/internet-identity/inputs.test.mjs testing/browser/tests/scratch.test.mjs
testkit-check: build-server-cli
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test -p ic-testkit -p ic-testkit-dev --all-features --locked --offline
testkit-installed-check: build-server-cli
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test -p ic-testkit --locked --offline --test client_installed -- --ignored
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test -p ic-testkit --features cli --locked --offline --test server_runner real_server_runs_a_separate_process_using_environment_startup -- --ignored --exact
testkit-package-check: build-server-cli
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo run -p ic-testkit-dev --bin check_packages --locked --offline
testkit-graph-check:
	bash scripts/ci/test-pocket-ic-source.sh
	bash scripts/ci/check-testkit-graphs.sh

# Current-source downstream qualification; never a supplier release prerequisite.
# Existing CI/release owners retain their full gates and native host matrix.
.PHONY: dependency-adoption-check
dependency-adoption-check: toko-tooling-check
	+$(MAKE) --no-print-directory build-server-cli
	@"$(SERVER_CLI)" check
	@printf 'Current workspace composition; not qualification of an older published client.\nConsumer HEAD: '
	@git rev-parse HEAD
	@if ! git diff --quiet HEAD --; then printf 'Consumer includes uncommitted changes.\n'; fi
	@printf 'Locked all-feature source inventory; isolated graphs are checked separately.\n'
	+@metadata="$$(cargo metadata --all-features --locked --offline --format-version 1)" && \
		printf '%s\n' "$$metadata" | jq -r '.packages[] | select(.name | test("^(ic-(testkit|host-tools|platform(-.*)?|wire|agent|management-canister-types|transport-types)|candid|ic_principal)$$")) | "\(.name) \(.version) source=\(.source // .manifest_path)"'
	+bash scripts/ci/check-tools-features.sh
	+bash scripts/ci/check-testkit-graphs.sh
	+$(MAKE) --no-print-directory test-tools-candid test-tools-agent test-tools-registry test-tools-ledger-evidence
	+cargo test -p ic-testkit --locked --offline --test client_management_identity
	+cargo test -p ic-testkit --locked --offline --lib pocket_ic::tests::client_failures::
	+cargo test -p ic-testkit --features wire --locked --offline --lib pic::wire::tests::
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test -p ic-testkit \
		--locked --offline --test client_installed installed_client_preserves_management_and_snapshot_state -- --ignored --exact
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test -p ic-testkit-dev \
		--locked --offline --test canister_wire
