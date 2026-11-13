# IC Tooling release notes

## Shared Tooling 0.31.4 adoption

### 🩹 Fixed

- Adopt the release-command checker repair for the
  [three-host CI failure](https://github.com/dragginzgame/ic-tooling/actions/runs/42357031568).
  The fixture substitutes both runners and checks fresh outer dispatch and
  captured inner dispatch without inheriting a parent release's settings.
- Refresh shared LOC scratch diagnostics and release-guide companion links.

## Shared Tooling 0.31.2 repair — 2026-11-07

### 🩹 Fixed

- Adopt Shared's committed checksum-parser fix for macOS system Bash 3.2.
  Preserve strict backend failure and malformed-output rejection; crate APIs
  and the completed Build 0.77.0 / Testkit 0.90.0 publication remain unchanged.

## Shared Tooling 0.31 deployment — 2026-11-06

### 🔧 Changed

- Adopt Shared Tooling 0.31.0's release-batch interface, schema-2 publication
  custody and checksum-verification fixes. Rename the local release settings,
  tests and guide references together; retired roster settings refuse explicitly.
  Shared owns automatic single-group capture; Tooling retains setup/publication
  choices and preserves historical batch records with their original identities.
- Derive focused release package identities from the release-group catalog and
  retain reverse-consumer contracts in one coverage policy. Unknown or changed
  scopes continue to select the complete gate.

## Shared Tooling 0.30.9 deployment — 2026-11-06

### 🔧 Changed

- Adopt Shared Tooling 0.30.9's frozen setup and batch custody interfaces. Use
  Shared's status, resume and guarded reconciliation commands; remove the local
  resume wrapper and parser bootstrap prerequisite. Recovery checks prepared
  parsers offline. Explicit retirement remains a separate operator action.

## Frozen deployment — 2026-11-06

### 🔧 Changed

- Use captured source for normal release and publication commands, deferring
  later normal-checkout edits. Select supplier-first batches once and resume
  using the retained batch ID; successful default releases already publish.
- Adopt Shared Tooling 0.30.8 so generated underscore-containing batch IDs can
  resume from their original recovery records.

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
| Testkit | Simulator integration and custody | [Testkit ledger](releases/testkit/CHANGELOG.md) |
| Testkit Wire | Native typed calls, independently aligned with Wire | [Wire adapter ledger](releases/testkit-wire/CHANGELOG.md) |

The retired allocation package retains its [Native history](releases/native/CHANGELOG.md);
it has no active release selector. Each active group owns its versions and notes. See
[release ownership](docs/design/release-groups.md) for commands and boundaries.

[Joint integration history through 0.76.1](releases/integration/CHANGELOG.md) and
[coordinated history through 0.73.0](releases/history.md) retain their original scope.
