Revision 2 — 15 November 2026. Accepted implementation contract.

The maintainer accepted this revision on 2026-11-15. Current implementation and
qualification boundaries are recorded in [the handoff](status/current.md).

Scope update, 2026-11-16: the maintainer confirmed Profiler is retired. Its
historical observation imports no longer require a replacement owner or native
qualification. Historical source evidence remains retained below.

This design supersedes the previous “IC Tooling rewrite architecture and launch gates” plan wherever it requires retaining existing crates, exports, signatures, release groups or test trees. The earlier source review remains evidence about behavior and consumers; it is not the target architecture.

**Build a new system from required workflows. Do not populate the new repository by importing the old workspace.** Preserve necessary user-visible outcomes and safety invariants. Redesign the mechanisms and Rust APIs that deliver them. Existing code may be reused only after the new owner, contract and state model are defined and the specific implementation has passed a reuse review.

The first milestone is a small, complete build-to-owned-artifact workflow implemented against newly designed interfaces. Six copied crates plus passing compatibility tests do not meet that milestone. Neither does a cosmetic rename of the old types.

Implementation elaboration: [the first-slice contract](build-owned-artifact.md)
selects bounded owned bytes as the initial artifact custody model. Capture stages
in memory and closes private build scratch before success; the returned artifact
has immutable byte access and ordinary Rust lifetime rather than fallible file
open/close. File-backed artifacts require a demonstrated workload. The
[workload register](workloads.md) scopes Stage 0 by required workflow; unrelated
unknowns do not block the first slice. Shared-target collision qualification is
deferred until shared-target support in Stage 3.

This is a standalone replacement design. It does not depend on access to the in-progress checkout and authorizes no deletion, repository publication or registry release. It establishes the rules for the next implementation.

## Implementation contracts

Read the affected owner's contract before changing its implementation. These
elaborate the accepted design; current work and blockers stay in the handoff.

| Owner | Contract |
| --- | --- |
| Build | [Owned builds](build-owned-artifact.md) |
| Cargo metadata command | [Release candidate rendering](cargo-metadata.md) |
| Artifacts | [Producer commitments](byte-commitments.md) |
| FS | [Preparation](directory-preparation.md), [publication](directory-publication.md), [locks](sidecar-locks.md), [custody](filesystem-custody.md) |
| HTTP / Text | [Bounded bodies](http-body.md), [presentation](text-presentation.md) |
| Process | [Execution](process-execution.md), [handoff/reaping](process-handoff.md) |
| Sign integration | [External keyring signing](sign-only-integration.md), outside generic Process and the core byte/build/simulator graphs |
| Testkit | [Held replies](testkit-held-replies.md), [managed server](testkit-server.md), [server command](simulator-ownership.md) |

# Why the first plan produced a transplant

The previous plan combined “greenfield” with four preservation defaults: keep eight package identities, retain unresolved exports, keep current signatures and import proven primitives with their tests. Those defaults made whole-crate adoption the easiest compliant action. They also made downstream compilation the main evidence of progress.

That was the wrong optimization for this task. Compatibility can validate a migration without demonstrating that the destination architecture is better.

The supplied LOC report shows 11,554 application/library lines and 19,304 test lines across six crates. It establishes scale, not architectural quality or unnecessary-test count. It does not establish a reduction against a matched old baseline. The replacement must earn its design through fewer obligations, clearer ownership and demonstrably removed mechanisms.

| Previous default | Replacement rule |
| - | - |
| Preserve eight crates | Derive packaging from completed workflows and dependency isolation |
| Preserve existing public exports | Start with no admitted public exports; add only requirements-backed capabilities |
| Keep signatures unless removal is proved safe | Design new signatures; migrate supported callers explicitly |
| Import a crate, then simplify | Specify behavior, implement a vertical slice, then evaluate narrowly scoped reuse |
| Copy tests to preserve safety | Extract invariants and independent failure cases; implement tests against the new contract |
| Treat all current consumers as architectural constraints | Preserve required workload outcomes; allow consumer code and dependency graphs to change |
| Optimize release orchestration first | Establish the product architecture first; keep development and release tooling subordinate |
| Passing old tests demonstrates progress | Passing new acceptance tests plus a recorded structural improvement demonstrates progress |

