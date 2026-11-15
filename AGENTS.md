# IC Tooling New

This repository adopts the installed [Toko Tooling baseline](TOKO-TOOLING.md).
Read it and [README.md](README.md) before work. The snapshot receipt identifies
the selected immutable revision; local product choices remain here.

- The maintainer authorized starting `docs/design/DESIGN.md` on 2026-11-15 to
  reduce technical debt and improve workflow and performance. Read
  `docs/status/current.md` for the implemented slice and validation boundaries.
  The latest local `../ic-tooling` is the source of truth for retained contracts.
- Preserve the existing design evidence. `../ic-tooling` remains the production
  owner and a read-only reference for work in this repository.
- Keep all replacement packages unpublished during development. Consumer
  migration and canonical publication cutover remain separate delivery work.
- Foundation and Build are retained with existing public contracts and tests.
  Keep product packages under `crates/`; `qualification/` is excluded and will
  become an independent workspace when its first real package is introduced.
- `make check` admits Cargo tooling selection and snapshot integrity.
  `make test-build-selection` runs the focused first-slice checks.
  `make test-scheduling-consumer` tests a pinned, migrated consumer export under
  `target/`; its patch and source contract live in `qualification/scheduling/`.
  Keep docs under
  `docs/`; reference shared policy through the installed baseline.

Preserve the selected source provenance and failure tests. Do not remove an
export based only on absent lexical matches or claim performance gains without
matched measurements. No policy exceptions are introduced by this slice.
