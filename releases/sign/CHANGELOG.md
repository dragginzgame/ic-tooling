# Sign changelog

Package: `ic-host-sign`. Initial package; no registry release is claimed.

## [0.1.0] - 2026-11-17

### ➕ Added

- `Signer` and the optional `ic-sign` helper integrate existing ICP secp256k1
  keyring identities with Platform's authenticated exact update and independent
  original-request-ID status envelopes. The
  parent receives bounded public signed bytes, while only the helper accesses
  credentials. Protocol/configuration/tool commitment, recipient/effective
  distinction, raw arguments, nonce presence, absolute expiry and completion
  deadline remain explicit. No submission, retries or identity migration.
- Retain cancellation and cleanup custody across signing-future drop, refuse
  another invocation before settlement, recover an already-completed original
  result, and join the worker when the signer itself is dropped.
- Require an already-unlocked/authorized OS credential. Linux reads the existing
  Secret Service item without invoking unlock/prompt methods; macOS disables
  keychain UI inside the helper. Refusal creates no abandoned unlock dialog and
  introduces no password channel, fallback store or automatic signing retry.

### 🧪 Testing

- Disposable ICP CLI 1.7.0 Linux keyring qualification checks principal fidelity,
  exact signed update/status content, missing/locked credentials and cancellation
  with an unresponsive private Secret Service. Socket metadata tracing observes
  only local Unix IPC; prompt-header monitoring proves no prompt request occurs.
  No operator credential is accessed. Native macOS remains unqualified.

### 🚧 Pending

- [IC Tooling #12 — Provide bounded external sign-only integration with exact expiry and existing identity custody](https://github.com/dragginzgame/ic-tooling/issues/12)
  stays open for native macOS provider acceptance. Published Platform 0.34.15
  resolves the signed-status admission gap in
  [Platform #29](https://github.com/dragginzgame/ic-platform/issues/29).
- Process 0.81.0 is published and the extracted Linux Sign archive/installed
  provider gate passes against registry dependencies. Complete native macOS
  fixture acceptance remains required before this initial publication.
