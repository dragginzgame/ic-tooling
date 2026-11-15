# Current Platform raw_rand qualification

`raw-rand-session.patch` migrates only the current Platform entropy fixture to
upstream PocketIC 16.1.0. The driver owns its Tokio runtime and instance, requires
an explicit external URL and awaits instance deletion. It removes that fixture's
Testkit dependency and managed-server tuple; other Platform drivers retain their
existing infrastructure. No Platform or Canic implementation enters product crates.

Run `make test-platform-consumer` or `make test-platform-agent` directly. By default,
the installed Toko checker admits Scheduling’s already prepared `ic-testkit`
0.106.0 CLI (feature `cli`), and the server assets come from
`../ic-scheduling/.tools/ic-testkit-server`. No tools are installed automatically.
Override `PLATFORM_TESTKIT_CLI` with a prepared executable and
`PLATFORM_SERVER_DIRECTORY` with its prepared assets to use another location. `PLATFORM_SOURCE` defaults to the current local `../ic-platform`.
The sibling is read-only. The shared snapshot helper records current tracked and
unignored source, checks capture stability and refuses stale patches. The exported
fixture uses the sibling's prepared host-tool links and an already installed
`ic-wasm`; there is no installation or network fallback.

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

## Agent ingress and upstream type identity

`make test-platform-agent` selects `agent-session.patch` independently of the
raw_rand patch, using the same prepared CLI and server-directory variables.
The shared runner selects only one fixture per invocation. No optimizer is required
for Agent: the existing consumer builds its tiny echo canister from WAT.

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
