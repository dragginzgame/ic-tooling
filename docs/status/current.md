# Current handoff — 2026-10-31

## Authorized deployment

The maintainer has requested deployment of this accepted batch: Foundation
**0.73.2**, then integration **0.75.1**, through the standard Git release and
separate registry-publication flows. PR #284 carries source delivery, including
the Process fix. Required release gates remain intact; hosted native CI is async.
Live deployment results and evidence coordinates are recorded at
`.git/release-state/deployment-foundation-0.73.2-integration-0.75.1/delivery-result.json`.
The sections below record admission and focused evidence; consult that result,
actual manifests and remote/registry observations for delivery completion.

## Admission source and scope

HEAD/main remains released `c40c66b92120dbfb745219bf8f4e2a34bb3ce1f4`, tagged
`integration-v0.75.0`. Its three public packages are published; the private Dev
and probe members are 0.75.0 without publication. Foundation's seven packages
remain published at 0.73.1. Their manifests and existing release identities stay.

The current task fixes Process version-capture qualification in
[#286](https://github.com/dragginzgame/ic-tooling/issues/286), preserving the pending
Shared Tooling/Metrics adoption. Changes remain local and unstaged.
No own numeric package versions, Git refs, release plans, tool pins or registry
publications changed. There is one workspace/catalog/lock, unchanged crate
ownership and features, edition 2024 and MSRV 1.91.

The current refresh's incoming dirty patch, index entries, source export and
focused logs are retained at `/var/tmp/ic-tooling-shared-refresh.M3bsiQ/`.
Earlier Shared/Metrics evidence remains at
`/var/tmp/ic-tooling-shared020-metrics024.vmORyU/`; that earlier task started clean.
Sibling checkouts were inspected only; none changed.
The Process fix's incoming patch, Intel artifact and focused source/archive
evidence are retained at `/var/tmp/ic-tooling-286.eTpfLb/`.

## Shared Tooling adoption

The canonical exporter adopted the latest committed Shared revision
`143a9abb9ae81feae9400ff3844b03b1ddfdd75a` from an isolated clean source. Upstream
titles this commit **0.27.22**, while its `VERSION` and exported manifest report
**0.27.21**. Record both without modifying managed payloads. Its impact report
and independently verified 151-file snapshot are retained. This refresh updates
the engineering core, code-hygiene/release/workspace rules and release-group
admission helper/projection. Executable tool pins are unchanged.

All twelve workspace members now have root catalog entries with their full
current versions and actual local paths, including private and unused entries.
Contiguous release-group blocks name their ledgers, replacing the duplicate header
map. External production-only and mixed production/test blocks reflect real
member usage; there are no build-only or test-only external entries. Both policy
alias rosters cover their entire groups. Bare requirements remain bare, and
existing feature/default selections stay unchanged. Catalog-only declarations
introduce no graph edges. Admission checks coverage, ownership and actual paths
across both groups; the existing transaction still owns selected release edits.

Plain Cargo cleanup retains its default whole-workspace target semantics.
There is no target-directory redirection or new cleanup wrapper. Explicit cleanup
may remove nested caches and evidence beneath target; retention forbids automatic
cleanup rather than narrowing operator requests. Durable release evidence stays
outside target under `.git/release-state/`. This task ran no cleanup.

## Metrics and compatibility

The catalog and lock select published **ic-metrics 0.24.0**, with default features
still disabled. Its native Wire schema generation changes upstream; Testkit uses
only exact arithmetic and maps failures to its own public benchmark error types.
Benchmark values, public errors and production feature boundaries remain unchanged.
The lockfile update changes only the Metrics version/checksum record. Platform
0.15.2, Types 0.15.1, Wire/Derive 0.13.0 and every other selection are preserved.

Pending compatible notes are Foundation **0.73.2** for the maintained release and
dependency-admission changes, and integration **0.75.1** for arithmetic-only Metrics
adoption. The upstream Metrics minor does not break Testkit's own public boundary.
These are changelog proposals only; actual member versions stay 0.73.1/0.75.0.
See the [Foundation](../../releases/foundation/CHANGELOG.md) and
[integration](../../releases/integration/CHANGELOG.md) ledgers.

## Focused qualification

Actual full locked/offline metadata, snapshot integrity, inherited declarations
and native/Wasm Testkit graph policies pass. Metrics remains feature-free in the
native all-feature and Wasm graphs. Strict all-target/all-feature Clippy passes
for Testkit, Native, Dev and the runtime probe. Rust 1.91 passes all 23 benchmark
tests; Testkit/Native Wasm profiles and the probe compile on the same compiler.

The Metrics/compiler/graph results above are from the earlier adoption evidence;
this catalog refresh changes no production source or selected dependencies.
Fresh full locked/offline metadata confirms identical resolved package identities,
edges, feature selections and member versions; the lockfile remains byte-identical
to this refresh's admission copy. Actual read-only release queries still return
0.73.1 and 0.75.0. The earlier disposable consumer proves Cargo accepts an
abbreviated compatible requirement while normal admission rejects it.

Fresh real consumer release-adapter fixtures pass for both groups, including all
selected catalog requirements, unselected-file preservation, rollback and committed
receipt checks. They use disposable Git and the existing published editor; setup
and complete CI/MSRV gates are substitutes. Additional real admission probes
reject a missing member, an incomplete alias roster and an unused integration
alias pointing at the wrong crate when Foundation is selected.

Fresh snapshot, dependency admission and actual formatting/hooks pass, including
dirty-file preservation and dependency sorting perturbation. Completing the
catalog inserted HTTP between the old fixture pair. The probe now swaps adjacent
Artifacts/FS entries inside Foundation, leaving the label-bearing Build entry
fixed. Both the initial refusal and corrected passing attempt are retained.
Earlier Make routing evidence remains applicable; Make routing itself is unchanged.

The Process version-probe fixture now gives output overflow the existing five-second
guard while retaining a separate 100 ms waiting deadline and strict error/capture/
cleanup assertions. Failure diagnostics include the actual error. Rust 1.91 passes
all ten version-admission tests in both source and the real normalized archive;
strict Process Clippy and focused formatting pass. A disposable 250 ms delayed
startup reproduces a `TimedOut` failure with the old short overflow deadline and
passes with the fix. The original Intel artifact has no actual error variant;
this controlled reproduction establishes the race, not that missing observation.

No full workspace/release gate, publication or native macOS execution ran for this
adoption. Previous release qualification does not qualify these changed inputs.
Released main `c40c66b9` now has complete passing Linux and native macOS 15
Intel/Apple Silicon CI/runtime results; #276 is closed with that evidence.
The Process fix remains local and needs delivery/native qualification under #286.
GitHub remains the only follow-up tracker.

## Retained release evidence

The preceding 0.75 deployment's fresh Linux CI/MSRV gates, source/archive consumers,
publication dry run/upload and registry/ref confirmations remain under
`.git/release-state/deployment-integration-0.75.0/`, including `delivery-result.json`
and the verified `cohort-evidence.tar.zst`. Canonical group receipts/logs remain
directly under `.git/release-state/`. Its fixture root was
`/var/tmp/ic-tooling-release075.p0B3Z7/`; no cache or Git ref was deleted by that run.

Earlier `/tmp` supplier, cohort, deployment and migration directories disappeared
during that deployment. Their old paths in Git history are historical coordinates,
not proof of current availability. Preserve remaining working artifacts, package
attempts, tool installations, archived refs and migration evidence wherever retained.
Old grouped and ungrouped plans keep their recorded source/adapter/policy; never
reinterpret them under the new admission rules. Recovery uses the recorded source
in isolation and retains its own authority and destination checks.
