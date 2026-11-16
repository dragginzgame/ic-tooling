# Cargo release metadata rendering

[Tooling #8](https://github.com/dragginzgame/ic-tooling/issues/8) requires a maintained
successor to the displaced release editor. The product owner is `ic-cargo-metadata`,
a narrow native command under `crates/`, outside Build and its dependency graph.
Toko continues to own group policy, file transactions, rollback, source admission,
Cargo graph verification and delivery. The renderer never writes its inputs or
runs Cargo. Its independent release group follows this installable tool boundary;
the private external-lock qualification utility retains its existing owner.

## Required command contract

The command requires a positive `--max-input-bytes N` for each source/context
document. `group-manifest SOURCE PREVIOUS NEXT SELECTION_JSON` edits only the
explicit package/workspace version and workspace dependency projections in the
selection. `manifest SOURCE PREVIOUS NEXT [WORKSPACE_INPUT_JSON]` renders the
coordinated workspace version and explicitly bound local-member requirements.
That context contains current `manifest` and `members` projections; saved-root
contexts are not needed by the inspected Toko callers and are not admitted.
Simple local requirements retain their operator/precision; compound ranges and
same-name dependencies outside the admitted member paths remain byte-identical.
Toko's independent Cargo check admits or refuses those unchanged ranges.
`lock SOURCE PREVIOUS NEXT PACKAGE...` admits lock formats 3/4 and edits selected
source-less package versions and their unambiguous version-only dependency
references. Bare references and source-qualified references remain unchanged.
Unselected bytes, comments, ordering,
registry identities/checksums and requirement operators are preserved.

Admission rejects malformed documents, stale projections, duplicate identities,
ambiguous lock references and unsupported version forms. All source inputs are
bounded UTF-8 regular files, with final symlinks refused through FS custody.
Only a completely validated candidate reaches stdout; diagnostics use escaped
stderr, with status 1 on refusal and status 2 for argument syntax errors.
A stdout write failure can have partial
output; Toko owns candidate retention and never installs failed output. Parent
namespace stability and exclusion of concurrent writers remain caller obligations.
This is the current Toko protocol, not a restoration of the old Build facade.

## Reuse admission and acceptance scope

After defining this owner/contract, the old `ic-build` 0.73.0 renderer was reviewed
as one coherent parsing/span-edit unit. Its TOML literal-span editing strategy was
admitted into the private renderer; the old crate, public projection framework,
test tree, Cargo operations and CLI feature graph were excluded. New input
admission uses FS/Artifacts and diagnostics use Text. The reviewed archive SHA-256
is `0ce0bb6da2f6bb84feae678564426ae3cf653b52f9494bac35dd38fac80ef21d`,
with VCS identity `5b658971c66f82e124b3efcbdfbc2efc77d8752c`.
Focused document and command tests cover exact untouched bytes,
stale/duplicate/bounded input refusal and ambiguous external/local identities.
This removes the caller's dependency on the old Build CLI feature graph; Toko's
existing transaction and independent Cargo admission mechanisms remain its own.

Current read-only Scheduling capture: HEAD
`df5d14a482973e427ea79cae022bd38971b39fa7`, tar SHA-256
`35181357d57eb68d42af60567d795bc08c958930efe99d26c8b56ace8e0376c2`.
Current Toko capture: HEAD `08d63df7661910554ef72980037f55edd914aba0`, tar SHA-256
`8047f6a37f0b2946d42a82ebf651b81ee1648c7e9fbf791c6f92335a2587220f`.
Both include current tracked edits and non-ignored files, with repeated capture
agreement. The runner verifies Scheduling's captured rewrite ancestry and refuses
stale binding patches. Scheduling's qualification adapter only selects the new
executable; no sibling source or consumer assertion is changed.

Linux acceptance uses the extracted `ic-cargo-metadata` 0.1.0 archive with SHA-256
`2788d23b9c4b0dbf0890511b3a7d8edb954a3a36a70a2652fa04231b06edd9fd`.
Selected Clippy, document/command tests, package/source admission and packaged
tests/documentation pass. Evidence is under `target/issue-8/` and
`target/library-release.iyvyM9xy/`.
Toko's coordinated and multi-lock release fixtures pass, including rollback,
original failure status and unrelated-byte preservation. Scheduling's actual
Timers and Jobs release transactions pass, including qualification lock
synchronization, invalid public selections, standard increments, unselected
preservation and downgrade refusal. Consumer evidence is under
`target/cargo-metadata-consumer.fOyo9LeP/`; Scheduling's log is
`target/scheduling-unrestricted.log`. Its phase was rerun with subprocess access
after the sandbox refused Node spawning Make; the passing Toko phases were reused
with the same captured sources and unchanged archive executable.
The first consumer run caught compound requirements being refused; the corrected
renderer preserves those ranges and outside-member paths, with a focused command
regression and successful current-consumer checks.

Native macOS archive checks are wired into CI; no result for this release is recorded here.
They remain asynchronous under maintainer direction. Publication and registry
installation are verified below; Tooling #8 is closed.

## Publication and installed acceptance

Version 0.1.0 is published from `de86fe21dc746eda79e662008f0d670f20970ac7`, tag
`cargo-metadata-v0.1.0`. The non-yanked registry archive matches clean-source
publication admission and upload bytes, SHA-256
`5430d9278184df2c90d096712bd44b5dffa5c619dba550a646ee979c13400d0d`.
Toko's ordinary registry installer succeeded in release profile; its offline
`--check` admitted the exact executable, reporting `ic-cargo-metadata 0.1.0`.
That installed command passed the entire current-source consumer runner at
`target/cargo-metadata-consumer.vDBN9VkA/`, with the same source identities recorded
above. This covers Toko coordinated/multi-lock transactions and both Scheduling
groups, including rollback, status preservation, unrelated files, lock
synchronization and downgrade refusal. No consumer source or delivery was changed.
Installation and registry evidence is under `target/deploy-metadata-libproc/`.

## Local use and installation boundary

Build with `cargo build --locked --offline -p ic-cargo-metadata`. Run
`make test-cargo-metadata` for the document/command boundary and select an absolute
prepared executable through `CARGO_RELEASE_METADATA_EDITOR` for
`make test-cargo-metadata-consumer`. The latter captures current Toko and Scheduling
source, runs their selected release fixtures and performs no delivery effects.

For example, `group-manifest` accepts a selection file containing:

```json
{"version":"package","dependencies":[]}
```

```sh
target/debug/ic-cargo-metadata --max-input-bytes 8388608 \
  group-manifest Cargo.toml 0.1.0 0.1.1 selection.json > candidate.toml
```

The version selector is `package`, `workspace` or `unchanged`; dependency entries
contain `alias`, `previous` and `next`. Coordinated `manifest` context contains
`manifest` (the parsed TOML JSON) and `members` (each with `name` and an absolute
`manifest_path`). The renderer does not choose versions or replace source files.

In a consumer with Toko's Rust installer selected, prepare the published command
once (network access):

```sh
CARGO_RELEASE_METADATA_EDITOR="$(bash vendor/toko-tooling/scripts/dev/install-rust-tools.sh \
  --consumer "$PWD" --package ic-cargo-metadata --version 0.1.0 \
  --bin ic-cargo-metadata --profile release)"
export CARGO_RELEASE_METADATA_EDITOR
```

Normal release checks use the same installer selection with `--check` for offline
admission of its absolute path. No features or legacy executable alias are needed.
Consumers retain their own version policy, candidate transaction and Cargo checks.
