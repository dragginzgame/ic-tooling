# IC Tooling

This repository adopts the installed [Toko Tooling baseline](TOKO-TOOLING.md).
Read it, [README.md](README.md), [the current design](docs/DESIGN.md) and
[the current handoff](docs/status/current.md) before work.

- The maintainer accepted revision 2 of `docs/DESIGN.md` on 2026-11-15 and
  authorized restarting implementation. It replaces the preservation-first
  design: derive interfaces from required workflows, not existing crates.
- Start with no admitted old exports. Do not bulk-import crates, source trees,
  test trees or workspace configuration. Define the new owner and contract
  before evaluating a coherent unit for reuse; record the decision and evidence.
- Preserve required outcomes, persisted bytes, identities and independent
  failure guarantees. Rust signatures, helper names, crate boundaries and old
  test structure are not compatibility requirements. Unknown required workloads
  block their migration, not independent implementation slices.
- This checkout is the current implementation, not its own legacy reference.
  Use the exact historical source identities in `docs/status/current.md` for
  read-only predecessor research; do not guess a sibling path or restore old
  architecture to satisfy obsolete commands. Preserve unrelated pending work.
- The current handoff owns stage completion and historical migration evidence.
  Every increment must name its concrete caller obligation or removed mechanism
  and focused acceptance evidence.
- The existing holds on Tooling-owned Auth, Storage, Observability and Wire
  migration work remain in force. They do not declare those repositories globally
  frozen or override their own maintainer-authorized work. Preserve completed
  evidence and do not expand these migration lanes without authorization.
- Qualify against current local sibling working trees, including tracked edits
  and non-ignored new source, using read-only snapshots under `target/`. Record
  source identity, refuse changes during capture and stale migration patches;
  never fall back to historical commits. Keep consumer implementation in its
  owning repository; local patches are qualification adapters, not product code.
  Canic is being rewritten: current requirements, not its older implementation,
  determine whether further Tooling capabilities are needed.
- Scheduling qualification targets the rewrite now at `../ic-scheduling` (formerly
  `../ic-scheduling-new`); the runner verifies its qualified rewrite ancestry.
  Legacy source is excluded; installed checks require explicit prepared tool paths.
  Preserve its Testkit client/held-reply-fixture boundary; dependency removal alone
  is not an architecture improvement. Old Scheduling experiments are historical.
- Product packages belong under `crates/`; exclude `qualification/` from the
  root workspace. Separate qualification packages get their own workspace when
  introduced. Packaging follows demonstrated ownership and dependency isolation.
- Existing crates.io names may be reused when the new owner fits. Increment
  versions from the published line; incompatible 0.x APIs require a new minor.
  No legacy API compatibility layer. Package versions belong in Cargo metadata
  and `make deps-list`, not this overlay.
- Build owns Cargo execution and owned artifacts; FS owns held custody; HTTP owns
  bounded bodies, with Reqwest/TLS policy in callers. Keep HTTP outside Build and
  portable bytes. Text owns presentation primitives, not output policy. Process
  owns execution/bounded observation; caller decoding and terminal policy stay local.
- Read the affected contracts linked from [the current handoff](docs/status/current.md).
  Keep archive qualification and release editing outside Build. Testkit preserves
  original PocketIC types and bounded owned reply fixtures. Package or test counts
  do not establish migration; source-bound qualification stays with its owner.
- Release ownership is declared in `ci/cargo-release-groups.json`; group ledgers
  live under `releases/`. See `docs/release-ownership.md`. Root changelog is an index;
  qualification fixtures remain outside product groups.
- Publication status and verification evidence live in `docs/release-ownership.md`.
  Production consumer migration remains separate delivery work.
- Keep product documentation under `docs/`. Do not claim performance improvements
  without matched measurements, or present old test results as new qualification.

No policy exceptions are introduced by the restart.
