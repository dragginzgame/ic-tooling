# IC Tooling release notes

## Shared Tooling 0.30.7 adoption — 2026-11-06

### 🔧 Changed

- Adopt Shared Tooling 0.30.7 release-efficiency guidance and its opt-in frozen
  release runner. Frozen delivery remains disabled until Tooling's actual hooks
  are qualified; ordinary release commands retain their existing source custody.

## Workspace tooling — 2026-11-06

### 🧹 Cleanup

- Remove the unused root Semver catalog entry. Semver may remain transitively;
  unrelated lock selections are preserved. Logging support used only by the
  application-owner test stays in test dependencies.

### 🧪 Testing

- Default local shell/browser qualification fixtures to workspace `target/tmp`,
  including nested probes invoked through Make. Explicit `TMPDIR`, physical path
  custody and failure retention remain supported; existing evidence is retained.
  [#334 — Keep qualification fixture scratch under the workspace target](https://github.com/dragginzgame/ic-tooling/issues/334).

- Adopt Shared Tooling 0.30.6 and enforce its Candid source boundary across
  maintained Rust packages, tests and fixture drivers. Exact external-protocol
  declarations remain permitted; new owned Candid payloads fail source admission.
  See [#337 — Enforce the maintained Rust Candid boundary](https://github.com/dragginzgame/ic-tooling/issues/337)
  and the [coverage and exception guide](docs/design/candid-boundary.md).

## Release groups

| Release group | Packages | Release notes |
| --- | --- | --- |
| Foundation | Artifacts, FS, HTTP, Process and the Wasm inspector | [Foundation ledger](releases/foundation/CHANGELOG.md) |
| Build | Cargo/Wasm builds and artifact caches | [Build ledger](releases/build/CHANGELOG.md) |
| Text | Bounded text, Unicode and optional YAML | [Text ledger](releases/text/CHANGELOG.md) |
| Host Tools | IC inspection, Agent, certificate and ledger helpers | [Host Tools ledger](releases/tools/CHANGELOG.md) |
| Testkit | Simulator integration and private development/probe packages | [Testkit ledger](releases/testkit/CHANGELOG.md) |
| Native | Dependency-free native allocation measurement | [Native ledger](releases/native/CHANGELOG.md) |

Each group owns its package versions and release notes. See
[release ownership](docs/design/release-groups.md) for commands and boundaries.

[Joint integration history through 0.76.1](releases/integration/CHANGELOG.md) and
[coordinated history through 0.73.0](releases/history.md) retain their original scope.
