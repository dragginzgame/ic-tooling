# IC Tooling rewrite: consumer-driven contract reduction

Status: proposed, unapproved design, 2026-11-15. Implementation has not begun.
Repository setup and Toko Tooling adoption do not approve this proposal.

## Decision and scope

Build a smaller replacement from explicit consumer contracts, rather than copying the old workspace and deleting until it compiles. Preserve the Host/Build/Testkit ownership boundary. Public signatures may change; migrate supported callers together with each replacement. No compatibility aliases, forks, dual production implementations or retired product machinery should enter the new repository.

This is an architectural design based on the current manifests, exported surfaces, selected core implementations, release orchestration and local sibling source references. It is not a completed file-by-file correctness audit or proof that every unreferenced public API is unused. The accompanying inventory lists files, not claims of semantic review. No build, benchmark or native macOS qualification was performed for this design. Eliminating all future technical debt is not a verifiable promise; reducing independently maintained mechanisms is.

Source baseline: ic-tooling commit `3aaaec1aef866aa106a2445d1394cd4c74eafe02`, plus the existing uncommitted Platform 0.33.6 adoption. The original checkout and its dirty locks are preserved. This new repository is local, with no remote or publication configured. Existing release identities remain owned by the original repository until an explicit cutover.

## Evidence and scale

`inventory.tsv` records physical lines for current product, qualification and executable tooling files. `consumer-evidence.txt` records local sibling manifest/source/script matches with paths and lines. These are lexical observations: imports can be aliases or multiline, manifests can be unused, and current sibling trees can be dirty or on old package versions. They do not establish successful compilation or remote deployment.

All Rust files beneath the eight product crate directories currently total:

| Crate | Physical Rust lines, including tests/examples | Rewrite disposition |
|---|---:|---|
| ic-host-artifacts | 5,298 | Keep bounded bytes, hashes and required Wasm admission; scrutinize auxiliary surfaces |
| ic-host-fs | 11,077 | Keep safety contracts; consolidate implementations only after tracing publication states |
| ic-host-process | 9,186 | Keep bounded execution and ownership; review observation separately |
| ic-build | 9,982 | Keep required build/output and release-editor contracts; challenge package qualification machinery |
| ic-host-http | 363 | Keep narrow reader and selected Reqwest identity; little reduction available |
| ic-host-text | 531 | Keep terminal safety/table behavior; little reduction available |
| ic-testkit | 9,019 | Keep admitted server/client composition; challenge convenience lifecycle layers |
| ic-testkit-wire | 741 | Keep passive codec/call adapter and explicit budgets |

Total: 46,197 physical Rust lines. `src/` alone contains 40,918, including inline/separate source-tree tests. These are not comment-excluded application LOC. Tooling scripts, qualification, historical docs and vendor payload must be reported separately. Removing docs or tests must never be presented as equivalent production complexity savings.

## Required consumer capabilities

