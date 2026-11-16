# Current implementation — 2026-11-16

## Authority and scope

[Design revision 2](../DESIGN.md) is accepted. Required outcomes drive interfaces;
no old product source/test tree was admitted wholesale. Historical production
reference: `a00369fc29fa837c6deb022cceb7e09aa447292c`; superseded transplant/design
checkpoint: `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`. Preserve that evidence.
The public canonical repository is `dragginzgame/ic-tooling`; the predecessor is
`dragginzgame/ic-tooling-legacy`. Historical local checkout names are not current
source selections.

Use [read-only current-source snapshots](../qualification/local-sources.md),
including sibling edits. Refuse capture drift and stale patches. Consumer product
code stays in its owner. Auth is on hold; Storage, Observability and Wire are
frozen. Canic's current requirements must justify further Tooling capabilities.
Backup adoption remains user-owned; no sibling changes are part of current work.

## Delivery and active readiness

Artifacts/FS 0.80.2, Process 0.80.2 and Build 0.87.3 are published. Registry
readback matches uploaded bytes, checksums and release source. Foundation's latest
release is `828c2692523a7a906188c111c9b58f30d0f52468`, tag `foundation-v0.80.2`;
its [native acceptance](https://github.com/dragginzgame/ic-tooling/actions/runs/43862099971)
passes Linux and both macOS architectures. See [release receipts](../release-ownership.md).
FS includes original retained-file admission for create-only seal/commit without
rewriting recovery bytes; all 29 FS tests passed on each native host.
Issues [#1](https://github.com/dragginzgame/ic-tooling/issues/1) (hosted CI) and
[#2](https://github.com/dragginzgame/ic-tooling/issues/2) (retained publication) are closed.

Toko 0.44.6 is pinned in both workspaces and installed through its verified exporter.
The 49-file snapshot changes only the baseline's explicit asynchronous hosted-CI
rule; executable files and the cache-action implementation are unchanged. CI
action references select the reviewed 0.44.6 source commit. Snapshot/tool admission,
formatting and Actionlint pass locally. Earlier [0.44.5 CI](https://github.com/dragginzgame/ic-tooling/actions/runs/43868115486)
remains evidence for its own delivered source.

The [Platform migration](../testkit-held-replies.md#platform-034-migration-review)
now adopts Protocol 0.2.3's bounded HTTP encoder and Types 0.21.1. Legacy Platform
0.33.7 / Types 0.20.4 are removed. [Platform #5](https://github.com/dragginzgame/ic-platform/issues/5)
supplied the missing capability; no sibling implementation was modified.
Testkit no longer assembles generic management DTOs. Required request semantics,
original callback bytes and module limits remain enforced; Protocol's narrower
Candid type table can change the generated Wasm bytes.
Final archive run `target/library-release.sCTEKGgV` passes six unit and both
installed tests, lint/docs for both feature selections, default-graph isolation
and independent deletion observation. The old Platform/Types graph is absent.
The refreshed Testkit publication dry run passes with an identical archive;
no upload occurred. Platform #5 is closed with the local adoption evidence.

The maintainer requested preparation, not publication, of HTTP 0.80.0, Text 0.77.2
and Testkit 0.107.1. Their metadata now permits only crates.io; the private
`ic-cargo-qualification` 0.1.3 remains unpublished with `publish = false`.
[Library readiness](../release-ownership.md#independent-library-readiness) records
registry availability, current first-party selections and the explicit group gate.
Linux archive run `target/library-release.vBZQpJd2` passes 11 unit tests, both
installed Testkit tests, default-feature isolation, Clippy and Rustdoc. Independent
server inventory confirms deletion of all three instances. All three Cargo
publication dry runs pass without uploads. Formatting, ShellCheck and Actionlint
pass. Candidate source and tool hashes remain unchanged across the archive gate.

Readiness delivery stays on `main`, as directed by the maintainer. Its push uses
asynchronous CI; the new Linux, Apple Silicon macOS and Intel macOS
library results remain pending. Prior four-crate results do not qualify these
packages. Release notes remain undated; no candidate tags or uploads have been
made. The Canic source review is recorded in a separate documentation commit.

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
| Testkit | [Held replies](../testkit-held-replies.md), [simulator ownership](../simulator-ownership.md) |

`ic-cargo-qualification` owns external-lock subset admission, not archive running
or release editing inside Build. HTTP/Tokio and simulator dependencies stay outside
Build and portable bytes. Testkit preserves PocketIC's original client types;
its held-reply feature is explicit and caller-owned servers remain external.

## Consumer boundaries and remaining work

[Canic's current source review](../../qualification/canic/README.md#current-source-review--2026-11-16)
captures HEAD `37270cb30f840d45592781a4fd957e823c39c895` plus working-tree edits.
Its product lock selects registry Artifacts/FS 0.80.2 and Process 0.80.2. CLI report
publication delegates to Host, which already uses held report-directory allocation
and the published FS API. The old report patch correctly refuses this source;
that migration obligation is obsolete. The earlier 54-case paired composition,
real builds and metadata refusals remain historical evidence for their own captures.
Further Canic work must start from current requirements and consumer-owned
acceptance. Store-build, archive and native macOS consumer qualification are not
inferred from source inspection or old recipes. No current runtime test was run
for that source review.

[Scheduling](../../qualification/scheduling/README.md) targets the rewrite now at
`../ic-scheduling` with verified ancestry and explicit prepared tools. Its paired
Once aggregate, metrics-feature and executor checks share exact source identity;
old Scheduling experiments are historical. [Backup](../../qualification/backup/README.md)
assembly has four passing captured tests; earlier restore baseline failures and
its next adoption remain outside this work. [Query](../../qualification/query/README.md)
and [DB](../../qualification/db/README.md) retain scoped Text/installed-console
evidence. [Platform](../../qualification/platform/README.md) evidence must not drive
its rewrite; [Auth](../../qualification/auth/README.md) stays on hold.

Managed-server lifetime/cancellation, multi-canister partial snapshot effects,
remaining Platform drivers, Profiler ownership/native acceptance, current-consumer
archive acceptance and production deployment remain open. Profiler's checkout was
absent at its last review. Borrowed-server success does not justify a wrapper;
creation may leave remote effects after cancellation and async deletion is best-effort.

These contracts assume trusted source/tools, cooperative process groups, exclusive
child reaping and caller exclusion of namespace/content writers. Identity checks
are not hostile-writer atomic compare-and-unlink. App builds are serial rather
than transactional. Loopback fixtures require local socket access. No performance,
peak-memory or shared-target improvement is claimed without matched measurements.
