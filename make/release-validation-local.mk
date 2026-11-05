# Consumer-owned group validation. Ordinary CI and publication retain their full gates.
.PHONY: release-validation-plan-check release-validation-force
release-validation-plan-check:
	+bash scripts/release/test-validation.sh
release-validation-force:
release-stage-%: release-validation-force
	+bash scripts/release/validation-stage.sh "$*"

.PHONY: release-scope-tools-check release-scope-lint release-scope-graphs release-scope-tests release-scope-docs release-scope-archives
release-scope-tools-check:
	+bash scripts/release/validation-focus.sh tools
release-scope-lint release-scope-graphs release-scope-tests release-scope-docs release-scope-archives:
	+bash scripts/release/validation-focus.sh "$(@:release-scope-%=%)"