Unknown exports stay in the old release, not automatically in the new system. An unresolved *required workload* blocks its migration; it does not justify importing an entire module. Old versions remain available while the replacement is developed.

The old code is neither presumed good nor presumed bad. “Already tested” establishes useful evidence; it does not establish that its abstraction belongs in the new architecture.

# Scope and nonnegotiable outcomes

The product exists to support these workflows. Every included capability must trace to at least one of them.

| Workflow | Required result | Irreducible failure behavior |
| - | - | - |
| Admit and identify bytes | Complete bounded bytes or exact stream digest/length | Distinguish limit, short input, allocation and original I/O refusal; no false complete result |
| Safely read and publish local artifacts | Access through appropriate held identity; publish an explicitly selected destination | Distinguish before-publication failure from visible-but-not-durable outcome; preserve foreign entries |
| Run a native tool | Selected command executes with bounded communication and explicit cleanup scope | Drain both streams; represent cancellation, incomplete evidence and cleanup failure |
| Build and retain an artifact | Exact requested Cargo target captured under owned custody | Refuse stale/ambiguous/failed output; do not return a mutable target-directory pathname as the product |
| Exercise canisters | Explicitly prepared simulator, caller-owned runtime, deliberate instance/snapshot lifetime | Preserve external-server ownership, partial effects and upstream limitations |
| Qualify a release | Exact candidate artifacts accepted by selected consumers on required hosts | Refuse stale evidence, incomplete stages and source/tool identity drift |

HTTP body reading, terminal rendering and Wire calls are small adapters used by workflows, not reasons to predetermine package topology. Process sampling, release metadata transformation and generic release execution must each justify their owner separately.

Preserve persisted application formats, exact hash inputs, application Wire schemas, required terminal/CLI outputs, caller identities, destination semantics and recovery information unless a separately designed consumer migration explicitly changes them. Preserve upstream type identity when values cross an upstream API. **Rust source compatibility, old helper names, old crate boundaries and existing internal error nesting are not requirements.**

Required host behavior remains Linux x86-64 and native macOS Intel/ARM for launch. Portable code must not acquire native dependencies accidentally. Do not add a PocketIC transport fork, handwritten cryptography, general plugin framework, universal operation executor, lifecycle registry or generic transaction framework.

The first launch scope includes the required behaviors of the previously observed Backup, Scheduling, Canic, Auth, Storage and DB consumers, subject to the maintainer's current hold/freeze directions. Profiler was explicitly retired on 2026-11-16. This is a workload list, not a commitment to reproduce all of their current imports. Resolve the Platform/Query evidence gaps and any additional supported callers during contract freeze. Moving a capability to a different owner requires its own migration; omission from the new tree is not proof that the workload has been retired.

# Target ownership model

Define five product owners and one external development plane. These are semantic modules first. No old package or public module tree is imported to establish them.

| Owner | State it owns | Public boundary | Deliberately excluded |
| - | - | - | - |
| Byte admission | Byte accounting and exact content identity | Bounded read/hash/copy operations; digest and length | Paths, child processes, credentials, persistence |
| Filesystem custody | Held directories/files, staging, publication and locks | Open/admit, stage, seal, commit, explicit cleanup | Build recipes, consumer data formats, retry policy |
| Process session | One child or explicitly owned process group and its communication | Spawn, bounded communicate, explicit shutdown or detach | Cargo JSON, simulator protocol, process sampling policy |
| Build operation | Invocation plan, target lease, decoded Cargo selection and captured output | A build request producing one owned artifact | Release version choice, simulator ownership, global cache |
| Simulator session | Managed server/borrowed endpoint and owned test instances | Explicit preparation/attach/start/close and narrow harness operations | Build execution, alternate transport, implicit global runtime |
| Development and release plane | Consumer qualification recipes and product release acceptance | Thin product data/adapters to the selected Toko interface | Runtime library dependencies or a second generic runner |

Dependencies point down: Build uses Process, Filesystem and Bytes; Simulator uses Process and Filesystem where necessary; Filesystem uses Bytes; Process may use narrowly scoped filesystem admission. Bytes has no upward dependency. Build and Simulator do not depend on one another. A test fixture may compose them outside product code.

Keep publication and process cleanup as separate state machines. They have different authority and failure semantics. Sharing an error container or “resource manager” is not a simplification.

