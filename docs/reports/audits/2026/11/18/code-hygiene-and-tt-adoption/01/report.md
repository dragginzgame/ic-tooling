# Code hygiene and Toko adoption — 2026-11-18

Verdict: **PASS WITH FINDINGS** for the maintained implementation and local
Toko 0.45.8 adoption. Two supplier capabilities prevent replacing the existing
setup collector; neither requires weakening the consumer's current guarantees.
This is a source hygiene/adoption result, not a security, deployment, performance
or whole-consumer migration verdict.

## Identities and scope

Trigger: maintainer request to get new Toko Tooling and perform a full hygiene
and adoption pass. Inspection and authorized local repairs were performed on
`main`, starting at `1263f6f`. Existing dirty compiler-policy workflow changes,
compatible dependency selections, Toko 0.45.1 preparation and documentation edits
were preserved. No package version, published archive or release tag changed.

Method: [Toko code hygiene](https://github.com/dragginzgame/toko-tooling/blob/12af9e64f4a0ba03c8c94d3eee00987080ade98a/audits/code-hygiene.md)
and [common audit contract](https://github.com/dragginzgame/toko-tooling/blob/12af9e64f4a0ba03c8c94d3eee00987080ade98a/audits/README.md),
fixed at source `12af9e64f4a0ba03c8c94d3eee00987080ade98a`.
The [consumer overlay](../../../../../../../audits/code-hygiene.md) was introduced
in this working batch from the existing design/contracts; there was no prior
audit overlay or comparable hygiene report. Comparison verdict: N/A (new baseline).
The AST comparison against the unchanged starting Rust source is a separate,
comparable mechanical-change anchor.

Both workspaces and CI action pins adopt Toko 0.45.8. Published archive SHA-256:
`adbc76769b2231de6704851c363550ddc486bb6204a0f89c2ad365eb7cf740a3`.
The canonical exporter installs 49 selected files; no managed file was edited
manually. The optional setup module was evaluated and remains unselected.

Scope: all ten root workspace crates, their tests and executable boundaries;
the independent owned-Wasm probe; Make/CI setup and qualification entrypoints;
the design, current contracts and handoff. Loose historical and held migration
adapters were reviewed as retained source-bound evidence, without refreshing
their old consumer patches. No sibling implementation was edited or requalified.
Ignored tool stores and disposable build evidence are not product source.

Execution host: Linux x86_64, rustc 1.100.0
(`0ef1bc386 2026-11-09`), Cargo 1.100.0 (`3acc79c0c 2026-11-05`).
Locked dependency selections include the pre-existing Platform 0.34.16 /
Protocol 0.3.5 updates. Root lint/tests/docs used all features; qualification
lint used its complete independent workspace/all targets. See the commands below.

## Owner and boundary review

| Owner | Trace and disposition |
| --- | --- |
| Artifacts / FS | Length and digest admission precede authoritative bytes; held descriptors and stage/seal/commit preserve identity, partial effects and original I/O causes. Recovery and foreign-entry refusals remain tested. |
| Process / Build | Process owns bounded pipes, cancellation and group cleanup; callers decode. Build requires complete Cargo records, owns execution/capture and retains primary plus cleanup evidence. Input's manual Debug intentionally omits bytes. |
| HTTP / Text | Complete bounded bodies remain distinct from prefixes; Reqwest/TLS policy stays with callers. Terminal escaping, width and table policy stay with Text/callers. |
| Sign | Strict helper JSON and exact Platform admission remain separate from raw signing bytes. Existing credential custody, original-ID status authorization and retained worker cleanup are unchanged. No operator credential was accessed. |
| Testkit | Original PocketIC types and exclusive held-reply control remain. Interrupted clock/control consumes authority before awaiting; independent cleanup remains observable. Generated modules are bounded WAT data, not generated Rust endpoints. |
| Metadata / qualification commands | TOML edit/admission retains original structure and refusal causes; qualification reads held bounded files and compares external source/checksum identities without writing inputs. Private argument parsing now uses Clap. |

Public/private boundaries, source/test/generated roots and package declarations
were inspected. No tracked symlink or working-tree symlink was found in the
reviewed product, qualification, script, documentation or workflow roots.
Ignored prepared-tool pointers remain installer-owned. No product gzip fixture
engine, metrics/logging engine, path override or generic compatibility facade
was introduced.

Production panic candidates in Testkit are the documented private live-instance
invariant behind borrowing/consuming methods. No recoverable input path was
converted to a panic. Artifact buffer and held-reply integer conversions remain
bounded before narrowing. Typed errors already use thiserror. Build's three
scoped large-error expectations preserve typed original/cleanup evidence; the
fixture's unsafe-symbol expectation is limited to its Wasm export. Product unsafe
code remains forbidden. The workspace's intentional `too_many_lines` allowance
does not replace responsibility/caller inspection.

## Repairs and intentional retention

- Group imports by crate within their original scope/attributes/visibility;
  move test-module declarations before implementation and separate adjacent
  functions/methods. A syntax-aware comparison of 65 Rust files preserves import
  binding multisets, function signatures/bodies and module attributes/visibility
  against HEAD, excluding the deliberately migrated private CLI. No public API
  or behavioral redesign accompanies these layout edits.
- Label workspace dependency roles from maintained member dependency edges.
  Preserve PocketIC's reviewed exact client/server constraint. Apply the shared
  manifest formatter; avoid blanket pin relaxation.
- Use Clap for the private external-lock command. Actual-executable tests retain
  exact help/usage output, exit statuses, non-UTF-8 paths, zero/bounded reads and
  unchanged input bytes; existing malformed/duplicate/source/checksum cases pass.
- Allow ordinary locked Cargo commands to retrieve dependencies. Explicit
  `CARGO_NET_OFFLINE=true` still opts out; prepared archive/source-bound jobs and
  Build's designed offline execution remain offline. Real-keyring qualification
  also retains its no-external-socket execution contract.
- Correct stale Process/Sign delivery statements, predecessor location and
  migration-hold scope. Preserve dated release/consumer evidence as historical.
  Use the Qualification changelog heading for demonstrated consumer acceptance.

## Findings

| Severity / owner | Evidence and consequence | Disposition / trigger |
| --- | --- | --- |
| MEDIUM — Toko Tooling | The shared optional setup helper's environment-name selector omits `ACCESS_KEY`. A synthetic short value printed without a label survives `excerpt`; migration would lose an existing consumer redaction guarantee. Labelled access-key filtering does not cover this case. | Retain the existing collector. [Toko Tooling #396](https://github.com/dragginzgame/toko-tooling/issues/396) owns the correction; migrate only with equivalent regression evidence. Consumer fixture now asserts omission of an unlabelled synthetic environment value. |
| LOW — Toko Tooling | Shared `excerpt` drains an admitted handle to EOF and exposes no hard read ceiling. A log growing after admission could exceed the local collector's independent 1 MiB read budget. | Retain the bounded file-reading collector. [Toko Tooling #397](https://github.com/dragginzgame/toko-tooling/issues/397) requests a shared bounded-file entrypoint without weakening normal command-output draining. |

[Toko Tooling #393](https://github.com/dragginzgame/toko-tooling/issues/393)'s
original environment-assignment, URL-fragment and labelled access-key cases are
fixed in 0.45.8. They are not reported as remaining failures. The new findings
concern the optional supplier helper; it is absent from the selected snapshot.

## Fresh verification

Local logs are retained under `target/hygiene-adoption/`; they are disposable
checkout evidence, not published release receipts.

| Command / evidence | Result |
| --- | --- |
| `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Pass (`clippy.log`). |
| `cargo test --workspace --all-features --locked` | 108 passed, 9 ignored (`tests.log`), before adding the two executable CLI regressions. |
| `cargo test -p ic-cargo-qualification --locked` | Three existing and two new tests pass (`qualification-cli-tests.log`); 110 unique passing native tests across the two selections. |
| `cargo clippy -p ic-cargo-qualification --all-targets --locked -- -D warnings` | Pass after adding the executable tests (`qualification-cli-clippy.log`). |
| `RUSTDOCFLAGS='-D warnings' cargo doc --workspace --all-features --no-deps --locked` | Pass (`docs.log`). |
| `cargo clippy --manifest-path qualification/Cargo.toml --workspace --all-targets --locked -- -D warnings` | Pass (`qualification-clippy.log`). |
| Independent qualification workspace Rustdoc, warning-denied, no dependencies | Pass (`qualification-docs.log`). |
| `perl scripts/ci/test-setup-diagnostics.t` | 58 checks pass (`setup-diagnostics.log`); synthetic values only. |
| `make check` and independent qualification selection checker | Both pin/lock selections and the 49-file installed snapshot pass (`snapshot-check.log`, `qualification-selection.log`). |
| `make tools-check` | Prepared common/Rust tools pass admission (`tools-check.log`). |
| `actionlint .github/workflows/build-release-candidate.yml` | Pass. Existing compiler component/target selections and timeout/diagnostic wiring are preserved. |
| Syntax-aware mechanical comparison | 65 files pass (`layout-proof.log`); private CLI deliberately excluded. |
| `make fmt`, `make fmt-check`, `git diff --check` | Pass after the final documentation batch (`final-format.log`, `final-format-check.log`). |

Native macOS execution, the ignored PTY/keyring/installed simulator cases,
archive release gates, hosted CI, deployment and registry publication were not
rerun. Their earlier receipts remain evidence for their original source/host
identities, not new executions in this audit. No runtime owner contract changed
outside the private CLI parsing surface. No performance improvement is claimed.
At the audit handoff all changes were uncommitted. Subsequent authorized delivery
is recorded in the [release receipt](../../../../../../../release-ownership.md#hygiene-cohort--2026-11-18);
its additional checks do not turn this audit's skipped checks into executions.
