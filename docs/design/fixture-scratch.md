# Qualification fixture scratch

Tooling-owned shell fixtures use `scripts/ci/product-fixture.sh` to select
`TMPDIR` when supplied, otherwise the selected workspace's `target/tmp`.
`tooling_fixture_create` delegates creation and physical-path anchoring to Shared's
unchanged `fixture_create`. A private browser test helper applies the same default
to generated Node children and input fixtures. Paths with CR/LF refuse before
directory creation. Explicit relative or aliased parents resolve physically;
the existing source-copy and retention helpers keep their symlink/refusal rules.

The product tooling driver admits and exports the scratch parent once, then owns
both the selected checks and final imported release-command probe. Nested Shared
probes therefore inherit the same parent. Standalone product fixture entrypoints
use the helper too, including release/publication substitutes and formatting,
source-policy, CI-selection and evidence checks. A caller's explicit `TMPDIR`
selection retains its meaning; no absolute path is rewritten inside Shared's
creation primitive.

Only new disposable qualification fixtures change location. Existing `/tmp`
evidence, active Git-side release plans/receipts, product outputs, server assets
and the shared tool store are untouched. Production release metadata scratch is
separate from generated test workspaces. Shared's execution snapshots/export
staging remain supplier-owned; control-plane copies can need to survive consumer
Cargo cleanup and are not changed by this local policy. External-path/security
fixtures continue selecting their own explicit paths. Rust callers retain their
explicit scratch paths and `CARGO_TARGET_TMPDIR` ownership.

Successful cleanup and failed-attempt retention still belong to the existing
fixture lifecycle. No historical artifact cleanup is implied. `cargo clean`
retains its ordinary whole-workspace target semantics.

Focused checks are `bash scripts/ci/test-product-fixture-scratch.sh`, the
initialization/retention probes in `test-host-fixture-retention.sh`, the
`test-make-snapshot.sh --case tooling-roster` route, and the browser scratch tests.
They cover default/explicit/aliased parents, refusal, retained failures and actual
Make routing without replaying Shared's generic regression matrix or publishing.
