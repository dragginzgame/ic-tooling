# IC Tooling code hygiene overlay

Use the shared [code-hygiene method](https://github.com/dragginzgame/toko-tooling/blob/aa962a23937b9daea4b15521b7af7e5c15eac54f/audits/code-hygiene.md)
and its [common evidence contract](https://github.com/dragginzgame/toko-tooling/blob/aa962a23937b9daea4b15521b7af7e5c15eac54f/audits/README.md).
The installed snapshot and selected package identify the applicable baseline;
record a fixed method revision in each report when the selection changes.

IC Tooling's authorities are [design revision 2](../DESIGN.md), its linked
implementation contracts, the [current handoff](../status/current.md),
[workload register](../workloads.md) and [release ownership](../release-ownership.md).
Review product crates, their tests and commands; review the independent
`qualification/owned-wasm-probe` workspace separately. Include CI/scripts and
source-bound qualification interfaces where they select product behavior.
Treat frozen/historical migration adapters as retained evidence, not current
consumer implementations. Respect the migration holds in the handoff.

Trace byte admission, held filesystem custody, bounded process observation,
complete Cargo/HTTP/Sign input admission, original PocketIC type identity and
uncertain-effect recovery through their owners. Keep output policy and decoding
with callers. Testkit's generated WAT is data, not a Rust endpoint tree.
Admit target conditions against concrete API or implementation differences;
review the HTTP contract, native process cleanup and credential-backend owners
before changing their target support.

Select checks from `make help` and each affected contract. Workspace Clippy,
tests and Rustdoc cover portable/native library changes; independent fixtures
need their own manifest. Installed simulator, native credential and archive
gates are separate obligations and must be named explicitly when executed.
Do not infer macOS execution from Linux compilation or fresh qualification from
an old receipt. Tooling adoption needs both workspace selections, authenticated
snapshot admission, actual Make callers, formatting and workflow validation.

Reports live under `docs/reports/audits/YYYY/MM/DD/<scope>/<run>/report.md`.
Keep actionable follow-ups in GitHub issues owned by the affected repository;
reports retain the evidence and disposition rather than a second work queue.
