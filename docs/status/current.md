# Current handoff — 2026-11-02

## Cleanup deployment

The accepted cleanup is integrated for sequential patch delivery:
Foundation 0.74.2, Build 0.74.1, Tools 0.78.1, then Testkit 0.80.2.
Text 0.74.0 and Native 0.80.0 are unchanged. Release commits/tags and registry
publication use separate standard commands. Hosted native CI runs asynchronously;
Linux release gates remain mandatory and do not establish native macOS results.

- Testkit uses the independent `ic-platform-pocket-ic` 0.18 owner directly,
  retaining the facade only for Candid/management adapters. Its private driver
  selects Types directly and keeps facade contract checks in dev-dependencies.
  Server 16.1.0 and public client/DTO identities remain unchanged.
- Registry-only Tools selects canonical Types, excluding the facade/Candid in
  both native and Wasm profile graph checks.
- Build gives progress and maintenance private module owners, shares input
  identity comparison and successful outcome counting, derives stamp bounds from
  its encoder, and uses declarative fake-Cargo output/hooks in tests.
- Testkit centralizes construction custody/admission while preserving async
  cancellation drop order, separates private benchmark preparation, shares WAT
  fixture mechanics and scans raw held requests without allocating every token.
- FS tree observers inherit the no-op entry callback; admission callbacks remain
  explicit. Batch hashing emits canonical digest bytes directly.
- The registry integration cohort generates its manifest and identity checks from
  one selection, preserving registry-only admission and the facade exception.

Inventory serialization and discovery error-source formatting were retained:
trial changes altered encoding order or public cause identity. Cargo protocol
convergence remains deferred because draining and error precedence differ.

Supplying-owner work remains [Platform #134](https://github.com/dragginzgame/ic-platform/issues/134)
(typed client failures) and [Platform #136](https://github.com/dragginzgame/ic-platform/issues/136)
(bounded snapshot transfer). No DFINITY/PocketIC issues or sibling source changes
were made. Testkit retains those adapters until owner capabilities exist.
The private checked-average exception remains in AGENTS.md, with
[Shared #347](https://github.com/dragginzgame/shared-tooling/issues/347) and
[Metrics #127](https://github.com/dragginzgame/ic-observability/issues/127).

## Evidence and preservation

Incoming source, untracked modules, full handoff and exact dirty lock are retained
under `/tmp/ic-tooling-cleanup-deploy.VLiO5k/`. Delivery uses the retained isolated
checkout `/tmp/ic-tooling-five-groups.iOEuyP/checkout`; the ordinary checkout's
unrelated registry patch selections stay outside the release source commit.
The delivery lock adds only the direct owner edges required by these changes.
Release plans, source-bound qualification logs and delivery/publication evidence
remain under the isolated checkout's `.git/release-state/` and `target/publish/`.
No automatic artifact cleanup is authorized.

Prior focused Linux evidence remains attributable to its original inputs:
`/tmp/ic-tooling-direct-pocket-ic.G9EWL6/`,
`/tmp/ic-tooling-complexity-cleanup.yXwlfj/` and
`/tmp/ic-tooling-small-cleanup.BVyBVe/`. These include native/Wasm graph and compile
checks, warning-denied Clippy, startup/held/cycle tests, public interoperability,
Build artifact and private driver checks, formatting and local document links.
They do not replace the full release gate or claim new native macOS qualification.
