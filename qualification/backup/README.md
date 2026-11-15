# Backup filesystem migration qualification

`make test-backup-custody` exports committed Backup
`d8955cf1e34f24934027f448ef16ee6841eafbb0` from `../ic-backup` into a fresh directory
under `target/`, applies [the migration patch](migration.patch), admits its locked
Cargo graph and runs the real store/payload/capture tests plus five named ledger
and append-custody tests. No dirty sibling file is exported or modified.
`BACKUP_SOURCE` may select another Git repository containing that exact commit.
The export/runtime/metadata remain beneath the printed target path; Cargo's build
cache is shared at `target/backup-consumer-build`. No network setup is implicit.

The patch switches **all filesystem imports** to local `ic-host-fs` 0.80.0 and
uses the new byte owner for stage length policy. Store initialization now consumes
stage → written → sealed → commit. Partial `format.pending` is retained, directory
creation is explicit, and ambiguous publication carries its phase-specific outcome.
Other retained capture records keep their separate ordered-write/ledger protocol.
Directory enumeration is complete and bounded, so the consumer explicitly maps
an overflowing emptiness check to Occupied. The single-link refusal maps to the
existing InvalidLayout result. One FIFO test selects the new typed Kind refusal;
its file-preservation assertions remain intact. Original byte/hash tests are retained.

The original `ic-host-artifacts` 0.79.0 digest/format supplier and `ic-host-http`
0.79.0 remain **explicitly unmigrated**. The new filesystem product depends only
on the redesigned byte owner 0.80.0. Metadata admission rejects an old filesystem
supplier anywhere in the consumer graph. The patch's lockfile removes only the
retired filesystem dependency graph and adds the two new local suppliers; retained
registry package versions/checksums are unchanged. There are no legacy product
wrappers in this repository and no production dual-dispatch or fallback path.

This is focused Linux/native-development evidence, not HTTP, installed simulator,
archive, macOS or production cutover evidence. Keep the patch while qualifying the
new boundary; replace it with a consumer-owned migration at separately authorized
cutover, then remove this export patch. Do not publish the replacement to make this
qualification pass. The [Stage 2 contract](../../docs/filesystem-custody.md) owns scope.