**Packaging decision:** implement the first workflow with private, separately named modules and explicit dependency boundaries. At the architecture checkpoint, choose physical crates using these tests: independent external use, feature/target isolation, public type ownership and independent release need. The expected split follows the five owners, but HTTP/Text/Wire or byte-only consumers may require separate leaves. A module becoming a crate must not change its semantic owner.

Do not publish a catch-all facade, and do not make all consumers enable native filesystem/process dependencies merely to hash bytes. Conversely, do not create a package solely to preserve an old name. Record the final package graph before the first consumer migration is accepted; “we will decide later” cannot survive that checkpoint.

Start with no release-group inheritance. Coordinate packages that exchange tightly coupled public types; give optional adapters independent release boundaries when their upstream contracts require it. Crate count and release-group count are outcomes, not success metrics.

The maintainer confirmed that existing crates.io names may be reused. Prefer an
existing name when it still describes the selected owner, with an incremented
version based on the published line. Incompatible 0.x APIs require a new minor;
keeping the name grants no legacy export or implementation admission. The first
build owner uses `ic-build` 0.87.1 and remains unpublished during development.

# New operation contracts

The sketches below define the proposed shape and semantics. They are design interfaces, not existing or compiled Rust APIs. Implementers may adjust Rust spelling and lifetimes while preserving the stated authority, transitions and outcomes. Any semantic change requires updating the contract before code.

## Byte admission

Keep ordinary Read/Write integration rather than a configurable I/O backend. Separate complete buffered reads from constant-memory hash/copy operations. Length policy is explicit: AtMost(n) or Exactly(n). On exact/ceiling verification, one extra byte may be observed to detect growth; no successful identity is returned for a refused stream.

Return the byte count actually admitted and SHA-256 over the original bytes. Preserve the original I/O cause; use fallible allocation for bounded owned buffers. Do not promise that an allocator or upstream decompressor uses no additional memory.

A copy to an arbitrary writer can have partial effects. Only the Filesystem owner can turn that stream into atomic named publication. No Byte API reports “durable,” “trusted origin” or “immutable.”

Compression and Wasm inspection are adapters over byte admission. Distinguish compressed-input, decoded-output and work limits; distinguish Wasm structural inspection from validation. Reuse maintained upstream codecs/parsers after selecting their required features.

## Filesystem custody

The new API centers on an owned publication attempt, not a family of path wrappers and callback variants:

```rust
// Conceptual API; error and platform types are specified by the contract.
let directory = Directory::open(root, directory_policy)?;
let mut stage = directory.stage(name, publication_policy)?;
stage.write_from(reader, length_policy)?;
let sealed = stage.seal()?;
let result = sealed.commit();
```

Directory holds the admitted directory identity. A single-component destination name is admitted before effects. Publication policy explicitly selects CreateNew or Replace and staging permissions. Parent creation is a separate explicit operation; ordinary publication does not silently create a hierarchy.

Stage owns its temporary entry and the only library-granted writable access. Seal ends writable access and syncs the file before commit. The safe API does not leak clonable writable handles that bypass the lifecycle. Integration requiring an external writer is a distinct admitted protocol with identity revalidation, not a callback allowed to do anything.

Commit consumes the sealed stage. Its outcome distinguishes:

| Outcome | Meaning | Ownership/evidence returned |
| - | - | - |
| Committed | Publication and required directory synchronization completed | Destination observation and content identity, with documented future-use limits |
| NotPublished | This attempt did not publish | Primary cause, cleanup cause if any, and retryable stage ownership when safely recoverable |
| PublishedUnconfirmed | New destination is visible, final durability/cleanup is not confirmed | Exact target/content evidence and failed phase; never a misleading “safe to retry” flag |

No automatic publication retry. CreateNew checks absence at the publication operation. Replace does not follow a final symlink. Held-directory operations remain relative to that descriptor through parent rename. Foreign replacements of staging are never removed by cleanup.

Expose a separate read-only staged validation operation only when a required consumer needs it, such as executable admission. Directory commit, in-place synchronization and non-aliasing export remain distinct *requirements* where effects differ; do not force them into this file-publication API. They enter the implementation only in the vertical slice that needs them.

Explicit abort/close reports failure. Drop is fallback cleanup and must not be the only observable cleanup path. Neither file ownership nor a read-only descriptor proves immutability against unrelated writers; the consumer's concurrency assumptions remain explicit.

## Process session

