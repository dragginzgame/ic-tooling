# Current handoff — 2026-11-14

## Authorized simplification and deployment

The maintainer approved large breaking cuts: artifact transaction caching,
persistent Wasm caching, reusable baseline pools/reset recipes and unused Host
capabilities/writer wrappers. IcyDB is retiring; Canic Legacy is being replaced.
Read-only inspection at IC DB `14a2810b` and new Canic `ec7fc3de`
found ordinary Cargo builds and fresh PocketIC instances, with neither framework.
IC DB still uses the Build release-metadata editor.

Build removes both cache frameworks, automatic input resolution, batches,
fingerprint/evidence wrappers, progress and maintenance/pruning. Local callers
compose the existing Cargo executor, exact recipe selection and invocation-owned
capture/read under caller custody. Selection requires one successful completion
and distinguishes library/example outputs. `CargoBuildFacts` moves to
`cargo::execution`. Declared input evidence, depfiles, package qualification and
metadata editing remain. Existing disk caches are untouched.

Testkit removes pools, leases, reset recipes, automatic rebuild and associated
outcome/timing/failure types. Fresh-instance concurrency and caller-configured
fixtures remain. Explicit snapshots and `CachedPocketIcBaseline` preserve
controller/funding policy and timer/heap/stable-memory/hook/chunk restoration.
Callers own stop/drain, non-snapshot reset, readiness and explicit close.

FS/Process remove command-sidecar custody/quiescence, the separate bounded
process/lock inspector, symlink publication, cache-tag maintenance and directory
size observation. The custody-only existing-sidecar opener also retires, removing
`command-fds` and its sole `nix` dependency from both lockfiles. The writer pass
removes `write_named_with`, `write_existing_parent_validated_with` and
`write_retained_at_with`. Their found external callers are in IcyDB or legacy
repositories and retain their older published selections. Current Backup's
capabilities/retained-file creation/publication, Build's executable admission,
Profiler's observations/gate, ordinary locks and shared publication engines remain.
This Host round removes 2,310 net Rust lines, including dedicated tests.

The [extraction contract](../extraction.md), current guides and migrations
describe this smaller surface. Historical designs/provenance stay retained.
Foundation 0.78.0, Build 0.85.0 and Testkit 0.105.0 have pending minor-release notes;
package versions are unchanged. All batches remove 22,511 net physical Rust lines,
including tests, leaving 58,740 across product and qualification packages versus
81,251 at the preceding delivered HEAD. The maintainer has now authorized
deployment of Foundation 0.78.0, Build 0.85.0, Tools 0.93.0 and Testkit 0.105.0
from this normal checkout through frozen delivery to main/crates.io. Text and
Wire have no product release in this batch. Sibling delivery and artifact cleanup
remain outside scope. Existing artifacts/tags remain retained.

## Focused validation

The final Host batch passed on Linux x86-64:

- FS 174 and Process 139 unit tests, plus 17 integration tests, using
  `cargo test -p ic-host-fs -p ic-host-process --all-features --all-targets --locked --offline`.
- Strict all-target/all-feature Clippy for FS, Process and their Build/Testkit
  callers; both workspaces resolve with their locked offline selections.
- FS's Wasm library/CLI/unsupported-lock compile contract, warning-denying
  FS/Process Rustdoc, scoped formatting, documentation links and diff checks.

Earlier Build/Testkit batches passed on Linux before these Host API removals;
their exact archived evidence remains retained:

- Final exact Build archive: 152 unit tests, retained capture and real Cargo
  library/example, output-collision, warning-policy, native-executable, input/
  depfile and package-engine regressions. Evidence:
  `target/package-qualification-2-0/` (only Build selected).
- Installed private Wire probe preserves state, caller identity and typed errors
  using direct Cargo execution and bounded byte reading under its target lock.
- 3 PocketIC concurrency and 7 snapshot regressions, including explicit timer,
  memory, hook and chunk-store restoration without a pool.
- Real native optimizer/Wasm execution; final evidence is
  `target/evidence/large-cuts-20261114/optimizer-final/`.
- First-batch exact Testkit and Wire adapter archive qualification through the
  existing engine: `target/package-qualification-783704-0/`.
