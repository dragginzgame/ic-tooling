# Current implementation — 2026-11-16

## Authority and scope

[Design revision 2](../DESIGN.md) is accepted. Required outcomes drive interfaces;
no old product source/test tree was admitted wholesale. Historical production
reference: `a00369fc29fa837c6deb022cceb7e09aa447292c`; superseded transplant/design
checkpoint: `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`. Preserve that evidence.
Canonical repository: `dragginzgame/ic-tooling`; predecessor: `ic-tooling-legacy`.
Historical checkout names are not current source selections.

Use [read-only current-source snapshots](../qualification/local-sources.md),
including sibling edits. Refuse capture drift and stale patches. Consumer product
code stays in its owner. Auth is on hold; Storage, Observability and Wire are
frozen. Backup adoption remains user-owned. Stay on `main`; delivery and hosted
CI are separate from ordinary local fixes, and CI runs asynchronously.

## Current local work and delivery

[Tooling #6](https://github.com/dragginzgame/ic-tooling/issues/6) supplies the
`managed-server` Testkit capability selected for the 0.107.3 release. The
[contract and acceptance](../testkit-server.md) record explicit pinned preparation,
offline admission, bounded Process-owned startup/output/lifetime, instance-held
server ownership, deletion attempts and abandonment evidence. Four refusal tests
and both real Linux installed tests pass, including gateway use, held replies,
independent deletion observation, cancellation, unwind and expiry. Clippy,
Rustdoc and feature isolation pass. No native macOS execution is claimed.
Canic's read-only current capture and required callers are recorded there;
[Canic #15](https://github.com/dragginzgame/canic/issues/15) owns actual harness
adoption, application/gateway/failure qualification and removal of 0.106.0 imports.
No sibling source or release was changed by this fix.

Testkit 0.107.2 is published from `74006ca`, tag `testkit-v0.107.2`; registry
readback matches the verified archive. It permits compatible Types 0.21.2 patches.
The [Tooling #5 release evidence](../qualification/testkit-types-compatibility.md) records
passing unit/installed/archive checks and registry-only Scheduling driver
resolution/Clippy with Runtime 0.23.13 and one guard. Current Timers source still
fails its Wasm build. Tooling #5's installed scenarios remain blocked in
[Scheduling #2](https://github.com/dragginzgame/ic-scheduling/issues/2); the supplier
version conflict is resolved. Do not relabel old Scheduling checks as new acceptance.

Artifacts/FS 0.80.2, Process 0.80.2, Build 0.87.3, HTTP 0.80.0 and Text 0.77.2
are published; [release receipts](../release-ownership.md) retain source identities,
archive checksums and native acceptance. `ic-cargo-qualification` remains private.
The maintainer authorized deployment after Toko adoption. Testkit 0.107.3 is the
selected compatible candidate for Tooling #6 and Protocol 0.2.6. Its complete
archive gate and Cargo publication dry run pass; [delivery evidence](../release-ownership.md#testkit-01073-delivery-preparation)
records exact source/metadata comparison and remaining publication. Other libraries
have no changed public source/requirements beyond the inactive Toko marker;
they need no republication. Private qualification changes deliver through Git.

Toko 0.44.11 is selected in both workspaces and installed through its verified
exporter (source `08d63df7661910554ef72980037f55edd914aba0`). The 49-file receipt,
package markers and locks agree; executable tooling is unchanged. Snapshot,
formatting, Actionlint and qualification-layout checks pass. Details and version
selection remain in Cargo metadata and [release ownership](../release-ownership.md).

The [Platform adoption inventory](../qualification/platform-adoption.md) now selects
Protocol 0.2.6 / Types 0.21.2 across the affected product graph; qualification and
native-tool dispositions are recorded there. The HTTP encoder is unchanged from
0.2.5; six unit tests, all-feature Clippy, metadata and feature admission pass.
This refresh is included in the 0.107.3 candidate. Earlier installed held-fixture checks
remain evidence for the unchanged encoder; 0.2.5 was published with Testkit 0.107.2.
Pending qualification/docs changes retire obsolete
Platform executable adapters and update Scheduling's supplier-only adapter.
Preserve those edits and the maintainer's release-ownership policy edits; they
are independent of Tooling #6. Historical frozen comparisons are not renewed migrations.

## Owners and evidence entrypoints

Use `make deps-list` for versions and [release ownership](../release-ownership.md)
for groups. Read the affected contract before changing an owner:

| Owner | Contract |
| --- | --- |
| Build | [Owned builds](../build-owned-artifact.md) |
| Artifacts | [Producer commitments](../byte-commitments.md) |
| FS | [Preparation](../directory-preparation.md), [publication](../directory-publication.md), [locks](../sidecar-locks.md), [retained custody](../filesystem-custody.md) |
| HTTP | [Bounded bodies](../http-body.md) |
| Text | [Presentation](../text-presentation.md) |
| Process | [Execution](../process-execution.md), [handoff/reaping](../process-handoff.md) |
| Testkit | [Held replies](../testkit-held-replies.md), [managed server](../testkit-server.md), [historical external recipe](../simulator-ownership.md) |

Build does not own archive running or release editing. HTTP/Tokio and simulator
dependencies stay outside Build and portable bytes. Testkit preserves original
PocketIC types; held fixtures and managed servers are separately optional.

## Consumer boundaries and remaining work

[Canic](../../qualification/canic/README.md#current-source-review--2026-11-16)
already owns held report-directory publication; its old report patch correctly
refuses current source. Historical paired build/metadata checks do not qualify
new application, Store-build, archive or native macOS outcomes. New simulator
requirements and the later source capture belong to the Tooling #6 contract above.
[Scheduling](../../qualification/scheduling/README.md) targets its rewrite with
verified ancestry and explicit tools; pre-rewrite experiments remain historical.
[Backup](../../qualification/backup/README.md) retains four captured assembly
passes and earlier restore baseline failures; its next adoption is user-owned.
[Query](../../qualification/query/README.md), [DB](../../qualification/db/README.md)
and [Platform](../../qualification/platform/README.md) retain scoped historical
evidence. [Auth](../../qualification/auth/README.md) remains on hold.

Tooling #3's [setup diagnostics](../simulator-ownership.md#qualification-setup-diagnostics)
repair retains bounded escaped stderr and original status, with controlled failure
checks. The earlier Intel download failure's underlying HTTP cause remains unknown;
no service repair or new hosted acceptance is claimed. [Store #3](https://github.com/dragginzgame/ic-wasm-store/issues/3)
owns its setup migration. Tooling #4 retired the executable pre-rewrite Scheduling
lane, preserving historical identities and checking refusal/current dispatch.

Multi-canister partial snapshot effects, remaining Platform drivers, Profiler
ownership/native acceptance, current-consumer archive acceptance and production
deployment remain separate work. Profiler's checkout was absent at its last review.
Creation cancellation may leave remote effects; async deletion is best-effort.
Contracts assume trusted tools, cooperative process groups, exclusive child reaping
and caller exclusion of namespace/content writers. Identity checks are not a
hostile-writer fence. App builds are serial, not transactional. Loopback fixtures
need socket access. No performance improvement is claimed without measurements.