Replace overlapping launch/capture/run/drain wrappers with one session owner and narrow convenience functions only where a consumer needs them:

```rust
let mut session = ProcessSession::spawn(command, ownership)?;
let report = session.communicate(input, communication_policy, observer);
let cleanup = session.close(cleanup_policy);
```

Ownership is DirectChild or OwnedGroup. A borrowed PID never implies kill authority. Communication policy defines stdout/stderr retention ceilings, overflow behavior, cancellation and optional elapsed deadline. Termination-on-limit and truncate-and-drain remain different policies.

One engine pumps input and drains both outputs, tracks leader exit independently of pipe EOF, and retains primary plus cleanup failures. An observer sees bounded chunks; it may not take child ownership or mutate the command. Session close is idempotent in library state: it cannot signal a reused process identity after ownership ended.

Define explicit transitions: spawned → communicating → leader exited/draining → shutdown attempted → reaped/closed, with detach as a deliberate ownership transfer. Returning from communication does not implicitly mean cleanup succeeded. A convenience run-to-completion operation must execute and report both steps.

Async consumers must retain the session owner outside a cancellable communication future, or use an explicitly owned supervisor. Dropping a future must not silently lose the only cleanup handle. Do not promise guaranteed async teardown from Drop or a hard bound on every kernel operation.

Remove public wrappers whose only purpose is choosing hidden defaults. Keep application-specific command construction outside this owner. Process observation is not folded into execution.

## Build operation

The main product result is an owned artifact, not a triple of independent Results plus a pathname that every consumer must assemble correctly:

```rust
let request = BuildRequest {
    source,
    target,
    recipe,
    toolchain,
    limits,
    evidence_policy,
};
let artifact = build(request, cancellation)?;
consume(artifact.open()?);
artifact.close()?;
```

BuildRequest specifies exact manifest/package/target identity, library or named example, target triple, profile/features, explicit environment policy and output/protocol/candidate limits. Avoid a builder with a large set of unrelated optional flags. Unsupported combinations refuse before spawning.

The operation owns the ordering:

1. Validate request and establish the target-directory lease.

2. Render one immutable invocation plan.

3. Execute through ProcessSession and decode Cargo records once.

4. Require process success and the necessary complete protocol/evidence.

5. Select exactly one matching artifact from bounded retained facts.

6. Capture bytes into owned staging while the target lease remains held.

7. Return the owned artifact and release the lease.

A successful result cannot contain failed protocol/tool observations. Failure retains the command status, relevant protocol refusal, bounded diagnostics and cleanup state without making callers traverse arbitrary nested generic errors. Nonzero process exit remains primary when JSON is also malformed.

Default to a private invocation target directory for the first implementation. An explicitly shared incremental target requires a held lease for the entire execute/select/capture interval; an advisory lock coordinates only participating writers. Model it as a separately admitted mode. Do not add a persistent artifact cache.

One private streaming accumulator retains completion, exact output candidates and only required build-script facts. It does not retain every Cargo message. Raw diagnostics may be streamed to a bounded caller-selected sink; canonical third-party message structs are not automatically reexported as the product API. Add a diagnostic extension only for demonstrated consumers.

Separate ordinary developer builds from release-qualified builds. Ordinary builds need correct output/custody; they do not automatically need a release proof. Qualified execution adds immutable source inventory, admitted tool identities and before/after mutation checks. Shared execution machinery remains one owner. Existing identity probes can be replaced only by an equally explicit tool-custody guarantee, never merely to reduce subprocess count.

This is a substantive API break: callers stop manually combining spawn, decode, success checks, selection, lock lifetime and capture. The old workflow is the behavioral oracle, not the construction template.

## Simulator session

Preparation is separate from use. A session either owns a managed server or attaches to an externally owned endpoint. These states must not grant the same shutdown authority. Use the caller's runtime and the published upstream client; do not rebuild its transport or invent guarantees beyond it.

Instances retain their server dependency until explicit close finishes. Report what is actually known: “local server stopped” and “remote deletion attempted” are not equivalent to confirmed remote cleanup.

Keep snapshot sets, install policy and simulated progression as separate operations. They enter only with required workflows. Snapshot capture preserves caller identity and partial-effect cleanup evidence; restore does not imply rollback of wall/simulated time or unrelated state. Stop/drain, funding and restart are explicit caller decisions.

