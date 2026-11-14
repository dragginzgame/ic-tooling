# Current handoff — 2026-11-14

## Platform 0.33 deployment preparation

The current authorized batch adopts published Platform 0.33.0 (source
`dd15c2c8`, archive SHA-256
`1ff16542d2cb6c8307afced4b0b1dc10b31640ea49f26304de6ff94e02eeb8bf`).
Its bounded Registry protobuf capability is opt-in and remains unselected;
Tooling retains its existing protocol/transport and feature boundaries. Both
roots require Types 0.20.3; the private probe requires Runtime/macros/guard
0.22.3. Preserve the entry lock selections and the completed Host FS opener work.

Deployment selects Foundation 0.77.3 for the compatible FS addition and Testkit
0.104.1 for private Platform adoption in one supplier-first patch batch, followed
by Tools 0.93.0 for its exposed Platform codec/ledger identity. Consumers sharing
those Tools values select Platform 0.33. Testkit's public API, native PocketIC
16.1 and the independent Wire adapter remain unchanged. Each batch uses the
standard frozen release and publication owners; hosted CI remains asynchronous.
The previous complete gates took about 429–583 seconds per group, including
239–245 seconds of tooling fixtures. Fresh source/FS changes require fresh gates;
only already qualified stage admissions can reuse unchanged inputs.

