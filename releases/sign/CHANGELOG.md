# Sign changelog

Package: `ic-host-sign`.

## [0.1.1] - 2026-11-18

### 🔧 Changed

- Refresh the package development tooling selection to Toko Tooling 0.45.8.
  Public APIs, persisted bytes and runtime behavior remain unchanged.

## [0.1.0] - 2026-11-17

### ➕ Added

- `Signer` and the optional `ic-sign` helper integrate existing ICP secp256k1
  keyring identities with Platform's authenticated exact update and independent
  original-request-ID status envelopes. The
  parent receives bounded public signed bytes, while only the helper accesses
  credentials. Protocol/configuration/tool commitment, recipient/effective
  distinction, raw arguments, nonce presence, absolute expiry and completion
  deadline remain explicit. No submission, retries or identity migration.
  Resolves the producer integration in
  [Tooling #12](https://github.com/dragginzgame/ic-tooling/issues/12).
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
  No operator credential is accessed.
- Native Intel and ARM macOS qualification passes for exact update/status
  signing, missing/locked refusal, inherited network denial and disposable
  keychain cleanup. The helper is preauthorized before ICP populates its test
  credential; no signing-time prompt or operator ACL change is introduced.
- Extracted archive, isolated provider graph and installed CLI qualification pass
  against published Process 0.81.0 and Platform 0.34.15. Canic's Backup adapter
  and installed acceptance remain with
  [Canic #5](https://github.com/dragginzgame/canic/issues/5).
