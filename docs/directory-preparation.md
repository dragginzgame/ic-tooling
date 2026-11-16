# Explicit directory preparation

Current Canic artifact/configuration output and starter staging require missing
parent creation before publication. The new filesystem operation starts from a
caller-selected held `Directory`, accepts a slice of single-component names, and
returns the held leaf. The caller owns path decomposition and root authority;
publication can use the returned directory without resolving its path again.

`Directory::prepare_directories(names, mode, forbidden)` validates every name and
the creation mode before effects. Empty names, separators, NUL, `.` and `..`
refuse. An empty slice returns the same held root. Existing children must be
directories, never symlinks, and satisfy the explicit permission restriction.
Existing modes are not changed; new modes are reduced by the process umask.
The root's admission policy remains the caller's earlier choice.

For each missing component, create relative to the held parent, open without
following links, sync the child, then sync the parent link. A competing creator
is admitted through the same type/permission checks and synchronization. This
does not grant authority to follow a foreign symlink or replace an occupied file.
Success returns custody of the leaf, not a mutable path or publication receipt.

Failure reports the component index and original filesystem cause; policy
failure before traversal has no component index. Any created prefix, including
the failed component, remains in place. There is no rollback, deletion, retry or
permission repair. A sync failure may leave a visible but unconfirmed directory.
The caller retains the root for explicit inspection/reconciliation. Cooperative
namespace exclusion is required; this is not protection against hostile writers.

The operation removes manual per-component open/create/sync sequencing and the
need to reopen a prepared pathname. It does not provide implicit file parents,
absolute-path policy, recursive deletion, directory publication or Canic formats.
Existing low-level directory admission is reused; the old pathname-based helper's
ancestor search and path reopening are not copied. No dependencies are added.

Requirement inspection on 2026-11-15 used current Canic HEAD
`8412c1edca540e6f299ab1cf9fc2d3e9f2c3ed5a` plus local working-tree changes
(`artifact`, `config`, `starter` and `store` callers). This is source inspection,
not consumer execution evidence. The read-only legacy helper reviewed afterward
was at `a00369fc29fa837c6deb022cceb7e09aa447292c`.

Independent acceptance covers invalid later names before effects, permissions,
symlink/file refusal, preserved partial creation, held-root rename, leaf custody,
and child/parent sync failures. Current Canic artifact-output qualification now
passes, as do six configuration comparisons with the original lock protocol; see
[scoped evidence](../qualification/canic/README.md#held-parent-artifact-publication).
Other Canic writers and native macOS execution remain separate gates.
