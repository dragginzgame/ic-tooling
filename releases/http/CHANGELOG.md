# Http changelog

Packages: `ic-host-http`.

This group starts with the replacement implementation. Earlier shared development
evidence is in [checkpoint history](../history.md), not a prior release of this group.

## [0.80.3] - 2026-11-21

### 🔧 Changed

- Adopt Toko Tooling 0.46.0 and document the roles of packaged Rust types.

## [0.80.2] - 2026-11-19

### 🔧 Changed

- Adopt Toko Tooling 0.45.21 and its Rust source layout in the packaged crate.

## [0.80.1] - 2026-11-18

### 🔧 Changed

- Refresh the package development tooling selection to Toko Tooling 0.45.8.
  Public APIs, persisted bytes and runtime behavior remain unchanged.

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
