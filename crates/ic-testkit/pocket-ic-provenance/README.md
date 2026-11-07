# PocketIC client transfer

## Public adapter boundaries

Tooling #340 extends the latest `loc-surface-cut.patch` adaptation with the
Testkit-owned CBOR failure classification and private supplier-error conversion.
The original typed source and its nested cause remain intact; packet codecs,
protocol bytes, budgets and no-replay behavior remain with their existing owners.
The layer also records the previously delivered polling/cancellation fixture
handshakes, restoring their exact reconstruction. Only this latest adaptation
and its checksum are regenerated against its unchanged parent; donor, transfer
and upstream fingerprints remain unchanged. The source guard reconstructs and
checks that complete chain, including the new boundary tests.

`loc-surface-cut.patch` records the authorized removal of unused Candid helper
exports, default-effective-ID lookup, server-filesystem snapshot APIs/DTOs,
no-tick polling, authenticated ingress-status APIs, custom blockmaker ticks,
subnet deletion and unused topology shortcuts. Ordinary ticks and anonymous
ingress status retain their request bytes, results and failure custody; topology
keeps complete server DTOs and its used application/NNS/II/range projections.
This layer also requires explicit finite request budgets at every construction,
attachment and listing boundary. One checked deadline covers dispatch, body reads,
accepted-operation polling, pauses and best-effort pruning; zero/overflowing
durations fail before dispatch. Protocol bytes and no-replay custody are unchanged.
The two Candid helpers needed by management calls are now private. Client failure
tests reuse Host Process's bounded request reader while retaining custom stalled,
truncated and replay responses. This layer is reversed before management routing;
original protocol codecs, request custody, snapshot transfer and donor hashes stay
unchanged. Public removals are a Testkit minor-version cut, without aliases.

`management-routing.patch` centralizes the repeated canister-scoped management
destination, effective-principal and optional-sender projection. Public method
signatures, method strings, DTOs, protocol bytes and fallible/no-replay execution
are unchanged. The source guard reverses this separately checksummed layer next,
then reconstructs every existing donor/upstream layer without changing its hashes.

The separately checksummed `logging-owner.patch` removes implicit client log
directory creation, worker retention and global subscriber initialization.
Imported tracing events remain compatible with a caller-selected subscriber;
applications own their `ic-logging` sink, clock, limits and retention. The guard
reverses this layer before the surface cut and original reconstruction chain.
Original donor/upstream fingerprints, licenses and other patches are unchanged.

Tooling #324 / Platform #139 transfer the complete maintained client from
Platform commit `41688ced81066b90aee756def7468a8388ee67e9`, package source 0.20.0.
This includes the subsequent macOS blocking-socket test fix from `79f905f`.
The donor archive SHA-256 is
`bf15bce368afc3daf1383e8df0d40611d3915abe6308af84fa56cfe0f5ad79ca`.
The donor was qualified on Linux and macOS 15 Intel/ARM in Platform #137.
That historical evidence is not qualification of this new package layout.

`src/lib.rs` becomes `src/pocket_ic/mod.rs`; internal crate paths become module
paths. Rustfmt orders those imports. The module retains the donor's Clippy
policy for imported source by expecting Testkit's additional pedantic/nursery
lint groups only there. The transfer changes no client function bodies, protocol constants, tests,
cryptographic operations or retry semantics change.

`module-transfer.patch` records exactly that packaging adaptation and has its
own checksum. `transfer-source.sha256` binds every donor source, license and
provenance file. The source guard reverses the module patch, checks all donor
hashes, then runs the unchanged Platform upstream reconstruction checks against
the reconstructed source. Existing upstream hashes and local fallible/snapshot
patches are retained, not refreshed. The mutation fixture rejects altered bodies,
imports, patches and licenses. The original notices remain under `upstream/`;
their Platform paths and release references describe the donor history.

The imported source remains Apache-2.0; see LICENSE and upstream/LICENSE.
Testkit's package license is MIT AND Apache-2.0. The source remains maintained in
one Testkit module, without a second wrapper package or separate release group.

## Tooling maintenance after transfer

`tooling-maintenance.patch` records the subsequent explicit-server API cut and
blocking-forwarder simplification and the subsequent async-only cut. Its independent checksum is checked before
reversing it, then the unchanged transfer and upstream reconstruction checks run.
Original donor/upstream hashes and patches are not refreshed to admit these edits.
Mutation checks also reject changes to this maintenance patch.

The client no longer provisions or starts a server. Builders require
`with_server_url`; missing selection returns `ClientError::MissingServerUrl`
before runtime construction or network effects. Sync/async `new` and
`list_instances` take a URL. `pic::server_assets` and the managed startup surface
own provisioning and server custody. Protocol bytes, crypto, typed failures,
accepted-operation custody, snapshot transfer and external DTO identities stay
unchanged. Existing private state-directory custody remains imported behavior.

## Async-only client maintenance

The next Testkit minor removes the blocking facade and its private runtimes,
channels and destructor I/O. The maintained async implementation moves from the
public nonblocking path into the private client module and is exported once as
PocketIc. Builders and helpers await the caller-owned runtime. Explicit close
retains the managed server through deletion; cancellation and uncertain remote
outcomes require caller reconciliation. Original protocol identities, request
custody, no-replay behavior and bounded snapshot transfer remain intact.

The maintenance patch reverses the complete cut, including test migrations and
the private module move, before the unchanged donor and upstream checks run.
Original provenance hashes, imported licenses and upstream patches are unchanged.

## Typed IC packet codec maintenance

The maintainer authorized the CBOR boundary consolidation. The separately
checksummed protocol-codec.patch changes only the subnet-metrics packet codec
calls and their error/import plumbing to typed Platform APIs. The upstream
transport and certificate identities, self-described envelope bytes, metric
ranges and no-authentication behavior remain unchanged. The guard reverses
this layer before the existing maintenance/transfer patches and original
fingerprints; none of those protected hashes is refreshed.

## Supported client surface

The maintainer authorized removing unused client APIs. `surface-cut.patch`
records removal of sender-info call variants, server-side signature verification,
Motoko EOP upgrade convenience and flexible HTTP response mocking, together with
their endpoint-only DTOs. It also removes the unused count/boolean subnet
configuration conversion and orphaned server response/blob DTOs. Ordinary calls
retain their routing, absent sender-info
field, accepted receipts and no-replay behavior. Flexible-outcall response grammar
remains decodable. The guard verifies and reverses this layer before the unchanged
codec, maintenance and original-source checks; mutation coverage binds the new
patch as well. Original donor/upstream hashes and licenses remain unchanged.

The same pending surface patch narrows builder configuration to maintained
consumers. One typed request replaces the component forwarding path. Persistent
directories remain caller-owned; temporary state handles/read-only copying,
unused subnet/configuration families and dead server-validation copies are
removed. The server retains validation, feature provisioning and defaults for
unselected options. Topology response enums remain complete. Selected creation
requests and installed persistent-state reuse have their own focused checks.

The pending patch also removes unused JSON-schema derivations and production
enum iteration. The rejection-code catalog retains test-only iteration, so its
exhaustive protocol-value checks do not require a production macro/trait surface.
Serde derives, field attributes, external identities and codec bodies are unchanged.
