# Backup byte, filesystem and HTTP qualification

## Complete producer commitments

`make test-backup-commitments` is a separate current-source comparison for the
new [complete producer operation](../../docs/byte-commitments.md). It first runs
unchanged restore, assembled-payload and completion tests, then applies
`commitments.patch` only if that baseline passes. It does not compose the older
filesystem/HTTP migration or silently reuse historical source.

The adapter replaces incremental `HashingWriter<sink>` use in restore request
schedule and assembled-payload commitments with `hash_with`. Domain tags, ordering,
length encodings, member hashes and failure checks remain in Backup. Legacy digest
representation and multi-step capture hashing remain selected; this is not full
legacy Artifacts removal. A fresh external-lock subset check and exact local
supplier admission are required before replacement execution. Each selected test
group must be nonempty, and both branches must pass the same test inventory.

Current baseline on 2026-11-15: HEAD
`b7c2c6ae754a5ae2452e65b9be537b517614ebd2`, source-tar SHA-256
`41a7f83cfc9e8dc2fe94ff9a9c092b411b5b959a1356e8e2204b9d88751fd21b`.
Export: `target/backup-commitments.ZmQ9ruo7`; baseline log inside that directory.
Thirty restore tests ran: 15 passed and 15 failed **before applying the patch**.
The isolated original `restore_plan_binds_original_source_and_retains_exact_replayed_bytes`
also fails at `plan_tests.rs:49`: fixture server count is 0, expected 1
(`target/backup-commitment-baseline-failure.log`). No assertion was weakened and
the sibling was not changed. Baseline failure stops automated migration acceptance.

For compile-only inspection, the adapter was subsequently applied manually to
that exported tree. `cargo check --all-targets` succeeds with the same 15 existing
Backup warnings (`target/backup-commitments-compile.log`); external-lock admission
and local byte-owner selection pass. This is not warning-denied Clippy or runtime
equivalence. The new byte owner independently passes five tests, host and Wasm
Clippy and warning-denied Rustdoc; all 19 Build and ten Filesystem tests also pass
(`target/byte-commitments-tests.log`). Consumer qualification remains open until
the current baseline passes; prior custody/HTTP results below are not refreshed.

## Earlier custody and HTTP slices

`make test-backup-custody` snapshots the current `BACKUP_SOURCE` working tree
(default `../ic-backup`) under `target/`, including local edits and non-ignored
additions, without modifying the sibling. It applies [the migration patch](migration.patch),
admits its locked graph and runs selected store/capture tests. See
[source selection](../../docs/qualification/local-sources.md). The refreshed patch preserves current local metadata and transfer-binding work,
including its 13-entry record budget and new retained evidence. The earlier
`d8955cf` run remains historical; current-source evidence is recorded below.
The export/runtime/metadata remain beneath the printed target path; Cargo's build
cache is shared at `target/backup-consumer-build`. No network setup is implicit.

The patch switches **all filesystem imports** to local `ic-host-fs` 0.80.1 and
uses the new byte owner for staged writes, bounded record/manifest reads and
exact payload streams (including the held-reader block scan). Store initialization now consumes
stage → written → sealed → commit. Partial `format.pending` is retained, directory
creation is explicit, and ambiguous publication carries its phase-specific outcome.
Other retained capture records keep their separate ordered-write/ledger protocol.
Directory enumeration is complete and bounded, so the consumer explicitly maps
an overflowing emptiness check to Occupied. The single-link refusal maps to the
existing InvalidLayout result. One FIFO test selects the new typed Kind refusal;
its file-preservation assertions remain intact. Original byte/hash tests are retained.

The original `ic-host-artifacts` 0.79.0 remains only for Backup’s existing digest
value/format and in-memory digest operations. Its stream helpers, artifact identity
and admission errors are no longer used by Backup. The HTTP body reader now uses
`ic-host-http` 0.80.0, with direct Reqwest selection and consumer-owned TLS policy;
no old HTTP supplier remains. The new filesystem product depends only
on the redesigned byte owner 0.80.1. Metadata admission rejects an old filesystem
supplier anywhere in the consumer graph. The patch's lockfile removes only the
retired supplier graph and adds the new local suppliers; retained
registry package versions/checksums are unchanged. There are no legacy product
wrappers in this repository and no production dual-dispatch or fallback path.

The preceding byte/filesystem slice passed 65 tests: 54 store/payload/capture tests, five original
ledger/advance cases, and six metadata/transfer-binding cases. These preserve exact
retained bytes, cold replay without renewed signing/network spending, orphan and
damaged evidence refusal, and uncertain-frame custody. The runner requires every
named test to exist. Metadata fixtures need local loopback sockets; no live IC
endpoint is used. Warning-denied library/test Clippy passed on the latest export. Backup advanced
its transfer-binding signature during this work; the fresh runner captured those
current changes, and the qualification patch does not alter that implementation.

Source: local HEAD `203b2083009b3aad9b5892a4023fc4ea5ee6d7a5` plus
uncommitted work, captured in `target/backup-consumer.DpWVvvV3`. Snapshot SHA-256:
`a52296e7d065c3064e1e70dfe4e5c2f96fb6d5e40a64289f75160058015a9827`.
Execution log: `target/backup-bytes-runner.log`. External-lock subset admission
confirmed retained registry identities/checksums were unchanged.

Backup now compares expected commitments itself: `StoreError::Digest` identifies
manifest or member mismatch; `Read` and `Payload` retain the new byte error with
original I/O, short-input or limit evidence. Error nesting changes deliberately;
manifest, ledger and signed bytes remain unchanged. The initial manifest comparison
uses the identity from its bounded read rather than hashing those bytes again.
Existing failure tests retain their byte/custody assertions with the new variants.
No legacy stream code was copied; the byte/filesystem slice added no product export.

`make test-backup-http` selects 13 exact HTTP tests through the same runner.
All passed against current local HEAD plus edits in `target/backup-consumer.LPMPmyPy`,
with snapshot SHA-256
`3eb525e151cd0581ee4cac7bd45c32da1c1a47d5cfb31056e121e75f3c0f54de`.
The log is `target/backup-http-runner.log`; warning-denied consumer Clippy passed
on the identical snapshot. These cover body bounds and truncation, authenticated
results, redirection/encoding refusal, cancellation/deadlines, durable replay and
TLS refusal without plaintext fallback. No retry is introduced. The runner refuses
a missing named test and any legacy HTTP supplier in this graph. The 65-test result
above belongs to its preceding snapshot; it was not repeated for this HTTP slice.
The [body contract](../../docs/http-body.md) defines the new adapter and its limits.

This is focused Linux/native and selected local HTTP evidence, not installed simulator,
archive, macOS or production cutover evidence. Keep the patch while qualifying the
new boundary; replace it with a consumer-owned migration at separately authorized
cutover, then remove this export patch. Do not publish the replacement to make this
qualification pass. The [Stage 2 contract](../../docs/filesystem-custody.md) owns scope.
