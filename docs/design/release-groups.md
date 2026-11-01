# Release ownership

One workspace, dependency catalog and lockfile retain three independent release
families. This implements [#299](https://github.com/dragginzgame/ic-tooling/issues/299)
without changing package versions or public APIs.

| Group | Public packages | Private packages | Primary version | Ledger |
| --- | --- | --- | --- | --- |
| foundation | Artifacts, FS, HTTP, Process, Text, Build, Wasm inspector | none | ic-build | [Foundation](../../releases/foundation/CHANGELOG.md) |
| tools | ic-host-tools | none | ic-host-tools | [Host Tools](../../releases/tools/CHANGELOG.md) |
| testkit | ic-testkit, ic-testkit-native | ic-testkit-dev, ic-testkit-perf-probe | ic-testkit | [Testkit](../../releases/testkit/CHANGELOG.md) |

## Dependency and type ownership

Host Tools' normal/build graph uses Foundation and optional Platform capabilities;
it does not depend on any Testkit package. Its public registry principals and
certificate/hash-tree results belong to Platform. Certificate changes therefore
classify the Tools release at that public boundary, independently of Testkit.

Testkit's normal/build graph uses Foundation, arithmetic-only Metrics, optional
Wire and Platform's PocketIC/system capabilities. It does not use Host Tools.
Its public transport APIs expose Wire limits/errors and external PocketIC DTOs.
Native's optional memory adapter exposes Platform stable-memory traits. These
Testkit-family contracts retain their own compatibility assessment; changing a
supplier does not automatically imply that all public identities changed.

The private development driver depends on Build, Testkit and the private probe;
it has no Host Tools dependency. The probe supplies Testkit's installed fixtures
and uses Wire/Platform for Wasm. Both stay in the Testkit family. Native remains
in that existing measurement/testing family; this split does not create one group
per crate. Platform selects canonical Types 0.16.0; opt-in Wire metadata must
align that identity without a Tools/Testkit dependency or a duplicated owner.

Foundation error/artifact types still cross genuine package boundaries. A real
Foundation break can require independently classified downstream releases and
Foundation-first publication. The root catalog's inherited minimums track the
supplier contract; package numbers do not need to match across groups.

## Selection, history and recovery

Select `RELEASE_GROUP=foundation`, `tools` or `testkit` explicitly for the standard
release, resume, publication and publication dry-run commands. For example:

```sh
make release-patch RELEASE_GROUP=tools
make publish-check RELEASE_GROUP=tools
make publish RELEASE_GROUP=tools
make release-resume RELEASE_GROUP=tools VERSION=X.Y.Z
```

These commands retain separate effect authorization and complete release gates.
One repository-wide lock coordinates all groups. Tags and journal namespaces use
`foundation-`, `tools-` or `testkit-` prefixes. Publication selects only the chosen
group's public members; `PACKAGE` may narrow it but cannot infer a group or cross
its boundary. The read-only workspace qualification still covers every public
package and never uploads. Cargo owns publication dependency order.

At the split, published versions are Foundation 0.73.5 and Tools/Testkit 0.76.1. No reset,
new baseline tag or publication is implied by this policy change. Pending notes
are compatible patches because the contributor release interface changes while
library consumer APIs and build requirements remain unchanged.

[Joint integration history through 0.76.1](../../releases/integration/CHANGELOG.md)
remains frozen once, with original dates, contents and release scope. Both new
ledgers link that published baseline. [Earlier coordinated history](../../releases/history.md)
also remains intact. Root CHANGELOG.md indexes the three current ledgers.

The old `integration` selection is intentionally refused by the new policy.
Existing `integration-*` tags, journals, receipts, partial-upload records and
backups retain their original meaning. Never migrate or reinterpret them under
this roster. Recover a specific old attempt with its recorded source, original
adapter and destination in its preserved isolated checkout. An occupied lock is
still an ownership/recovery condition, not permission to switch groups.

## Existing mechanics and focused qualification

The reviewed Shared snapshot and managed Build 0.73.0 metadata editor still own
graph admission, selected edits and transactional preparation. No local renderer
or parallel release runner is added. Each root member retains its full literal
version and path. Unselected versions, ledgers and irrelevant minimums remain
unchanged; a workspace qualification refresh alone is not a release requirement.

`bash scripts/release/test-adapter.sh` exercises all three plans, rollback,
wrong-group receipts (including equal Tools/Testkit numbers), exact payloads,
unselected metadata and source/commit custody. Setup and full gates are simulated;
this fixture cannot release or publish. The publication and Make fixtures verify
exact selection, rejection of the retired group and cross-group refusals.

The opt-in `scripts/release/test-integration-cohort.sh EDITOR [tools|testkit]`
qualifies one disposable next minor at a time against the actual published
Foundation and unchanged integration peer. Without a group it checks both.
Current and Rust 1.91 callers run against source and real normalized archives.
The certificate type mismatch control uses the actual old Platform identity and
must fail compilation. Fixture preparation never edits real package versions or
uploads. Native macOS execution remains distinct from local Linux evidence.

[Prior design and qualification](https://github.com/dragginzgame/ic-tooling/blob/722c88c22f738c75f74546ba01bf1e8a0e461661/docs/design/release-groups.md)
preserves the original Foundation/integration split and historical recovery paths.