Do not introduce CachedBaseline, StandaloneFixture or pool layers by default. First implement the workflow using session, instance and snapshot-set ownership. Add a wrapper only when two real consumers demonstrate the same invariant that the wrapper can enforce. A tuple of values is not automatically a new public lifecycle type.

Wire remains a narrow adapter: encode once, call once, decode once, with explicit caller and per-direction budgets. Decode refusal after update may follow a committed mutation; the adapter never retries. Upstream reply allocation and transport panic behavior must remain accurately described.

# Reevaluate peripheral responsibilities

| Existing area | New default | Admission condition |
| - | - | - |
| HTTP body reader | Small adapter over the selected upstream client | Required consumer needs complete bounded body semantics; preserve type identity/TLS isolation |
| Terminal text/tables | Small pure presentation adapter or consumer-local utility | Actual consumers need identical escaping/width/output semantics; no framework |
| Process sampling/procfs/macOS observation | Outside ProcessSession; no admitted replacement | Profiler is retired by maintainer direction; any future consumer must demonstrate its own measurement contract |
| Cargo manifest/lock release editor | Development-plane tool, outside the core build-to-artifact operation | Use an admitted released editor initially; transfer to Toko or implement a separately scoped tool only against a specified protocol |
| Archive and external-lock checks | Qualification capability, not ordinary application-build API | Auth/release consumers still need them; expose a focused separate boundary if public use is required |
| Generic runner, receipts, release transaction | Selected Toko supplier | Verify its real supported interface; no local fork or assumed future feature |
| Managed downloads and tool installation | Explicit preparation plane | Authenticated asset/source selection and no implicit install during use |
| Test support | Local helpers or dedicated private qualification | No public test-support surface merely to preserve old tests |
| Old aliases, pools, caches, fork transports and historical interfaces | Excluded | A newly approved requirement would need a new design; history is not a requirement |

“Outside the core” is an ownership decision, not authorization to drop an active workload. For example, an Auth external-lock checker must keep working after migration even if it no longer lives in the Build library. Choose its destination and qualify it before switching Auth.

Reusing the old release editor as an explicitly pinned development executable does not make the old product library part of the new runtime architecture. It is a temporary bootstrap dependency with an owner and removal/retention decision, not a compatibility facade in the replacement.

Dependency selection starts from the admitted operation, target and features. Carry forward neither the entire root manifest nor a pruned copy of its lockfile as a design decision. Cargo generates the replacement graph from chosen requirements; compare it with the baseline to catch unintended drift. Maintain separate locked product and qualification contexts where needed. Reuse upstream libraries for cryptography, capability I/O, parsing, HTTP and simulation where appropriate; minimizing dependency count is subordinate to eliminating duplicate mechanisms.

# Strict implementation rules

These requirements apply to every implementation increment. They intentionally override the previous preservation-first design.

1. **No bulk import.** Do not copy a crate, src tree, test tree or old workspace configuration into the new implementation as a starting point. A newly created module begins with its requirement and interface. A tiny adapter may ultimately reuse all of a small implementation, but it still requires the same prior design and reuse record; package scaffolding is newly selected.

2. **No automatic export admission.** Every public item names a supported workflow and caller or enforces a necessary ownership invariant. Unknown old exports stay in the old release until evaluated.

3. **No compatibility accumulation.** The replacement has no forwarding aliases, deprecated old entrypoints, hidden old/new dispatch or “temporary” legacy modules. Migration adapters live in the consumer change or isolated comparison harness and have a removal milestone.

4. **No public generic framework before use.** A trait, builder, provider, callback hook or registry must solve a demonstrated boundary. Ordinary Read/Write integration is legitimate; a custom filesystem/process abstraction solely for mocking is not.

5. **One effect owner.** Exactly one component owns publication, child cleanup, artifact lifetime or managed-server teardown for each object. Convenience layers may not silently duplicate that authority.

6. **No public invalid success states.** A successful build result must already satisfy its documented admission checks. A failed write attempt cannot later be sealed as complete without an explicit new attempt. Shutdown/cleanup refusal cannot disappear behind success.

7. **No fallback guesses.** No stale artifact lookup, implicit download, inherited policy defaults, unrequested retries or path-based substitute for held identity.

8. **No test-tree transplantation.** Extract protected behavior and the oracle first. Reuse individual failure fixtures/tests only after mapping them to the new contract. Tests of retired implementation structure do not enter.

