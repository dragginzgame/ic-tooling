# Http changelog

Packages: `ic-host-http`.

This group starts with the replacement implementation. Earlier shared development
evidence is in [checkpoint history](../history.md), not a prior release of this group.

## [0.80.0] - 2026-11-16

### ⚠️ Breaking

- Replace the former HTTP surface with a complete-body adapter. Consumers select
  Reqwest directly and retain client, TLS, deadline and request policy; there is
  no client re-export, retry policy or digest dependency.

### ➕ Added

- Bound complete response bodies before copying, with typed size, transport and
  allocation failures and capped fallible buffer growth.

### 🧪 Testing

- Prepare the 0.80.0 registry candidate with complete package metadata and add
  extracted-archive loopback, lint and documentation checks to the native matrix.

- Qualify current Backup HTTP policy, body limits and transfer behavior using
  local-loopback fixtures, retaining original transport causes and refusal cases.
