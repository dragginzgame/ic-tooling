# First slice: build to owned bytes

Stage 2 update: shared byte admission and descriptor access now delegate to
`ic-host-artifacts` and `ic-host-fs`; the checkpoint below records the initial
Stage 1 graph. Temporary directory creation explicitly requests mode 0700, with
a permissions assertion, rather than relying on tempfile's directory default.
See [filesystem custody](filesystem-custody.md) for the extraction and qualification.

This is the Stage 1 elaboration of [design revision 2](DESIGN.md). It admits one
ordinary developer build: one selected package's cdylib library for
`wasm32-unknown-unknown`, using Cargo with `--locked`, an explicit compiler,
profile, features, cleared/repopulated environment and cancellation/deadline.
Scheduling's manual execute/decode/check/select/capture sequence motivates this
boundary; its full installed simulator flow is not migrated in this slice.

## Boundary and custody

`build(request, cancellation, diagnostics)` returns one `Artifact` or a
`BuildFailure`. There are no public process, JSON selection or staging exports.
The request names an absolute manifest, package selector, exact Cargo package ID,
library target name, Cargo and rustc executable paths, scratch parent, explicit
environment entries, profile/features and all limits. Source/tool selection is
caller authority; this is not a release-proof or hostile-build sandbox.
Cargo uses its configured network mode. A caller selecting offline mode through
the environment passes `CARGO_NET_OFFLINE` in `BuildRequest::environment`.
Reject unsupported/invalid inputs before spawning. Cargo config/build scripts are
trusted executable build inputs. The caller must prevent concurrent source/tool
mutation; qualified release source/tool custody remains later work.

Each invocation creates a mode-0700 private target directory beneath the supplied
scratch parent. That parent is caller-owned, stable and not concurrently renamed
or modified. Cargo and its cooperating descendants are the only writers in this
private directory; same-UID hostile processes and children that escape the owned
process group are outside this first slice's custody guarantee. No shared-target
mode or advisory lock is offered.

Capture opens the selected file relative to a held target directory, walking
components with no-follow directory opens and refusing non-regular output. It
reads with an explicit byte ceiling and fallible buffer growth. The buffer is the
owned staging area: no partial buffer becomes an Artifact. SHA-256 identifies
exact captured bytes. No source-file immutability or source authenticity is claimed.
The operation explicitly removes its private target before returning success.
A failed cleanup returns failure, including any earlier primary failure.

An Artifact owns the completed buffer and digest; it exposes immutable byte
access, with no target pathname or writable file handle. Rust ownership ends its
memory lifetime; there is no fallible artifact close operation. This deliberately
narrows the conceptual file-backed open/close sketch: a bounded Wasm consumer
already needs complete bytes and does not need a second filesystem lifecycle.
Do not add file-backed output without a demonstrated memory/workload requirement.
Durable named publication, locks and externally writable staging are Stage 2+.

## Process and protocol

The private process owner creates an owned process group, uses null stdin for
Cargo, and fairly drains nonblocking stdout and stderr. Both retained diagnostics
and individual JSON records are bounded. Total stdout and elapsed time are bounded.
Caller diagnostics are raw bytes sent once to a supplied Write sink; the caller
must provide a responsive sink. Sink refusal is retained and never becomes success.
Cancellation is checked during communication and capture. No async runtime is added.

Observe leader exit without reaping, finish/terminate group ownership before
reaping, and never signal a recycled leader ID. Explicit shutdown reports failures;
Drop is fallback only. Cancellation/deadline/bounds trigger termination. Malformed
protocol retains its refusal while draining: a natural nonzero process exit stays
primary when protocol is also malformed. Cleanup failure is never discarded.

Decode newline-delimited Cargo JSON once. Keep one exact matching candidate and
completion state, not all messages. Validate relevant field shapes, require exactly
one successful build-finished record and complete EOF framing. Reject ambiguity,
missing completion, unsuccessful completion, stale/fresh artifacts, invalid paths,
and candidate-count or record limits. Ignore well-formed unrelated records.
No build-script facts enter until a concrete consumer requires them.