9. **No tooling expansion as substitute progress.** Compiler/tool preparation is allowed when it unblocks a slice. New dashboards, release planners, elaborate audit generators and generic fixture runners are not product redesign milestones.

10. **No progress inflation.** Report reused, newly implemented and migrated code separately. Passing compatibility checks is migration evidence. Architectural progress requires naming the old obligation, state, representation or path removed.

11. **No broad “retain for safety” exception.** If a needed failure guarantee is not understood, investigate and specify it before implementing that operation. Do not import the owning crate to make the uncertainty disappear.

12. **No launch by partial qualification.** The final candidate must pass its supported consumers, packaged-archive checks and all required native hosts. Reduced surface does not excuse weaker evidence.

At project restart, the implementation owner must align README, AGENTS and active status instructions with this document. Remove directives to retain Foundation/Build contracts and tests wholesale. Preserve those documents as historical evidence where useful, but clearly mark them superseded. The restart performs this alignment; the current handoff records its implementation status.

# Reuse admission

Reuse is allowed after architectural selection. Rewriting a correct hash loop just to claim new code is not useful; importing its surrounding API family without reconsideration is equally unjustified.

Each proposed reuse has a compact record:

```text
Requirement: exact stream digest without complete buffering
New owner: byte admission
Selected source: repository + commit + exact file/function
Why this unit fits: no filesystem/process policy; explicit length ceiling
Inherited assumptions: Read behavior, interrupted I/O, one-byte overflow probe
Public surface carried forward: none unless independently selected
Dependencies introduced: explicitly listed
Tests carried forward: named invariant and independent oracle
Changes needed: new error/type boundary, if applicable
Decision: reuse unchanged / adapt / reimplement / exclude
Reviewer evidence: source read and focused acceptance results
```

A reuse decision must consider global state, hidden I/O, defaults, target assumptions, error identity, allocation limits, dependency leakage and cleanup authority. “Stable,” “good,” “free of drift” and “already tested” are conclusions requiring that evidence, not sufficient explanations.

Reuse the smallest coherent semantic unit. That may be a pure function, a parser or a complete carefully bounded OS mechanism. Splitting a delicate native mechanism merely to make the reuse look smaller is also unacceptable. Preserve attribution and original safety rationale.

The review is part of the implementation increment, not a separate user-permission request for every function. The implementer makes routine decisions and records them. A significant change to the architectural contract must be called out for review.

# Tests designed from obligations

Build a small requirements-to-test map. It is not a second test framework. Keep fast parser/state tests, native fault tests and actual consumer integration tests at their proper levels.

| Obligation | Required independent oracle |
| - | - |
| Exact stream identity | Known bytes/digest plus short/growing/erroring readers; not round-trip through the same implementation |
| Publication phase truth | Inject failure before publication and after visibility; inspect actual destination and staging entries |
| Descriptor custody | Rename/replace ancestors or staging entries; assert held identity and refusal to clean foreign state |
| Process communication | Real pipes filled concurrently, blocked stdin, child/descendant lifetime observations and cleanup result |
| Exact build capture | Real Cargo fixture with ambiguous names/shared-target collision; retained output remains the selected bytes |
| Simulator ownership | Managed process lifetime versus external endpoint survival; explicit instance close behavior |
| Snapshot semantics | Raw stable-memory/state oracle; capture sender and partial cleanup rejection evidence |
| Wire side effects | Raw state query shows one committed mutation after response decode refusal; no retry |
| Release proof | Mutated input/tool/host/receipt and failed logger/stage all refuse success |
| Migration completeness | Real consumer workflow runs against packaged replacement, with no hidden old dependency |

Preserve independent failure modes even when their test bodies look similar. Consolidate setup only when it does not hide the assertion or introduce configurable fixture machinery. A low test count is not a goal; a smaller set of maintenance obligations is.

Do not initially copy all test-support machinery to run an old suite. The old suite remains in the reference checkout as an oracle. Port only relevant behavior tests against the new interface. Full old-suite parity is neither sufficient nor always appropriate when APIs and defaults deliberately change.

Source inspection is not execution evidence. Distinguish compile-only, unit, real OS, installed simulator, archive and native-host results in every report.

# Implementation sequence and hard gates

