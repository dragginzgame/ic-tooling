# Foundation changelog

This group covers Artifacts, FS, HTTP, Process, Text, Build and the Wasm inspector.
Build owns the group version. Earlier releases retain their coordinated scope in
[shared history](../history.md#0730---2026-10-31).

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
