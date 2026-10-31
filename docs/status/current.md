# Current handoff — 2026-10-31

## Deployment baseline and retained evidence

This deployment starts from main `a7369dc222ca59fb6ca9a2a6b8ec0687a0bae238` (PR #289), with the
completed Foundation **0.73.2** and integration **0.75.1** deployment.
Seven Foundation and three integration packages were published and independently
confirmed. The two private integration packages advanced without publication.
Package versions remain 0.73.2/0.75.1; pending changelog entries are proposals.
One workspace/catalog/lock serves the independently versioned
[release groups](../design/release-groups.md). Edition is 2024; MSRV is 1.91.0.
Crate ownership and feature boundaries remain in the [extraction contract](../extraction.md).

Canonical deployment results, receipt locations and retained evidence are in
`.git/release-state/deployment-foundation-0.73.2-integration-0.75.1/delivery-result.json`.
The isolated release checkout and original journals remain under
`/var/tmp/ic-tooling-deploy0732-0751.CewSpX/release`.
The deployment directory also retains the verified product-evidence archive,
source-drift refusal and concurrent-lock backups. No cleanup is authorized here.

## Accepted deployment batch

The current task implements five contained audit findings: shared package-command
capture/log retention, typed Wasm acquisition state, bounded startup port-file
reading, safe private-worker signaling, and this handoff/extraction refresh.
The maintainer now authorizes adopting Wire 0.13.2 and deploying this batch.
Deliver through a PR, then release Foundation 0.73.3 before integration 0.75.2
using the standard complete gates and separate publication commands.
Public benchmark API cuts and cache-configuration changes are separate future work.
Foundation 0.73.3 and integration 0.75.2 have pending compatible notes only.

The ordinary checkout has unrelated Metrics 0.24.1 and Platform/macros/guard
0.15.3 lock selections. Preserve them outside this deployment: the isolated
release retains Metrics 0.24.0 and Platform 0.15.2. Its scoped lock changes are
Wire/Derive 0.13.2 and the private worker's direct Unix Rustix edge. Incoming
patches and lock bytes are retained under
`/var/tmp/ic-tooling-deploy0733-0752.orDI1k/`; the canonical delivery evidence
root is recorded in
`.git/release-state/deployment-foundation-0.73.3-integration-0.75.2/evidence-root`.
The earlier concurrent-lock backup stash remains
`9eadfae0d0593d35183b82bd951c9d1a4bf35848`.

Focused Linux qualification passes: 93 tests across cache acquisition, package
evidence/selection, startup port admission and cooperative worker shutdown;
strict Clippy for the affected libraries and driver binaries; Rust formatting,
the changed manifest's dependency sorting, local documentation links and diff checks.
No full workspace/release gate or native macOS execution ran for this cleanup.
These focused checks do not qualify all incoming supplier changes for release.

## Shared baseline and qualification limits

The adopted 151-file Shared snapshot is revision
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a`. Upstream calls the commit 0.27.22,
while VERSION/export declare 0.27.21. Preserve the exact managed payloads.
The root catalog includes all twelve members and both complete release rosters.

Both delivered groups passed full isolated Linux CI and Rust 1.91 release gates,
publication checks, registry publication and provenance confirmation. Their
released suppliers were Metrics 0.24.0, Platform 0.15.2 and Wire 0.13.0.
Native macOS qualification is separate: browser fixtures passed on macOS 15 Intel
and Apple Silicon, as well as Linux, in
[run 41228142267](https://github.com/dragginzgame/ic-tooling/actions/runs/41228142267).
The earlier full Rust-native run has now completed successfully on its exact
source a7369dc2: [run 41228142298](https://github.com/dragginzgame/ic-tooling/actions/runs/41228142298).
This is prior-source Linux/native macOS evidence, not qualification of the current
cleanup or Wire adoption. New hosted qualification remains asynchronous.
GitHub issues remain the only follow-up tracker.

## Recovery and historical evidence

The superseded admission narrative and focused evidence descriptions are preserved
in `docs/status/current.md` at Git revision `a7369dc2`. Existing detailed evidence
remains under `/var/tmp/ic-tooling-286.eTpfLb/`,
`/var/tmp/ic-tooling-shared-refresh.M3bsiQ/` and
`/var/tmp/ic-tooling-shared020-metrics024.vmORyU/`.
The preceding integration 0.75.0 deployment retains its delivery result and
verified cohort archive at `.git/release-state/deployment-integration-0.75.0/`;
canonical group receipts/logs remain directly under `.git/release-state/`.

Earlier `/tmp` supplier, cohort, deployment and migration directories disappeared
externally during that deployment. Old paths in Git history are historical
coordinates, not proof of availability. Preserve remaining artifacts, package
attempts, tool installations, archived refs and migration evidence wherever kept.
Old grouped and ungrouped plans retain their recorded source/adapter/policy;
never reinterpret them under newer admission rules. Recovery uses the recorded
source in isolation with its own authority and destination checks.
Plain `cargo clean` retains whole-workspace target semantics; durable release
receipts are outside target. This cleanup task runs no artifact cleanup.
