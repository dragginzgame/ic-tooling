# Foundation changelog

This group covers Artifacts, FS, HTTP, Process and the Wasm inspector.
Artifacts owns its version. Build and Text now have their own
[Build](../build/CHANGELOG.md) and [Text](../text/CHANGELOG.md) ledgers.
History through 0.73.6 below retains the original seven-package Foundation scope;
earlier coordinated releases remain in [shared history](../history.md#0730---2026-10-31).

## [0.76.5]

### 🔧 Changed

- Select Toko Tooling 0.39.10 for development and CI.

## [0.76.4] - 2026-11-10

### 🔧 Changed

- Adopt Toko Tooling 0.39.9 in both workspaces, including native PocketIC
  dev-dependency admission for simulator tests and corrected online guide links
  in retired release-setting diagnostics.

## [0.76.3] - 2026-11-10

### 🔧 Changed

- Adopt Toko Tooling 0.39.6 in both workspaces, retire managed reference copies
  in favour of commit-pinned upstream links, and enable the explicit shared-release
  interface. The installed checker now admits
  Testkit's approved native upstream PocketIC dependency.
- Share Linux process-stat field parsing across process samples and identity
  observations, preserving each observer's admission and counter semantics.
- Keep release fixtures and their reuse inventory independent of retired shared
  reference directories.

## [0.76.2] - 2026-11-09

### 🔧 Changed

- Adopt Toko Tooling 0.38.2 and its command/metadata naming contract. Use
  `make toko-tooling-check`, `toko-tooling-update` and `toko-tooling-install`;
  the baseline and receipt are now `TOKO-TOOLING.md` and
  `.toko-tooling.snapshot`. Update automation using the retired Shared names.
  Preserve the existing product release gates and package boundaries.

- Move private qualification packages into the independent `qualification/`
  workspace. Keep product release groups separate; atomically update qualification
  product requirements and its lockfile through Toko's shared release transaction.
  Formatting, source checks and validation cover both workspaces.
- Adopt SHA-2 0.11.1 for artifact hashes while preserving digest bytes and bounded
  streaming behavior. Serialization consumers use the existing HashingWriter.

### 🩹 Fixed

- Ignore two exact historical secret-scan false positives in generated Mermaid
  package identifiers. Other files, findings and future changes remain scanned.
- Retain failed PocketIC setup diagnostics in CI product evidence, excluding
  server binaries and partial download archives.

### 🧹 Cleanup

- Move the Wasm inspector into `crates/ic-host-wasm-inspect`, preserving its
  package, binary, Foundation membership and publication contract
  ([#348](https://github.com/dragginzgame/ic-tooling/issues/348)).

## [0.76.1] - 2026-11-08

### 🔧 Changed

- Select Toko Tooling 0.36 through one exact Cargo workspace pin inherited as
  a dev-dependency by every real package. Use Cargo.lock and the authenticated
  snapshot receipt; remove the separate tooling lock and metadata selection.
  Install explicitly after changing the Cargo selection.

## [0.76.0] - 2026-11-08

### 🔧 Changed

- Select toko-tooling 0.34 through root workspace metadata and a dedicated
  `toko-tooling.lock`, independent of Cargo dependencies. Remove the prior 0.33
  dev-dependency selection; preserve real qualification packages and release
  requirements. Direct publication/release adapters refuse tooling drift before
  effects.

### ⚠️ Breaking

- Remove the generic `wasm-encoding` feature and call-module API. Raw-reply
  fixture consumers use Testkit’s `reply-fixtures` API; structural inspection,
  full validation, custom-section encoding and WAT compilation remain available.

### 🧪 Testing

- Qualify checkout-local tooling commands and CI's early tooling-drift guard
  through the root-owned installer entrypoint.
- Admit unchanged release lint through recursive Make's repeated leading tool
  paths, with a real Make publication-admission regression and retained
  changed-environment refusal
  ([#345](https://github.com/dragginzgame/ic-tooling/issues/345)).
- Exercise cross-group release receipt rejection against an active group and
  remove an unused process fixture.

## [0.75.1] - 2026-11-08

### ➕ Added

- Add caller-bounded process-tree, boot/namespace identity and opened-inode flock
  observations extracted from Canic. Child budgets apply independently per thread;
  observations grant no process or lock custody
  ([#343 — Extract bounded process and lock observations](https://github.com/dragginzgame/ic-tooling/pull/343)).

### 🗑️ Removed

- Retire the unused `ic-testkit-native` allocation package and its active release,
  publication and qualification machinery. No API moved into PocketIC Testkit.
  Published versions and historical release/recovery records remain available.

### 🩹 Fixed

- Make the non-Linux kernel-lock observation stub const, satisfying the native
  macOS lint gate without changing its unsupported-observation result.

### 🔧 Tooling

- Adopt Shared Tooling 0.31.11, including authenticated hosted download caching,
  workspace-member formatting, hashing admission and scoped LOC reports. Cache
  direct CI-tool setup before tests; secret scanning retains one repository CI
  owner and stays outside release/publication gates. Adopt focused-check issue
  closure with explicit delivery status and issue-specific acceptance preserved.

## [0.75.0] - 2026-11-07

### ⚠️ Breaking

- Remove Host Process's unused Git-provenance and process-tree RSS sampling
  modules with the retired optional fixture benchmark workflow. Consumers retain
  their own provenance policy; general process execution, signals and child
  custody remain supported.

## [0.74.4] - 2026-11-05

### 🧪 Testing

- Place private Artifacts and Process test helpers under their owning `tests/`
  modules, preserving their effective visibility and test behavior.

## [0.74.3] - 2026-11-04

### 🧹 Cleanup

- Move FS, Process and inspector unit tests and private helpers into test-only
  modules. Preserve Cargo targets, test names, visibility and runtime behavior;
  source counts now distinguish these tests from library code.

## [0.74.2] - 2026-11-02

### 🩹 Fixed

- Bound the deadline-preservation socket test's final accept and verify its peer.
  Avoid assuming immediate nonblocking readiness on macOS; production socket
  behavior is unchanged.

### 🧹 Cleanup

- Let observation-only tree visitors use the default directory-entry callback,
  retaining explicit admission and budget callbacks where needed. Batch file
  hashing writes canonical digest bytes directly with unchanged output and
  all-or-nothing admission.

## [0.74.1] - 2026-11-02

### 🩹 Fixed

- Run descriptor-sensitive filesystem tests as separate libtest executables.
  Remove the FS-to-Process test dependency so Foundation packages can publish
  in dependency order, preserving descriptor isolation and panic-retained fixtures.

## [0.74.0] - 2026-11-02

### ⚠️ Breaking

- Require Rust 1.99.1 and pin contributor/release compilation to that version.
  Native qualification now owns the former compiler-lane feature and Wasm
  obligations; the stable MSRV status reuses its results. Older compiler support
  is retired; package release groups remain independent.
  [#313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313).

- Process owns isolated libtest execution. Replace FS's `testing::run_isolated`
  with `ic_host_process::testing::run_isolated` under its `test-support` feature,
  supplying caller-selected output limits and timeout. Children are captured and
  reaped with explicit process custody; FS's normal library graph stays unchanged.

### 🧹 Cleanup

- Use the standard integer bit-width operation for custom-section LEB length
  admission, preserving zero handling and exact size checks on Rust 1.99.1.

- Release Build and Text independently from Foundation, preserving package
  versions, dependency requirements and historical release custody.

- Move detailed Host API contracts into a linked reference; keep workspace
  ownership, setup and focused validation visible in the root README.

- Keep optional artifact-profile tests focused on their own behaviour and reuse
  release-group membership selection during publication admission. Public-package
  allowlisting and source, registry and recovery checks remain independent.

## [0.73.6] - 2026-11-01

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
