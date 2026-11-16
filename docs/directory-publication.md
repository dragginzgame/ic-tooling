# Publication of a prepared directory

Current Canic starter creation and fleet-plan retention build a staging tree,
publish its files durably, verify its application content, then acquire a new
sibling name without replacing another tree. Those content and recovery rules
belong to Canic. Tooling owns the namespace transition and its durability result.

`Directory::publish_directory(staging, expected, destination)` operates on two
distinct single-component names under the held parent. The caller retains the
expected staging `Directory`, excludes namespace/content mutation, and completes
all child-file and child-directory synchronization before calling. The operation
does not recursively traverse, hash, validate or sync a tree.

It admits the staging name without following links, compares its identity to the
held expected directory, syncs that directory, rechecks the named identity, then
renames with atomic no-replace semantics. Finally it syncs the held parent. Success
returns the parent/directory identities and destination name; it does not certify
the contents or grant future immutability. Parent rename does not change custody.

Failure before rename retains the original cause and performs no cleanup. A
successful rename followed by parent-sync failure returns a distinct
`PublishedUnconfirmed` error containing the publication receipt and original cause.
The destination is visible, the staging name has moved, and automatic retry is
not justified. No failure removes a staging or foreign entry. An occupied
destination, including an empty directory or symlink, must refuse atomically.

Missing staging never triggers implicit recovery from an existing destination.
The consumer must inspect and validate retained evidence before deciding recovery
is legitimate. Concurrent cooperative publishers may compete for the destination;
only one wins. Identity observations do not fence hostile namespace writers.

This removes pathname reopening and generic preparation/failure-hook callbacks
from the publication owner. It reuses existing held-directory admission and the
selected Rustix rename primitive, not the old directory framework or test tree.
The reviewed legacy helper combines caller preparation, publication and recovery;
those are separate responsibilities in this design. No dependency is added.

Requirement inspection used current Canic HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus working-tree changes on
2026-11-16 (`starter` and `fleet_plan` writers). The read-only legacy helper was
reviewed at `a00369fc29fa837c6deb022cceb7e09aa447292c`; none of its implementation
or public hook/recovery types was imported.

Five independent tests pass, inspecting disk state for occupied names, concurrent
publishers, parent rename, substituted staging identity, missing staging, and
injected sync failure before and after rename. All 19 filesystem and 19 Build
tests, warning-denied Clippy/Rustdoc, formatting and snapshot admission pass.
Linux execution and native macOS execution are separate evidence.
Current Canic starter/fleet adapters now pass the same 16 exact tests with both
suppliers, plus warning-denied Clippy and dependency admission. See the
[source-bound evidence](../qualification/canic/README.md#starter-and-fleet-directory-publication).
This qualifies the namespace transition; tree creation/writing and consumer
recovery decisions still use their retained protocols. Production migration and
native macOS acceptance remain open.
