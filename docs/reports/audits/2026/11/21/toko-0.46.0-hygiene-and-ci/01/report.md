# Toko 0.46.0 code hygiene and CI review

**Local scope:** IC Tooling on `main` at `3f8f577`, including the pending
`ic-build` Cargo network-mode fix. The previous [hygiene review](../../../19/toko-0.45.22-code-hygiene/01/report.md)
predates the handwritten type-header rule.

## Adoption and source

Both workspaces select Toko 0.46.0. The authenticated crates.io archive SHA-256
is `4601d2785aaa14e01fd8656d6b3a57653337b214e027810c953595f1d91d6a8d`.
All 52 snapshot payloads were admitted. Only the installed policy document
changed. Six workflow cache action pins select the archive’s source revision,
`b95958a0710fd9aea5952d2654f67710dc5b1c6d`. Neither workspace changed
another package selection during adoption. There are no local callers of the
removed release-version or producer fixture scripts. IC Tooling inspects
canister exports and produces test fixture modules; it owns no supported
public Wire canister API requiring a manifest baseline.

The [hygiene overlay](../../../../../../../audits/code-hygiene.md) covers product
and test Rust sources and the independent Wasm probe. Checked all 71 source
files for module ordering and verified 114 module-level struct and enum headers.
Moved function-local helper types next to the sections they serve, retained their
implementations, and separated reading, validation and execution steps in the
artifact inventory and build operation. Compared top-level help for the signer,
server, Cargo metadata and qualification commands before and after the edits.

## CI finding

The [last Intel library job](https://github.com/dragginzgame/ic-tooling/actions/runs/44749271521/job/136333461398)
explicitly reported a step timeout at 12 minutes. Its retained evidence shows
that package tests, Rustdoc, CLI preparation and CLI lifecycle checks succeeded.
Installed simulator qualification had started. Its completion time is unknown.
The Intel archive step now gets 20 minutes and its job gets 53, preserving the
existing reserve for evidence. The Linux and ARM budgets remain 12 and 45.
[Deadline evidence](../../../../../../../qualification/ci-setup.md#job-deadlines--2026-11-17)

The workflow uses a read-only content token, disables persisted checkout
credentials and pins its external actions. Cache saves are restricted to pushes
on the default branch. Existing failure uploads remain eligible after archive
failure. Actionlint accepts the Intel matrix expressions. The revised deadline
still needs a hosted Intel run. This review used the producer’s
[CI integrity method](https://github.com/dragginzgame/toko-tooling/blob/b95958a0710fd9aea5952d2654f67710dc5b1c6d/audits/ci-release-integrity.md).

## Linux verification

- Workspace Clippy (all features and targets), warning-denied Rustdoc and all-feature tests passed: 115 tests and two doctests, with nine opt-in cases skipped by the workspace command.
- The independent probe passed Clippy and its release Wasm build. Artifacts with method inventory, Text and HTTP passed their Wasm checks.
- Snapshot and lock admission, formatting, installed tools, dependency-pin admission, all 13 manifests’ lint settings, dependency usage and the 69 setup diagnostics checks passed.
- Four packaged native candidates passed in `target/build-release.Q1BQpf9Y/`. The Linux signer passed both locked and unlocked keyring checks.
- The packaged HTTP, Text, Testkit and metadata libraries passed in `target/library-release.gpRWmLNL/`, including three installed fixture checks, two managed simulator checks and final checksums. The first run detected a rebuilt input CLI at its closing checksum check. The successful rerun used a dedicated copy of the prepared CLI.

Local logs and retained hosted evidence are under `target/toko-046-adoption/`.
