# Current handoff — 2026-11-03

## Local cleanup and client ownership transfer

The ordinary checkout contains uncommitted implementation of the accepted audit
batch: snapshot-transfer convergence, narrow fixture-retention checks, current
extraction documentation, #324's client transfer, and Build's prepared-input owner.
No Tooling commits, pushes, package version changes, tags or publication were
performed. The approved Shared contribution is committed separately below.

Testkit now owns the maintained native client under `ic_testkit::pocket_ic`;
`pic` retains lifecycle, sender, funding and rollback policy. All local callers
use the new owner, and the separate client dependency is removed. The pending
Testkit ledger is 0.84.0; manifests remain 0.83.0. See the
[consumer migration](../testkit/client-migration.md). Snapshot copying now uses
that client's bounded transfer engine. Known upload IDs enter the pending set
before data copying, with no implicit replay or deletion in the client.

The transferred source comes from the existing isolated Tooling contribution
`69f1be9af8080374393d7c65d65916175a01ec36`, recorded by Platform #139. Its donor
is Platform `41688ced81066b90aee756def7468a8388ee67e9`, including the qualified
macOS mock-socket fix. Source, tests, licenses, reconstruction patches and hashes
are now preserved in this ordinary checkout; they no longer exist only beneath
Platform's `target/`. The original isolated contribution and full bundles remain
at the locations recorded by Platform's handoff. Platform's retirement edits are
separate, dirty sibling work and were not changed here.

The boundary blocker is resolved by the approved Shared contribution at
`/home/adam/projects/shared-tooling-contributions/testkit-pocketic-owner`, branch
`fix/testkit-pocketic-owner`, commit `bd193bd407c01f3fb95c8302e45cbdde31609f4b`.
It is based on Shared main `0025cd4`, preserves the UTF-8 fix, and admits only
Testkit's four normal native IC SDK inputs. Portable/Wasm, build/dev, unrelated
inputs, retired-client ownership and consumer policy relaxation remain refused.
Shared's pending entry is 0.29.0 because the retired-owner exception is removed;
no Shared release or tag was made. The contribution is local, not pushed or in a PR.
Tooling adopted the committed source through the exporter. All 162 installed
payloads match that independently selected source, and the real workspace's
external IC dependency-boundary check now passes.

Focused Linux checks pass: 15 client failure/transfer tests; management identity,
controller snapshots, rollback, funding, timer and hook restoration against the
installed PocketIC 16.1 server; source reconstruction and mutation guards; native
Testkit/Dev Clippy and native/Wasm dependency profiles; Wasm Testkit Clippy;
Build's 13 batch cases, concurrent/saturating reader count and three real prepared
Wasm integration cases; packaged native client imports; CI dispatch, fixture
retention, script lint, manifest formatting and documentation links. Product gate
effects are substituted in CI routing fixtures. Full release qualification and
new native macOS qualification of this batch were not run.

Evidence and incoming lock backup are in `target/cleanup-pass/`. Incoming registry
selections remain unchanged except removal of the retired client package;
Testkit's local lock dependencies now select the transferred implementation.
Failed sandbox loopback tests and initial command refusals remain with corrected
successful runs. The original boundary refusal is retained alongside the
successful adopted check. Supplier native-client and UTF-8 fixtures, ShellCheck,
pending-version and documentation checks pass; the supplier's full suite was not run.
Fixture suites now have explicit `tooling` roster entries; relative-path retention
probes exercise their actual initialization/EXIT boundaries without rerunning bodies.

## Delivered baseline

[PR #325](https://github.com/dragginzgame/ic-tooling/pull/325) merged CI roster and
release-proof consolidation at `aed47efc72ae0fe31b1329ad675e0a97b31f63ca`.
All 15 head checks passed, including native Linux and macOS 15 Intel/Apple Silicon.
Its head was `1706830496efa777423f0abae03a950660351fd2`; those results do not
qualify the new local client/layout changes above.

The installed Shared snapshot is pending 0.29.0 source
`bd193bd407c01f3fb95c8302e45cbdde31609f4b`, with 162 managed files. This follows
the merged #325 snapshot at `150c304d70381f15601ea5114a4aea7bfa09bc60`.
[Shared PR #350](https://github.com/dragginzgame/shared-tooling/pull/350) has merged;
its UTF-8 fix is retained in the combined committed contribution above.
Shared owns plan/event/timing proof. Tooling version-4 receipts seal live proof
before preparation and retain a complete frozen inventory for prepared/committed
recovery; historical receipts keep their original adapters. Full CI and release
selection share the admitted roster, with focused comparisons only on explicit
qualification dispatches. Shared-input changes retain full release fallback.

[PR #322](https://github.com/dragginzgame/ic-tooling/pull/322) delivered Platform
and its independent client 0.20 at `e0921d8e`. Tools 0.80.0 and Testkit 0.83.0 are
published; Runtime 0.19, Types 0.18, Wire/Core/derive 0.16 and server 16.1.0 remain.
Private Dev/probe packages follow Testkit and are not published.

| Group | Published version | Release source |
|---|---|---|
| Foundation | 0.74.2 | `aa62709bff680ac7c8a050c88cff769c818eebf2` |
| Build | 0.74.1 | `aeff79aa1b6d3e6466b7f990b679db9379215466` |
| Text | 0.74.1 | `df9d9cdffe8d56fb82c3132a4860d2567d6f01a7` |
| Tools | 0.80.0 | `109244b35678e9d89ecec6963ab535c8d65b2f25` |
| Testkit | 0.83.0 | `4b5f2408b3c1325192c40eade4183ffa1d583aaf` |

Native remains 0.80.0. Build's 0.74.2 notes remain pending. The
[group ledgers](../../CHANGELOG.md) retain earlier release history.

## Qualification and custody retained from delivery

Tools/Testkit passed their complete 35-obligation source-bound Linux release
gates, publication dry runs, uploads, independent archive/checksum/VCS readbacks
and registry-only public contracts. The first Testkit dry run failed on disk
exhaustion before upload; retry passed on unchanged source after temporary cache
relocation. The cache is restored; nothing was implicitly deleted.
The final explicit release qualification includes
[CI 41631091012](https://github.com/dragginzgame/ic-tooling/actions/runs/41631091012).
Do not substitute the later #325 head results for an exact release-source proof.

Release logs and backups remain under `/tmp/ic-tooling-platform020-deploy.gNM3md/`,
with durable receipts in `.git/release-state/deployment-platform020-20261103/`.
The isolated release checkout/artifacts remain at
`/tmp/ic-tooling-platform-deploy.BGZn8d/release/`; adoption evidence remains at
`/tmp/ic-tooling-platform020-adoption.jRLpyl/`. Earlier Wire 0.82 custody remains
under `/tmp/ic-tooling-testkit082-deploy.DsxSsd/` and its durable release-state copy;
earlier Platform evidence remains at `/tmp/ic-tooling-platform-deploy.BGZn8d/`.

#314 closed after complete Linux/macOS Intel/ARM comparison proof at `18e3eba9`
in [CI 41621801740](https://github.com/dragginzgame/ic-tooling/actions/runs/41621801740).
Checker/artifact evidence remains under `/tmp/ic-tooling-314-dispatch.JnxkDE/closure/`
and its durable issue release-state copy. Earlier #313 three-host evidence belongs
to `fc66f67b` and [CI 41521867036](https://github.com/dragginzgame/ic-tooling/actions/runs/41521867036).
Neither baseline qualifies this new client transfer.
