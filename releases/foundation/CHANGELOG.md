# Foundation changelog

This group covers Artifacts, FS, HTTP, Process, Text, Build and the Wasm inspector.
Build owns the group version. Earlier releases retain their coordinated scope in
[shared history](../history.md#0730---2026-10-31).

## [0.73.6] - Unreleased

### ➕ Added

- Build supports caller-qualified target cfg recipes after Cargo resolves flags,
  preserving ordinary/config/encoded selections and refusing conflicts unless
  an explicit qualified override is selected. The optional CLI wrapper records
  actual compiler arguments/identities in the existing exact cache; isolated
  compiler state prevents stale shared objects from claiming new settings.
  Separate post-link cache/tool evidence and installed Platform ECDSA vectors
  qualify the final Wasm. No consumer tool pin or deployment profile changes.
  ([#300](https://github.com/dragginzgame/ic-tooling/issues/300))

### 🔧 Changed

- Keep Foundation release ownership while splitting Host Tools and Testkit into
  independent groups. Preserve existing versions, requirements and historical
  release custody. ([#299](https://github.com/dragginzgame/ic-tooling/issues/299))

## [0.73.5] - 2026-11-01

### ➕ Added

- Build can qualify already-produced package archives with caller-selected
  checks over the admitted extracted graph. Preserve frozen external selections,
  bounded source traversal and final source/archive/graph custody. The existing
  convenience API retains packaging and its all-target/doctest roster; this new
  boundary does not grant release/publication authority or select consumer gates.

## [0.73.4] - 2026-11-01

### ⬆️ Dependencies

- Replace the deprecated YAML parser with maintained `yaml_serde` and
  `libyaml-rs` behind Host Text's existing optional API. Preserve byte admission,
  duplicate-key refusal and explicit tag/merge policies; remove `serde_yaml` and
  `unsafe-libyaml` from the selected graph.
  ([#285](https://github.com/dragginzgame/ic-tooling/issues/285))

### 🧪 Testing

- Select document-only validation for explicitly admitted prose changes while
  retaining stable CI job names, lint/security, fail-closed classification and
  full native/MSRV qualification for code. Release gates remain complete.
  ([#287](https://github.com/dragginzgame/ic-tooling/issues/287))
- Explicitly refuse failed classification on Bash 3.2; qualify the guards without
  relying on shell `errexit` behavior.

## [0.73.3] - 2026-10-31

### ➕ Added

- Build exposes package-command capture with bounded output and independent
  retention of both logs, preserving execution and log-write failures together.

### 🧹 Cleanup

- Keep the acquired Wasm cache target and its shared-lock/maintenance evidence
  together without changing cache validation or lock ordering.

## [0.73.2] - 2026-10-31

### 🔧 Changed

- Adopt Shared Tooling at `143a9ab` (upstream's 0.27.22 commit, declaring
  version 0.27.21). The root catalog now includes every member with its full
  version and local path, grouped by release ownership with ledger links.
  Admission checks coverage, paths and each group's complete alias roster,
  including unused and private entries. Preserve ordinary Cargo cleanup of the
  complete workspace target; durable release evidence stays outside it.

### 🧪 Testing

- Give version-probe overflow qualification time to start on loaded hosts while
  retaining the separate short-deadline check and exact failure evidence.
  ([#286](https://github.com/dragginzgame/ic-tooling/issues/286))

## [0.73.1] - 2026-10-31

### 🔧 Changed

- Release Foundation and IC integration independently through the existing
  Shared runner, preserving full workspace qualification. Maintainers now select
  `RELEASE_GROUP=foundation` or `RELEASE_GROUP=integration`; publication and archive
  qualification use each package's actual version.
  Selection diagnostics remain outside Build's initially empty qualification
  directory; evidence uploads retain logs and exclude its isolated Cargo cache.
  [Release ownership and migration](../../docs/design/release-groups.md)
  ([#276 — Reduce unrelated Host and Build version churn](https://github.com/dragginzgame/ic-tooling/issues/276)).

### 🧹 Cleanup

- Adopt Shared Tooling 0.27.19: give both groups explicit package versions and
  ledgers under `releases/`, with a root index and preserved shared history.
  Release/publication commands refuse missing group selections before effects;
  workspace-wide CI qualification retains one lint pass. Package numbers stay
  unchanged during adoption. The managed editor remains IC Build 0.73.0.

- Invoke the bundle-selection test's shell fixture through its interpreter,
  avoiding transient executable-write locks during parallel filesystem tests
  while preserving relative runtime and rejection checks.
