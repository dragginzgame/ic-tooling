# Replacement release ownership

The maintainer approved group ledgers on 2026-11-15. The executable ownership map
is [ci/cargo-release-groups.json](../ci/cargo-release-groups.json); package manifests
remain the version authority. [CHANGELOG.md](../CHANGELOG.md) is navigation only.
The initial map did not change package versions or publication settings; Testkit
now has its own group as its requirements-driven implementation begins.

| Group | Contract and reason for separate release |
| --- | --- |
| foundation | Byte admission and held filesystem custody share the existing 0.80 development line and capture/publication contracts. Build and the qualification executable consume them; they remain coordinated in this slice. |
| build | Cargo execution and owned Wasm capture have distinct consumer migrations from storage/byte operations. A presentation or HTTP change must not force a Build version change. |
| http | Complete response-body admission depends on Reqwest and has its own native consumer requirements. It stays outside Build and portable byte graphs. |
| text | Pure escaping and table layout serve Query and DB independently of HTTP, process execution and filesystem authority. |
| process | Native child/group execution and bounded communication serve Build and foreground tools without Cargo, IC or observation policy. |
| testkit | Shared PocketIC client identity, bounded generated fixtures and exclusive held-reply control serve test drivers independently of Build and product runtimes. |
| tools | The external-lock executable has its own command contract. Its ledger also owns maintained repository qualification/setup workflows; these scripts are not executable library features or runtime dependencies. |

All eight product packages belong to exactly one group. `qualification/owned-wasm-probe`
is an unpublished fixture in its independent workspace, not a product group.
The Testkit group owns an actual admitted package, not a future simulator wrapper.
Frozen Wire work and absent packages receive no group. Root dependency aliases list only actual member usage; the policy's
alias rosters reflect those declarations without adding display-only dependencies.

Current pending headings match the existing package versions. They describe
unpublished replacement candidates, not claimed crates.io releases. Breaking
consumer migrations are explicit in the affected ledgers. Registry availability,
version reconciliation and release admission remain required before publication.
Existing development version increments were preserved by this organizational work.

Earlier Stage 1–2 notes are retained verbatim in
[history](../releases/history.md), explicitly labelled unpublished. They are not
retroactively assigned to individual groups. Later pending notes now live with
their owners; detailed test results remain in the existing qualification records.

`make deps-list` reports ownership across both workspaces. No release/publication
entrypoint is introduced by this change; future release tooling must require an
explicit group selection. The four first-cutover manifests now permit only
`crates-io`; the other four retain `publish = false`. Nothing has been published.

## First production cutover scope

The maintainer selected first production adoption as the next milestone, ahead
of completing every legacy workload. The proposed initial publication consists
of the dependency-complete owned-build stack:

| Package | Local candidate version |
| --- | --- |
| `ic-host-artifacts` | 0.80.1 |
| `ic-host-fs` | 0.80.1 |
| `ic-host-process` | 0.80.2 |
| `ic-build` | 0.87.3 |

Publish Artifacts before FS, and all three dependencies before Build. The public
sparse index checked on 2026-11-16 ends at 0.79.0 for Artifacts/FS/Process and
0.86.0 for Build; none of the four candidate versions appears in it. Public API
owner lookups list `borovan` for all four. Recheck versions and upload credentials
immediately before delivery; this read-only check neither reserves a version nor
proves upload authorization. Qualify the packaged candidates and
their registry dependency graph, then publish and migrate the selected consumer.
Current Canic owned-build qualification is the starting consumer evidence;
production adoption must use its then-current source and account for its ongoing
rewrite. Native macOS acceptance is required by the adopted baseline and remains
a release gate; Linux success does not waive it.

Text 0.77.2, HTTP 0.80.0 and Testkit 0.107.1 are independent later adoption batches;
they are not dependencies of this first stack. The private qualification executable
0.1.3 remains unpublished. Backup restore failures and frozen/on-hold consumers
do not block this scoped build release. This milestone does not itself constitute
publication or a completed consumer migration; candidate packaging and cutover
checks are the next work, rather than additional unrelated qualification slices.

## Candidate qualification and delivery boundary

`make test-build-release-candidate` creates four Cargo archives under a fresh
`target/build-release.*`, records hashes, extracts them, and qualifies only those
four packages. Normalized manifests retain registry dependency requirements.
Local registry overrides supply the unpublished candidate dependencies during
qualification; they are not a claim that crates.io can already resolve them.
Tests run against extracted source, not product checkout paths. Build's separate
owned-Wasm probe is copied as qualification input; it is not shipped in Build.

The command runs native unit and foreground-terminal tests, warning-denied Clippy
and Rustdoc, then captures current Canic read-only for its real Wasm build and
20 metadata/tool refusal cases. Dependency admission requires the extracted
candidate paths. It does not expand this first cutover to the whole CLI migration.
Existing deployment, signing and application data remain untouched.

Linux archive evidence: `target/build-release.xrogPDbD` passed 61 unit cases,
Clippy, Rustdoc and the current Canic capture `target/canic-build.QveKsBGz`.
That consumer capture has HEAD `2ee4add03126aa2f5259c609fa05eae8a14bcd14`
plus local edits, source archive SHA-256
`fc0d640b0caa0a5a9e0122bca15810968b8ab09123d19d9a07fa8754ecd03ef6`.
The later native-only run `target/build-release.asOphwy1` also passes the explicit
foreground-terminal case (62 cases total). The first terminal evidence parser
refused colored output; selecting libtest `--color never` corrected the parser,
without weakening the test. Archive hashes and per-command logs live with each run.

The GitHub `Build release candidate` workflow checks Ubuntu 24.04, macOS 15 Apple
Silicon and macOS 15 Intel. It prepares Rust 1.100.0, Clippy and the Wasm target;
`BUILD_RELEASE_ONLINE=true` permits dependency preparation before locked offline
tests. `BUILD_RELEASE_CONSUMER=none` explicitly excludes the unavailable sibling
on hosted runners; the Linux current-consumer evidence is separate. It uploads
archives, source identity, logs and completion evidence even after failure.
The required Gitleaks history scan runs once outside the host matrix using the
installed pinned shared installer. Native jobs need Bash, Git, jq, ripgrep, Perl,
tar, shasum and script; no GNU-only script arguments are used on macOS.

Before publishing: require the selected commit's native jobs to pass, review the
archive inventory and candidate hashes, and recheck registry versions and owner
credentials. Publish in dependency order, verifying each supplier's registry
checksum before resolving and publishing its dependent. A publication request
remains separate from this readiness work. Current source is intentionally
incompatible with legacy APIs; consumers need the explicit Build adapter before
changing versions. After publication, rerun the selected consumer against registry
packages without local overrides before production adoption.
