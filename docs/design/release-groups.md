# Foundation and IC integration releases

The maintainer approved two release groups for
[#276 — Reduce unrelated Host and Build version churn](https://github.com/dragginzgame/ic-tooling/issues/276)
on 2026-10-31. They share one workspace, dependency catalog, lockfile and complete
qualification gate. Package locations, public APIs, features and published history
retain their existing ownership.

## Version ownership

| Group | Public packages | Private members | Canonical version | Ledger |
| --- | --- | --- | --- | --- |
| Foundation | `ic-host-artifacts`, `ic-host-fs`, `ic-host-http`, `ic-host-process`, `ic-host-text`, `ic-build`, `ic-host-wasm-inspect` | None | `ic-build` package version | [Foundation](../../releases/foundation/CHANGELOG.md) |
| IC integration | `ic-host-tools`, `ic-testkit`, `ic-testkit-native` | `ic-testkit-dev`, `ic-testkit-perf-probe` | `ic-testkit` package version | [IC integration](../../releases/integration/CHANGELOG.md) |

[The executable policy](../../ci/cargo-release-groups.json) covers every member
exactly once. Every member uses an explicit literal version equal to its group's
primary package. Neither primary owns the other group; root Cargo metadata has
no version. Dependencies, edition 2024 and Rust 1.91 remain inherited. Group maps
in root Cargo comments name the canonical packages and release ledgers.

Both groups began with their existing 0.73.0 identities. Foundation 0.73.1 and
integration 0.75.0 are now published. Pending notes select Foundation 0.73.2 for
release-admission changes and integration 0.75.1 for arithmetic-only Metrics
adoption. Neither change breaks its packages' public API. Pending notes do not
prepare numeric versions.
Root `CHANGELOG.md` indexes both groups and
[coordinated history through 0.73.0](../../releases/history.md). Existing Testkit
package-specific history remains in the integration ledger with its original
scope; the old crate-local changelog is a packaged navigation pointer.

Tools belongs with Testkit because its registry signatures expose Platform
principals. Native's optional memory integration uses Platform stable structures.
Seven foundation packages can keep their compatible generation during an
integration-only minor. Selected upload batches alone could not achieve this
under the old adapter: all identities and internal requirements advanced together.
Ten independently versioned packages would add decisions without helping this case.

## Existing release machinery

The reviewed Shared 0.27.21 snapshot at `143a9abb9ae81feae9400ff3844b03b1ddfdd75a`
supplies the [group contract](../releases.md#explicit-cargo-release-groups),
graph admission, selected manifest/lock edits and transaction. The managed editor
is published IC Build 0.73.0 with `cli`, selected through normal tool-store admission.
No local renderer or second release runner owns these decisions.

The consumer adapter retains source admission, full CI/MSRV qualification and
receipts. It permits only the selected ledger before qualification; preparation
then owns the group's exact declared manifest/lock/ledger files. Validation binds
source, group, policy, version and destination. The saved prepared payload binds
the exact selected file bytes; committed checks read the named commit, independently
of later working edits. Failed rendering/graph admission rolls back selected files.
A failed payload-receipt write retains prepared metadata and refuses subsequent
commit/delivery until explicitly reconciled.

Maintainer commands require an explicit group:

```sh
make release-minor RELEASE_GROUP=integration
make publish-check RELEASE_GROUP=integration
make publish RELEASE_GROUP=integration
# Exact-attempt recovery uses the same group and saved version:
make release-resume RELEASE_GROUP=integration VERSION=X.Y.Z
```

Select `foundation` for a foundation release. These commands retain their ordinary
effect authorization: release pushes Git refs; publication uploads separately.
Group tags are `foundation-vX.Y.Z` or `integration-vX.Y.Z`; journals and receipts
use the same group prefix. One repository-wide release lock protects all groups.
No group selection narrows the complete workspace/release gate.

Public release, resume and publication commands refuse missing, empty or unknown
groups before setup, validation or effects. Both command-line and environment
selections are explicit; `PACKAGE` never infers a group. A selected group includes
its public members only, and `PACKAGE='name ...'` can narrow it. Cross-group batches
are refused. Every package uses its own exact version in Cargo selectors, archive names,
license/provenance checks and recorded intent. Source, registry and roster checks
still cover the workspace. The private archive driver also obtains individual
versions from locked offline metadata before invoking Build's existing engine.

The separate `workspace.sh qualify` mode is read-only workspace qualification:
it always checks the complete public roster with Cargo's dry run and one full
lint gate. Combined native CI uses it before the remaining checks. It cannot
upload or accept a package subset; ordinary publication always requires a group.

## Qualification

The normal [release adapter fixture](../../scripts/release/test-adapter.sh)
exercises both actual group plans with real disposable Git, locked metadata,
published managed rendering and receipt custody. Setup and full CI/MSRV commands
are substitutes; their required routing and failures are checked. It checks
unselected versions/ledgers, rollback, wrong-group receipts, tampered prepared
bytes and committed-source checks. The publication fixture checks mixed package
versions, group/batch selection, licenses, provenance, interruption and retention.
Shared retains its generic group runner/journal and failure matrices.

The opt-in [archive consumer fixture](../../scripts/release/test-integration-cohort.sh)
checks actual public source and normalized archives against the current published
foundation. With prepared dependencies and an already published foundation version:

```sh
editor="$(bash scripts/release/editor.sh check)"
YQ="$PWD/.tools/host/bin/yq" bash scripts/release/test-integration-cohort.sh "$editor"
```

Its disposable integration minor never changes actual versions or uploads. Rust
1.91 consumer tests compose the next Tools/Testkit/Native minor with the current
published Foundation version. Build's original public error types,
canonical registry/facade identities, direct Wire limits and Native memory traits
remain compatible. Bounded core Wire frames round-trip the selected Principal
and Account types; the consumer explicitly enables their optional Wire schemas
and verifies canonical metadata support. Production features remain unchanged.
The fixture retains candidate manifests, exact package archives,
metadata and logs. It is opt-in because a foundation version under preparation
need not already exist in the registry; it is not a pre-publication gate.

Local activation evidence is `/tmp/ic-tooling-276-activation.Gjb80U/`; actual
archive consumer evidence with Platform 0.15.1 is `/tmp/integration-cohort.xlmg5c/`;
supplier-update qualification is `/tmp/ic-tooling-platform015.v8GsIj/`. Earlier dependency
qualification, including refusal/error behavior and Wasm checks, remains at
`/tmp/ic-tooling-268-274-276.m4ejqy/`. New native macOS qualification is outstanding;
released 0.73.0 CI evidence is a separate historical result.

## Remaining coupling and recovery

Groups do not make incompatible public identities interchangeable. Process
consumes Artifacts digests; Build errors carry Artifacts/Process errors. A real
foundation break can require an integration upgrade and foundation-first
publication. Cross-group requirements track their supplier's contract and need
not numerically match the consumer. Updating a workspace requirement is not
publication of a changed declaration under an already published consumer version.
Every member has a root catalog entry with its full current version and local
path, including private and unused entries. Contiguous group blocks name their
ledgers; each alias belongs to that group's policy roster. Admission checks both
groups and actual paths; preparation updates selected requirements transactionally,
preserving existing operators. Unused catalog entries introduce no graph edges.

Tools exposes Platform principals, Testkit exposes Wire limits/errors, and Native
optionally exposes Platform memory traits. Their direct consumers must align
those suppliers after incompatible changes. Platform 0.15.2 and Types 0.15.1 keep
their independent release ownership. Wire 0.13.0's optional bridge selects the
same Types generation. The consumer fixture checks registry/facade type ownership,
core Wire limits and the explicitly selected schema pairing. Tooling production
profiles do not enable that bridge or Endpoint. A future
incompatible Types release still requires explicit integration adoption.

Existing ungrouped tags, receipts, plans and partial-publication evidence retain
their original identities. Never copy them into a group namespace or reinterpret
them as group qualification. If an old attempt needs recovery, use its recorded
pre-group source and original adapter in an isolated checkout, with explicit
recovery authorization and the existing saved evidence. An occupied global lock
still requires owner/result reconciliation; changing groups cannot bypass it.
