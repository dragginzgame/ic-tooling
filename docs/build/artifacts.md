# Cargo execution and artifact APIs

## Persistent Wasm cache retirement

Build 0.85 removes the persistent Wasm cache, automatic Cargo input resolver,
cache batches, progress/fingerprint records, maintenance/pruning and cached-build
evidence/export receipts. It also removes generic artifact transaction caching.
There is no replacement cache, compatibility shim or automatic cleanup. Existing
on-disk entries remain untouched for caller-selected retention or deletion.

Use the existing owners for each operation:

1. Construct a complete `cargo::execution::CargoInvocation`. Select packages,
   library/example recipe, manifest, environment, target, features and profile.
2. Run `execute_cargo` once. Admit successful process status, complete messages
   and tool/command observations in that order. Cargo owns incremental compilation.
3. Select the exact completed output with `select_cdylib_wasm`, passing the same
   recipe. It refuses missing, failed or duplicate completion and ambiguous output.
4. Read bytes while holding target custody or use `CapturedArtifact::capture`.
   Keep the capture owner alive through consumption; transfer retention explicitly.
5. Hold source/configuration/tool custody through measurement. When an inventory
   is needed, declare it through `InputEvidence`; there is no automatic claim to
   discover every Cargo/build-script input. Preserve consumer receipt encodings.

`CargoBuildFacts` is exported from `cargo::execution`, its execution owner. The
artifact module retains input inventories, capture and native Candid extraction.
The [compiled example](../../crates/ic-build/examples/wasm_build.rs) composes these
existing APIs with a caller-owned target lock and byte allowance.

Relay, Scheduling and other consumers must adopt this breaking Build minor on
their own release boundary. They may retain the published 0.84 selection until
their source/report migration is qualified. Supplier publication does not wait
for old clients. Toko is outside automatic contribution scope.

## Artifact transaction retirement

Remove transaction preparation/commit/pruning and generic transaction batches.
Run transforms once per invocation through admitted Host process operations;
capture and admit outputs explicitly. Library caches no longer own a lifecycle.
Source/target custody, byte budgets and durable product publication remain with
their existing owners. This is a minor-release API cut.

## Cargo observations

Pending 0.19 replaces public `cargo::execution::communicate_cargo` with
`CargoInvocation` and `execute_cargo`. Describe the complete environment, selected
Cargo executable, absolute working directory, graph selection, packages, manifest
and library/example/compiler recipe. Build renders the command from that single
selection. Explicit profile configurations and trailing compiler arguments remain
consumer recipes; no arbitrary Cargo argument parser is introduced. Observed
execution requires a nonempty `RUSTC` environment entry so configuration cannot
silently choose another compiler. Explicit environment removals stay removed.

`execute_cargo` observes Cargo and that compiler before spawning, drains stdout
once into canonical messages, then rechecks tools after success. Inspect
`execution` and its exit status first, then `messages`, then `facts`.
Facts are `Ok(Some(_))` only for successful, protocol-valid execution with matching
tool observations and an unchanged command. A failed process/protocol returns
`Ok(None)` facts; a post-build observation refusal is retained separately as an
error, alongside original raw execution evidence. These observations do not guard
source/configuration mutations, retain artifacts or admit a release.

The caller's spawner selects child/group custody and may override stdio. Changing
its executable, environment, arguments or cwd refuses successful facts after
draining. Identity probes use bounded complete output and caller-selected Host
group cleanup, share the cancellation predicate and have no finite deadline.
Build retention/deadline limits remain explicit. Protocol refusal drains the child;
output-sink failure uses Host cleanup. Source custody, build-script interpretation
and provenance redaction remain caller-owned.
`select_cdylib_wasm(messages, manifest, &invocation.recipe)` requires exactly one
successful build completion and selects only reported outputs for that manifest
and library or named-example recipe;
capture or retain those bytes before another build can replace them.

