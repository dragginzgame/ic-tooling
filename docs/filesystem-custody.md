# Stage 2: Backup filesystem custody

The initial consumer oracle was Backup `d8955cf1e34f24934027f448ef16ee6841eafbb0`.
That pinned run is historical; active qualification snapshots current local
working trees as specified in [source selection](qualification/local-sources.md).
Original store/payload/capture tests remain in that export, not copied into products.

## Required behavior and selected boundary

Backup stores `ic-backup/local-store/1\n` at `format`, with private `bundles`,
`operations` and `targets` children. `format.pending` claims initialization before
children are created. Interrupted initialization remains occupied; it must never
be silently reset. Payload manifest bytes, exact member hashes and ledger frame
bytes remain unchanged. Held roots survive rename; changed child identities,
symlinks, public permissions, hard-linked files and oversized enumerations refuse.

Two effects are deliberately separate:

- Completed named publication: `Directory::stage(staging, destination, policy)` →
  consuming `write_from(reader, length)` → `seal()` → `commit()`. A failed write
  cannot produce a sealed value. There is no public writable staging handle.
- Retained recovery records: explicit create-new or open-existing-update file
  access, then caller-controlled ordered writes and synchronization. Backup owns
  ledger replay and durable spending. These are not publication-stage aliases.

`CreateNew` uses an atomic create-only hard link followed by stage removal and
parent sync. `Replace` uses descriptor-relative rename without following the final
symlink. Commit returns Committed, NotPublished (with sealed attempt ownership),
or PublishedUnconfirmed (receipt, failed phase and original cause). No retry.
Sealing syncs bytes first. Publication receipts include directory and content
identity, but grant no future immutability. Explicit stage abort reports cleanup
failure. Failure retention is selected before effects: discard owned staging or
retain it for recovery; Backup selects retention for `format.pending`.

Directory creation is explicit and separate from opening. Only single-component
child names are admitted. Root resolution permits normal ancestor aliases but
refuses a final symlink; child operations remain relative to held descriptors.
Every enumeration is bounded and completes before returning entries. A directory
identity comparison replaces callers opening extra synchronization descriptors.
Advisory locks have one owner, nonblocking acquisition and explicit release;
cooperating callers must not infer that locks fence hostile writers.

As in Backup's existing contract, callers exclude concurrent namespace/content
writers while operating on admitted state. Identity checks refuse observed foreign
stage replacements, including cleanup. Unix lacks atomic compare-identity-and-unlink:
checks do not protect against hostile mutation between observation and mutation.
No stronger guarantee is claimed. Required create-only races remain safe atomically.

## Reopening an original publication file

Backup's interrupted manifest publication requires completing publication of the
original `manifest.pending` after reopening, without regenerating bytes or
renewing application authority. `Directory::admit_retained` borrows the caller's
held original file and admits its staging/destination names, private permission
policy, single-link regular-file identity and complete expected length/SHA-256.
It opens its own descriptor at offset zero; the caller's offset is unchanged.
Admission never writes, changes permissions or removes an entry, including on
refusal. Backup supplies the expected identity from its reconciled journal;
Tooling does not interpret the manifest or decide whether to retry.

Success returns the existing `Written` state with create-only destination
authority and explicit retained-recovery semantics: failure or Drop preserves
the original stage. `seal` synchronizes it before `commit` can expose it.
Explicit `abort` remains an intentional removal request. Existing commit phase
outcomes, identity-checked cleanup and held-parent behavior remain unchanged.
Callers exclude namespace/content writers and writable aliases from admission
through commit; borrowing an original descriptor cannot revoke their authority.
This is not recovery of a visible-but-unconfirmed destination, which must first
be reconciled by the consumer.

Reuse decision: extend this rewrite's publication owner with independently
written admission; reuse its `Directory::open_file`, `FileId`, bounded byte copy
to a sink, and `Written`/`Sealed` lifecycle. No legacy exports, Backup code or
recovery framework are imported. This removes the caller's need to recreate a
retained stage or implement pathname publication syscalls.

Local Linux qualification on 2026-11-16: all 29 FS library tests pass, including
six new retained-publication cases. They cover drop/reopen and original inode,
mode and bytes; parent rename; short/excess/mismatched content; invalid names,
permissions, aliases, links and substituted identities; occupied destinations;
explicit abort; and file-sync, cleanup and parent-sync failures. FS Clippy
(`--all-targets -- -D warnings`) and Rustdoc (`-D warnings`) pass. These changes
are unpublished. [Hosted run 43858083308](https://github.com/dragginzgame/ic-tooling/actions/runs/43858083308)
at `a66eae8d500451aff613fa8819598c2257f4ffb5` also passes the packaged tests,
Clippy and Rustdoc on Linux, Apple Silicon macOS and Intel macOS. Retained
macOS artifact logs confirm all six new cases and the original source identity,
completing the native producer qualification required by
[#2 — Support publication of original retained files after reopen](https://github.com/dragginzgame/ic-tooling/issues/2).
Backup migration and current-consumer qualification remain separate work.

[Explicit directory preparation](directory-preparation.md) extends this owner
with held-root traversal and creation of named children. It returns the held leaf
and retains partial effects on failure; file publication never creates parents
implicitly. Current Canic artifact-output qualification passes; other writers remain pending.

## Byte and package ownership

Build and filesystem publication now share the independently useful byte owner,
`ic-host-artifacts` 0.80.1: length policy, bounded read/copy and exact SHA-256
identity. No paths, processes, codecs, HTTP or legacy digest wrapper family enters.
`ic-host-fs` 0.80.1 owns native custody/publication. Both version lines follow registry
0.79.0 observed on 2026-11-15 and remain unpublished. Build remains 0.87.1.
Rustix and standard File own native primitives; no legacy subsystem is imported.

The isolated Backup migration switches its filesystem supplier completely. Its
initial Stage 2 qualification kept 0.79 byte/digest and HTTP suppliers outside this
filesystem slice. Later [Backup qualification](../qualification/backup/README.md)
migrates stream admission and HTTP bodies; legacy digest values/in-memory hashing
remain explicit. None of those legacy suppliers enters the new filesystem/build
products. This is not full Backup supplier cutover or archive/host acceptance.

## Acceptance

New tests independently inspect disk effects for parent rename, create-only races,
foreign stage substitution, partial/short/growing writes, before-publication refusal,
and visible-but-unconfirmed publication. File-sync and parent-sync failures are
injected only at the owning private syscall seam; no public mock backend is added.
Actual Backup store/payload and capture tests qualify retained recovery behavior,
file/child identity, permissions, exact persisted bytes and cooperative locking.
Native Linux evidence is distinct from pending native macOS and packaged consumers.

## Jobs recovery-file cleanup

The Stage 3 Scheduling capture migration requires removing an optimizer producer
only after its independent retained copy is committed. `Directory::remove_file`
compares the named entry with the caller's still-held file before descriptor-relative
unlink. It refuses observed replacement or links; it does not synchronize the
parent implicitly. Publication reuses that same identity/removal owner and retains
its existing phase-specific parent synchronization. The caller excludes concurrent
namespace writers; no atomic compare-and-unlink guarantee is added.

Jobs owns optimizer flags, log/producer names, retained `.capture.pending` recovery,
and when a failed publication may be retried after explicit reconciliation. These
policies do not enter Build or a generic producer callback API. Capture replaces
only the selected destination entry and preserves independent complete staging
bytes if publication fails. Root paths passed to the external optimizer remain
stable caller-owned inputs during that invocation.