The following sequence replaces “Artifacts crate, then FS crate, then Process crate…” as the implementation plan. Every stage produces a usable vertical result, not a collection of imported packages.

## Stage 0 Freeze the requirements and reset the implementation contract

Deliver a workload register with required success/effect/refusal behavior and a migration destination for each supported consumer. Classify every old capability as needed in the new core, needed elsewhere, retired, or unresolved. An unresolved required capability blocks its cutover; other independent slices proceed.

Preserve an identifiable reference snapshot and any useful evidence before a user-directed reset. Do not delete original production code, histories, release receipts or consumer data. The new workspace starts with selected tooling, contracts and tests only. Existing in-progress code receives no privileged status.

**Gate:** no old crate structure is treated as mandatory; every first-slice capability has an owner and observable acceptance contract.

## Stage 1 Prove build to owned artifact

Implement the minimum byte, custody and process operations required to compile one real Wasm target and return an owned captured artifact. Use a private target directory initially. Add zero filesystem/observation/snapshot utilities merely because they existed before.

Exercise cancellation, malformed Cargo output, ambiguous output, sink failure and capture cleanup. Demonstrate that the consumer cannot accidentally consume the ordinary mutable Cargo output after another build.

**Gate:** one real end-to-end flow passes; no entire old crate imported; Build owns lease/select/capture ordering; new API has no independent unchecked success results. Review the module dependency graph and choose initial physical package boundaries before Stage 2.

## Stage 2 Prove durable publication and Backup custody

Extend the Filesystem owner with explicit staging/sealing/commit and required held-directory reads/locks. Migrate the needed Backup flow to the new API in an isolated consumer checkout. Parent creation and destination policy become explicit.

Prove exact bytes, root rename behavior, create-only races, foreign staging replacement, partial write and visible-but-unsynced failure. Preserve Backup's persisted format and original I/O causes where its recovery needs them.

**Gate:** Backup behavior passes using new interfaces; no legacy path-helper family added merely to compile the caller.

## Stage 3 Prove shared build reuse and actual consumers

Add an explicitly leased shared incremental target only if measured workloads justify it. Migrate Scheduling's build workflow to the owned-artifact API and qualify its installed behavior, not just its host helper tests. Handle Auth/Storage package and external-lock needs through the chosen development-plane boundary. Migrate Canic's required host behavior.

**Gate:** each supported caller has a real workflow result; old supplier references are absent from the migrated path; any remaining public extension is justified by a concrete requirement. Cold/private versus warm/shared measurements are reported separately.

## Stage 4 Prove simulator ownership

Implement explicit prepared-server attach/start, instance lifetime and close behavior around upstream PocketIC. Add snapshots only through the DB/Platform acceptance workflow. Add Wire adapter behavior with the committed-update oracle.

**Gate:** no transport fork, pool or baseline convenience layer imported; external servers survive client cleanup; partial effects and upstream limits are accurately exposed; portable dependency isolation passes.

## Stage 5 Resolve leaf and displaced capabilities

Evaluate HTTP/Text reuse individually. Profiler observation and native acceptance are no longer required after its explicit retirement. Finish any required CLI/editor/qualification capability that moved out of the core. Do not merge supplier work into this repository merely because it is easier to edit.

**Gate:** no required workload is silently lost; no unneeded peripheral surface has entered the product; final export/package/release graph is documented.

## Stage 6 Qualify and cut over

Use one product qualification roster through the selected Toko runner. Until focused equivalence is demonstrated, release gates use complete required validation. Package artifacts, remove path overrides and execute actual consumers against the candidate archives. Run Linux x86-64 and both native macOS architectures.

Select new incompatible release lines where API changes require them. Publish dependency-ordered packages through one canonical authority, verify registry readback and only then switch the supported consumer cohort. Explicitly handle partially published cohorts; publication is not atomic.

**Gate:** complete candidate/consumer/host evidence, no old runtime implementation or aliases in the replacement, migration ledger complete and rollback rehearsed.

# Compatibility and cutover

A ground-up redesign deliberately breaks Rust API compatibility where that removes caller obligations or ambiguous ownership. Migrate imports, data types and orchestration together. Retaining an old name is allowed when it fits; preserving all old names is not a design goal.

Preserve consumer data formats and protocol bytes unless separately versioned. Treat CLI flags, environment variables, stdout and exit behavior as external contracts: either retain them in a narrow tool boundary or migrate all supported script callers in the same delivery. Unknown callers remain on the old release line.

