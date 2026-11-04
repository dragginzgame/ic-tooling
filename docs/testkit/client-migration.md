# Maintained client in Testkit

The published Testkit 0.84 line owns the PocketIC client and lifecycle helpers in
one package. Use `ic_testkit::pocket_ic::{PocketIc, PocketIcBuilder, ClientError}`;
async clients live in `ic_testkit::pocket_ic::nonblocking`, and REST DTOs in
`ic_testkit::pocket_ic::common::rest`. Existing lifecycle helpers stay in
`ic_testkit::pic`. Replace `ic_platform_pocket_ic::...` and
`ic_platform::pocket_ic::...` imports. Remove the separate client dependency and
the Platform `pocket-ic` feature. No compatibility re-export is provided.

Select the Testkit 0.84 line directly from crates.io; the earlier 0.83 package
does not supply this module. Client and helper changes need only one Testkit selection.
Platform remains a dependency for general external-protocol adapters; it must
not depend on Testkit through normal/build dependencies.

The module is native-only. Wasm measurement consumers retain the lightweight
Runtime/Types graph, without HTTP, client, server, Tokio, tempfile or Candid.
The PocketIC server remains 16.1.0. External Candid Principal, management DTOs,
certificate and transport identities and protocol bytes are unchanged. Client-owned
DTOs/errors acquire Testkit identity, so callers must migrate together.

The transfer preserves fallible operations, accepted-operation custody, explicit
Busy-only retry, no replay after uncertain requests, bounded snapshot chunks and
total budgets. The 0.85 API requires an explicit server endpoint; `pic` owns
provisioning and lifecycle policy.
Application budgets, retries, topology, authentication and fixture policy remain
caller-owned. No Runtime, Types, Agent or crypto implementation moves here.

## Explicit server selection in 0.85

Provision with `ic-testkit-server setup` or `pic::server_assets::setup`, admit an
existing installation with `check`, and start it through `PocketIcStartupConfig`.
Managed `build_instance(builder)` supplies the endpoint and retains server custody.
For an externally owned server, pass its URL to `PocketIcBuilder::with_server_url`.
Builders without a URL return `ClientError::MissingServerUrl`; they do not inspect
`POCKET_IC_BIN`, download executables or start processes.

Replace `PocketIc::new()` with `PocketIc::new(server_url)` and
`PocketIc::list_instances()` with `PocketIc::list_instances(server_url)`; the async
client has the same explicit arguments. Replace `with_server_binary`,
`StartServerParams` and `pocket_ic::start_server` with the managed `pic` startup
surface. Keep its server handle alive while its instances are in use. This is a
minor-release API change; the currently published 0.84 line retains the inherited
convenience behavior.

[Provenance](../../crates/ic-testkit/pocket-ic-provenance/README.md) records the
exact Platform donor and reconstruction guards. Run
`bash scripts/ci/test-pocket-ic-source.sh`, `make testkit-graph-check` and the
focused Testkit commands in [host qualification](../hosts.md). Native Linux tests
are separate from required macOS Intel/ARM and installed-server evidence.
