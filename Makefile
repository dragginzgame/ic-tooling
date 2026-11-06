.DEFAULT_GOAL := help
# Shared companions: scripts/release/group.sh ci/cargo-release-groups.json scripts/ci/check-rust-toolchain.sh Cargo.toml rust-toolchain.toml make/release-validation-local.mk scripts/ci/workspace-validation.sh scripts/ci/run-tooling-checks.sh scripts/ci/check-product-candid-boundary.sh
PACKAGE ?= ic-host-artifacts
RELEASE_DELIVERY ?= direct
export RELEASE_GROUP
RELEASE_FROZEN_GROUPS ?= $(RELEASE_GROUP)
RELEASE_FROZEN_PUBLISH_TARGET ?= publish
RELEASE_FROZEN_SETUP_TARGET ?= release-candidate-setup
override RELEASE_DELIVERY_POLICY := $(CURDIR)/ci/release-delivery.txt
# Host commands use this checkout's reviewed snapshot, including parse-time probes.
override SHARED_TOOLING_ROOT := $(CURDIR)
export SHARED_TOOLING_ROOT
LOCAL_TOOL_INSTALL_TARGETS += install-release-editor install-server
LOCAL_TOOL_CHECK_TARGETS += release-editor-check server-check
include ci/tool-versions.env
IC_TOOLS := wasm-opt
include make/consumer.mk
include make/testkit.mk
include make/release-validation-local.mk
export YQ := $(CURDIR)/.tools/host/bin/yq

.PHONY: release-candidate-setup
# Shared selects prepare before fresh version lookup, check during recovery.
# The standard preflight still owns editor/server setup and the product gate.
release-candidate-setup:
ifeq ($(RELEASE_FROZEN_SETUP_MODE),prepare)
	+$(MAKE) --no-print-directory install-host-tools
else ifeq ($(RELEASE_FROZEN_SETUP_MODE),check)
	+$(MAKE) --no-print-directory host-tools-check
else
	@echo 'Select RELEASE_FROZEN_SETUP_MODE=prepare or check' >&2; exit 2
endif

# Refuse before the shared runner acquires its lock or any publication starts.
ifneq ($(filter $(SHARED_RELEASE_TARGETS) publish publish-check,$(MAKECMDGOALS)),)
_tooling_release_selection := $(RELEASE_GROUP)
_tooling_release_selection_mode :=
ifneq ($(filter release-patch release-minor release-major,$(MAKECMDGOALS)),)
ifneq ($(FROZEN_RELEASE_ACTIVE),1)
_tooling_release_selection := $(RELEASE_FROZEN_GROUPS)
_tooling_release_selection_mode := batch
endif
endif
ifneq ($(shell bash '$(subst ','"'"',$(CURDIR))/scripts/release/group.sh' '$(subst ','"'"',$(CURDIR))/ci/cargo-release-groups.json' '$(subst ','"'"',$(_tooling_release_selection))' $(_tooling_release_selection_mode) && printf admitted),admitted)
$(error Explicit RELEASE_GROUP=foundation, build, text, tools, testkit or native is required)
endif
endif

.PHONY: help check clippy docs-check test test-artifacts-minimal test-tools-default tools-features-check compiler-check ci install-hooks tooling-command-check shared-tooling-check dependency-pins-check check-doc-links release-adapter-check publish publish-check publish-command-check release-version release-preflight release-verify release-prepare-version release-prepared-check release-files release-commit-check release-committed-check release-tagged-check release-push-check
help:
	@echo 'Selected package: check, clippy, docs-check, test (PACKAGE=<crate>)'
	@echo 'Artifact profiles: test-artifacts-minimal, test-artifacts-wasm, test-artifacts-encoding, test-artifacts-text'
	@echo 'Filesystem profiles: test-fs-minimal, test-fs-capability, test-fs-temporary-file'
	@echo 'Unsupported-target compile contract: check-fs-non-unix (prepared Wasm target required)'
	@echo 'Tools profiles: test-tools-default, test-tools-interface, test-tools-candid, test-tools-combined, test-tools-agent, test-tools-encoding, test-tools-registry and tools-features-check'
	@echo 'Formatting and metadata: fmt, fmt-check, shared-tooling-check, dependency-pins-check, check-doc-links, release-adapter-check'
	@echo 'Hook qualification: formatting-hook-check (disposable Git fixtures; prepared formatters required)'
	@echo 'Common setup: install-tools (host and Cargo); install-hooks is separate'
	@echo 'Release editor setup/admission: install-release-editor, release-editor-check'
	@echo 'Offline setup checks: tools-check (complete set), host-tools-check, ic-tools-check, rust-tools-check'
	@echo 'Individual setup: install-host-tools, install-ic-tools, install-rust-tools'
	@echo 'Source reports: cloc (this workspace); fleet reports run in Shared Tooling'
	@echo 'Full release validation: ci (pinned compiler, workspace Clippy, native/Wasm and isolated features); compiler-check admits identities'
	@echo 'Native CI with publication dry run: ci-publication (one workspace lint gate, then offline checks)'
	@echo 'Frozen Git + crates.io releases: release-patch/minor/major RELEASE_GROUP=foundation|build|text|tools|testkit|native'
	@echo 'Batch selection: RELEASE_FROZEN_GROUPS=group,group; resume: release-batch-resume BATCH=batch-XXXXXXXX'
	@echo 'Shared batch custody: release-batch-status, release-batch-reconcile, release-batch-retire BATCH=batch-XXXXXXXX'
	@echo 'Legacy resume: release-resume RELEASE_GROUP=<same-group> VERSION=X.Y.Z'
	@echo 'crates.io publication: publish-check (dry run), publish (upload); offline fixture: publish-command-check'
	@echo 'Publication: select RELEASE_GROUP=foundation|build|text|tools|testkit|native; PACKAGE="crate ..." narrows that group'
