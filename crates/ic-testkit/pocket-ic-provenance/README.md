# PocketIC client transfer

Tooling #324 / Platform #139 transfer the complete maintained client from
Platform commit `41688ced81066b90aee756def7468a8388ee67e9`, package source 0.20.0.
This includes the subsequent macOS blocking-socket test fix from `79f905f`.
The donor archive SHA-256 is
`bf15bce368afc3daf1383e8df0d40611d3915abe6308af84fa56cfe0f5ad79ca`.
The donor was qualified on Linux and macOS 15 Intel/ARM in Platform #137.
That historical evidence is not qualification of this new package layout.

`src/lib.rs` becomes `src/pocket_ic/mod.rs`; internal crate paths become module
paths. Rustfmt orders those imports. The module retains the donor's Clippy
policy for imported source by expecting Testkit's additional pedantic/nursery
lint groups only there. No client function bodies, protocol constants, tests,
cryptographic operations or retry semantics change.

`module-transfer.patch` records exactly that packaging adaptation and has its
own checksum. `transfer-source.sha256` binds every donor source, license and
provenance file. The source guard reverses the module patch, checks all donor
hashes, then runs the unchanged Platform upstream reconstruction checks against
the reconstructed source. Existing upstream hashes and local fallible/snapshot
patches are retained, not refreshed. The mutation fixture rejects altered bodies,
imports, patches and licenses. The original notices remain under `upstream/`;
their Platform paths and release references describe the donor history.

The imported source remains Apache-2.0; see LICENSE and upstream/LICENSE.
Testkit's package license is MIT AND Apache-2.0. The source remains maintained in
one Testkit module, without a second wrapper package or separate release group.