Preserve `target/evidence/platform033-deploy/` (entry patch/locks, authenticated
supplier source and qualification/deployment logs). No sibling source edits,
release tags or publications have occurred in this preparation section yet.
Focused `dependency-adoption-check` passed: isolated native/Wasm graphs, Tools
codec/Agent/Registry/ledger adapters, snapshot refusals, nine adapter unit tests,
installed upstream snapshot restoration and the actual Wire canister's state/error
contract. Formatting and local document links pass. The preceding `d02fc37a`
push has complete Linux and native macOS 15 Intel/Apple Silicon passes in
[CI 43573475462](https://github.com/dragginzgame/ic-tooling/actions/runs/43573475462);
that is prior-source evidence, not qualification of this new Platform/FS batch.

## Local Host FS update opener

The Backup maintainer approved this scoped supplier fix. The new
`FileEntry::open_regular_file_for_update(forbidden_permissions)` reuses held-parent,
no-follow/nonblocking descriptor admission without create/truncate or implicit
writes. Caller identity/link-count checks, locking, append/sync and recovery stay
explicit; read-only APIs remain read-only. See [Host IO](../hosts.md#configured-evidence-io-helpers).
Linux passed 18 capability tests, 40 shared reader tests, strict package Clippy
with/without capability and warning-denying Rustdoc. Native macOS and consumer
append/recovery are not qualified. Both pre-existing dirty locks are unchanged.
The API/source/docs are local and uncommitted; no version, commit, push or registry
publication was performed. Backup adoption requires a published Foundation release.
The following sections retain prior delivery context and recovery information.

## Previous Git-only delivery

The preceding batch delivered Toko 0.43.0, the Linux Make fixture repair, Wire
generation qualification and issue reconciliation to `origin/main`. All changed
surfaces are contributor tooling, private qualification or documentation;
published library code/APIs were unchanged in that batch. No product version, release tag or
registry upload was needed. Testkit 0.104.0 and adapter 0.1.0 were rechecked as
non-yanked registry releases with matching existing remote tags. The frozen
increment workflow remains the owner of future product releases.

Focused snapshot/tool/pin admission, Make routing, four installed Wire-generation
tests, ShellCheck and documentation links passed on Linux. Hosted CI follows the
push asynchronously; no fresh native macOS result is claimed for this batch.

## Toko Tooling 0.43.0 adoption

Both workspace pins, locks and exact-pin exceptions now select authenticated,
non-yanked crates.io 0.43.0 (source `bd24a0b7`, archive SHA-256
`060d3507b66eb290ce12bc2572098f7e7cb1e12e9d6697a9b1a5d1552fcb2588`).
The exporter preserved all entrypoints and prior local issue fixes. Its preview
changed only managed `make/tools.mk`, the LOC reporter's help and the new launcher;
the installed snapshot now admits 117 files. Baseline, native catalogs, compiler,
release/publication runners and required product gates are byte-identical.
Both locks differ from entry only in the tooling package block.

Use `toko-cloc` rather than `make cloc` or `CLOC_*` settings. The existing PATH
launcher already matches the reviewed payload byte-for-byte, so no user-global
file was changed. Local help, setup documentation and the real launcher caller
fixture migrate together. The focused fixture covers nested cwd, arguments with
spaces, direct failure status and retirement of the old Make target. Snapshot
and existing prepared host/Cargo/editor/PocketIC admission pass without tool
reinstallation. Evidence and the entry patch/locks remain under
`target/evidence/toko043-20261114/`. No publication, sibling source change or
artifact cleanup is included.

## Issue reconciliation and focused fixes

The maintainer retired Canic's acceptance scope: #263 is closed as superseded,
not as proof of its historical two-case regression. Snapshot/timer integrity,
two-slot capture, caller funding and effect reconciliation remain unchanged.
The Rust 1.100 policy (#354), structured warning matrix (#355) and published
Wire alignment (#356) now have their missing acceptance evidence reconciled.
The independent adapter's composition proof completes #357; these
qualification/docs fixes belong to the same Git batch, with no new release or upload.

The Linux CI fixture inherited `IC_TOOLING_FULL_CI=1` while expecting ordinary
release routing. The fixture now unsets it before testing both explicit routes.
The exact failure reproduced; the focused roster and complete Make fixture pass
with the CI flag set. No production gate or refusal was weakened.

The new explicit `make testkit-wire-generations-check` runs four existing
installed query/update/error tests against real Wire 0.21.5/0.22.1 selections,
one shared current Testkit/PocketIC, and byte-identical adapter/test source.
The old adapter manifest is test-only; no old-codec product, alias or new release
is introduced. Native Linux evidence: `target/tmp/wire-adapter-generations.WUBgPW/`.
The sandbox-only attempt could not bind loopback; the native attempt passed.
Keep both attempts and `target/evidence/issues-20261114/`.

## Deployment delivered — Toko 0.42.8 and independent Wire adapter

The authorized current-repository batch is published to GitHub and crates.io.
Frozen `batch-aVh51g1i` captured corrected source `a4818cd8`, then delivered:

| Group | Version | Release commit | Tag |
| --- | --- | --- | --- |
| Testkit | 0.104.0 | `79804eb4f304e1ff41b1a90198d81117f3297831` | `testkit-v0.104.0` |
| Testkit Wire | 0.1.0 | `fbbbde7c9fac8b4b4f7b44aafea6b1eae795a57a` | `testkit-wire-v0.1.0` |

Both non-yanked registry archives match index checksums and exact clean VCS
commits. Remote annotated tags/main were verified; the ordinary checkout
reconciled cleanly to `fbbbde7c`. Git-only qualification corrections followed
in `edfedc6c`; this handoff is subsequent documentation. No sibling source,
release, cleanup or artifact retirement was included.

Foundation 0.77.2, Build 0.84.1, Text 0.75.0 and Tools 0.92.1 remain current;
private qualification packages retain 0.96.0. Source selection and release
ownership cover ten products, two private members and six release groups.

## Qualified selection and tooling

The preceding deployment selected published Toko 0.42.8. Its authenticated export retained all 116
managed files and entrypoints, changing only the hook and hook installer for
CDPATH-independent resolution. The baseline, native pins, release/publication
code and required gates are unchanged. Snapshot/tool/server admission and actual
consumer formatting/preservation/hook installation pass with `CDPATH=/tmp`.

The Toko refresh preserved all non-tooling package blocks byte-for-byte. Final
locks differ from entry only in Toko and the two owned product versions. Entry
already selected Bumpalo 3.20.4, whose published multithreaded ZST fix was reviewed.
Platform 0.32.2, Types 0.20.2, Runtime/macros/guard 0.22.2, Wire 0.22.1 and Mio
1.2.5 remain selected. Do not reapply older preservation locks over these inputs.
Optional native-catalog/editor updates remain separate maintenance; selected
cloc source counting does not expand archives. Never hand-edit managed catalogs.

## #357 — shipped ownership cut and completed local acceptance

Testkit has no Wire/Serde adapter dependency, `wire` feature or compatibility
exports. Lifecycle and raw transport retain the selected upstream client.
`ic-testkit-wire` alone owns typed native calls, independently aligned to Wire
0.22. Wire remains sole framing/validation owner. Exact method/caller identity,
all limits, typed causes, panic behavior and one-shot updates are preserved.
See [migration and publication ownership](../testkit/wire-adapter.md).

Auth and Blob independently migrated to published Testkit 0.104/adapter 0.1 with
Wire 0.22.1; read-only inspection found passing qualification Clippy and focused
installed checks. Auth retains five installed ingress/certification/upgrade and
three persistent lifecycle cases in `target/upstream-1114/`; Blob retains seven
ingress/typed-error/lost-ack/lifecycle cases in `target/testkit-refresh-20261114/`.
These are local Linux results, not sibling delivery or macOS claims. Together
with the new actual multi-generation composition probe, they complete
[#357](https://github.com/dragginzgame/ic-tooling/issues/357). Future Wire-only adoption selects the
`testkit-wire` group without a lifecycle release when its client API is unchanged.

## Validation and asynchronous hosts

Both complete Linux gates passed all 32 stages: strict Clippy/Rustdoc, feature
and native/Wasm boundaries, installed PocketIC, extracted archive tests and
installed workspace tests. Rounded stage sums were 583/429 seconds, including
245/239 seconds of tooling fixtures. Only four admitted unchanged fixture proofs
were reused across groups. Both publications freshly admitted source/archives
and reused only their qualified lint receipts. Whole batch wall time was
1,498.37 seconds, excluding planning, focused qualification, the failed attempt,
reconciliation and registry readback. Individual delivery phase timings remain
unavailable; these are observations, not controlled speedups.

The registry-only caller proof passed all three public-contract tests against
exact published packages, with no source overrides or normal-lock edits.
`edfedc6c` corrects its selected-client import and explicit ledger features.
The adapter ledger's pending 0.1.1 note describes this private-only maintenance;
it requires no product version increment or additional upload. Focused caller
Clippy, shell checks, formatting and document links pass too.

Explicit [native CI](https://github.com/dragginzgame/ic-tooling/actions/runs/43502371156),
[optimizer](https://github.com/dragginzgame/ic-tooling/actions/runs/43502379157) and
[runtime](https://github.com/dragginzgame/ic-tooling/actions/runs/43502385864) runs
were originally dispatched at exact `edfedc6c`; browser qualification succeeded.
Native Intel macOS 15 passed completely in
[job 132151267353](https://github.com/dragginzgame/ic-tooling/actions/runs/43502371156/job/132151267353).
Apple Silicon initially failed before qualification: retained download stderr
reports HTTP 403 (curl 56). A retry is asynchronous. Linux failed in the inherited
Make-fixture flag described above. No current all-green matrix is claimed.

For #354/#355, applicable Apple Silicon macOS 15 evidence already exists in
[job 131916054060](https://github.com/dragginzgame/ic-tooling/actions/runs/43434488465/job/131916054060)
at `4ea33bbf`, using actual Rust/Cargo 1.100. Both root lint tables, compiler floor
and pin are unchanged. Build's Cargo source and warning tests are byte-identical
to current `b608af88`; both Mac logs show the warning matrix and native executable
tests passing, including archive qualification. This supports those scoped
compiler/policy issues, not qualification of all subsequently changed suppliers.

## Retained evidence and failures

Preserve `.git/release-state/preserve/deploy0428-20261113.au1Irv/`: entry patch,
locks, handoff, authenticated supplier archives/indexes, logs, `registry/`,
`registry-cohort/` and `deployment-records/` (receipts and gate logs). Current raw
consumer evidence is `target/published-integration.a5ZH8P/`; #357's focused
checks remain under `target/evidence/issue357/`. Retain the completed journal,
candidate Git and managed checkout for `batch-aVh51g1i`; cleanup is not authorized.

First capture `batch-g7FKs6lB` stopped after eight passing stages before version
preparation/delivery: the Make fixture omitted the new adapter Wasm check from
its expected command roster. The corrected fixture passed before recapture;
the new source ran both complete gates. Keep the failed journal, checkout and
logs. No old receipts were relabelled or claimed as collector-only recovery.

## Outstanding work and preserved contracts

- [#353](https://github.com/dragginzgame/ic-tooling/issues/353): phase timing and
  collector recovery remain incomplete; generic runner work is now owned by
  [Toko Tooling #382](https://github.com/dragginzgame/toko-tooling/issues/382).
  No sibling source was changed, whole-gate reuse added or benchmark release run.
  Preserve explicit manual comparison qualification; avoid redundant routine
  same-source dispatches operationally. Remaining timing must come from a real
  independently authorized deployment, not reconstructed file timestamps.
- #263 is superseded by the maintainer's Canic replacement decision. Keep
  sender, funding, stop/drain/start and effect reconciliation caller-owned.
  [Timers #87](https://github.com/dragginzgame/ic-timers/issues/87) is separate.
- Independent fixes: exact Build probe/archive paths, Cargo-config FS errors,
  malformed successful process observations. Consumer work remains in
  [Canic #247](https://github.com/dragginzgame/canic/issues/247) and
  [#762](https://github.com/dragginzgame/canic/issues/762).

The [preceding handoff](https://github.com/dragginzgame/ic-tooling/blob/5c6961f4120b6c500efe767dd37b3730291b059a/docs/status/current.md)
records prior verified releases and recovery paths. Preserve all original
journals/adapters, `preserve/*` refs and durable records, including
`deploy0425-20261113.clk428/`, `batch-4szjd3du` and `batch-_HKWam5i`.
Earlier target losses were external, cause unknown; only surviving logs/copies
are evidence. Retired client archive remains
`/tmp/ic-tooling-upstream-migration/retired-client.tar.gz`, source
`0c17900090f0c2d2ebf3182598b49e651223845e`. Keep two-pass generated-root discovery,
Darwin sync durability, error downcasts and the snapshot callback's Send-safe future.