.PHONY: formatting-hook-check
.PHONY: install-release-editor release-editor-check check-no-symlinks
install-release-editor:
	+@bash scripts/release/editor.sh install
release-editor-check:
	+@bash scripts/release/editor.sh check
check-no-symlinks:
	perl scripts/ci/check-no-symlinks.pl --consumer "$(CURDIR)"
formatting-hook-check:
	+bash scripts/ci/check-host-formatting.sh
check:
	+cargo check -p $(PACKAGE) --all-targets --all-features --locked --offline
clippy:
	+cargo clippy -p $(PACKAGE) --all-targets --all-features --locked --offline -- -D warnings
.PHONY: clippy-workspace
clippy-workspace:
	+cargo clippy --workspace --all-targets --all-features --locked --offline -- -D warnings
docs-check:
	+RUSTDOCFLAGS="-D warnings" cargo doc -p $(PACKAGE) --all-features --locked --offline --no-deps
test:
	+cargo test -p $(PACKAGE) --all-targets --all-features --locked --offline
test-artifacts-minimal:
	+cargo test -p ic-host-artifacts --no-default-features --lib --locked --offline
.PHONY: test-fs-minimal test-fs-capability test-fs-temporary-file
test-fs-minimal:
	+cargo test -p ic-host-fs --no-default-features --lib --locked --offline
test-fs-capability:
	+cargo test -p ic-host-fs --no-default-features --features capability --lib capability:: --locked --offline
test-fs-temporary-file:
	+cargo test -p ic-host-fs --no-default-features --features temporary-file --lib scratch::temporary:: --locked --offline
.PHONY: test-artifacts-wasm test-artifacts-encoding
test-artifacts-wasm:
	+cargo test -p ic-host-artifacts --no-default-features --features wasm --lib --locked --offline wasm::
	+cargo test -p ic-host-artifacts --no-default-features --features wasm --test consumer_contracts --locked --offline
test-artifacts-encoding:
	+cargo test -p ic-host-artifacts --no-default-features --features wasm-encoding --lib --locked --offline wasm_encoding::
	+cargo test -p ic-host-artifacts --no-default-features --features wasm-encoding --test wasm_encoding --locked --offline
.PHONY: test-artifacts-text
test-artifacts-text:
	+cargo test -p ic-host-artifacts --no-default-features --features wasm-text --lib --locked --offline wasm_text::
.PHONY: check-fs-non-unix
check-fs-non-unix:
	+cargo check -p ic-host-fs --lib --bins --test unsupported_lock --all-features --target wasm32-unknown-unknown --locked --offline
test-tools-default:
	+cargo test -p ic-host-tools --no-default-features --all-targets --locked --offline
.PHONY: test-tools-interface
test-tools-interface:
	+cargo test -p ic-host-tools --no-default-features --features ic-interface --all-targets --locked --offline interface::

.PHONY: test-tools-candid test-tools-combined test-tools-encoding test-wasm-inspection
test-tools-candid:
	+cargo test -p ic-host-tools --no-default-features --features candid --all-targets --locked --offline candid::
test-tools-combined:
	+cargo test -p ic-host-tools --no-default-features --features candid,ic-interface --all-targets --locked --offline
.PHONY: test-tools-agent
test-tools-agent:
	+cargo test -p ic-host-tools --no-default-features --features agent --all-targets --locked --offline agent::

test-tools-encoding:
	+cargo test -p ic-host-tools --no-default-features --features encoding --all-targets --locked --offline encoding::
.PHONY: test-tools-registry
test-tools-registry:
	+cargo test -p ic-host-tools --no-default-features --features registry --all-targets --locked --offline registry::
test-wasm-inspection:
	+cargo test -p ic-host-wasm-inspect --all-targets --locked --offline
.PHONY: isolated-features-check test-tools-icp-project test-tools-icp-ledger test-tools-ledger-evidence test-text-yaml
isolated-features-check:
	+$(MAKE) --no-print-directory test-tools-icp-project
	+$(MAKE) --no-print-directory test-tools-icp-ledger
	+$(MAKE) --no-print-directory test-tools-ledger-evidence
	+$(MAKE) --no-print-directory test-text-yaml
