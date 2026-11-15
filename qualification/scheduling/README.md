# Scheduling host-consumer qualification

`make test-scheduling-consumer` exports IC Scheduling commit
`0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e` from local `../ic-scheduling` into a fresh
`target/qualification/scheduling.XXXXXX/source` directory. The source commit must
be available locally; the runner does not fetch or modify that checkout.

The [runner](../../scripts/qualification/scheduling-consumer.sh) applies
[streaming-selection.patch](streaming-selection.patch) to the actual Timers build
callback. It replaces retained Cargo messages with `CdylibWasmSelector`, with
consumer limits of eight distinct candidates and 64 KiB of encoded path bytes.
Diagnostics continue streaming to stderr. Process/protocol/fact admission, target
locking, captured-artifact lifetime and before/after input checks remain intact.

Cargo applies local patches for Build, Artifacts, FS and Process to the complete
consumer graph. The runner rejects duplicate or registry-selected identities for
these suppliers. Resolution is offline, and the original consumer lock is retained
for inspection. In the initial run the lock differs only by removal of the four
registry source/checksum pairs; all unrelated selections remain unchanged.
Consumer source is not copied into a maintained product or extra test package.
Its own workspace owns these tests; this directory owns the migration patch and
invocation recipe. The original source keeps its own Toko 0.43.2 selection.

## What runs

- Compile the migrated `ic-timers-pocketic` library and existing test dependencies.
- Run its seven `build::tests`: input edits/additions and symlink refusal,
  exact subject discovery, failure evidence, descendant deadlines, alias refusal
  and scratch-path admission.
- Run its five `report::tests`: complete build identity, context matching,
  incomplete-receipt refusal, replacement and hard-link-safe publication.

All twelve pass on Linux with the replacement supplier graph. The separate
`make test-build-selection` fixture exercises streaming through actual Cargo Wasm
builds and capture. These results do not prove an installed Scheduling runtime,
PocketIC behavior, consumer Wasm builds, packaged archives or macOS acceptance.
No baseline performance comparison or speedup is claimed.

## Invocation and evidence

Requires Git, tar, Bash, Perl with JSON::PP, the pinned Rust compiler and cached
consumer dependencies. No server preparation is needed. For a source checkout at
a different location, run:

```sh
bash scripts/qualification/scheduling-consumer.sh /path/to/ic-scheduling
```

Every run uses fresh scratch and prints its location. It retains original and
resolved locks, generated patch configuration, dependency metadata and test logs.
Nothing is cached as a reusable qualification proof. A failure stops the runner;
retained output remains available for diagnosis. The directory is ordinary build
scratch under `target/`, not the only copy of the migration or source identity.
Update the pinned revision and review/regenerate the patch together when advancing
this consumer; a newer sibling HEAD is never silently substituted.
