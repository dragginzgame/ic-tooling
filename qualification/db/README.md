# DB qualification ownership

DB now owns the direct PocketIC external-endpoint client, explicit async deletion,
endpoint-refusal tests and published Text 0.77.2 console integration. Tooling's old
external-session and Text source patches are stale because these changes have
already landed in the consumer. Their runners and Make targets are retired;
current DB qualification belongs to DB's `m0-rollback`, server and console paths.
No DB source, deployment, retained evidence or IcyDB comparison was changed here.

Read-only review on 2026-11-16 captured HEAD
`950f36807f8ffa7ebe4ca2cff6792fa2bbcbd03e` plus tracked edits and non-ignored source,
tar SHA-256 `276a6e7657118ac37c162b002e9bb5000e131b07a266fab835e05914eead1562`.
The repeated capture refused source drift; retained export and patch-refusal log:
`target/debt-db-review/`. The moved driver uses original PocketIC 16.1.0 directly,
with its own Tokio runtime and mandatory `IC_TESTKIT_POCKET_IC_URL`.
`tests/server.rs` checks absent/invalid endpoints without binary fallback.
This is source inspection, not a new DB installed-runtime pass.

DB's `scripts/with-pocketic.sh` still owns a separately prepared 0.106.0 launcher.
[Tooling #10](https://github.com/dragginzgame/ic-tooling/issues/10) owns the supported
command successor; DB #3 was closed after correcting the supplier ownership.
The managed-server library is published; pending Testkit 0.107.4 now supplies the
optional `cli` product executable. Publication and native acceptance remain separate
from local implementation. DB must not build another generic launcher.
The successor must retain bounded output/lifetime, failure evidence and endpoint
semantics. Console tests
also exercise standalone invocation with the endpoint removed; changing only an
outer Tooling runner would not qualify that caller. The supplying capability is
[already published as library operations](../../docs/testkit-server.md); the
[command guide](../../docs/simulator-ownership.md#db-adoption-boundary-and-historical-requirements)
documents Toko installation after publication and DB's thin wrapper adaptation.

[Historical Tooling DB evidence](https://github.com/dragginzgame/ic-tooling/blob/10e7d64e3103af84c18e433be434e4527dee75d6/qualification/db/README.md)
retains raw-memory rollback, Text console, instance deletion and source identities.
Those results remain valid for their recorded captures, not the changed consumer.
The old patches remain in that Git revision for inspection, not reapplication.
Current raw-memory rollback observations do not establish a multi-canister
snapshot-set requirement or qualify partial snapshot effects.
