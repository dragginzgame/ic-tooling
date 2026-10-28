# Current handoff — 2026-10-28

## Source and authorized scope

The upstream Cargo dependency adoption is local and unstaged over released
0.69.3, HEAD `e3a71a922b1604f53b3e5a1530e154aec96e2a81`. Pending notes select
0.69.4 for compatible dependency adoption. Workspace package versions remain
0.69.3. No commit, push, tag, release or publication ran.

The incoming working tree contained a Cargo.lock edit selecting Platform and its
macro package at 0.7.1 and unicode-width at 0.2.3. Preserve it: the final lockfile
is byte-for-byte identical to that incoming file. Wire and its derive were
already locked at 0.7.18 in released 0.69.3. The real index remains unchanged.

## Dependency selection and review

The root catalog now declares compatible minimums Platform 0.7.1, Wire 0.7.18 and
unicode-width 0.2.3. Child manifests still inherit from that catalog. No feature,
default-feature, dependency-role, tool pin, source implementation or package
identity changes. Cargo's registry-backed Rust 1.91 update preview reports no
further compatible lockfile changes; it was a dry run, not a lock regeneration.

Registry source identities and published code were inspected: Platform 0.7.1
records `5862651a57de3fba6c03cc5d7c502aaca3cdebcf`, and Wire 0.7.18 records
`c748379325266d7fa614fefee9d745eb4a88acca`. Platform's Principal/Account text,
serialization and error simplification retains its contracts and established
cryptographic implementation. Its optional native gzip capability is not enabled
here. Wire 0.7.18 has no runtime/protocol change from 0.7.17. Uncommitted sibling
work was inspected only as context; dependency bytes came from the registry.

Shared Tooling remains the exact 144-file snapshot at
`d08cbcafbaff4227e798a890232e2248bf167504` (0.27.1 plus the prior fixture fix).
This task changes Cargo dependency declarations, not that snapshot or executable
setup. Sibling working trees and their dirty source/lock/artifacts were preserved.

## Validation and evidence

Evidence is under `/tmp/ic-tooling-deps-0694.GN7ALu/`: starting HEAD/archive,
incoming patch and exact manifest/lock backups, registry-update previews, selected
metadata, qualification logs and final changed-source capture. Direct crates.io
API access was unavailable; version selection used Cargo's refreshed registry
and the cached published source identities. No host-tool setup/download ran.

Actual Rust 1.91 Linux evidence passes for text rendering, Candid service
integration, native stable-memory access and Wire call admission. The focused
results are 34 Text tests, 11 Candid tests, four Native tests and nine Wire tests;
the selected registry/Principal consumer result is recorded in `registry.log`.
The locked/offline Wasm compile passes for Testkit with Wire and Native with
stable-memory selected. Existing dependency ownership/inheritance and native/Wasm
Testkit feature-graph checks pass, retaining the lightweight default boundaries.

Two live prepared PocketIC 16.1 Wire cases pass: codec refusal/error identity and
structured query/update rejections. The native run used loopback access without
provisioning a server. Four-package all-target/all-feature warning-denied Clippy
passes for Text, Tools, Testkit and Native. Formatting, local documentation links
and final diff checks are recorded beside those logs.

These are focused adoption checks, not a full workspace/release gate. Native
macOS Intel and Apple Silicon qualification did not run for this dependency set;
report it separately from Linux and static graph evidence. No release or consumer
migration is claimed by local validation.

## Prior work and retained obligations

The [0.69.3 handoff](https://github.com/dragginzgame/ic-tooling/blob/e3a71a922b1604f53b3e5a1530e154aec96e2a81/docs/status/current.md)
retains the previous cleanup scope and evidence. Its source/logs remain under
`/tmp/ic-tooling-cleanup-0693.JiPDSR/`, with older cleanup and Shared contribution
records linked there. All prior public APIs, cache/fingerprint contracts and
capability owners remain intact.

This adoption supplies no crypto correction or advisory exemption. Historical
[Tooling #240](https://github.com/dragginzgame/ic-tooling/issues/240) and supplying
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) retain graph
and qualification evidence. Existing review/qualification obligations remain.

Migration/retirement archives remain under `/tmp/ic-tooling-migration/`. Original
Host refs, stale-lock evidence at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`,
release plans, failed attempts and earlier backup/stash evidence remain preserved.