Warning policy belongs to the caller. With Rust/Cargo 1.100, set
`CargoInvocation::render_diagnostics = true` when using
`CARGO_BUILD_WARNINGS=deny`: Cargo refuses both newly emitted and cached warnings
while retaining structured artifact and build-script messages. Plain
`--message-format=json` may still return success under that Cargo setting.
Strict plain-JSON recipes must retain compiler-level denial such as
`RUSTFLAGS="-D warnings"`, including for incremental Cargo builds and
`build_native_executable`. The native helper retains plain JSON so compiler
diagnostics reach its supplied sink without overriding caller-configured stderr.
Neither helper adds warning denial to ordinary application builds. Inspect exit
status before admitting messages/facts; a warning-refused Cargo process may still
report a cached artifact (`fresh = true`), which does not make execution successful.

Wasm builds and batches take a final Host spawner
argument. Pass `ic_host_process::child::OwnedChild::spawn_direct` for direct-child
ownership, or a closure calling `OwnedChild::spawn(command, cleanup)` for an owned
process group and explicit cleanup. Progress never chooses custody or cleanup.
Cargo starts with null stdin; the spawner may override it. Input probes delegate
capture to Host Process with their separate caller-selected policy; compilation
has no finite default deadline.

`cargo::observation::CargoMetadataSpec::command` renders onto a caller-selected
standard `Command`, retaining its executable, launcher arguments and complete
environment. The specification selects the manifest,
dependency inclusion, optional target filter, lock/offline policy and features.
No-dependency catalogs and resolved graphs are distinct observations; a catalog
or union graph does not prove role-specific dependency activation.

`CargoTreeSpec::command` renders caller-selected package ID specifications,
target, edge kinds, format and features. Cargo produces depth-prefixed,
non-deduplicated text; consumers own UTF-8 decoding and alias/role correlation.
Feature-safe grouping also uses this renderer. Pass the recipe's existing
`CargoSelection` after `target` to `cargo::compatible_package_groups`, followed by
a closure taking `&CargoTreeSpec` and returning admitted UTF-8 tree text. The
closure selects the command, applies the consumer environment, captures through
its runner and requires success then completeness before UTF-8 decoding. Build
retains isolated/combined probes and feature-equivalence checks. Every
isolated and combined probe uses its features, locked and offline choices;
the profile is not a Cargo tree argument. Canic's grouping adapter must pass
the same graph selection that its compilation recipe uses when adopting 0.17.
Build 0.23 removes the combined capture/decoding functions. Consumers
execute the rendered command through their selected Host, keeping its limits and
typed errors in that generation. Require success before completeness, preserving
failed exits as primary, then decode complete JSON through `decode_metadata` or
tree bytes through `String::from_utf8`. Keep the existing output budgets and
optional timeout; rendering neither executes nor adds a deadline. For example:

```rust,ignore
use ic_build::cargo::observation::decode_metadata;
use ic_host_process::tool::capture_command;

let mut command = metadata_spec.command(selected_cargo_command)?;
let output = capture_command(&mut command, selected_limits)?
    .require_success()?
    .require_complete()?;
let metadata = decode_metadata(&output.stdout)?;
```

The command/byte interface allows different Host generations for these callers.
Grouping forwards the observer's boxed errors unchanged; no Build-owned process
limit or error mirror is introduced. Execution/artifact APIs still exchange Host
values. Empty tree package selections are refused with
`io::ErrorKind::InvalidInput` before execution.