- Strict all-target/all-feature Clippy for Build/Testkit/adapter, warning-denying
  Build/Testkit Rustdoc, native/Wasm Testkit graph checks, scoped Rust formatting,
  ShellCheck, documentation links and diff checks. The changed private Wire
  caller also passes strict all-target/all-feature Clippy.

`bash scripts/release/test-validation.sh` passes all group selection, full-roster
equivalence, required/excluded failure, source/environment drift and receipt
custody fixtures with product effects substituted. The CI-selection caller
fixture now derives all comparison groups from policy instead of assuming three.
Native macOS and full repository CI were not executed for these changes.

## Release selection and remaining work

The existing selector now maps all six groups. Foundation selects every member
and declared reverse consumer; Testkit includes its Wire/private consumers;
the Wire adapter retains real typed calls and its exact archive. Leaf commands
select explicit packages. Shared, dependency, manifest-feature, unknown-consumer
or unqualified input changes retain the complete gate.

The new map still requires its matched Linux/macOS Intel/Apple Silicon comparison
through the existing explicit CI workflow before narrowing is admitted. Earlier
baseline configuration cannot qualify it. No deployment-time saving is claimed.
[#353](https://github.com/dragginzgame/ic-tooling/issues/353) retains deployment
phase timing and collector-recovery acceptance; generic runner ownership is at
[Toko Tooling #382](https://github.com/dragginzgame/toko-tooling/issues/382).

The local #353 follow-up adds explicit collector-only recovery through
`scripts/release/adapter.sh collect GROUP VERSION`. It re-admits original execution
and binds the normal retry to its exact receipt; source/context/evidence drift or
prepared versions refuse. Foundation's focused archives now also select Build,
Tools, Testkit and Wire reverse consumers. Linux selection and collector fixtures,
ShellCheck and document links pass; these substitute product execution and do not
qualify the new map on native hosts. Evidence lives under
`target/evidence/issue353-followup/`. The Toko session delivered #382's timing
implementation in published 0.43.1. Both workspaces now select that patch; the
authenticated 120-file snapshot preserves recorded entrypoints and local edits.
Only Toko's version/checksum changed relative to the saved dirty lockfiles. No
executable tools were installed. Real next-deployment timing and three-host
comparison remain required before #353 can close.

Relay/Scheduling and a Toko harness were found using older cache APIs. They retain
their published Build selections until their own explicit adoption; this checkout
contains no compatibility layer. Toko remains outside automatic contribution
scope. Scheduling also has a snapshot-restoration regression to preserve during
its own adoption. The current [Build guide](../build/artifacts.md) describes the
smaller execution/capture contract and links the historical API evidence.

## Concurrent release and preservation

The separate Platform 0.33 deployment was stopped at the maintainer's request.
Testkit 0.104.1 was already published at `df10ff25`, frozen `batch-bz31zGgk`.
Foundation 0.77.3 completed while that session stopped, at
`3e9aa1e08fa3b8baaac2732e6f456337f87ac63b`, tag `foundation-v0.77.3`,
frozen `batch-duIPtf2a`. Its complete Linux gate and publication succeeded.
Tools 0.93.0 was not started. Do not restart completed publication as a new batch.

After ownership handoff, the standard reconciliation fast-forwarded the ordinary
checkout to Foundation's delivered commit. The simplification work was restored
byte-exactly and continued there. The compact interruption recovery patch and
postimage/deletion manifest remain at
`.git/release-state/preserve/large-cuts-20261114.skOe7e/`; this is an earlier
partial checkpoint, not the final working diff. No release artifacts were removed.

Both workspaces retain the Platform 0.33 dependency family (existing lockfiles
select 0.33.2), Types 0.20.3 and qualification Runtime 0.22.3;
Toko Tooling is now 0.43.1; private qualification versions remain unchanged. The
bounded Registry capability stays unselected. Native PocketIC remains upstream
16.1 with the caller's runtime and unchanged native identities/panic behavior.

The [preceding handoff](https://github.com/dragginzgame/ic-tooling/blob/3e9aa1e08fa3b8baaac2732e6f456337f87ac63b/docs/status/current.md)
retains exact supplier provenance, prior host evidence, interrupted attempts,
registry observations and unresolved acceptance. Preserve
`target/evidence/platform033-deploy/`, all original frozen journals/checkouts,
`.git/release-state/preserve/*` and original recovery refs. Historical target
losses had an external, unestablished cause; only surviving evidence is proof.
