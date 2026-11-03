# Historical extraction and Platform boundary evidence

Frozen observations from the earlier extraction and Platform migration. These do
not select current dependencies or instruct new migrations. The maintained
[extraction contract](extraction.md) owns current boundaries.

## Fleet Candid/CDK boundary

The maintainer's 2026-10-18 direction is one controlled dependency boundary for
Candid and the CDK family, with IC Wire owning controlled application protocols.
Host consumes the workspace-selected published `ic-platform` for bounded Candid decoding,
native parsing and optional certification types. Platform's `docs/migration.md`
owns the fleet adoption contract. Host retains service-to-Wasm projections,
strict certificate CBOR admission and the released aggregate-byte adapter.
Per-reply decoder configuration belongs to Platform; the adapter maps limits and
typed failures while preserving debit-before-decode behavior. Diagnostics follow
Platform's compact policy. The historical observations below describe the wider
migration, not proof of completed sibling adoption.

The dedicated repository has one consumer-facing library, `ic-platform`:
its scope includes runtime integration as well as Candid. Keep
codec, host parser and executor/runtime capabilities separately selectable, with
no default activation. An executor-only consumer must not acquire the parser or
codec. Platform 0.2 owns the maintained Candid-free CDK adapters and explicit-codec
entrypoints, while preserving the registry executor and raw system-call identities.
As of Platform 0.9, Platform owns the executor and system-call implementation;
consumers align Platform versions rather than selecting external runtime packages.
Host enables none of those runtime features. Any necessary proc-macro implementation
crate belongs to this same boundary repository, not to a second runtime owner.

The boundary owns dependency selection, bounded Candid encoding/decoding,
external-protocol type integration and the supported CDK/executor interface.
It must expose explicit supported operations rather than a blanket upstream
re-export. Where generic external DTOs need Candid traits or derives, a deliberately
supported trait/macro surface is necessary and remains upstream-coupled until
replaced. Identity-only consumers select Platform's `principal` feature for its
owned `Principal`.
Registry shard APIs use that type; external SDKs and required Candid DTOs retain
`principal::ExternalPrincipal`, with explicit byte-preserving conversions selected
by `principal-external`. Neither the owned principal nor the new owned Account
selects a Candid codec or trait.
Canic retains dispatch, authentication and lifecycle policy; Timers retains
scheduling; Query retains NNS/SNS/ledger operations and schemas; Testkit retains
PocketIC lifecycle; Build retains native extraction execution. Host's five crates
retain their existing host responsibilities. The new boundary must not depend on
Host Tools or Testkit and create cycles when those consumers adopt it.

Historical evidence from working trees inspected before Platform adoption on
2026-10-18 (source identities below):

- Testkit's `crates/ic-testkit-candid/src/lib.rs` already owns bounded single-value,
  argument-list and exact-single-value decoders. Query selects that package in
  `crates/ic-query/Cargo.toml`; Testkit calls it from `src/pic/calls.rs`. Consolidate
  this code and Host's pending `candid/decode` under the boundary, preserving their
  distinct arity, diagnostics and caller-owned aggregate-accounting contracts.
- Host's `crates/ic-host-tools/src/candid/service` owns parser-backed service
  inspection. Move parser mechanics to the boundary's host capability; keep
  necessary projection into Host's Wasm method identities in Host, without making
  the boundary depend on Host types. Retire the old decoder owner after migration.
- Canic's `crates/canic-macros/src/adapter/mod.rs` emits `::candid` paths;
  `crates/canic-core/src/cdk/mod.rs` re-exports Candid and `infra/ic/call.rs` uses
  CDK calls directly. Update generators and compile fixtures with imports and
  manifests. Cached Candid derive 0.10.38 accepts `candid_path`; CDK macros 0.20.3
  accept a runtime `cratename` but still emit hard-coded `::candid` paths. A simple
  crate re-export does not establish that consumer manifests can omit both names.
- Timers' `crates/ic-timers/Cargo.toml` uses `ic-cdk-executor` and `ic0`, without
  Candid in its production dependency list. Its `src/platform` uses executor
  method contexts and trap cleanup. Preserve one executor/context identity across
  entrypoints, calls and timers; qualify cancellation and trap behavior together.
- Agent, PocketIC and external management/ledger type packages can bring Candid
  transitively. A ban on direct workspace dependencies does not remove those
  paths, published upstream types, or linked Wasm code. Record selected dependency
  graphs separately from installed-canister size and behavior measurements.

Reviewed checkout HEADs were Host `c8cd434`, Testkit `55ac397`, Canic `5583496bf`,
Query `fdcb0be3`, Timers `194204d`, Wire `ce56d5e` and Storage `009412d`.
Working-tree observations can include pending edits; these identities do not
claim the observations are committed, published or adopted.

The migration acceptance sequence is: qualify a boundary prototype with real
derive/endpoint consumers and host/Wasm feature graphs; publish a compatible
dependency; move the existing codec/parser owners and their callers together;
then migrate runtime integrations and enable direct-dependency bans per migrated
workspace. Include examples, tests, generated templates and independent fixture
workspaces. Shared's `scripts/ci/check-cargo-dependency-boundaries.sh` already checks
canonical package names despite aliases, across normal/build/dev/target tables.
Adopt it through a reviewed snapshot rather than writing a parallel checker.
Its scope is direct workspace dependencies, so separately enumerate CDK-family
packages, inspect transitive graphs and check unused root catalog declarations.

External management, NNS/SNS and ledger calls retain their specified wire formats.
Changing Rust dependency ownership must preserve request/reply bytes, errors,
decode limits, persisted records and retry identities. Protocol replacement for
owned endpoints is a separate IC Wire cutover with both ends qualified. Required
evidence includes malformed/quota/arity cases, macro expansion without forbidden
direct dependencies, installed query/update/management calls, async/trap cleanup,
and supported host checks. No fleet-wide source migration is qualified by this audit.

Storage is a separate boundary: current IC Storage delegates collection formats
to `ic-stable-structures` and allocation/bootstrap/exclusive handles to IC Memory.
Removing those dependencies requires an explicitly selected replacement for both
roles and retained-data qualification. The existing Shared stable-memory rule
requires IC Memory handles; retiring that owner also needs a coordinated baseline
change. No allocation or stored-format migration is implied by the Candid facade.
