# Replacement source baseline

Implementation started on 2026-11-15 using the maintainer-selected local
`../ic-tooling`, clean at `a00369fc29fa837c6deb022cceb7e09aa447292c`.
This is the same source commit reviewed by DESIGN.md; the historical handoff
contains earlier pending-state descriptions and does not override that identity.

Selected source was exported from that commit through Git, without Git history,
managed tools, build outputs, release receipts or local untracked files:

| Source | Disposition |
| --- | --- |
| `crates/ic-host-artifacts` | Retained implementation, feature surface and tests, version 0.79.0 |
| `crates/ic-host-fs` | Retained implementation, feature surface and tests, version 0.79.0 |
| `crates/ic-host-process` | Retained implementation including observations and tests, version 0.79.0 |
| `crates/ic-build` | Retained implementation and tests, version 0.86.0; streaming selection added locally |
| `Cargo.toml` | Reduced to these four members and their used dependency declarations; shared metadata/lints retained |
| `Cargo.lock` | Seeded from source, pruned by Cargo offline to the retained graph; existing registry versions/checksums retained |
| `rust-toolchain.toml` | Retained Rust 1.100.0 compiler and components |
| `ci/extraction-sources.json` | Retained historical source provenance; entries describe the original extraction, not new inclusion decisions |

All four local manifests set `publish = false`. Build is now prepared as 0.86.1;
Foundation retains 0.79.0. Repository metadata identifies the maintainer-authorized
GitHub destination `dragginzgame/ic-tooling-new`; it does not grant crates.io
publication authority. No public export has been removed. The first contract
change is additive: `CdylibWasmSelector`, documented in the
[consumer guide](../build/streaming-selection.md). The old selection entrypoint
shares its core; it is not a second production implementation.

Toko 0.43.2 was admitted from the locally cached crates.io archive against the
Cargo.lock checksum, through the supplier exporter with prior-Git-snapshot
ownership proof. Its canonical baseline, root documents, Make interface and hook
are installed with the exporter-owned receipt. No vendor file was hand-edited.

Observed consumer: local Scheduling at clean commit `0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`'s
`qualification/ic-timers-pocketic/src/build/mod.rs` buffers messages, checks
execution/protocol/facts, selects exact Wasm and captures under a target lock.
The new local example and collision fixture implement streaming at that boundary.
The [Scheduling recipe](../../qualification/scheduling/README.md) now exports
that same pinned consumer, applies the reviewed callback migration and runs its
12 existing host build/report tests against the replacement graph. All pass on
Linux; no consumer test bodies were copied into a maintained local test package.
Scheduling installed-runtime acceptance, Backup and Profiler acceptance, archive
qualification and native macOS qualification remain subsequent work. No sibling
was changed.
