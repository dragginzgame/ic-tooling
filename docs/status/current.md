# Current handoff — 2026-10-27

## Scope and source

The isolated contribution starts at release 0.69.0,
`f268aef5d205e51811e934648afcd13f026635f1`. The normal sibling checkout is
untouched. Package metadata remains 0.69.0; the compatible addition has pending
0.69.1 notes, with no release or publication performed.

[Tooling #257 — Expose mutable client access while retaining PocketIcInstance server custody](https://github.com/dragginzgame/ic-tooling/issues/257)
owns `PocketIcInstance::instance_mut()`. Native consumers can call mutable client
setup APIs while retaining the wrapper's managed-server owner and teardown order.
The exclusive borrow cannot outlive the wrapper. Explicit replacement still
transfers the old client's lifetime obligations to its caller.
[Auth #90 — Consolidate admission policy and simplify client and qualification state](https://github.com/dragginzgame/ic-auth/issues/90)
retains adoption after a supplying Testkit release; published 0.69.0 exposes only
an immutable borrow and Auth has four `make_live` callers.

## Focused evidence

Linux evidence is retained in this isolated checkout's
`target/auth-instance-access/`: the client/server drop-order test passes after
mutating the retained client, all three wrapper borrow/non-detachment compile-fail
doctests pass, and Testkit library Clippy passes with warnings denied.
Native macOS and full workspace/release qualification were not run.
No setup, server download, version bump, merge or publication was performed.

## Retained obligations and history

Host, Build and Testkit keep their extraction boundaries, Shared Tooling 0.26.22
and Platform 0.7 selection. Rust 1.91 remains the minimum; development uses 1.99.1.
Linux and native macOS 15 Intel/Apple Silicon qualification remain distinct.
Consumer-owned credentials, budgets, installation and recovery policy stay local.

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
and [Platform #50](https://github.com/dragginzgame/ic-platform/issues/50) remain
unresolved: crypto-bigint 0.5.5 is selected through actual ECDSA signing, so no
unused-code exemption applies. No local crypto fork, override or exemption was
added. The earlier constraint review and evidence remain retained, not requalified
by this accessor change.

The [release-0.69.0 handoff](https://github.com/dragginzgame/ic-tooling/blob/f268aef5d205e51811e934648afcd13f026635f1/docs/status/current.md)
retains prior Shared/Platform adoption, focused cleanup evidence and exact artifact
paths. Migration/retirement archives at `/tmp/ic-tooling-migration/`, original Host
refs, and stale-lock recovery evidence at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`
remain preserved. Earlier failed attempts, recovery plans, artifacts and backups
are not disposable. Normal releases start fresh; exact-attempt recovery uses the
saved source and never reconstructs state from partial preparation.