| Consumer evidence | Required behavior | Consequence |
|---|---|---|
| ic-backup/crates/ic-backup/src/store/payload/{mod.rs,source/mod.rs} | Exact stream hashing, bounded payload identity and filesystem custody | Preserve hash/length contracts and original I/O errors |
| ic-backup/src store capture paths and root Cargo.toml | Capability filesystem; native HTTP with explicit rustls selection | Do not replace held descriptors with pathname checks; retain bounded HTTP reader |
| ic-query/crates/ic-query-cli/src/presentation/*.rs | Bordered terminal tables | Retain safe escaping and Unicode width rather than introduce a formatting framework |
| ic-scheduling/qualification/ic-timers-pocketic/src/build/mod.rs | Cargo invocation, exact Wasm output, checksums, lock and durable publication | Keep one execution-to-admitted-artifact flow |
| ic-db/qualification/rollback-driver | PocketIC harness, Wasm inspection and terminal presentation | Preserve real rollback qualification; do not copy DB's product tests into Tooling |
| ic-platform/scripts/fixtures/cdk-runtime and qualification | Upstream client identity, server lifecycle, caller/runtime probes | Retain simulator dependency and explicit custody, never own its transport |
| ic-auth Cargo.toml and consumer evidence | Build and filesystem adapters | Compile actual callers before narrowing public selection |
| ic-storage Cargo.toml | Older Build/Foundation selections | Old versions require migration review, not automatic classification as obsolete |
| canic/crates/canic-host and qualification | Durable configuration/publication, bounded process capture and simulator client | Current source usage exists despite historical retirement discussions; obtain current workload decision |
| toko-tooling executable callers | Release metadata editor and package/release protocol | Treat command-line output and effect ordering as real external APIs |

Every implementation line must turn these observations into a caller contract matrix: exact symbol, caller, selected version/features/target, required success behavior, refusal/recovery behavior, and replacement. Resolve aliases and macro-generated callers. Include CLI/script uses; Rust imports alone are insufficient. Repositories not present locally and external public consumers remain a documented gap.

## Target architecture

Keep the eight package identities initially. Independent features and release cadence are useful here; merging HTTP or Wire into a heavier crate would increase coupling for negligible savings. Crate count is not the optimization target.

1. Artifacts: bounded byte handling, exact digest/length identity, explicitly selected Wasm parsing/text/gzip. No installation or lifecycle policy.
2. FS: descriptor admission, explicit scratch custody, locks and publication. One engine for each genuinely distinct semantic operation. Path convenience APIs resolve admission then delegate; capability APIs preserve held descriptor identity.
3. Process: owned child plus bounded communication. Build and server runners compose it. One cancellation/drain/reap authority, with distinct direct-child and process-group semantics represented explicitly.
4. Build: render one invocation, stream canonical Cargo messages, select exact outputs, retain invocation-owned artifacts. Separate package/release metadata obligations from generic application build calls within this owner.
5. HTTP/Text: retain their existing small responsibilities.
6. Testkit: published upstream PocketIC 16.1 client, authenticated server setup/admission, explicit server/instance custody, justified snapshot/install/progression helpers. Platform owns management DTO/codec behavior; Types owns portable principal identity.
7. Wire adapter: encode once, call once, decode once with caller budgets. No retries, server ownership, generated endpoint registry or application authorization.
8. Qualification: one independent private workspace for actual consumer composition and installed fixtures. Toko owns generic release machinery; local code supplies product contracts and qualified dependency coverage.

## Ranked implementation lines

### 1. Freeze requirements and define the deletion boundary

Read every public item and connect it to the caller matrix. Classify unobserved exports as unresolved until internal call chains, generated fixtures, CLI callers and maintainer workload intent are checked. Do not reproduce historical caches, retired Host Tools, native allocator crate, pools, renderer diagnostics, owned PocketIC transport or obsolete interface aliases. Most are already removed from current production; do not count their historical LOC as new savings.

Completion: every public capability is retained, moved to its actual owner, or explicitly retired; every required sibling has a migration/check command. No source deletion yet.

### 2. Simplify Build's qualification and observation pipeline

Inspect `crates/ic-build/src/cargo/{execution,observation,packages,lock_projection,release_metadata}` and `artifacts/staging`. Current execution already feeds Host communication into canonical Cargo message decoding; preserve that survivor rather than create another executor. Challenge intermediate metadata snapshots and repeated parsing only after comparing identity, freshness and error contracts. The package archive/lock evaluator must remain separate where it proves registry-packaged behavior rather than workspace compilation.

Hard cut: remove representations with no independent consumer or failure-state purpose, replace repeated query/parse passes with a single owned observation passed to directly affected operations. Keep release editor CLI behavior until Toko callers migrate in the same delivery.

Validation: real Cargo output selection, malformed/truncated output, competing artifacts, cancellation and captured evidence; package archive verification; release editor integration. Measure invocation count and wall time with identical inputs. Savings are unmeasured; do not impose a LOC quota before ownership mapping.

### 3. Collapse redundant release orchestration

Inspect `scripts/release/{validation,validation-focus,validation-stage,select-validation,validation-baseline}` and `scripts/ci/{workspace-validation,validation-roster,run-tooling-checks}`. The current focused runner still selects broad private Clippy and graph work, and product tests may recur through different leaves. Match executed commands and evidence identities before calling them duplicates.

Replacement authority: Toko's frozen release runner plus one consumer dependency/contract roster. Remove local general-purpose orchestration only when the installed producer supports the required contract. Never copy Toko machinery into the new repo. Keep archive admission, post-publication verification, failure receipts and clean frozen inputs.

Validation: changed product/reverse-consumer selection, stale evidence refusal, partial failure/resume, registry archive verification, Linux and both native macOS hosts. Do not remove required checks merely to accelerate deployment. Supplier contributions need separate destination scope.

### 4. Consolidate filesystem mechanics without flattening outcomes

Inspect `ic-host-fs/src/{read,capability,durable,scratch,owned_directory}`. `durable/mod.rs` exposes stream, named, retained, export and verified-copy paths; capability already delegates regular-file operations to descriptor engines. Find actual repeated mechanics before rewriting.

Candidate cut: one internal admission implementation per object type and one staged-file publication state machine, with thin intent-specific entrypoints. Keep create-only versus replacement, in-place synchronization, directory commit and non-aliasing export distinct where effects differ. Keep visible-but-not-synced errors separate from pre-publication failure. Remove wrapper types only when callers can still reconcile custody precisely.

Validation: symlink/rename races, partial writes, parent sync failures, occupied destinations, inode aliases, lock refusal, cleanup failure and retained staging ownership. Backup acceptance is mandatory. Persisted Backup formats stay consumer-owned and unchanged; destructive storage migration is outside this plan.

### 5. Simplify process communication and test scaffolding

Inspect `ic-host-process/src/{child,tool,framing,observation}` and consumer launch paths. Consolidate repeated output-limit/cancellation/error collection around the existing communication owner. Direct-child and process-group ownership cannot be collapsed into a generic kill-on-drop promise. Platform-specific procfs/libproc/Mach facts require a separate observed consumer before retirement.

Validation: full stdout/stderr pipes, truncation versus termination, cancellation, sink failure, child exit before drain, descendant lifetime, reap failures and macOS signals. Preserve typed execution/cleanup evidence. Replace duplicated fixture setup with a few direct helpers, not a configurable test framework.

### 6. Reduce Testkit to exercised harness policy

Inspect `pic/{startup,standalone,baseline,lifecycle,snapshot,time,server_assets}`. Determine whether StandaloneCanisterFixture and CachedPocketIcBaseline offer independently used behavior or just reconstruct startup/install calls. Retire a layer only with migrated actual callers. Explicit snapshot management remains justified by installed stable-memory restoration tests and management codec composition.

Keep setup/check separate from use; no implicit download. Keep caller runtime, sender, funding, deadlines and recovery explicit. Do not recreate the upstream client, own its DTOs, or promise typed transport errors it does not provide.

Validation: failed startup and exact child custody, external-server attachment ownership, instance cleanup, management refusals, cancelled operations, stable-memory restoration, caller identity and update side effects after reply decode failure. Platform/Runtime guard singleton and portable Types-only graph remain mandatory.

### 7. Rebuild tests around protected behavior

Tests are requirements evidence, not automatically baggage. Build a test-to-invariant table and delete tests only when their behavior is retired or another test proves the same inputs, side effects and independent failure mode. Keep small unit tests for combinatorial parsers/state transitions; installed tests prove composition and effects. Do not turn every unit test into an expensive PocketIC test.

Consolidate repeated test setup; retain independent crash/recovery, allocation refusal, cancellation and security cases. Keep generic Toko regression suites with Toko, product adapters locally, sibling product assertions in those siblings. Rebuild qualification scripts from the retained roster rather than copy the old directory tree.

### 8. Vertical migration and final cutover

Implement in dependency order: Artifacts, FS, Process, Build, narrow HTTP/Text, Testkit, Wire, qualification/deployment integration. Each slice contains implementation, caller migration instructions, failure tests and documentation. Maintain only one production owner after cutover; comparison binaries may exist temporarily as explicitly labelled validation artifacts.

The new repository must not publish existing crate names or change their repository metadata until registry ownership, release-group versions, provenance and the canonical Git destination are decided. User authorization here creates a local design repository, not duplicate publishing authority. Do not copy old Git state, build outputs, release receipts or managed tool payloads. Retain them in the original repository and reference exact identities.

## Tests and dependencies to retain deliberately

Keep upstream cryptography, HTTP, PocketIC, Wasm parsers and capability filesystem dependencies where they enforce required semantics. Replacing these with handwritten implementations merely to reduce dependency count increases risk. Restrict features and targets by actual consumer graphs. Keep native protocol adapters out of portable Testkit; keep Runtime in the private probe. Do not collapse independent Wire and Testkit releases.

A smaller macro source or Rust source tree does not prove smaller Wasm. Most of this workspace is host-only. Measure the private probe and any actual portable consumer separately with fixed compiler/features/optimization; report raw/optimized Wasm, instructions and allocation only when measured. No such savings are currently established.

## Acceptance and measurement

Before implementing each slice, record nonblank production, tests, scripts, generated/vendor and docs separately; exact caller graph; baseline command count/timing; and protected outcomes. Afterward compare the same inputs. Gross deletion and net reduction must be separate. A 10–25% reduction is a planning experiment, not a forecast supported by this inspection. The largest credible opportunities appear in qualification orchestration and repeated test setup, followed by Build/FS/Process representations. HTTP and Text together are under 900 physical lines and are poor rewrite priorities.

The replacement is ready only when required sibling callers compile against it, retained public/CLI contracts and independent failure modes pass, packages qualify from archives, and native Linux/macOS Intel/macOS ARM results are reported independently. Async hosted checks may run without blocking development, but failures cannot be described as qualification. No destructive storage changes are implied.

## Open decisions and rejected shortcuts

- Confirm the current Canic workload rather than infer retirement from old conversation history.
- Identify active consumers of process observation and each public baseline/snapshot helper before deleting them.
- Determine which release mechanisms the selected Toko version already owns; do not design against an assumed future API.
- Verify external consumers not represented by local siblings before any public retirement claim.
- Reject a universal Host crate, generic lifecycle registry, replacement PocketIC transport, unchecked std::fs simplification, loss of post-effect error evidence, and wholesale test deletion.
- A rewrite can reproduce the same debt with new names. Require a concrete deleted state/flow/representation for every proposed abstraction.

Best immediate work: finish the symbol-level caller and invariant matrix, then simplify one Build qualification flow and its tests end-to-end. Ideal end-state: the same required host and simulator capabilities, explicit custody, one authoritative execution path per semantic operation, a small product qualification roster, and no historical compatibility surface.
