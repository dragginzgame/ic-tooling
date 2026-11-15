# Testkit changelog

Package: `ic-testkit`. This replacement remains unpublished; this is not a
crates.io release. The selected consumer currently uses released Testkit 0.106.0.

## [0.107.0] - Unreleased

### Added

- Establish the original PocketIC 16.1.0 client/type re-export as the shared test
  boundary. No runtime, server launcher, instance pool or consumer scheduler.
- Add an owned immutable update-reply catalog behind `held-reply-fixtures`.
  Complete admission checks duplicate-slot aggregate limits, method names and
  Wasm32 representation before copying; ownership allocations are fallible.

### Dependency selection

- Select PocketIC 16.1.0 at the shared type boundary. Its exact thiserror 2.0.18
  requirement selects that permitted version for the workspace; previous external
  identities are otherwise retained. Build/bytes do not acquire the simulator graph.

### Validation and remaining work

- Focused tests cover exact/over-budget aggregates, later invalid names, empty
  catalogs, duplicate order and opaque page-spanning reply ownership.
- Scheduling continues using released Testkit; its installed
  baseline is not replacement acceptance. No old source or test tree was imported.

### Generated module follow-up

- Add bounded `Catalog::compile` using Platform-owned HTTP encoding and escaped
  WAT. Duplicate update methods consume immutable single-use reply slots; failed
  barriers and exhausted methods reject. No simulator operations or release
  authority are created by compilation.
- Five catalog/module tests and one installed two-target test pass: exact caps,
  independent Wasm validation, page-spanning bytes, reverse release, differing
  replies, HTTP rejection and exhaustion. Add `make test-testkit-module` for the
  explicitly owned-server check. Consumer migration remains open.

### Exclusive controller follow-up

- Add `Fixture` and opaque `Request` values over the original async PocketIC
  client. Construction binds a distinct namespace without installing or ticking;
  pending observations filter admitted slots, and release verifies exact identity.
- Consume exclusive local release authority before dispatch. Cancellation or an
  upstream panic cannot authorize retry; drop has no simulator effects.
- Six unit tests and two independently selected installed tests now pass. The
  controller checks different replies in reverse order, wrong server/instance,
  foreign or altered tokens, expiry, repeated release and automatic progress.
  Local fault injection covers cancellation/panic uncertainty; it is not a remote
  rollback guarantee. Add `make test-testkit-control` for the owned-server check.
- Scheduling still uses the released supplier pending current-source replacement
  qualification. macOS and publication remain outside this Linux development slice.
