# Current handoff — 2026-11-03

## Platform 0.20 delivery

[PR #322](https://github.com/dragginzgame/ic-tooling/pull/322) merged Platform and
its independent PocketIC client 0.20.0 at
`e0921d8ee9c879529da4435632cfae67d7633fa1`.
Host Tools **0.80.0** and Testkit **0.83.0** are published. Direct consumers must
align Platform/PocketIC dependencies when exchanging certificate, simulator,
DTO or error identities. Exhaustive client-error matches handle `ResponseTooLarge`.
Runtime remains 0.19.0, canonical Types 0.18.0 and Wire/Core/derive 0.16.0.
The PocketIC server remains 16.1.0. Private Dev/probe versions follow Testkit
0.83.0 and remain excluded from registry uploads.

Platform's bounded snapshot-transfer capability is available. This adoption
preserves Testkit's existing metadata/data transfer behavior and caller-selected
limits. Tooling does not select Wire's optional Platform schema adapter or send
Account DTOs. Account byte/contract migration remains with consumers; no old
Account reader or automatic migration is implied.

## Published groups

| Group | Version | Release source |
|---|---|---|
| Foundation | 0.74.2 | `aa62709bff680ac7c8a050c88cff769c818eebf2` |
| Build | 0.74.1 | `aeff79aa1b6d3e6466b7f990b679db9379215466` |
| Text | 0.74.1 | `df9d9cdffe8d56fb82c3132a4860d2567d6f01a7` |
| Tools | 0.80.0 | `109244b35678e9d89ecec6963ab535c8d65b2f25` |
| Testkit | 0.83.0 | `4b5f2408b3c1325192c40eade4183ffa1d583aaf` |

Native remains 0.80.0. Build's independent 0.74.2 notes remain pending; this
adoption released only Tools and Testkit. The local Shared snapshot is the pending
0.28.3 source `150c304d70381f15601ea5114a4aea7bfa09bc60`, with 162 managed files.
[Shared PR #350](https://github.com/dragginzgame/shared-tooling/pull/350) fixes UTF-8
runner evidence comparison; it is submitted, not merged or released. The approved
durable contribution checkout is
`/home/adam/projects/shared-tooling-contributions/validation-evidence-utf8`.
The [group ledgers](../../CHANGELOG.md) retain earlier delivery history.

The Shared refresh and validation consolidation are prepared on Tooling branch
`consolidate/release-validation` for PR delivery; package versions are unchanged.
Snapshot/source integrity, 99 CI
wiring assertions, Tooling's source-bound plan/shared-proof checks, group
admission, documentation and workflow/script lint passed on Linux. Make gate
effects were substituted in the fixture; these are focused adoption checks,
not full release or native macOS qualification of the refreshed snapshot.
The selected Testkit adapter fixture also passed prepared-version and committed
recovery, changed-proof/log refusal and legacy-receipt refusal. Shared's focused
UTF-8 producer regression passed for Latin-1/non-Latin reasons and Unicode log
paths, malformed text and changed evidence. Local migration evidence is under
`target/shared-verifier-adoption.zza5Cz/`; lasting code remains in this checkout
and the approved durable Shared contribution, not temporary patches.

## Qualification and custody

Both selected groups passed their complete 35-obligation source-bound Linux
release gates, including workspace Clippy, isolated profiles, installed archives,
live PocketIC/Wasm consumers, documentation and release/recovery fixtures.
Separate publication dry runs and uploads passed. Registry archives were
independently downloaded and checked against registry checksums and clean,
exact Git release identities. Registry-only public contract checks also passed.
The first Testkit publication dry run failed on disk exhaustion before upload;
a fresh dry run and upload passed on the unchanged release source after temporary
build-cache relocation. The cache is restored, with original failure evidence retained.

Native macOS 15 Intel/Apple Silicon qualification remains asynchronous; Linux
results do not establish it. Explicit qualification runs bind the final Testkit
release source across CI, optimizer, runtime and browser workflows, independently
of routine push cancellation. See [CI 41631091012](https://github.com/dragginzgame/ic-tooling/actions/runs/41631091012).
Historical three-host Rust 1.99.1 evidence for closed #313 belongs to source
`fc66f67b241af549034ac993c1254b46ab378934` and
[CI 41521867036](https://github.com/dragginzgame/ic-tooling/actions/runs/41521867036).

Incoming source/lock backups, release/publication logs and registry readbacks
remain under `/tmp/ic-tooling-platform020-deploy.gNM3md/`. Durable receipt copies
live in `.git/release-state/deployment-platform020-20261103/`. The reused isolated
checkout and full gate artifacts remain under
`/tmp/ic-tooling-platform-deploy.BGZn8d/release/`. Focused adoption evidence is
under `/tmp/ic-tooling-platform020-adoption.jRLpyl/`. The ordinary checkout retains
its incoming registry selections; only selected workspace versions advance.
Earlier Wire 0.82 delivery custody remains under
`/tmp/ic-tooling-testkit082-deploy.DsxSsd/` and its durable release-state copy;
earlier Platform delivery evidence remains under `/tmp/ic-tooling-platform-deploy.BGZn8d/`.

## Owning evidence

[PR #315](https://github.com/dragginzgame/ic-tooling/pull/315) implements source-bound
release plans; [PR #316](https://github.com/dragginzgame/ic-tooling/pull/316) removes
a stale Text fixture version. Foundation, Build, Tools, Testkit and uncertain/shared
changes retain all obligations. Native/Text narrow only from a complete admitted
three-host baseline. [PR #321](https://github.com/dragginzgame/ic-tooling/pull/321)
lets explicit qualification finish while routine CI retains newest-revision
cancellation. The Shared 0.28.2 adoption keeps full/focused comparisons on explicit
qualification dispatches; ordinary CI runs its full gate once per native host.
Hosted baseline admission still requires the actual comparison and retained proof.
Shared now verifies execution evidence before version-4 receipts are sealed.
Tooling retains source/group/environment admission and a frozen gate inventory
for prepared/committed payload recovery, removing duplicate event/timing parsing.
Historical version-3 receipts keep their original adapter and are not relabelled.
The #314 roster consolidation uses the same delivery branch: full CI and release
selection share the admitted policy, with no Make-recipe parser. Focused dispatch
tests preserve all 35 checks, one lint pass, failure stopping and pipe jobserver
access. All-six-group plan checks and the 99 workflow assertions pass with
substituted effects; evidence is under `target/issue-314-roster.nhB3Tc/`.
#314 closed after complete Linux and native macOS 15 Intel/Apple Silicon proof in
[CI 41621801740](https://github.com/dragginzgame/ic-tooling/actions/runs/41621801740).
Its verified source is `18e3eba9`, not this release; checker/artifact evidence remains
under `/tmp/ic-tooling-314-dispatch.JnxkDE/closure/` and the durable issue release-state copy.

[Platform #134](https://github.com/dragginzgame/ic-platform/issues/134) shipped typed
failures; [#136](https://github.com/dragginzgame/ic-platform/issues/136) shipped the
bounded snapshot API in the 0.20 client. Shared's benchmark-arithmetic rule now
covers Testkit's private average projection, so the redundant AGENTS.md exception
is removed. Metrics/Logging retain instrumentation and recording ownership.
