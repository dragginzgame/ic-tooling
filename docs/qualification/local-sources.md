# Current local consumer sources

Active Canic, Backup, Scheduling and DB qualification uses current local sibling
working trees. Scheduling's rewrite moved from `../ic-scheduling-new` to
`../ic-scheduling`; its runner verifies ancestry from a qualified rewrite commit.
`../ic-scheduling-legacy` is excluded as a consumer source. Historical pinned exports
remain historical evidence, not the
source selection for new runs. Auth remains on hold; Storage, Observability and
Wire remain frozen. Reading a current consumer's existing supplier selection does
not authorize modifying those suppliers.

`scripts/qualification/local-source.sh` owns capture for the active runners.
It copies tracked files as they currently exist plus non-ignored untracked files,
omits tracked deletions, and preserves file modes and symlink entries. Ignored
build outputs, tool caches and Git metadata are excluded. Submodule directories
and other unsupported entries refuse capture. The sibling is read-only throughout.

Two sorted inventories and tar snapshots must agree, and HEAD must remain unchanged
across capture. Each run retains HEAD, source path, file inventory, original tar and
its SHA-256 alongside the temporary export. This detects observed concurrent edits;
it is not an atomic filesystem snapshot or protection from hostile writers.
If the check fails, wait for the editing session to settle and retry.

Migration patches apply only to that snapshot; a stale patch stops the run rather
than falling back to an old commit. Qualification adapters belong here; consumer
implementation remains owned by the sibling. Never extend Tooling merely to recreate
an obsolete consumer mechanism. Current consumer contracts must justify new APIs.

Changing a runner to current-source selection does not qualify all current source.
Canic's six selected host tests passed against its current snapshot. Scheduling's
rewrite target admits its Testkit graph and host-driver compilation without old
migration patches; installed replacement acceptance remains open. Backup’s refreshed
patch preserves current metadata/transfer additions and passes
65 selected tests with the new filesystem supplier; see its
[qualification evidence](../../qualification/backup/README.md). Future source changes
still require patch admission and relevant qualification.

Run `bash scripts/qualification/test-local-source.sh` for the capture fixture:
modified binary bytes, new files, deleted tracked files, executable mode, symlinks,
ignored-file exclusion, the recorded snapshot digest and refusal of a file changed
between captures. All capture-fixture checks passed.
