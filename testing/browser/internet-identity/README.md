# Local Internet Identity browser fixture

This repository fixture supplies real local Internet Identity through IC Testkit.
Consumers select an immutable IC Tooling Git revision, prepare the explicit
inputs, build the private driver, and use `withFixtureSession` to retain its
lifetime around their acceptance body. It adds no published crate or application
authentication policy. Canic retains its grants, custody, admission and logout.

## Pinned inputs and schema ownership

[inputs.json](inputs.json) pins the matched official II backend/frontend release
[`release-2026-10-02`](https://github.com/dfinity/internet-identity/releases/tag/release-2026-10-02),
source revision `b9a8aaf43977adc1a2edf6ce98729826fc45133c`, and four SHA-256 hashes.
The backend Candid file is the release asset; the frontend interface comes from
that exact source revision. No `latest` URL is resolved at setup or execution.
Upstream owns the Apache-2.0 implementation and external initialization schemas;
the fixture encodes only configuration data against those original `.did` files
using the independently admitted `didc` 0.6.2 from `ci/ic-tools.tsv`.

This is the same matched II pair pinned by
[PocketIC 16.1's build inputs](https://github.com/dfinity/ic/blob/e9e42a6dd74acb9049017863e944cd5799be5b2b/mainnet-canister-revisions.json).
We install the pair explicitly because PocketIC's built-in II deployment enables
dummy authentication. This fixture disables dummy auth in **both** canisters,
disables CAPTCHA and external OpenID providers, sets local related/backend
origins, and enables the upstream frontend's `fetch_root_key` configuration.
Signature, ingress and certificate verification remain enabled. The installed
module identities are checked against the pins by the browser acceptance.

## Explicit preparation

From the IC Tooling checkout, use Node 24.21.0 / npm 12.2.0 and the pinned Rust
toolchain. Downloads happen only in these explicit preparation commands:

```sh
npm ci --prefix testing/browser/internet-identity --ignore-scripts --no-audit --no-fund
node testing/browser/internet-identity/inputs.mjs setup .tools/internet-identity/release-2026-10-02
make install-server
make install-ic-tools IC_TOOLS=didc
cargo build --manifest-path qualification/Cargo.toml --locked --offline -p ic-testkit-dev --bin internet_identity_fixture
```

Select `DIDC` from the verified `bin` directory printed by the IC-tool installer;
its offline admission is `make ic-tools-check IC_TOOLS=didc`. Set `CHROMIUM` to
an explicitly prepared Chromium executable. `playwright-core` does not download
a browser or run installation scripts. The caller owns browser admission;
acceptance records its reported version and executable SHA-256. The npm lock
selects **@icp-sdk/auth 10.0.2**, a single **SDK Core 6.1.1**, esbuild 0.28.2 and
Playwright Core 1.62.1. Platform's JS facade is unnecessary in this supplier proof.

## Real-browser acceptance

Use a fresh evidence directory on every attempt:

```sh
SERVER="$(target/debug/ic-testkit-server check)"
node testing/browser/internet-identity/acceptance.mjs \
  qualification/target/debug/internet_identity_fixture "$SERVER" "$DIDC" \
  .tools/internet-identity/release-2026-10-02 \
  target/ii-browser-acceptance-001 "$CHROMIUM"
```

The command creates a disposable PocketIC instance and local HTTP gateway, opens
a fresh Chromium context, and registers an actual CTAP2 virtual passkey through
the unmodified II UI. It invokes AuthClient `signIn()` and `getIdentity()`, then
submits signed ingress to a tiny canister returning its actual caller bytes. The
certified reply must match the authenticated principal. Negative controls require
a typed certificate-verification failure under a wrong root key and replica
HTTP 400 refusal of an altered ingress signature. Neither control retries the
update. The browser blocks non-loopback traffic and uses no production account,
mock provider protocol or validation override.

`proof.json` retains public authenticator RP/sign-counter evidence, provider pins,
matched JS/browser identities, the accepted caller and negative-control results.
`cleanup.json` confirms zero driver exit and closed gateway/management server after teardown.
`browser.zip` retains a Playwright trace; fixture readiness, exact interface copies
and server output remain under `fixture/`. Linux qualification on 2026-11-06 used
Chromium **156.0.8078.4**; this is virtual WebAuthn **browser integration** evidence,
not physical authenticator assurance. Native macOS qualification is separate.

## Reuse in consumers

The direct driver accepts `--server`, `--didc`, `--inputs` and a **new** `--output`
directory. It performs no download. It rechecks bounded, non-symlink input files,
keeps private verified schema/wasm bytes for initialization, and publishes
`OUTPUT/connection.json` only after installation. That document contains:

- `authorizeUrl` for AuthClient's provider and `backendCanisterId` for its signer.
- `backendOrigin`, `frontendCanisterId`, `gateway`, and the proof canister ID.
- `serverUrl` and numeric `instanceId` for native PocketIC attachment to the same
  running instance. `serverUrl` is the management API, distinct from `gateway`.
- `rootKey` as DER bytes for the consumer's local agent; never use a local root
  override for a production provider.
- Exact provider release/revision and installed module hashes.

Configure AuthClient with both `identityProvider.authorizeUrl` and
`identityProvider.canisterId`; its `agentOptions` must use the returned gateway
(or a consumer-owned API proxy) and `rootKey`. Keep normal SDK verification on.
Reuse the existing session helper with `readyPath: OUTPUT/connection.json`, a
distinct retained log, `startupMs: 180000`, `stopMs: 10000`, `pollMs: 50`.
The caller owns the body's deadlines, browser lifecycle and paths.

### Native attachment and cleanup ownership

Canic can create and install its issuer/receiver canisters in this instance using
`ic_testkit::pocket_ic::PocketIc::new_from_existing_instance(server_url,
instance_id, Duration::from_millis(request_timeout_ms))?`, with `server_url` parsed from `serverUrl`
and `instance_id` checked into `usize` from `instanceId`. The caller selects its
request timeout and Tokio runtime. This constructor borrows the existing
instance; it creates no new instance or managed server. Keep all native setup
and browser acceptance inside the same `withFixtureSession` body. The existing
`gateway` routes the new canisters too; II and the consumer share its root key.

The fixture process retains exclusive instance, auto-progress, gateway and
managed-server cleanup ownership. Attached clients must not delete the instance,
stop progress/gateways, terminate the server or restore a snapshot that replaces
the fixture state. Finish consumer requests and close consumer browsers/agents
before returning from the body. An attached Rust handle can use
`attached.drop().await?` to release its borrowed handle; it does not delete the
fixture instance. Creating a separate gateway on that handle is outside this
attachment contract. Canic owns application canister setup and may explicitly
stop/delete its own canisters while the session is live; whole-instance teardown
also removes them. Rust Drop performs no network cleanup.

These endpoints and IDs are local test capabilities, valid only while the
fixture session is alive; reading `connection.json` does not extend its lifetime.
Do not expose the management API to the browser or reuse a record after teardown.
Ownership is unchanged by this additive schema-1 record extension.

Newline/EOF on stdin and SIGINT/SIGTERM/SIGHUP request explicit instance deletion
before managed-server termination/reaping. The server has a 600-second hard TTL
as an abandonment backstop. Failed startup/body attempts and output remain;
reruns never replace a previous evidence directory. No persistent state is
imported, and this fixture establishes no clock-skew or snapshot-safety policy.

Offline input rejection checks run in the existing browser-session CI job:
`node --test testing/browser/internet-identity/inputs.test.mjs`. The real browser
command is an explicit installed qualification, not an automatic download in the
ordinary unit-test path.
