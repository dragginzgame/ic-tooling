# Current handoff

## Wire 0.17 adoption and Testkit deployment — in progress

The authorized deployment uses the ordinary `/home/adam/projects/ic-tooling`
checkout on main, targeting `dragginzgame/ic-tooling:main`. Publish Testkit 0.86.0
with tag `testkit-v0.86.0`; private Dev/probe members follow without publication.
Wire 0.17.0 is confirmed live. Testkit exposes Wire limits/errors, so the new
minor supplier identity selects a Testkit minor. No other public Tooling group
uses Wire. Build's additive archive API/pipeline changes remain pending 0.74.4.
Shared's local parser fix is outside this Wire deployment and is not adopted.

The root requirement now selects Wire 0.17.0. Its unused metadata derive leaves
the selected graph. Codec bytes, budgets and client/server protocol are unchanged.
The Testkit graph policy excludes the schema derive from production profiles.

Incoming unrelated Cargo.lock selections are preserved byte-for-byte under
`.git/release-state/deployment-wire017-20261104/incoming-Cargo.lock`, with the
incoming diff and SHA-256 evidence. The release uses committed selections plus
only Wire's authorized change. After all source-sensitive release/publication
steps, restore the user's selections with only Testkit's owned version entries
and Wire's required update aligned. Do not discard or blindly stage those bytes.
The actual earlier lock writer remains unidentified; the user confirmed no
concurrent writers, and no active repository Cargo/rustc process was observed.

The selected plan and current logs are under `target/deploy-wire017-20261104/`.
The complete selected release gate and registry publication still need execution;
no new release/publication is claimed yet. Native macOS qualification of this
batch is separate and will run asynchronously. Preserve all evidence and artifacts.

## Integrated release-speed implementation

Build adds caller-owned archive checks while retaining the default fresh-target
API. The private archive driver uses one owned compilation cache, invalidates
extracted workspace packages and preserves registry builds. Fresh packaging,
source/graph admission, all-target tests and doctests still execute. Cache custody,
same-path/fixed-mtime payload rejection, default qualification and evidence
exclusion checks passed focused Linux qualification before Wire adoption.

The adapter fixture uses three representative substituted gate targets, with
complete production-roster coverage retained separately. Common corruption/setup/
staging probes run once; each group retains real preparation, renderer/graph
rollback, cross-group receipt refusal and committed-payload checks. All six adapter
owners passed, as did changed/new-input invalidation, exact-input reuse of
retention/evidence fixtures, dispatcher routing and tampered proof/log refusal.
Ordinary CI executes those fixtures; release groups get fresh source/group
admissions and retain original execution/log custody.

ShellCheck, Perl syntax, document links and installed snapshot integrity passed.
Earlier local observations: selected adapter fixture 117s → 101s before the
common-probe consolidation; Text archive qualification 4s cold → 2s warm. These
are focused observations, not whole-release or hosted performance claims.
Evidence remains under `target/release-speed/`. Hosted cache effectiveness and
native macOS qualification of these implementation changes have not run.

## Pending upstream parser improvement

Shared [#354](https://github.com/dragginzgame/shared-tooling/issues/354) owns its
local manifest JSON assembly/catalog-join improvement, focused tests and pending
0.29.6 note in the existing Shared checkout. Fresh locked Cargo/path admission
and primary parser failures are preserved. Admission and committed-source ordering
fixtures pass on Linux. Actual Tooling Build/Testkit plans matched the installed
helper byte-for-byte. Single plan observations were 2.490s/2.193s and 2.514s/2.196s
(installed/pending). Native macOS remains unqualified. No Shared delivery/adoption
is implied by this Testkit deployment.

## Prior published baseline and evidence

Tools 0.82.0 (`1163ee6614f809c17fec1dc0398ffa8c5d69c7ec`) and Testkit 0.85.1
(`3d371d7e948dd4e34307a81f7a19ff85e1c5bc45`) are published, with pushed annotated
group tags. Platform facade 0.22, Runtime 0.19 and canonical Types 0.18 are adopted.
Tools' certificate/tree identities match upstream Agent; `^0.81` does not admit
Host Tools 0.82 and 0.81.1 was never published. The maintained PocketIC 16.1 client
remains in Testkit, with explicit server URLs, retained license/provenance,
no-replay errors and bounded snapshot transfer. Management DTOs remain pinned
at 0.11.0. Foundation 0.74.3, Build 0.74.3, Text 0.74.2 and Native 0.80 remain live.

Both prior groups passed complete 35-stage Linux gates, publication admission,
registry/archive checksum and VCS readback, and a registry-only public consumer.
[Final prior CI](https://github.com/dragginzgame/ic-tooling/actions/runs/41840534687)
passed Linux/macOS 15 Intel/Apple Silicon at source
`33bc4d2578a95eac3e640eca11b3ac471930dd6f`. This is not qualification of the new batch.
Other prior native runtime/browser/optimizer evidence remains linked through
`.git/release-state/deployment-platform022-20261104/`; bulk logs and artifacts
remain under `target/deploy-platform022-20261104/`.

Earlier incoming/updated lock states and restoration evidence remain under that
Git-side directory, with older release evidence under
`.git/release-state/deployment-cleanup-20261104/` and `target/deploy-20261104/`.
The obsolete nested release checkout was removed with explicit approval;
760 KB of its plans/receipts remain byte-for-byte under
`.git/release-state/deployment-tools-0.77.0-testkit-0.77.0/historical-release-records/`.
No additional artifact cleanup is authorized or performed by this deployment.
