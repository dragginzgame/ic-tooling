# Held-directory sidecar locks

Canic Store and configuration writers require an existing sidecar inode to remain
stable across operations. Their released helper opens or creates a regular file,
then a separate call takes an exclusive nonblocking advisory lock. Existing
contents and permissions remain unchanged; new files request 0600 before umask.
Existing public permissions and hard links are allowed by these callers. No file
or parent synchronization is promised, and a sidecar remains after later failure.

`Directory::lock(name, mode, FilePolicy)` owns open-or-create, regular-file
admission and acquisition as one operation relative to the held parent. It returns
the existing `Lock` owner. Name and creation-mode policy are checked before effects.
Open uses read/write, create, no-follow, nonblocking and close-on-exec flags, never
truncate or replacement. A no-follow observation refuses known special entries
before opening them. Metadata from the opened descriptor then admits regular-file
kind, forbidden permissions and optional single-link policy before acquisition.

Native contention remains `Error::Io` containing the original `WouldBlock` cause.
Other native causes and admission refusals remain distinct. A newly created file
may remain after refusal; the operation does not unlink, repair permissions,
retry, create parents, or synchronize anything. Concurrent creators open the same
entry rather than replacing a lock inode. The caller excludes namespace writers;
advisory locks do not fence noncooperating processes or lock-file replacement.

`Lock::release` retains explicit unlock failure reporting; Drop remains fallback.
The original `Lock::acquire(File)` remains appropriate when the caller already owns
an admitted descriptor. This operation removes the open/create/admit/acquire
sequencing burden without adding a general file-opening compatibility family.

Implementation reuses the existing lock owner and regular-file admission checks.
The old path helper's pathname resolution and public error family are not
imported; its special-entry-before-open refusal is preserved relative to held
custody. Canic retains path selection, contention messages, reload-under-lock
and recovery decisions. Independent tests cover content/mode preservation,
contention and release, concurrent creation, held-parent rename, links/special
files, and admission before effects. Native macOS and consumer migration require
separate execution evidence.

Requirement inspection on 2026-11-16 used Canic HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes (`Store::open`
and configuration edits). The read-only legacy helper was reviewed at
`a00369fc29fa837c6deb022cceb7e09aa447292c`. No old implementation/test tree was copied.

Four new lock tests pass on Linux; all 23 filesystem and 19 Build tests pass,
with warning-denied Clippy/Rustdoc, formatting and snapshot admission. Logs:
`target/sidecar-lock-tests.log` and `target/sidecar-lock-check.log`. Current Canic
Store/configuration lock migration now passes a 39-case combined comparison with
both suppliers, including released-helper interoperability and original inode,
mode and byte preservation. See [scoped evidence](../qualification/canic/README.md#store-and-configuration-sidecar-locks).
Earlier publication comparisons retain their original lock supplier; native macOS
and production migration remain open.
