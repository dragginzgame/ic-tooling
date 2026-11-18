# Toko adoption and code hygiene — 2026-11-19

Verdict: **PASS WITH FINDINGS** for local Toko 0.45.10 adoption and the maintained
code-hygiene scope. One optional supplier-helper redaction gap remains; the
existing consumer collector preserves the required behavior.

## Identities and scope

Trigger: maintainer request to adopt new Toko Tooling and apply its rules and
code hygiene. Starting source: `17b6da56abe3f565d7b9ca58b06a4c256bbf7430`, on
`main`, with only pre-existing compatible `Cargo.lock` updates dirty.
The work includes local metadata, snapshot, CI-pin, documentation and diagnostic
fixture edits. No Rust source or product version changes; no staging, commit,
push, tag, publication or sibling implementation change was performed.

Method: [code hygiene](https://github.com/dragginzgame/toko-tooling/blob/88b7cd83f292d5165f4585a861cd8c89b74397b3/audits/code-hygiene.md)
and [common evidence contract](https://github.com/dragginzgame/toko-tooling/blob/88b7cd83f292d5165f4585a861cd8c89b74397b3/audits/README.md),
fixed at `88b7cd83f292d5165f4585a861cd8c89b74397b3`, using the
[consumer overlay](../../../../../../../audits/code-hygiene.md) updated in this
batch. The new target-condition guidance changes the prior method's criterion;
target-guard comparison is N/A (method change). Unchanged Rust source and the
existing owner boundaries remain comparable to the
[2026-11-18 audit](../../../18/code-hygiene-and-tt-adoption/01/report.md).
That report's findings are historical; their current dispositions are below.

Scope: all root product/private tooling crates, their tests, native credential
fixtures, independent owned-Wasm probe, active qualification interfaces, selected
tooling closure and CI diagnostics. Existing consumer adapters were inspected
for target guards; no current sibling execution or held migration was restarted.
Auth/Storage/Observability/Wire holds remain intact. Generated WAT is data;
there is no generated Rust endpoint tree to regroup or patch.

Selected package: Toko 0.45.10, verified archive SHA-256
`690c847446a5edbf5e13c03996feee5a64daa4c28a9a1098ceb7d5ea64f820fc`.
The canonical exporter retains 49 files and updates only baseline bytes.
Selected executable/tool-catalog bytes are unchanged, as are CI cache-action
bytes; all six action references select the published source above.
Both workspaces' non-Toko lock bytes match their pre-adoption copies after
reversing only the Toko version/checksum projection. Existing compatible updates
are preserved. Compiler/lint tables and inherited member lint selections remain
coherent at Rust 1.100.0 in both workspaces.

## Target and hygiene review

| Owner | Concrete difference and decision |
| --- | --- |
| HTTP | The selected native Reqwest 0.13.5 `Response::chunk` API is absent from its Wasm response. Its optional browser stream API is a separate feature/platform contract; no browser/IC HTTP caller is admitted. Retain the native adapter boundary and document it in the HTTP contract. |
| Process | Darwin's exited-group `EPERM` resolution uses libproc and errno; Linux lifetime tests inspect `/proc`. Retain only those platform implementations/observations; shared execution and cleanup policy stays unconditional. |
| FS | Atomic create-only directory rename uses target-specific Rustix `renameat_with`/`NOREPLACE` support, with an explicit unsupported branch elsewhere. Retain that syscall selection; held-path and publication-state logic stays shared. |
| Sign | Linux Secret Service/session-bus admission and macOS noninteractive Keychain custody use different native APIs. Backend fixtures retain the same split. Manifest, signing-frame and absolute-time admission remain shared; no deployment-only guard is introduced. |
| Portable leaves and qualification | Artifacts, Text and the owned-Wasm probe remain unconditional and compile for Wasm. Canic viewer evidence guards only Linux procfs observations. Generated WAT remains an explicitly IC-specific fixture, not conditionally hidden Rust. |

All maintained Rust source remains identical to HEAD. The previous grouped
imports, top test declarations, module shapes, typed errors, bounded admission,
documented live-instance panic invariants and scoped lint expectations remain
in place. No path overrides, inline Rust injection, broad source lint allowances
or TODO/FIXME markers were found in the reviewed product/probe roots.
No tracked symlink or working-tree link was found in the reviewed source,
qualification, script, documentation or workflow roots; ignored installer-owned
tool pointers remain separate. No new gzip, logging, metrics, process or
filesystem implementation was introduced.

## Finding and disposition

**MEDIUM — Toko Tooling #398:** the optional shared setup excerpt retains the
complete synthetic lines `Bearer hidden-bearer` and
`{"payload":"hidden-body"}` when their values are not known environment secrets.
IC Tooling's existing collector omits both. The ordinary CI fixture already
covers a bare Bearer value; this batch adds explicit JSON payload coverage.
[Toko Tooling #398](https://github.com/dragginzgame/toko-tooling/issues/398)
owns the remaining correction. The helper is not selected or wired here; no
existing consumer redaction guarantee is weakened. Reconsider migration when
the shared implementation passes those cases along with the existing bounds,
command-status and failure-artifact guarantees.

[Toko Tooling #396](https://github.com/dragginzgame/toko-tooling/issues/396) and
[Toko Tooling #397](https://github.com/dragginzgame/toko-tooling/issues/397) are
closed and fixed in this package. Their ACCESS_KEY selector and hard file-read
ceiling are not reported as remaining failures. The new synthetic probe is
retained in `target/toko-adoption-2026-11-19/shared-redaction-probe.log`.
No real credential, operator identity or raw setup capture was accessed.

## Fresh checks and limits

Execution: Linux x86_64, rustc 1.100.0 (`0ef1bc386 2026-11-09`), Cargo 1.100.0
(`3acc79c0c 2026-11-05`). Root checks use the preserved current lock, including
Platform 0.34.16 / Protocol 0.3.5. Disposable evidence is under
`target/toko-adoption-2026-11-19/`.

| Check | Result / log |
| --- | --- |
| Canonical snapshot update/check | Both root pin/lock and 49-file receipt admitted (`snapshot-install.log`). |
| Independent qualification selection checker | Toko 0.45.10/checksum admitted (`qualification-selection.log`). |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Pass (`clippy.log`). |
| `cargo test --workspace --all-features --locked` | 110 passed, 9 ignored (`tests.log`). |
| `cargo clippy --manifest-path qualification/Cargo.toml --workspace --all-targets --locked -- -D warnings` | Pass (`qualification-clippy.log`). |
| Artifacts/Text `cargo check --lib --target wasm32-unknown-unknown --locked` | Pass (`portable-wasm-check.log`). |
| Independent probe workspace `cargo check --lib --target wasm32-unknown-unknown --locked` | Pass (`qualification-wasm-check.log`). |
| `perl scripts/ci/test-setup-diagnostics.t` | 59 checks pass (`setup-diagnostics.log`). |
| `make tools-check`, `make fmt-check` | Pass; prepared tools unchanged (`tools-check.log`, `fmt-check.log`). |
| Actionlint and `git diff --check` | Pass. Compiler selections, timeouts and failure-artifact wiring are preserved. |

The nine ignored cases are opt-in PTY, installed-simulator and native-keyring
tests; they were not freshly executed in this batch. Rustdoc, native macOS,
consumer snapshots, archive gates, hosted CI, deployment and registry
publication were not rerun. Prior receipts retain their original identities;
none is presented as fresh acceptance here. No performance or security-freshness
verdict is implied. The remaining supplier issue is the active follow-up;
the local adoption is complete and uncommitted.