test-tools-icp-project:
	+cargo test -p ic-host-tools --no-default-features --features icp-project --all-targets --locked --offline icp_project::
test-tools-icp-ledger:
	+cargo test -p ic-host-tools --no-default-features --features icp-ledger --all-targets --locked --offline icp_ledger::
test-tools-ledger-evidence:
	+cargo test -p ic-host-tools --no-default-features --features ledger-evidence --all-targets --locked --offline ledger_evidence::
test-text-yaml:
	+cargo test -p ic-host-text --no-default-features --features yaml --lib --locked --offline yaml::
tools-features-check:
	+bash scripts/ci/check-tools-features.sh
compiler-check:
	+bash scripts/ci/check-rust-toolchain.sh "$(CURDIR)"
.PHONY: candid-boundary-check
candid-boundary-check:
	bash scripts/ci/check-product-candid-boundary.sh
dependency-pins-check: candid-boundary-check
	+bash scripts/ci/check-dependency-pins.sh --cargo-inheritance
check-doc-links:
	bash scripts/ci/check-product-doc-links.sh
release-adapter-check:
	+bash scripts/release/test-adapter.sh
tooling-command-check: export IC_TOOLING_RELEASE_GROUP = $(if $(filter command line,$(origin TOOLING_RELEASE_GROUP)),$(TOOLING_RELEASE_GROUP),)
tooling-command-check:
	+bash scripts/ci/run-tooling-checks.sh
publish publish-check: export IC_TOOLING_PUBLISH_SELECTION = $(PACKAGE)
publish:
	+bash scripts/publish/workspace.sh publish $(if $(filter command line,$(origin PACKAGE)),--packages "$$IC_TOOLING_PUBLISH_SELECTION")
publish-check:
	+bash scripts/publish/workspace.sh check $(if $(filter command line,$(origin PACKAGE)),--packages "$$IC_TOOLING_PUBLISH_SELECTION")
publish-command-check:
	+bash scripts/publish/test-workspace.sh
.PHONY: ci-publication
# One policy drives ordinary CI and the source-bound release selector.
ci ci-publication:
	+bash scripts/ci/workspace-validation.sh "$@"
.PHONY: docs-workspace test-workspace-installed
docs-workspace:
	+RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features --locked --offline --no-deps
test-workspace-installed:
	+@server="$$("$(SERVER_CLI)" check)" && POCKET_IC_BIN="$$server" cargo test --workspace --all-targets --all-features --locked --offline
release-version:
	+@bash scripts/release/adapter.sh version
release-preflight:
	+@bash scripts/release/adapter.sh preflight
release-verify:
	+@bash scripts/release/adapter.sh verify
release-prepare-version:
	+@bash scripts/release/adapter.sh prepare
release-prepared-check:
	+@bash scripts/release/adapter.sh check
release-files:
	+@bash scripts/release/adapter.sh files
release-commit-check:
	+@bash scripts/release/adapter.sh commit-check
release-committed-check release-tagged-check release-push-check:
	@bash scripts/release/adapter.sh committed-check

# Preserve Build shell, workflow and history qualification gates.
SHELLCHECK ?= $(CURDIR)/.tools/ci/bin/shellcheck
ACTIONLINT ?= $(CURDIR)/.tools/ci/bin/actionlint
GITLEAKS ?= $(CURDIR)/.tools/ci/bin/gitleaks
.PHONY: install-validation-tools lint check-shell check-workflows check-secrets
install-validation-tools:
	@bash -ec 'test "$$(uname -s):$$(uname -m)" = Linux:x86_64; \
		. ci/tool-versions.env; \
		bash scripts/ci/install-ci-tool.sh shellcheck --version "$$SHARED_TOOLING_SHELLCHECK_VERSION" \
			--sha256 "$$SHARED_TOOLING_SHELLCHECK_SHA256_LINUX_X86_64" --install-dir "$(CURDIR)/.tools/ci/bin"; \
		bash scripts/ci/install-ci-tool.sh actionlint --version "$$SHARED_TOOLING_ACTIONLINT_VERSION" \
			--sha256 "$$SHARED_TOOLING_ACTIONLINT_SHA256_LINUX_AMD64" --install-dir "$(CURDIR)/.tools/ci/bin"; \
		bash scripts/ci/install-ci-tool.sh gitleaks --version "$$SHARED_TOOLING_GITLEAKS_VERSION" \
			--sha256 "$$SHARED_TOOLING_GITLEAKS_SHA256_LINUX_X64" --install-dir "$(CURDIR)/.tools/ci/bin"'

lint: check-shell check-workflows

check-shell:
	"$(SHELLCHECK)" -x -P SCRIPTDIR -P . scripts/ci/*.sh scripts/dev/*.sh scripts/release/*.sh scripts/publish/*.sh .githooks/pre-commit

check-workflows:
	"$(ACTIONLINT)" .github/workflows/*.yml

check-secrets:
	"$(GITLEAKS)" git --redact=100 --no-banner --no-color --log-level error "$(CURDIR)"
