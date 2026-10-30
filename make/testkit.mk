# Testkit owns simulator setup and native/Wasm qualification.
SERVER_CLI := $(CURDIR)/target/debug/ic-testkit-server
.PHONY: build-server-cli install-server server-check testkit-graph-check testkit-wasm-check testkit-msrv testkit-check testkit-browser-check testkit-package-check testkit-installed-check
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
	+cargo check -p ic-testkit -p ic-testkit-native --features ic-testkit/held-reply-fixtures,ic-testkit-native/stable-memory --locked --offline --target wasm32-unknown-unknown
	+cargo check -p ic-testkit-perf-probe --locked --offline --target wasm32-unknown-unknown
testkit-msrv:
	+RUSTUP_TOOLCHAIN="$(MSRV)" $(MAKE) --no-print-directory testkit-wasm-check
testkit-browser-check:
	node --test testing/browser/session.test.mjs
testkit-check: build-server-cli
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test -p ic-testkit -p ic-testkit-dev -p ic-testkit-native --all-features --locked --offline
testkit-installed-check: build-server-cli
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test -p ic-testkit --features cli --locked --offline --test server_runner real_server_runs_a_separate_process_using_environment_startup -- --ignored --exact
testkit-package-check: build-server-cli
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo run -p ic-testkit-dev --bin check_packages --locked --offline
testkit-graph-check:
	bash scripts/ci/check-testkit-graphs.sh
