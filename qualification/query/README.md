# Current Query Text qualification

`make test-query-text` snapshots the current local `../ic-query` working tree,
including dirty and new source, without modifying it. `QUERY_SOURCE` can select
another local checkout. Capture drift and stale patches refuse. `text.patch`
selects unpublished `ic-host-text` 0.77.1 and changes the nine presentation
callers to `escape` / `bordered_table`; it preserves report implementation, raw
values and JSON. This replaces the legacy Text supplier for this consumer, not
Query's report logic or Platform dependencies. There are no live network queries.

The runner admits the selected local Text identity, lists tests to refuse empty
filters, then runs the ten presentation tests and ten topology output tests.
These check escaping, precision, evidence labels, partial outcomes, JSON and
terminal injection refusal. The [Text contract](../../docs/text-presentation.md)
has separate exact Unicode/ragged-table and control-character tests. Other old
Text operations remain unqualified; no progress or clipping facade was added.

Earlier GitHub-checkpoint evidence (Text 0.77.0) on 2026-11-15: source HEAD
`263a8e92da873543825c25e7dd6530c4db5ab673` plus current edits, source tar SHA-256
`7690f1f6d6862df5f12ac74db1fceb7b6435d194d4d55b30462b4b5bad4c6ea3`.
Final export `target/query-consumer.E3hnGNx6`, log `target/query-consumer.log`:
all 19 selected tests passed. Equivalent snapshot `target/query-text.MT4rO2oX`
provides CLI Clippy and external-lock checks. Product tests, warning-denied Clippy
and Rustdoc, formatting and Toko admission passed. Registry source/version/checksum
identity is unchanged for retained external dependencies. No publication or
production consumer migration occurred. No performance improvement is claimed.

Current next-version evidence: HEAD `6649ef9e43c8153f733c45c513098a5a3f50abe6`
plus current edits, source tar SHA-256
`999a05edc4b8f5dea74a206896118c65e6435dcfa62e7a4175f922a4be0d9026`.
The old patch correctly refused new manifest and presenter context. The refreshed
patch includes the placements presenter and preserves Query's selected Platform
protocol dependency unchanged. Export `target/query-consumer.mdEyyh0k`: all 20
selected tests passed, including placements partial JSON and rejection escaping
(`target/query-next-run.log`). CLI/test Clippy and retained external-lock identity
admission passed (`target/query-next-clippy.log`). No sibling was edited.
