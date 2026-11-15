# IC Tooling New

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
- `../ic-tooling` remains the read-only production reference. Historical evidence
  is identified in `docs/status/current.md`; do not restore the old architecture
  to satisfy obsolete commands or instructions. Preserve unrelated pending work.
- The Stage 1 architecture checkpoint is complete; Stage 2 introduces shared byte
  admission and filesystem custody for the pinned Backup qualification. Continue
  through the design's staged workflows. Every increment must name a removed caller obligation
  or mechanism as well as its focused acceptance evidence.
- Product packages belong under `crates/`; exclude `qualification/` from the
  root workspace. Separate qualification packages get their own workspace when
  introduced. Packaging follows demonstrated ownership and dependency isolation.
- Existing crates.io names may be reused when the new owner fits. Increment
  versions from the published line; incompatible 0.x APIs require a new minor.
  The build owner uses `ic-build` 0.87.0; byte and filesystem owners use
  `ic-host-artifacts` and `ic-host-fs` 0.80.0. No legacy API compatibility layer.
- Keep all replacement packages unpublished. Production consumer migration,
  registry publication and canonical cutover are separate delivery work.
- Keep product documentation under `docs/`. Do not claim performance improvements
  without matched measurements, or present old test results as new qualification.

No policy exceptions are introduced by the restart.
