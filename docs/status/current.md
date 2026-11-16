# Current implementation — 2026-11-16

## Authority and delivery

The maintainer accepted [design revision 2](../DESIGN.md). Required outcomes drive
interfaces; old Rust exports, packages and release groups do not. Consumer code
stays in its owner; patches here are qualification migrations only.

Use current local sibling working trees, including edits, through
[read-only snapshots](../qualification/local-sources.md). Refuse capture drift or
stale patches; historical passing runs do not qualify changed source. Auth is on
hold; Storage, Observability and Wire are frozen. Canic is being rewritten and its
current requirements must justify further APIs.

`../ic-tooling` remains the read-only production reference, observed at
`a00369fc29fa837c6deb022cceb7e09aa447292c`. Superseded transplant/design evidence is
retained at `cd9f453ee66565a4feeea10e034f3dd5cb9d61e3`; preserve those deletions and
historical evidence. No old product source/test tree was admitted wholesale.

The public canonical repository is now `dragginzgame/ic-tooling`; the former
production GitHub repository is `dragginzgame/ic-tooling-legacy`. The local
checkout remains `ic-tooling-new`, and the local sibling remains read-only.
Release source `ca0bef0cd29ab272c3e3419d28292ca83ecff875` is on `origin/main`.
The first build-stack publication is complete, scoped in
[release ownership](../release-ownership.md). Artifacts 0.80.1, FS 0.80.1,
Process 0.80.2 and Build 0.87.3 were published on 2026-11-16. Downloaded registry
archives match uploaded packages, index checksums and product sources. Cargo
verified Build against all three published suppliers without local overrides.
Extracted candidates pass 61 Linux unit cases plus the
foreground-terminal case, Clippy and Rustdoc; a current Canic export builds with
the archives and passes 20 metadata/tool refusals. The first four-crate stack is
qualified by [native CI](https://github.com/dragginzgame/ic-tooling/actions/runs/43841772100), which
passes Linux, Apple Silicon macOS, Intel macOS and the history secret scan at
`6541ba0d53bb4d99fc358fbed5850f0f5cc2df8b` on `release/build-readiness`.
Registry-only consumer qualification and production adoption remain separate
delivery work. Publication was explicitly authorized after readiness completed.
Tools 0.1.3 starts the next local unpublished development line; library versions
are unchanged. All three tools tests and `make check` pass; only its local lock
entry changed. The version advance does not refresh prior runtime evidence.
Runtime evidence remains tied to the captured source and suppliers in the linked
qualification recipes; old Timers experiments are historical. Seven [release groups](../release-ownership.md) own their ledgers; root changelog is
an index. Text, HTTP, Testkit and the private qualification executable remain
unpublished. No production consumer migration is claimed.
Toko 0.44.4 is pinned in both workspaces; snapshot/tools admission and 19 Build tests pass after renaming `qualification/owned-wasm-probe` to match its package.
The snapshot now selects the shared CI installer (49 files at the same revision)
for the workflow's single Gitleaks history scan.

Issue repairs (registry publication pending):
[#1 — Hosted CI wiring](https://github.com/dragginzgame/ic-tooling/issues/1) adds
repository-wide snapshot/pin and formatting checks before the native matrix;
Actionlint, `make check` and `make fmt-check` pass locally. New hosted evidence
remains pending. [#2 — Retained-file publication](https://github.com/dragginzgame/ic-tooling/issues/2)
adds `Directory::admit_retained` at the [existing FS owner](../filesystem-custody.md#reopening-an-original-publication-file).
It verifies the original held identity and complete expected bytes, then reuses
create-only seal/commit with retained failure evidence. All 29 FS library tests,
FS Clippy and Rustdoc pass on Linux. The new native macOS cases remain pending;
neither publication nor Backup adoption is claimed. Package versions are unchanged.

## Owners and evidence entrypoints

Use `make deps-list` for local versions. Read only the contract affected by the task:

| Owner | Contract |
| --- | --- |
| Build | [Owned builds](../build-owned-artifact.md) |
| Artifacts | [Producer commitments](../byte-commitments.md) |
| FS | [Preparation](../directory-preparation.md), [publication](../directory-publication.md), [sidecar locks](../sidecar-locks.md) |
| HTTP | [Bounded bodies](../http-body.md) |
| Text | [Presentation](../text-presentation.md) |
| Process | [Execution](../process-execution.md), [handoff/reaping](../process-handoff.md) |
| Testkit | [Held replies](../testkit-held-replies.md), [simulator ownership](../simulator-ownership.md) |

`ic-cargo-qualification` is private external-lock subset admission, outside Build;
it is not an archive runner or metadata editor. HTTP/Tokio stay outside Build and
portable bytes. Testkit's thiserror 2.0.18 selection is scoped to that graph.

Consumer recipes retain source hashes, commands, results and limitations:
[Canic](../../qualification/canic/README.md),
[Query](../../qualification/query/README.md), [DB](../../qualification/db/README.md),
[Platform](../../qualification/platform/README.md),
[Scheduling](../../qualification/scheduling/README.md),
[Backup](../../qualification/backup/README.md) and
[Auth](../../qualification/auth/README.md).
Canic has Linux Build/Text/Process evidence; Query and DB have scoped Text evidence.
Scheduling's rewrite has paired held-reply, pressure, watchdog, Once, metrics-feature
and executor evidence; old Scheduling experiments do not qualify it. Platform's
old driver evidence must not drive the replacement. Auth evidence is historical
at `e52a40be75dd9809d0528189d84b43fe96952ad9`; its migration remains on hold.
Product and consumer test inventories stay in these existing contracts/recipes;
version bumps do not refresh them. No full release gate is claimed.

## Remaining gates and limitations

[Backup assembly](../../qualification/backup/README.md#complete-producer-commitments) passes four current tests with both suppliers and Clippy; restore remains blocked by its earlier 15 baseline failures.
Canic runtime composition passes 54 paired cases, one replacement-only report
traversal refusal, 15 custody/Process/viewer cases, real artifact/app builds and
20 metadata/tool refusals on one current 0.216.5 capture. Host/CLI Clippy and graph
checks pass; Host normal/build edges exclude legacy FS/Process. Report files use
held FS custody; automatic report-directory allocation retains legacy FS. The
report adapter deliberately rejects parent traversal accepted by the released
writer; this narrower path policy is explicit in its qualification recipe.
Next: automatic report allocation, registry Store-build runtime and archive
qualification. Live signing/submission remains outside runtime evidence.
The [native-execution review](../process-execution.md) records the shared Process
owner used by Build and current Canic group capture, foreground execution and
desktop handoff. Combined Linux qualification covers artifact/metadata, lock setup,
signer capture, Store-source lookup and viewer reaping; native macOS consumer
qualification remains open. The four-crate core's native release checks pass.
No Canic product code moves here. `../ic-profiler` is absent and its current
checkout/status has been requested. The inspected `ic-platform-new` has no current
Tooling dependency; old Platform fixture results must not drive that rewrite.
Managed-server lifetime/cancellation, multi-canister partial snapshot effects,
remaining Platform drivers, Profiler ownership/native acceptance, archive acceptance,
native macOS consumer qualification and production consumer migration remain open.
Borrowed-server success does not justify a wrapper; upstream creation can panic
or leave remote effects after cancellation, and async deletion is best-effort.
See [simulator ownership](../simulator-ownership.md).

These slices assume trusted source/tools, cooperative process groups, exclusive
child reaping and caller exclusion of namespace/content writers. Identity checks
are not hostile-writer atomic compare-and-unlink. App builds are serial, not an
all-or-nothing transaction. HTTP/installed fixtures require loopback. No production
consumer has migrated; no performance or peak-memory improvement is claimed. Shared-target reuse needs matched evidence.
