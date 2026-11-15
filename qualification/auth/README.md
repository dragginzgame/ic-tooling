# Auth external-lock qualification

Selected consumer: Auth commit `e52a40be75dd9809d0528189d84b43fe96952ad9`.
Storage `d2762f2dc7db339a88446e9c7c620ab95785711b` independently requires the same
external-selection rule in its archive admission. Both are read-only references;
Storage's dirty locks and Canic's in-progress work are excluded.

## Contract and owner

`ic-cargo-qualification` is an unpublished development-plane executable, initially
0.1.0, with no Rust library API and no dependency on Build. Its first operation is
`verify-external-lock ORIGINAL CANDIDATE MAX_BYTES`. Each file is bounded and opened
without following its final symlink. Every candidate external tuple
(name, version, source, optional checksum) must match an original tuple exactly.
Registry checksums are mandatory lowercase 64-character hexadecimal SHA-256.
Duplicate external identities and malformed records refuse, even if identical.
Local table rows are outside this comparison; empty candidates are allowed.
Auth retains independent exact local archive, nonempty graph and dependency-policy
admission. Neither lock is written; this command does not resolve or build packages.

The new implementation derives from this comparison contract, without copying old
Build's lock module or tests. Toko 0.44.3's selected installer/runner owns tool
preparation and general delivery mechanics; no equivalent external-lock command
was found in its shipped entrypoints. This focused executable is the separately
scoped qualification boundary permitted by the replacement design. Archive extraction,
release editing and generic release execution are not included. Recheck the proposed
new package name/version before any registry publication.

The isolated Auth patch selects this prepared executable in its package-consumer
checker. Other Auth tooling and the released metadata editor remain explicitly
unmigrated bootstrap dependencies; no compatibility API enters new Build.

## Reproduction and evidence

Run `make test-auth-consumer` after explicitly preparing Rust 1.100.0, its Wasm
target and the pinned consumer's registry dependencies. The runner exports only
the selected commit, applies [the script patch](package-check.patch), builds this
workspace's qualification executable and selects it from Cargo's artifact record.
It runs Auth's original graph/lock refusal fixtures, creates the three real Cargo
archives, and admits a separate consumer through Auth's original local-graph policy
plus the new external-lock executable. Native and Wasm checks enable all library
features. Canonical principal type identity is checked by [consumer.rs](consumer.rs).

The run records archive/unpacked-input identities before and after resolution,
then all consumer input identities around compilation. All remain unchanged apart
from the explicitly resolved consumer lock. Scratch and logs remain under the
printed `target/auth-consumer.*` path; the host/Wasm compiler cache stays under
`target/auth-consumer-build`. `AUTH_SOURCE` can select another Git repository with
that exact commit. No implicit downloads or package publication occur in the runner.

Linux evidence: all original admission/refusal fixtures pass; the new executable's
three focused tests, Clippy and Rustdoc pass; exact archives pass native and Wasm
all-feature compilation. One missing locked registry dependency was explicitly
fetched before rerunning offline. This is not Auth's full feature matrix, live
backend conformance, native macOS qualification, Storage archive migration or
canonical release cutover. Storage's committed source confirms the shared lock
contract but has not yet been migrated to this executable. Canic remains pending.
