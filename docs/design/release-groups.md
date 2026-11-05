# Release ownership

One workspace, dependency catalog and lockfile retain six independent release
groups. Build and Text leave the original Foundation group without changing
crate names, package versions, APIs, feature boundaries or dependency requirements.

| Group | Public packages | Private packages | Primary version | Ledger |
| --- | --- | --- | --- | --- |
| foundation | Artifacts, FS, HTTP, Process, Wasm inspector | none | ic-host-artifacts | [Foundation](../../releases/foundation/CHANGELOG.md) |
| build | ic-build | none | ic-build | [Build](../../releases/build/CHANGELOG.md) |
| text | ic-host-text | none | ic-host-text | [Text](../../releases/text/CHANGELOG.md) |
| tools | ic-host-tools | none | ic-host-tools | [Host Tools](../../releases/tools/CHANGELOG.md) |
| testkit | ic-testkit | ic-testkit-dev, ic-testkit-perf-probe | ic-testkit | [Testkit](../../releases/testkit/CHANGELOG.md) |
| native | ic-testkit-native | none | ic-testkit-native | [Native](../../releases/native/CHANGELOG.md) |

## Dependency and type ownership

Foundation retains the cohesive artifact/filesystem/process primitives and their
inspector. These packages share real error/artifact types. Build depends on
Artifacts, FS and Process; none of those packages depends on Build. A Build API
break therefore advances Build alone unless a separate supplier change is needed.

Text has no internal production dependencies. Its bounded decoding, Unicode and
optional YAML contracts evolve independently of Build and Foundation. Host Tools'
optional ICP-project adapter and the private Testkit driver consume it explicitly.
A Text-only change does not advance unrelated filesystem or build packages.

Host Tools' normal/build graph uses selected Foundation, Text and optional Platform
capabilities, without Testkit. Public Principal identities belong to Types;
selected external IC adapters belong to Platform. Testkit uses Foundation,
optional Wire and Platform's IC-format adapters, without Host Tools. It owns
the maintained PocketIC client and selects Runtime directly for Wasm IC0. Its private driver
also consumes Build and Text; those testing edges do not become public Testkit
production dependencies. The Wasm probe remains in Testkit's existing family.

Native owns std-only allocation measurement and has no production dependencies.
Its independent baseline is the already-published 0.79.0; creating this group does
not change package versions or tags. IC Storage owns memory IO metering under
its `test-support` feature. Native's next minor removes the old memory module and
feature, so neither Storage trait changes nor simulator changes advance this group.
Historical Testkit plans and receipts retain their original roster, including Native;
never replay them with the new ownership policy. Repository CI and publication
qualification retain their whole-workspace gates. The separately reviewed
[release-validation mapping](release-validation.md) governs any narrower release
gate; the group split alone does not establish one.

Real supplier changes can still require independently classified consumer changes.
Publicly exposed supplier types and minimum requirements retain their actual
identities; independent groups do not hide an incompatible dependency. Publish
needed Foundation before Build, and any changed Foundation/Text supplier before
its Tools or Testkit consumer. Text itself does not need Foundation publication.
Do not release unchanged groups merely because workspace qualification ran.

## Selection, history and recovery

Select `RELEASE_GROUP=foundation`, `build`, `text`, `tools`, `testkit` or `native` explicitly
for release, resume, publication and publication dry-run commands. For example:

```sh
make release-minor RELEASE_GROUP=build
make publish-check RELEASE_GROUP=build
make publish RELEASE_GROUP=build
make release-resume RELEASE_GROUP=build VERSION=X.Y.Z
```

These commands retain separate effect authorization and complete release gates.
The standard runner, editor and one repository-wide lock serve every group.
Tags and journals use the selected group prefix. Publication selects that group's
public members; `PACKAGE` can narrow it, never infer or cross the group. Read-only
archive qualification still covers all ten public packages. Cargo owns registry
publication dependency order.

Build and Text start at their existing published 0.73.6 version. Foundation stays
0.73.6, Tools 0.77.0 and Testkit 0.78.0. No version reset, new baseline tag, release
or upload is implied by the split. The independent pending 0.74 notes for these
three former Foundation owners include the already-approved compiler-floor change;
future Build-only changes need not advance Foundation or Text.

Published [Foundation history through 0.73.6](../../releases/foundation/CHANGELOG.md#0736---2026-11-01)
retains its original seven-package scope once. New Build/Text ledgers link it.
The [joint integration ledger](../../releases/integration/CHANGELOG.md),
[earlier coordinated history](../../releases/history.md), imported Git histories
and old tags remain untouched. Root CHANGELOG.md indexes the six current ledgers.

Old `foundation-*` attempts may describe the original seven members. Never
reinterpret or migrate their saved plans, receipts, partial uploads or tags under
the current roster. Recover an old attempt with its recorded source and original
adapter in the preserved isolated checkout. Source/group/payload checks refuse
changed ownership; an occupied lock remains a recovery condition. The retired
`integration` selector remains refused, while its original evidence is preserved.

## Focused qualification

The reviewed Shared snapshot and pinned Build metadata editor own admission,
selected edits and rollback; this change adds no renderer or release runner.
Every root catalog entry retains its literal version/path and one owning block.

`bash scripts/release/test-adapter.sh` exercises all declared plans, rollback,
source/commit custody, unselected versions/requirements/ledgers and wrong-group
receipts, including equal-version Foundation/Build/Text owners. Setup and full
gates are substituted; the fixture cannot release or publish. Publication and
Make fixtures check exact selection and cross-group refusals.

`bash scripts/release/test-validation.sh` compares every group's full fallback
against the existing CI roster on matched fixture inputs. It exercises focused
Native/Text selection, baseline discovery, exclusions, required failures, source
drift and receipt custody using substituted Make effects. Real package, archive
and native-host execution remain distinct proofs; see the
[mapping and evidence contract](release-validation.md).

The private Dev `public_contracts` test requires shared public error, principal,
subnet-metrics DTO and core Wire identities during ordinary qualification.
The standard package check independently qualifies normalized archives of all ten
public packages. After both integration groups are published, opt in to
`CARGO_NET_OFFLINE=false bash scripts/release/test-integration-cohort.sh` for the
same public caller tests against exact current registry versions, without source
patches or root-lock edits. Evidence remains under `target/published-integration.*`.
Wire's optional metadata bridge belongs to its producer and is not selected by
Tooling's core RPC profiles. Native macOS evidence remains separate.

[The original three-group design](https://github.com/dragginzgame/ic-tooling/blob/405b27ad13d95962d729b5e756634ee866c760a8/docs/design/release-groups.md)
records the earlier split and its historical recovery coordinates.
