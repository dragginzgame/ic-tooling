# Canic retained-record admission

`make test-canic-consumer` snapshots the current `CANIC_SOURCE` working tree
(default `../canic`), including local edits and non-ignored additions, without
modifying it. [Source selection](../../docs/qualification/local-sources.md) records
the capture and refusal contract. The [migration patch](record-admission.patch)
contains the private caller change, tests and dependency delta; it must apply to
the current source. Exports remain under `target/`.

## Selected contract and ownership

Canic's `store::read` admits retained plan, authority, signed-request and reply
bytes. It must return complete bounded regular-file bytes; reject a final symlink,
non-file, oversized input or missing file; and preserve supplier causes. Existing
ancestor symlinks remain supported. The consumer resolves the parent once, holds
that directory, opens its final component through `ic-host-fs` 0.80.1 and admits
bytes through `ic-host-artifacts` 0.80.1. Permissions and hard links retain the old
read policy; trusted path selection and exclusion of concurrent writers remain
caller obligations. Parent resolution is not a hostile namespace confinement claim.

This removes the old `read_file_no_follow` supplier from the record-read path.
No new product API, path-helper family or compatibility facade was introduced.
The private adapter is newly implemented against the existing owner contracts;
no legacy supplier source or test tree was copied into the replacement. Existing
Canic tests stay in its exported source and serve as consumer acceptance evidence.

Persisted JSON, hash inputs, signed bytes and reply bytes do not change. Failure
nesting can change; original I/O causes remain available. Missing final files carry
`NotFound`; the new bounded stream admits bytes without the old early metadata-size
shortcut. A rejected trailing slash cannot become a successful file read.

## Focused qualification

The runner checks both replacement aliases in Cargo's resolved `canic-host` node,
requires the two new tests to exist, then executes:

- Two new record-read tests: exact/empty/oversized bytes, original missing cause,
  non-file and final-link refusal, ancestor-link acceptance and trailing slash.
- Original plan identity/artifact custody and retained evidence/write-lock tests.
- Original signed recovery after effect expiry and uncertain expired attempt
  retention tests, including the exact retained reply and no fresh signature.

Latest current-source Linux run: all six tests passed in
`target/canic-consumer.aL9hWNgO`, from HEAD
`df9321fc17c99c93e07d631173200a99c2a890bc` plus local edits (including the
Wire 0.22.4 lock selection). Snapshot SHA-256:
`de4951e14d5efb3bf58671754715b44b65e5980fe46287ed6d38441b70ed9190`.
`target/stage3-canic/current-runner.log` retains execution evidence.
The previous `0086d49` run and its Clippy result are historical only.
These are local host tests, not network or installed canister qualification.
Prepared Rust 1.100.0 and cached dependencies are required; the runner is offline
and installs nothing. Metadata is retained alongside each export for inspection.

## Remaining boundaries

Legacy filesystem publication, parent preparation, directory commit and locking
remain explicit Canic dependencies. The existing writer implicitly creates missing
parents, so replacing it requires a separate parent-authority contract and its
actual callers. Artifact builds still use legacy process execution and path capture;
CLI presentation also remains unmigrated. Their presence is not a fallback for the
new record reader. No full Canic migration, archive/native macOS qualification or
performance improvement is claimed. Auth is on hold; Storage, Observability and
Wire remain frozen and are not migration targets for this slice.

## Current CLI Text

`make test-canic-text` snapshots the current local Canic working tree and applies
`text.patch` independently of the record-admission recipe. It selects Text 0.77.0,
migrates escapes and plain tables, and removes the repeated two-space/header-rule
options from the callers. Canic retains its own report selection, configuration,
colours, topology, shell quoting, status meanings and raw JSON. No Canic
implementation is admitted into product crates. The patch does not migrate other
Host/Build dependencies or import old Canic source.

The focused runner checks the CLI's actual Text dependency edge and requires
nonempty named test selections: eight status tests, eight fleet terminal tests and
three local declaration/output cases. These cover unknown/uncertain effects,
retained evidence, terminal controls, roles, partial inventories and raw JSON.
They do not deploy canisters, start a browser or contact a live IC network.

Linux evidence on 2026-11-15: local HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus edits, source tar SHA-256
`67f2b4b6cfb2b4ec0df7b0cf37f62e2482fe7a7c197ed55ea6bc227a580b5bb5`.
Export `target/canic-text.XBgmkNQb`, log `target/canic-text-run.log`: all 19 selected
cases passed. Equivalent source export `target/canic-text.JhcjASOF` supplies
warning-denied CLI/test Clippy (`target/canic-text-clippy.log`) and retained
external-lock identity admission. Three Text contract tests and product Clippy
pass; existing bordered-table/escaping tests remain unchanged. No performance or
production migration claim is made.

## Current owned build

`make test-canic-build` applies `owned-build.patch` independently to a current
local Canic snapshot. Canic keeps recipe/lockfile setup, manifest and package
selection, Wasm admission, default/explicit destination choice and publication.
The new Build owner replaces full Cargo-output retention, manual artifact-path
selection and the later mutable-file read with private execution and owned bytes.
No Canic implementation is copied into product crates and no new Build API is added.

The exported builder requires prepared `IC_OWNED_CARGO` and `IC_OWNED_RUSTC` and
cached dependencies. The runner selects Rust 1.100.0 explicitly. A bounded offline
metadata probe selects one workspace package by name and one cdylib target;
ambiguous or absent selections refuse. Compiler/manifest metadata runs from the
manifest directory. The default destination remains the selected Cargo target's
`wasm32-unknown-unknown/release/<library>.wasm`, but it receives separately captured
bytes after private build cleanup. Explicit destinations preserve their existing
publication behavior. This is an ordinary build, not a source-frozen release proof.

The runner uses this repository's maintained Wasm probe in a private export. It
seeds stale default output, runs the real Canic CLI for default and explicit
publication, checks returned SHA-256 against the resulting bytes and verifies
the probe marker. Compilation refusal preserves existing output; publication
onto an occupied directory refuses without removing its sentinel. All invocations
leave the selected private scratch empty. The CLI's actual Build dependency edge
is checked. No deployments, shared-target performance claim or automatic tool
installation is involved.

Linux evidence on 2026-11-15: source HEAD
`e3e7be86301c377db8b064d2ef914f5cde936265` plus current edits; final source tar SHA-256
`7950b7391ef6d8ded6d2258319b91853c3af0f68dea22c83b30f855127f1544b`,
export `target/canic-build.tnvbBh2y`, log `target/canic-build-final.log`.
The focused build/publication/refusal checks and final-snapshot host Clippy passed
(`target/canic-build-final-clippy.log`). The dependency diff adds Build and its
local byte/filesystem suppliers, plus `tempfile` 3.27.0 already selected by Build.
The strict external-subset checker correctly refuses that addition; existing
external selections were retained. Older Host FS/Process remain for consumer-owned
metadata, lock setup and publication. The separate Text/record recipes are not
implicitly combined or requalified by this run.