During development, old and new may run in separate comparison processes/workspaces. They must not coexist as selectable production implementations inside the replacement. No dual-write or retry-through-legacy fallback.

All replacement packages remain unpublished until packaging and authority are decided. Do not inherit old versions or six release groups simply because files were copied. Incompatible 0.x changes need an incompatible version boundary; new package identities may be appropriate if ownership has materially changed. Registry availability is checked at implementation/release time.

Rollback before publication abandons the candidate. After publication, old immutable releases remain available, and consumers can restore the previous qualified cohort where persisted-state compatibility permits. Rollback does not undo external effects or erase published versions.

# Acceptance scorecard and stopping rules

Every increment reports these five things:

1. Required workflow now supported and its new public boundary.

2. Old responsibility, representation or dispatch path eliminated.

3. Code reused versus newly implemented, with reuse records.

4. Exact acceptance tests executed and outstanding evidence.

5. Remaining public API/dependency obligations and any measured performance change.

Baseline and candidate measurements separate product code, tests, examples, local tooling, supplier/vendor code and documentation. Also count public capabilities, exposed supplier types, independently owned state machines, command invocations and caller steps needed to perform each workflow. Source-file count alone is not useful.

No arbitrary percentage deletion or mandatory “new code” quota is imposed. No wall-time or Wasm-size improvement is claimed from LOC. Set numerical performance budgets after matched baselines exist and before optimizing. Correctness gates do not substitute for latency/memory evidence, and faster private-target builds must not be compared misleadingly with warm shared-target builds.

Reject an increment if it imports a whole legacy subsystem before its new contract exists, adds compatibility wrappers instead of migrating callers, recreates the old multi-step caller obligations under new names, or presents dependency/tooling maintenance as architectural completion.

Stop expanding the implementation and repair the design if:

- a simple workflow requires unrelated generic providers or nested configuration;

- multiple objects believe they own the same cleanup;

- a successful result can still represent failed admission;

- a feature pulls an unrelated runtime into byte-only or portable consumers;

- qualification requires the old local workspace to make a package work;

- “we kept it just in case” is the only reason for a public API;

- tests mostly assert the old structure rather than the required behavior.

The first architecture checkpoint is Stage 1, before building out the remainder of the system. If build-to-owned-artifact cannot remove the consumer's manual sequencing burden with a small coherent implementation, do not continue importing functionality. Revise that boundary first.

# Source facts and proposal status

This revision uses the previously reviewed public Tooling baseline a00369fc29fa837c6deb022cceb7e09aa447292c and the user's supplied LOC report to explain the reset. It does not claim a fresh audit or execution qualification of the in-progress replacement.

Verified baseline facts relevant to this redesign: Build exposes separate execution/message/facts results and delegates communication to Host; filesystem publication already distinguishes pre/post-publication errors; Wire already has one encode/call/decode path; native Testkit selects upstream PocketIC; Profiler consumes process observations. Those existing safety facts are obligations to understand, not instructions to reproduce the same APIs.

Sources: [Build execution](https://github.com/dragginzgame/ic-tooling/blob/a00369fc29fa837c6deb022cceb7e09aa447292c/crates/ic-build/src/cargo/execution/invocation.rs), [filesystem publication errors](https://github.com/dragginzgame/ic-tooling/blob/a00369fc29fa837c6deb022cceb7e09aa447292c/crates/ic-host-fs/src/durable/named/mod.rs), [Wire adapter](https://github.com/dragginzgame/ic-tooling/blob/a00369fc29fa837c6deb022cceb7e09aa447292c/crates/ic-testkit-wire/src/lib.rs), [Testkit identity](https://github.com/dragginzgame/ic-tooling/blob/a00369fc29fa837c6deb022cceb7e09aa447292c/crates/ic-testkit/src/lib.rs), [Profiler consumer](https://github.com/dragginzgame/ic-profiler/blob/e4c165b28c4450bfd4fbda6dbc3f9c513b313753/profiler/system/src/host.rs).

The operation interfaces, ownership split, packaging gates and implementation rules were proposed by this revision and subsequently accepted by the maintainer. The source review above is historical evidence, not qualification of later work. See the current handoff for executed acceptance tests and remaining boundaries.
