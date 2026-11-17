# External sign-only integration

[Tooling #12](https://github.com/dragginzgame/ic-tooling/issues/12) owns the native
integration for Canic's Backup adapter. Sign is a committed release candidate;
the issue stays open for native macOS provider acceptance.
Consumer adoption belongs to [Canic #5](https://github.com/dragginzgame/canic/issues/5).

## Ownership and selected provider

The maintainer selected `canic-mainnet`. Its public ICP manifest records
`kind: keyring`, `algorithm: secp256k1`, and principal
`o5trf-oqyg7-cawjp-xs4pw-aomb3-iwki5-hyezf-qahfz-j3ffd-jh4fc-oqe`.
No operator keyring entry, password, seed or private key was read or exported.
Acceptance uses uniquely named disposable identities of the same storage/profile.

The new `ic-host-sign` package owns the IC-specific protocol, exact output
admission and asynchronous observation. Its optional `provider` feature builds
`ic-sign`, the only production component accessing native credentials. Platform
owns cryptography and IC envelopes; Process owns pipes and child/group cleanup.
Backup keeps approval, spending, durable signed attempts, submission and recovery.
No retry, identity migration or submission is introduced.

This split keeps Platform/keyring dependencies outside generic Process, Build,
portable bytes and Testkit. It removes Canic's need for pipe-writer threads,
scratch-file communication or detached signing workers. No legacy identity tree,
provider framework or parallel execution implementation is imported. Existing
Process execution and Artifact byte admission remain at their owners; Process
gains only the independently qualified finite-input increment.

Pinned ICP CLI 1.7.0 source `eda771516212045748dc14933c10148c118dbc2c` selects
`keyring::Entry::new("icp-cli", identity_name)` and PEM secp256k1 material. Its
unpublished `icp-app` loader is not copied. Linux uses maintained Secret Service
5.2.0 to search those existing service/username attributes, refuse locked or
ambiguous matches, and read one unlocked item without invoking unlock or prompt
methods. macOS uses keyring 4.2.0 with Security Framework 3.7.0's safe process-local
UI-disable guard. No alternate backend, plaintext export or key migration exists.
These direct suppliers were checked against current non-yanked registry releases.
The adapters are independently written; transport, credential cryptography and
native API bindings stay in their maintained dependencies.

Published Platform 0.34.15 (archive SHA-256
`70d95425a6f387880d21e9ce37e83ff6aea3fde380685f25573720231e71c30c`)
supplies `Secp256k1Identity::from_pem`, `prepare_update(UpdateArgs, maximum)` and
`prepare_request_status(original_id, effective, exact_expiry, maximum)` through
`native-signed-secp256k1`. Parent update admission uses `PreparedUpdate::from_bytes`
and decoded exact content; Tooling implements no cryptography or CBOR codec.
Independent status admission uses `PreparedRequestStatus::from_signed_bytes`.
The new crate name has no sparse-index entry as of 2026-11-17.

## Client contract

`Signer::new(absolute_helper, trusted_sha256, Configuration)` admits the selected
executable and public ICP manifest. The executable must be regular, nonsymlink,
executable and without group/other write bits; its hash is checked before and
after execution. Paths assume cooperative owners, not a hostile same-user sandbox.
The expected hash comes from a qualified installation, not a version string.
Group-writable Cargo build outputs require an explicitly owned installation copy
with write permissions narrowed before admission; fixtures never chmod Cargo's file.

Configuration binds the canonical self-authenticating sender, existing identity
name and absolute manifest path. Only `keyring`/`secp256k1` records are admitted.
ICP's ordinary public-manifest group-write mode is allowed: selected fields must
match original configuration and the actual key's derived principal. Credential
custody never follows from metadata alone. Linux requires one explicit local Unix
D-Bus address; TCP, autolaunch and fallback lists refuse. macOS uses native
Keychain and requires no bus. The child inherits no environment except the
admitted Linux session address.

The fixed provider profile is **noninteractive**: the credential must already be
unlocked and the qualified helper authorized by the OS. It refuses otherwise;
unlocking/preauthorization stays with existing OS tools before signing. No password
channel, unlock RPC, authentication dialog or automatic retry is supported. The
protocol identifier commits this profile. This removes the external prompt
lifecycle rather than making Canic clean up an OS dialog it does not own.

`Signer::commitment()` binds protocol, public configuration and executable hash.
Each `sign(request, original_commitment)` retains the original sender, recipient
versus effective target, method, raw arguments, absent/present-empty nonce,
absolute nanosecond expiry, completion deadline and envelope cap. Only zero
external network requests are supported. No clock rounding or renewal occurs.
Success retains exact authenticated bytes and their public binding; it grants
neither dispatch authority nor a fresh lifetime.

| Admission limit | Inclusive value |
| --- | --- |
| Complete JSON request/response frame | 32 MiB |
| Complete signed envelope | Caller cap, at most 4 MiB |
| Public ICP manifest | 1 MiB |
| Helper executable hashing | 256 MiB |
| Completion interval at admission | At most 300 seconds |
| Provider stderr | 64 KiB, drained but never returned/logged |
| Retrieved PEM | 16 KiB checked after the OS API allocates its response |

Strict JSON frames travel over stdin/stdout with exact EOF and no request scratch
files or secret argv. The helper checks protocol/commitment/tool/configuration
before key access, parses PEM internally, verifies the derived principal and
uses Platform's constructor. Secret buffers are zeroized; no heap-quota guarantee
is made. Error/panic diagnostics are static; provider stderr and private keyring
errors never cross the client API. The parent independently authenticates the
signature and compares exact content. The helper is selected trusted code, not
a network-egress sandbox.

`Operation::Status` carries only the original update ID. The helper signs exactly
`request_status/<original ID>` through Platform, without retained update bytes.
The parent calls `PreparedRequestStatus::from_signed_bytes(bytes, original_id,
sender, effective, cap)` and compares its authenticated expiry with the exact
approved expiry. Wrong ID/path, sender/key, expiry, malformed signatures, trailing
bytes, operation kind or envelope limit refuse. Returned `Signed::request_id()`
is the original update identity. Original update bytes remain unchanged. Platform
0.34.15 resolves [Platform #29](https://github.com/dragginzgame/ic-platform/issues/29).

## Cancellation and original-result custody

The signer owns at most one dedicated worker. Its first future poll starts
admitted work; there is no unbounded launch handoff or detached blocking task.
The worker owns execution, output admission and cleanup. Dropping `sign` requests
cancellation while the signer retains the worker/result. `state()` exposes
running, cancelling or awaiting-settlement work. A second invocation refuses
`Busy` until `settle()` joins the original worker and returns its original result.
Completed signed bytes survive observer drop; settlement never signs.

Dropping a settlement future preserves custody. Uncertain cleanup/worker failure
permanently refuses reuse. Dropping the signer cancels and joins; this may block
within the existing native cleanup contract. Cancellation does not prove the key
was unused, undo signing, authorize retry or permit a second owner over live work.

Process adds `Input::Bytes(Vec<u8>)` to its existing executor, fairly writing finite
nonblocking chunks while draining both outputs and closing stdin at exact EOF.
Short delivery, cancellation, deadline and output limits preserve failure and
cleanup evidence. Debug reports length only. `Input`/`Policy` lose `Copy` and
retain `Clone`, requiring pending Process 0.81.0 before registry publication.

## Executed evidence and remaining acceptance

Twelve Linux library tests pass: pre-dispatch refusal; tool/manifest drift; bounded,
malformed and failed output; authenticated exact update fields/corrupt signature;
cancellation/reentry; owner-drop join; and original signed bytes recovered after
a completed observer is dropped, with exactly one launch; independent status
authentication with exact/over caps and changed ID/sender/expiry/signature refusal.
Shell fixtures prove
lifecycle/refusal, not native keyring behavior.

`make test-sign-keyring` requires absolute `SIGNER_TEST_ICP` selecting prepared ICP
1.7.0. Linux uses a private D-Bus session and temporary Secret Service. ICP creates
disposable keyring identities under isolated `ICP_HOME` paths. A test-only password
unlocks the temporary encrypted collection over stdin; its binary storage header
is checked before cleanup. Authenticated sender
matches ICP's independently derived manifest principal; exact update/status fields,
missing credentials and retained bytes pass. For cancellation only the runner's
daemon is paused: the actual helper connects to native credential IPC before
future timeout/drop, is killed and reaped, and refuses reentry until settlement.
The backend remains with its owner and resumes for entry deletion. A third case
locks the actual collection and proves normal provider refusal, unchanged public
identity and retained signed bytes, and the existing credential still present.
That locked fixture's store teardown stays with the runner; destructors do not
attempt to unlock it. Temporary credential directories are removed. No operator
credentials or mainnet calls occur.

`target/sign-keyring.HWoxqfD7/` records three passing real-provider tests and only
`AF_UNIX` sockets. The trace observes the cancelled helper's credential thread
connecting before group termination. Only executable/socket metadata is traced,
never IPC buffers or environment contents. The runner caps its trace file and
execution time, records before/after source and prepared-tool hashes, and refuses drift.
Secret Service prompt-header monitoring records no `Prompt` call or signal.
The initial automatic-unlock experiment in `target/sign-keyring.9q8tBW3O/` showed
that killing a helper can leave an external prompt pending. It is failed evidence,
not a cancellation pass, and is why the provider now refuses locked credentials.
A stripped disposable helper excludes debug-symbol hashing from the IPC timeout
measurement; production build outputs are untouched.

Sign Clippy/Rustdoc and Process prerequisite logs live under `target/issue-12/`.
The complete Process/Build native archive phase passes on all three hosts in
[candidate CI](https://github.com/dragginzgame/ic-tooling/actions/runs/44173667281).
The separate Sign lint step lacked prepared offline dependencies; preparation is
now explicit. Linux provider collection also required an undeclared `rg`; the
runner now uses standard grep. The initial macOS provider job compiled the helper
and passed network denial, but refused the credential rather than showing UI.
Successful native credential access is not yet claimed.

The macOS fixture now requires `SIGNER_TEST_DISPOSABLE_KEYCHAIN=1` in an ephemeral
account. It snapshots default/search preferences, creates a password-protected
private keychain through the safe native API, and preauthorizes an empty test item
for exactly ICP, the qualified helper and the fixture owner. ICP then populates
the real secp256k1 key in place, without exporting it. This satisfies the provider's
already-authorized precondition rather than weakening its no-UI behavior. Apple's
[security tool contract](https://github.com/apple-oss-distributions/Security/blob/main/SecurityTool/macOS/security.1)
documents the creating-app default and explicit trusted application selection.
The fixture restores preferences and deletes only its private keychain on exit;
it never changes operator keychain ACLs. It also locks that owned keychain to
qualify immediate refusal without an unlock prompt.

Native provider jobs use Toko's pinned ICP installer. macOS runs inside inherited
`sandbox-exec` network denial with a loopback-denial probe; Linux observes only
Unix sockets and no prompt-header traffic. The macOS fixture correction still
requires successful hosted execution; source wiring is not native acceptance.

Finish native macOS provider acceptance before closing #12. The supported profile
has no signing-time unlock UI to cancel; operators unlock/authorize it beforehand.
Canic's installed
Backup acceptance stays with Canic. Sign has a separate group/ledger but is not
archive-qualified yet. Process 0.81.0 is published and the matching dependency
minimum is selected. The Sign archive gate admits extracted source, isolated
provider dependencies and an installed CLI through explicit prepared manifest/
helper paths. Local path success alone is not registry readiness. Consumer cutover
remains separate from this deployment.