`cargo::observation::schema` exposes the upstream `cargo_metadata` representation
and build-message types. Consumers use those types directly rather than retaining
local metadata mirrors. Metadata is decoded from complete JSON using that schema.
[Cargo's `build-script-executed` messages](https://doc.rust-lang.org/cargo/reference/external-tools.html#build-script-output)
identify the exact `package_id` and `out_dir`, including cached build-script
results. They can bind consumer-owned
retained export evidence; unlabelled target-directory `output` files cannot prove
that binding. Package/recipe selection, retained receipts, source custody and
before/after input validation remain consumer responsibilities.

## Deployable Wasm LTO

Full LTO is the leading candidate for a shared deployable-Wasm release default.
Qualify it against complete applications, then apply the selected policy in their
common build adapter. A library's own profile does not control its consumers:
[Cargo reads profiles from the workspace root](https://doc.rust-lang.org/cargo/reference/profiles.html#lto),
and full LTO optimizes across the linked dependency graph. There is no requirement
for every application to maintain a separate manifest edit.

### Initial whole-canister evidence

IC Timers measured its four linked size-probe canisters at released source
`44935d6188a0da82314e00a56f5f68a9d0c59638` (0.21.4), using Rust 1.88.0,
`wasm32-unknown-unknown`, and the same frozen Cargo.lock (Metrics 0.8.3). The
`timer-probe` profile inherited release with `opt-level = "z"`, `panic = "abort"`
and `strip = "symbols"`. Only LTO changed. All variants used the same source
path and locked/offline inputs. These are complete probe artifacts containing
CDK, Candid and other dependencies, rather than an isolated library size.

| Canister cohort | Thin LTO bytes | Full LTO bytes | Reduction |
| --- | ---: | ---: | ---: |
| Baseline | 264,414 | 214,565 | 18.85% |
| Once | 324,104 | 263,405 | 18.73% |
| After-completion | 324,974 | 264,288 | 18.67% |
| Watchdog | 325,763 | 264,670 | 18.75% |

Export names matched. No Wasm optimizer or PocketIC execution was performed;
instruction cost, heap use and comparable build-time changes remain unmeasured.
These small canisters establish a promising candidate, not the best setting for
every production app or compiler. The comparison excludes the later incoming
Metrics 0.8.4 selection. The source and four feature selections are maintained
in [IC Timers](https://github.com/dragginzgame/ic-timers/tree/44935d6188a0da82314e00a56f5f68a9d0c59638/testing/crates/ic-timers-size-probe).

### Qualifying a common release default

Compare thin and fat at the same application source, lockfile, compiler, target,
features and optimization recipe. Use the current profile's optimization level;
do not attribute simultaneous flag changes to LTO. Keep both retained outputs
alive through inspection. Apply the normal final transforms before comparing
deployable bytes, required exports, Candid and required Wasm features. Measure
representative instruction costs and clean/warm build time; run the application's
existing PocketIC acceptance against the selected final artifact.

If full LTO wins the representative application matrix within its accepted build
time and instruction budgets, adopt it once in the common deployable-Wasm adapter
and reuse it across applications. Keep development, native tools and deliberately
different product recipes scoped separately. Revisit the evidence after compiler
or optimizer changes. Build identity and execution remain in IC Build; product
acceptance and release publication remain with consumers. No new comparison
framework or public configuration type is needed for this initial qualification.

## Standalone input evidence and depfiles

`cargo::diagnostics` selects bounded library/proc-macro artifacts from complete
Cargo output and runs a passing compiler control before negative cases. Each case
must exit with code 1 and contain every nonempty expected literal in complete
stderr. Consumers own compiler commands, cases, output budgets and retained log
directories. No retry, tool installation or elapsed-time deadline is introduced.

`InputEvidenceSpec` declares absolute file/directory/missing roots, an opaque
caller-derived non-file identity, generated-root exclusions and optional exact
child-name exclusions. It selects no recipes, tools, output roots or time limits.
`InputEvidence::capture` records complete non-excluded directory membership, hashes regular file bytes
through Host's no-follow reader, and refuses child links or special files. Caller
custody must cover sources and parent directories through capture, work and
validation; evidence does not provide a lock or release admission.

`entries()` supplies per-path `Missing`, `Directory`, or raw SHA-256 `File` facts
for consumer-owned records. File hashes use Host's
`ic_host_artifacts::artifact::Sha256Digest`; use `Display`/`to_string()` for lowercase
hex text. `InputDigest` remains the derived inventory identity.
`digest()` retains the existing consumer encoding:
identity followed by UTF-8 paths in lexical string order and their values, each
field prefixed by its u64 big-endian byte length. Values are `absent`, `directory`,
or lowercase file SHA-256. Capture preserves native paths; this particular digest
encoding refuses non-UTF-8 paths. Inventory field encoding remains unchanged. Consumers keep their record formats and namespaces.

IC Build 0.7.0 and newer provide `digest_relative_to(root)` for descriptive source
fingerprints that survive checkout relocation. It projects the same captured facts using
sorted UTF-8 relative labels with `/` separators and the existing field framing.
The root must be absolute; parent traversal, outside inputs and non-UTF-8 relative
names refuse. Standard path-component normalization applies. Projection reads no
filesystem, so it can run after the captured sources have been removed. Consumers
select complete inventories, source custody and rebuild directives; a descriptive
fingerprint does not attest that an executable was built from those sources.
The original `digest()` encoding remains unchanged.

`let after = before.validate_after(&after_spec)?` captures and validates a
replacement or expanded inventory with the same identity and exclusion policy.
Pass an `InputEvidenceSpec`, rather than capturing it first. The operation observes
the union of original and selected roots, reusing overlapping facts, and returns
only the selected inventory with its existing digest encoding. It rechecks
every original root, even if a later depfile no longer names it, and rejects
changed bytes, absence or directory membership. Newly discovered bytes cannot
gain retrospective authority. A newly named missing path is admissible only when
an earlier complete directory scan proved its first child absent; excluded or
unobserved paths cannot establish that proof. The caller retains source custody through this fresh
observation and handles refusal according to
its own build/release policy.

`read_cargo_depfile_inputs(current_dir, record, excluded_roots,
maximum_record_bytes)` reads one exact record through a caller-sized bounded,
regular-file/no-follow read. Malformed, oversized or unsupported records refuse;
unrelated records in the output directory are not inspected. Relative inputs bind
to the explicit working directory. Existing aliases resolve before output
exclusion; missing paths containing parent traversal remain conservative external
inputs. The record itself is not a source input: Cargo may replace it during work.
Consumers select records from their own Cargo graph, combine their input sets,
and decide whether `NotFound` permits a cold-build inventory. Release 0.16 removes
directory-wide discovery; migrate callers to exact record selection. This
selection never grants pre-build authority to newly named source bytes; capture
and validate the resulting inventory through `InputEvidence` as above.

`parse_cargo_depfile` accepts Cargo/rustc Make dependency rules, escaped spaces,
tabs, backslashes, hash/colon characters, doubled dollars, LF/CRLF continuations,
multiple rules and empty phony targets. It refuses other Make expressions instead
of interpreting them or returning partial evidence. Full-line comments are
ignored, so environment dependencies belong in caller identity. Profile/target
directory selection and generated-output policy remain consumer-owned. These APIs
do not invoke Cargo, create another artifact cache or manage maintenance.

## Temporary staging and captured outputs

`artifacts::produce_and_retain_artifact` handles a caller-executed
transform with explicit producer, log and retained-destination paths and a byte
allowance. Build creates producer/log exclusively, passes the owned log to the
callback, then captures/retains successful output before removing the producer.
Failed statuses, callback errors/panics and capture/retention errors preserve
existing evidence. The log always remains. Callers retain tool/process policy,
physical path distinctness, parent custody and publication; the helper starts no
process, selects no deadline and removes no unrelated artifact.

The 0.18 cut removes `ArtifactStagingDirectory`. For direct staging, call
`ic_host_fs::directory::create_unique_directory` with an existing parent, prefix,
permissions and collision allowance, then retain the returned path in
`ic_host_fs::owned_directory::OwnedDirectory`. Callers choose candidate filenames
and qualify them before publication. Consuming `cleanup()` attempts removal once
and reports failures; Drop performs best-effort cleanup while armed. Callers keep
custody of the parent and staging paths against unrelated writers. Build's private
capture allocation retains mode 0777 reduced by umask and a 64-collision allowance.

`CapturedArtifact::capture(source, parent, maximum_bytes)` copies a completed
producer output synchronously with the caller's byte budget. Keep the returned
record through every use of `path()`, including later Cargo commands that may
overwrite or delete the original output. Multiple captures remain independent,
including captures of identical source bytes. Closing or dropping a capture
releases only its temporary staging directory.
Capture streams through Host's bounded copy and durable writer with constant
scratch memory. Regular-file and initial size admission happen before staging;
the stream independently enforces the budget. Keep the source stable throughout
capture. An open descriptor retains its inode rather than freezing its contents.
`retain()` explicitly transfers the captured path and its staging child to
caller-owned evidence retention. It preserves the bytes after the record is
consumed; the caller owns later cleanup and custody. It creates no cache or
release admission. Normal close/Drop behavior remains unchanged.

`retain_to(destination)` transfers custody before moving the exact capture with
`std::fs::rename` and removes only its empty staging child on success. The caller
admits the destination and owns replacement policy, subsequent custody and
cleanup. `CaptureRetentionError` preserves the native I/O cause, the actual
retained file path and the staging directory: rename refusal leaves the original
capture, while cleanup refusal leaves the final file and any extra evidence.
Neither failure retries or deletes retained bytes. The API is published since
0.3.3; Jobs owns its consumer adoption at
[#26 — Capture finalization](https://github.com/dragginzgame/ic-build/issues/26).

These temporary owners create no cache entry, cache hit or release admission.
They do not prune retained artifacts, run transforms, install tools or select
an elapsed-time deadline. Canic retains transform order, validation, release
binding, build coordination and durable product publication. Its producer and
consumer adoption evidence is tracked in
[Canic #529](https://github.com/dragginzgame/canic/issues/529).

## Caller-executed Cargo examples

Cargo owns compiler freshness. Reuse its incremental target when appropriate;
each invocation still runs Cargo and selects its own completed messages. Never
infer an artifact filename or admit an existing file after a failed build.

Use `CargoRecipe::Example(name)` for named cdylib examples and pass that same
recipe to artifact selection. Root libraries and examples from one manifest are
distinguished by their reported Cargo target. Custom library names are supported.

The [Wasm example](../../crates/ic-build/examples/wasm_build.rs) accepts workspace
and package. It holds a caller-owned descriptor lock through build/capture, then
reads its captured bytes after releasing the target. Other producers sharing that
target must cooperate with the same custody contract. Source writers remain
excluded separately. The installed Wire probe builds once into its shared target,
holds its own target lock until Wasm bytes are owned in memory, then installs.

## External lock selection admission

IC Build provides the pure
`ic_build::cargo::packages::verify_external_lock_subset(original, candidate, max_bytes)`
API for callers that need the archive engine's external-selection check without
running a package/test workflow. Both byte slices must fit the explicit
per-document allowance before UTF-8/TOML decoding. It performs no file or process
I/O, dependency resolution, network access or mutation.

Every candidate external name/version/source identity and optional checksum must
match the original. Registry checksums require canonical lowercase SHA-256;
non-registry optional checksum fields compare exactly, including absent versus
empty. Duplicate external identities refuse in either document. Extra valid
original selections and an empty candidate package array are allowed. Non-table
records and malformed identity/checksum fields refuse with typed role/index
evidence; UTF-8 and TOML failures retain their original error sources.

The archive qualification workflow is available only on Unix hosts, using Host's
descriptor-based source traversal. The in-memory subset verifier has no Unix gate.

Unsourced tables are outside this check. Consumers retain exact local package
names, versions, paths, dependency edges, forbidden-feature policy, input custody
and release admission. Cargo archive qualification uses this same reader and
comparison, then independently requires a nonempty external graph. Valid archive
flows retain their selected-graph, evidence and lifetime contracts; malformed
records and changed non-registry checksum fields now refuse there too.

Auth's existing `apps/tooling` command boundary can read both locks with its
explicit file budget and invoke this check, leaving its library-graph admission
in the existing script. Publish the producer before adopting that import; a
source rehearsal or temporary path override does not establish registry adoption.

## Strict Cargo lockfile version projections

IC Build 0.7.0 and newer provide
`cargo::lock_projection::verify_workspace_version_projection`.
Pass saved before/after TOML and explicit `WorkspaceVersionProjection` entries
containing each unsourced package name, expected prior version and permitted next
version. The verifier compares the complete parsed lockfiles after projecting only
those package versions. Comments and table-field ordering may differ; dependencies,
source/checksum fields, array order and all unrelated values must remain identical.
Invalid/duplicate selections, missing or ambiguous packages and unexpected prior
versions return typed `LockProjectionError` evidence.

Callers retain input byte limits, saved-file custody, version policy, rendering,
receipts and release ordering. This pure verifier reads and writes no files.
Memory's adoption is tracked by [#96](https://github.com/dragginzgame/ic-memory/issues/96).
Canic's fixture normalization also rewrites version-qualified dependency references;
that broader transformation is deliberately refused by this strict contract.

## Inherited Cargo Rust flags

Cargo owns target/configuration precedence and effective compiler arguments.
The helper below composes only the environment values supplied to it.

Release 0.16 uses `cargo::flags::combined_encoded_rustflags(encoded, ordinary, &extra_flags)`.
Pass each additional argument as a separate `String`; when it returns `Some`,
set that value as `CARGO_ENCODED_RUSTFLAGS` on the Cargo command. An empty slice
returns `None`, so leave the command's inherited flags unchanged.

Supply `Option<&str>` values from the command's selected environment; no ambient
environment is read. Treat non-Unicode values as unavailable. Encoded flags take precedence
over ordinary `RUSTFLAGS`, including an explicitly empty encoded value. It splits
ordinary flags on whitespace and preserves each addition's argument boundaries.
Resolve explicit command overrides before inherited values. Callers retain
target/configuration precedence, remap roots and destinations, and build policy.
Canic can pass its source, Cargo-home and sysroot remaps as three additions rather
than replacing inherited flags with their encoded concatenation.

## Cargo configuration observations

Published 0.6.1 exposes the engine's configuration walker through
`cargo::environment`. `configuration_directories` enumerates invocation ancestors
and the explicitly supplied Cargo home without reading ambient environment.
`preferred_configuration` selects `config` before `config.toml`;
`observe_configurations` follows selected include trees and returns lookup paths,
stable locations, raw SHA-256 identities and typed compiler-override declarations.
See the [historical Build 0.84 migration](https://github.com/dragginzgame/ic-tooling/blob/3e9aa1e08fa3b8baaac2732e6f456337f87ac63b/docs/build/artifacts.md#build-084-migration) for the conservative refusal
surface; it neither merges declarations into an effective selection nor binds
compiler/wrapper executable identities.
Release 0.16 stores `ConfigurationFile.sha256` as Host's `Sha256Digest`;
use `as_bytes()` for its unchanged raw bytes or `Display` for hexadecimal text.
Within one call, recorded lookup paths are not read or expanded again; distinct
aliases retain their own include lookup directories. A new call observes fresh
bytes under the caller's source-custody contract.

For Canic adoption, retain its source-custody and release recipe. Keep absent
candidate paths in pre-build `InputEvidence`; observing existing configuration
files does not establish authority for a newly discovered include. Choose
effective versus conservative roots explicitly and validate the expanded input
set after execution. Preserve typed refusal and retained evidence; no retry or
cache/release admission comes from observation. Configuration contents are
excluded from returned facts and malformed-TOML diagnostics. Cargo itself still
owns which configuration syntax its selected version accepts.

## Source-independent cohort comparisons

Compare the actual `CargoBuildFacts` selection, target, compiler/Cargo observations
and caller-selected inputs under the consumer's schema. There is no library cache
fingerprint or recipe-identity receipt. `InputEvidence::digest_relative_to` can
describe a declared inventory; it is not an executable attestation. Hash the exact
captured output through Host Artifacts when the report needs its SHA-256.

Retain measured artifacts and the original receipt encoding through a deliberate
consumer format migration. Do not relabel old cache-generation records as fresh
compilation or compare unlike evidence as if its authority were unchanged.

## Historical extraction references

The [frozen pre-cut guide](https://github.com/dragginzgame/ic-tooling/blob/3e9aa1e08fa3b8baaac2732e6f456337f87ac63b/docs/build/artifacts.md)
retains older migration details, cache formats, source provenance and retired API
contracts. It is historical evidence, not the current supported surface. See also
the retained [ownership design](design/build-ownership.md) and release ledgers.
