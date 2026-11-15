.DEFAULT_GOAL := check

include vendor/toko-tooling/make/consumer.mk
include vendor/toko-tooling/make/cargo-dependencies.mk

.PHONY: help
help:
	@printf '%s\n' \
	  'Dependencies:' \
	  '  make deps-list                 List local packages and versions (offline)' \
	  '  make deps-update               Update workspace locks within manifest constraints' \
	  '    DEPENDENCY_PACKAGE=name      Limit update to one package' \
	  '    DEPENDENCY_PRECISE=version   Select an exact version for that package' \
	  '  cargo tree --locked --offline  Inspect resolved dependency graphs' \
	  '' \
	  'Development:' \
	  '  make fmt / fmt-check           Format or check Rust and manifests' \
	  '  make test-build-selection      Run focused Build selector/capture checks' \
	  '  make test-scheduling-consumer  Check the isolated real Scheduling host consumer' \
	  '  toko-cloc [-d N] [crate]        Report Rust LOC (standalone command)' \
	  '' \
	  'Tools:' \
	  '  make install-tools             Prepare pinned common tools' \
	  '  make tools-check / tools-report Check prepared tools / report all failures' \
	  '  make install-hooks             Activate the repository formatting hook' \
	  '  make check                     Verify Cargo tooling selection and snapshot' \
	  '  make toko-tooling-status        Show installed and locked tooling identities' \
	  '  make toko-tooling-preview       Preview installation of the locked tooling' \
	  '  make toko-tooling-update        Install the locked tooling and prepare tools'

.PHONY: check test-build-selection test-scheduling-consumer
check: toko-tooling-check

# Focused selector/protocol checks and the real shared-target capture workflow.
test-build-selection: toko-tooling-check
	cargo test -p ic-build --lib cargo::execution::tests --locked --offline
	cargo test -p ic-build --features test-support --test cargo_execution reported_custom_libraries_and_exports_survive_shared_target_collisions --locked --offline

# Pinned consumer source is exported into target; the sibling remains read-only.
test-scheduling-consumer: toko-tooling-check
	bash scripts/qualification/scheduling-consumer.sh
