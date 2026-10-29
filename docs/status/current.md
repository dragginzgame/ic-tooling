# Current handoff — 2026-10-29

## IC Platform 0.8.1 adoption

The requested dependency adoption is complete locally and unstaged over
`645ae21d` (Release 0.69.14). Incoming worktree/index were clean, with no active
Cargo/rustc process visible. Workspace package versions remain 0.69.14; no
staging, commit, push, release, publication, tool install or sibling edit ran.
The Shared snapshot remains 0.27.1 at
`d08cbcafbaff4227e798a890232e2248bf167504`.

The root requires published IC Platform 0.8.1 with defaults disabled; its macro
package resolves to 0.8.1. Registry search and package metadata verified the
published version and Rust 1.91 floor. Existing registry Wire, PocketIC 16.1.0,
management, executor and ic0 versions remain selected. No unrelated package
version was updated. The lockfile drops seven obsolete packages: base32,
icrc-cbor, icrc-ledger-types, libm, minicbor, minicbor-derive and syn 1.x.

Pending notes are **0.70.0**, because the public Tools ledger value changes from
registry ICRC types to Platform-owned types. Consumers constructing LedgerBlock
or using verified values must select Platform 0.8.1's Value. Consumers exchanging
owned Registry principals likewise need the same Platform 0.8 identity. Candid/
Serde representations and ledger hashes retain the supplying contract; no data
rewrite or uncertain-operation replay is required. No compatibility bridge was
added. Released 0.69 history is unchanged; package versions were not bumped.

Registry keeps the minimal owned-principal feature. Tests validate its bytes and
intervals without importing external conversions. Docs identify the optional
principal-external feature for SDK boundaries; graph policy rejects external
Principal and SHA-2 in the registry-only profile. Ledger/all-feature graphs now
reject the retired registry ICRC/CBOR packages instead of requiring them.

Testkit explicitly selects principal-external to preserve its public external
Principal identity, including fake_principal on Wasm. Its existing native
PocketIC/Candid/management selections remain; Wasm remains Candid-free. The probe
retains its CDK/raw runtime, and stable-memory adapters retain their backing API.
Examples name the new Platform release. Native observation APIs from PR #265
are separate and are not included by this dependency update.

## Validation and evidence

Evidence: `/tmp/ic-tooling-platform-081.DQsDFy/`. Admission source/lock/handoff,
exact check commands, logs, final patch/source and identity records are retained.
Warning-denied all-target/all-feature Clippy passes for the five direct Platform
consumer packages on prepared Rust 1.99.1, before focused tests.

Actual Rust 1.91 tests pass: 12 cases comprising isolated Registry (3), ledger
evidence (4), Testkit deterministic Principal (1) and native stable memory (4).
Rust 1.91 Wasm checks pass for Testkit/Native selected fixtures and the private
probe. Registry-only strict Rustdoc passes without external-conversion features.
Tools' actual native Linux profile graph script and selected Testkit native/Wasm
normal/build graphs pass; Wasm explicitly retains ic_principal/ic0 and excludes
Candid, PocketIC, Wire, the executor, macros and Host packages. ShellCheck, shell
syntax, formatting, local doc links and diff checks pass.

No full local workspace or release gate, live PocketIC run or native macOS
qualification ran. Native macOS 15 Intel/Apple Silicon evidence is separate;
Linux and Wasm compilation do not establish it. No Wasm-size saving is claimed.

## Preserved obligations and prior work

[PR #265 — Own raw native observations and workload mechanics in Host](https://github.com/dragginzgame/ic-tooling/pull/265)
was repaired and pushed separately at efb49570. Its repair evidence remains at
`/tmp/ic-tooling-265.xBrNN0/`; original Host/consumer evidence remains under
`/tmp/profiler-host-ownership-20261029`. Its native CI and supplier acceptance keep
their own source identities; this adoption neither merges nor retargets that PR.

[Testkit snapshot consumer acceptance (#263)](https://github.com/dragginzgame/ic-tooling/issues/263)
retains scope at `/tmp/ic-tooling-263.WQ4ch3/`. This batch establishes neither
Canic adoption nor its retained four-case sequence qualification.

Earlier cleanup evidence remains at `/tmp/ic-tooling-cleanup-06914.hJOCPI/`,
`/tmp/ic-tooling-cleanup-06913.uEdXff/`, `/tmp/ic-tooling-cleanup-06912.cqgTkd/`,
`/tmp/ic-tooling-cleanup-06911.Pznlfz/` and `/tmp/ic-tooling-cleanup-06910.ZwEZDQ/`.
Preparation race/package failures, migration/retirement backups under
`/tmp/ic-tooling-migration/`, original Host archive refs, stale release-lock
backup `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`, release plans and stash
evidence remain preserved. Earlier supplying-owner obligations retain their
recorded scope. No artifact cleanup ran beyond existing owned fixture behavior.