## Acceptance map

| Contract | Independent evidence |
| --- | --- |
| Exact bounded capture | Known SHA-256 bytes; ceiling, interrupted/erroring reader and cancellation cases |
| Filesystem custody | No-follow traversal rejects replaced directory/file symlinks; non-regular output refuses |
| Process completion | Real pipes, natural failure, cancellation/deadline and descendant-held pipe tests |
| Protocol completeness | Hand-authored malformed, truncated, duplicate/missing completion and ambiguous target records |
| Error precedence | Nonzero executable plus bad protocol; sink refusal and cleanup refusal remain distinguishable |
| End-to-end ownership | Real dependency-free Wasm fixture; two independent builds; first bytes unchanged after second and scratch removed |
| Packaging boundary | One build-operation package with private modules; no portable byte API exposing native dependencies |

No old implementation or tests are admitted for reuse initially. Rust standard
library and selected registry libraries supply process primitives, native descriptor
operations, private directory creation, JSON decoding and cryptography. Their
implementation text is not copied. Reconsider any legacy reuse with the design's
per-unit record before adding it.

Stage 0 inventories workload families rather than every lexical export. Unknown
required capabilities block their own migrations. Stage 1 tests private-target
isolation; shared-target collision qualification belongs to Stage 3.

## Stage 1 architecture checkpoint — 2026-11-15

The initial physical package is `ic-build`, unpublished at a development
version of 0.87.0. Its public boundary is build request/limits, build, owned artifact
and typed failure evidence. Byte/custody/process/protocol modules are private.
This is a build owner, not a facade exporting the five proposed owners. No release
groups are selected. The maintainer permits existing crates.io names; `ic-build`
retains the build owner name with the incompatible 0.87.0 development version,
above registry 0.86.0 observed on 2026-11-15. Recheck the registry before cutover. Before independent byte or simulator
use, extract the required semantic owner to preserve portable/native isolation.
No consumer migration has been accepted against this provisional package graph.

Removed caller obligations: manual spawn/decode/result reconciliation, independent
success flags, target-lock sequencing, mutable-path retention and post-build capture.
The caller selects inputs, calls build and consumes immutable bytes. Explicit
scratch/process cleanup is part of that operation; artifact memory needs no close.

No old product source, test bodies or helper families were copied. Existing
repository metadata and common lint policy were deliberately retained; the
product dependency catalog and package selection were rebuilt for this workflow.
Registry dependencies are newly selected for the required mechanisms: rustix 1.1.6
(descriptor/process primitives), tempfile 3.27.0 (private target creation/removal),
serde_json 1.0.154 (bounded-record parsing), sha2 0.11.1 (SHA-256), thiserror 2.0.21
(error source forwarding). Latest stable non-yanked selections were checked against
registry index records on 2026-11-15. Toko Tooling was subsequently refreshed to 0.44.3 at the maintainer's request,
including both workspace locks and the verified exporter-owned snapshot.
Cargo generated new product and qualification locks from these requirements.

Cargo's `json-render-diagnostics` option puts rendered compiler refusals on the
already bounded stderr path; no second compiler-diagnostic decoder is needed.
The parent process must retain ordinary SIGCHLD behavior and exclusive reaping of
this operation's children. Group close reports signalling and leader reap, not
reaping of grandchildren or custody of processes that deliberately escape the group.
Native macOS, archive consumers, build-script fact admission and installed
Scheduling remain unqualified. No speed or peak-memory improvement is claimed.

The initial surface has six root exports: the build operation and five request,
limit, result and failure types. One supplier type is exposed deliberately:
`serde_json::Error` preserves the original parse cause. No Cargo message model,
SHA implementation type or native descriptor escapes. Process lifetime, private
scratch lifetime and protocol completion each have one private owner; artifact
lifetime is ordinary owned memory. These are current obligations, not a measured
reduction against an exhaustively counted old API.
