# Current handoff — 2026-10-27

## Source and scope

The accepted complexity-cleanup batch is local and unstaged over released 0.69.1,
HEAD `1c0002df2b856e43010aa1c0ca5c343a95a17112`. Pending notes select 0.69.2:
this is compatible internal cleanup and contributor tooling; published API paths,
cache representations and failure/recovery contracts remain intact. Package
versions, lockfile, product-tool pins and the real index are unchanged. No IC
Tooling commit, push, release, publication or artifact cleanup ran.

The [0.69.1 handoff](https://github.com/dragginzgame/ic-tooling/blob/1c0002df2b856e43010aa1c0ca5c343a95a17112/docs/status/current.md)
retains integration, release and older adoption evidence. Host, Build and Testkit
keep their capability boundaries; consumers own installation and recovery policy.

## Cleanup outcome

Process custody now records owned/reaped/lost lifecycle states together, retaining
bounded-reap timing and external-reaper protection. Named private exchange modes
replace positional behavior flags. Generic Cargo observations live below Wasm
orchestration while keeping their released public type/path. Cache execution uses
admitted mode/maintenance combinations and borrows prior observations without an
extra heap allocation; lock ordering and revalidation are preserved.

Pool acquisition finalizes total timing and outcomes in one place, preserving
fatal discard, one recovery attempt, both recovery causes and unwind invalidation.
Unix fixture process observations have one bounded owner, retaining zombie versus
reaped distinctions. Fake server/downloader scripts share their actual repeated
setup without introducing a generic server-fixture framework.

CI executes each Make/release/publication/record suite once under the existing
relative/aliased temporary-parent proof. Standalone targets remain available.
Retention still probes failures and premature success for every selected entry.
The source-prefix injector was reassessed and retained: it stops before executable
setup/publication effects without requiring a larger test-execution mechanism.

## Shared contribution and adoption

[Shared Tooling PR #318](https://github.com/dragginzgame/shared-tooling/pull/318)
derives hook-fixture companion closure through the existing admitted copier.
It preserves opaque gitlinks and supports quoted newline-bearing roots while
retaining strict relative payload admission. Focused producer fixture/hook and
source-following ShellCheck evidence passes. The PR is open; its integration and
release remain separate from local consumer adoption.

The canonical exporter adopted exact committed revision
`d08cbcafbaff4227e798a890232e2248bf167504` (0.27.1 plus that correction), with
144 snapshot files. The intervening supplying-owner advisory-tracking guidance
was reviewed; dependency/security gates are unchanged. The ordinary Shared
Tooling checkout and its unrelated dirty governance edits were not modified.

## Validation and retained evidence

Evidence and source preservation are under `/tmp/ic-tooling-cleanup-0692.DCU3sj/`.
It retains the starting HEAD/archive, incoming patch, failed attempts, isolated
Shared clone, exporter impact/adoption logs and selected validation logs.

Actual Rust 1.91 Linux checks pass: 18 child lifecycle tests, 77 final process
execution tests, the new process-state observation, 75 final Wasm-cache tests,
seven Cargo execution tests and four pool schema tests. The server-runner suite
passes 13 tests with four existing ignored cases. Startup passes 26 cases in the
sandbox; its two socket-dependent failures pass individually with native
loopback access. Six existing startup cases remain ignored.

Four live prepared PocketIC 16.1 Linux pool cases pass: ordinary restore/rebuild,
fatal discard with finalized timing, failed recovery preserving both causes and
recipe-panic invalidation. Three-package all-target/all-feature warning-denied
Clippy and Build/Process warning-denied Rustdoc pass. Actual consumer hook
preservation, snapshot integrity, source-following consumer ShellCheck and 669
local documentation links pass. Complete adapter-retention and release-command
routing checks pass (`consumer-tooling.log`); substituted effects do not qualify
a real release.

An initial overly broad process-testing filter also selected unrelated HTTP/net
cases; 12 failed on sandbox socket denial. Those retained failures are not passes;
the changed process-state contract passed separately. Compilation attempts and
fixture invocation corrections remain recorded. No full workspace/release gate,
tool download or native macOS qualification ran. Native macOS Intel and Apple
Silicon evidence is still required for this changed source.

## Retained obligations

This batch supplies no crypto correction or security exemption. Historical
[Tooling #240](https://github.com/dragginzgame/ic-tooling/issues/240) and supplying
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) retain the
crypto/Agent graph evidence and qualification obligations. No consumer migration
or publication is claimed by these local changes.

Migration/retirement archives remain at `/tmp/ic-tooling-migration/`; original
Host refs, stale-lock evidence at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`,
release plans, failed attempts and earlier backup/stash evidence remain preserved.
The linked prior handoff identifies their original evidence locations.
