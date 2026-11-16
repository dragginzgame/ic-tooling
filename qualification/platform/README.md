# Historical Platform fixture qualification

## Retirement after the Platform rewrite — 2026-11-16

Current Platform capture HEAD `7e0e3af86372bc77729c808b42a04e154d801147`, tar SHA-256
`cd72a802156cb2c7f127f40baf41214166b247f943951fee86643bdeca9a5202`, has removed
the raw_rand and Agent scripts, fixture sources and identity checker targeted by
these adapters. Both patches refuse this source with missing-path errors, recorded
under `target/platform-adoption-next/platform-*-refusal.log`.

The two patches, three exclusive session/identity/runner scripts and Make targets
are removed. There is no runnable legacy lane or historical-source fallback.
Platform owns its replacement qualification workspace. The evidence below remains
bound to the old capture and does not qualify the rewrite or justify restoring
its old APIs. See the [current adoption inventory](../../docs/qualification/platform-adoption.md).

## Historical raw_rand qualification — 2026-11-15

The former `raw-rand-session.patch` migrated only the then-current entropy fixture to
upstream PocketIC 16.1.0. The driver owns its Tokio runtime and instance, requires
an explicit external URL and awaits instance deletion. It removes that fixture's
Testkit dependency and managed-server tuple; other Platform drivers retain their
existing infrastructure. No Platform or Canic implementation enters product crates.

The historical runs used Scheduling's prepared Testkit 0.106.0 launcher and server
assets plus an installed `ic-wasm`. The shared helper captured tracked and
non-ignored source read-only and checked capture stability. These tool locations
and source assumptions are no longer current runner instructions.

The runner uses Platform's focused `check-platform-raw-rand.sh`, retaining its
source-owner identity checks, feature-graph restrictions, compiler/lint policy,
source-drift checks and both reference/narrow codec artifacts. Each installed
artifact exercises entropy byte interoperability, structured rejection,
callback-trap cleanup, pending-call cancellation and a subsequent successful call.
Artifact size output belongs to the existing consumer fixture; this migration
makes no performance or size improvement claim.

The qualification additionally checks the driver's resolved graph has PocketIC
16.1.0 and no Testkit, and independently observes a deleted instance while the
external server remains reachable. The released CLI is still the explicit
bootstrap server owner. Upstream creation can panic or leave remote effects after
timeout; async deletion is best-effort. See [ownership](../../docs/simulator-ownership.md).
The raw_rand slice does not qualify Platform's signature, BLS, Wire or broader
runtime drivers, nor resolve Query's supported workload requirements.

Linux evidence on 2026-11-15: source HEAD
`c24e3a59583eabcf5b6bb87987269c13a209a51f`, captured tar SHA-256
`1d7702b7ea6f3dfeef7dc0dac561aa94460d40cca9ce0bca6f56621b23e14e16`.
The final export is `target/platform-consumer.dx9ocBbv`; its consumer evidence is
`source/target/evidence/raw-rand.dCHC8z`, with outer log `target/platform-final.log`.
Both installed codec subjects, warning-denied Wasm/driver Clippy, source/owner
admission and graph checks passed. Missing endpoint refused with `POCKET_IC_BIN`
set to an unavailable path; inventory remained one deleted instance. Prepared
CLI/server/optimizer hashes matched after execution. The fixture generates its
own lock offline from the source lock; this is not a claim that every dependency
was already selected by the root workspace. No production sibling was edited.

## Historical Agent ingress and upstream type identity

The Agent run selected its own session patch independently of raw_rand, using
the same prepared CLI and server assets. The consumer built its tiny echo canister
from WAT without an optimizer.

The patch removes only the Agent fixture's Testkit dependency and server custody.
It preserves the existing NNS/application topology, explicit root key, query,
update-and-wait and read-state checks for Ed25519, delegated, P-256 and secp256k1
identities. Every identity must also reject certificate verification with the
wrong root. The upstream gateway is owned by the instance; the outer command
continues to own the server process. Missing endpoint refuses before construction.

The consumer's identity checker now follows either direct PocketIC or the retained
Testkit re-export chain. It requires one client supplier for an Agent fixture and
compares the actual `ic_principal` package IDs behind Agent and PocketIC's Candid
edge. Merely finding similarly named packages in metadata is insufficient. Other
consumer ownership checks and the old Testkit path remain in place.

Agent Linux evidence on 2026-11-15: the same source HEAD and tar hash recorded
above, export `target/platform-consumer.5wsJhl9S`, consumer fixture
`source/target/agent-installed.SExA5J`, log `target/platform-agent.log`.
All four installed identities passed query/update/read-state and wrong-root
refusal. Warning-denied all-target driver Clippy, no-Testkit graph, missing-endpoint
refusal and deleted-instance/server-survival checks passed; prepared tool hashes
were unchanged. `platform-principal.sh` deliberately substitutes a different
Principal supplier on PocketIC's Candid edge and requires identity refusal.
That negative check and Platform's existing focused identity-checker tests passed,
including the retained Testkit path. Agent and PocketIC both select `ic_principal`
0.1.5 in the admitted graph. No production consumer was edited.
