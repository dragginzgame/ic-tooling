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
