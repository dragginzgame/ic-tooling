# External simulator ownership: first qualification

The current local IC DB rollback driver needs an application subnet, its caller's
Tokio runtime, exact upstream client types, raw stable-memory observation and
explicit instance teardown. It does not require a pool, baseline cache, installation
policy wrapper or new public Tooling session type for this workflow.

The [DB qualification adapter](../qualification/db/README.md) selects PocketIC
16.1.0 directly. Its private consumer helper requires `IC_TESTKIT_POCKET_IC_URL`
and always supplies it to the upstream asynchronous builder. Missing configuration
refuses before construction even when `POCKET_IC_BIN` exists. It creates no runtime,
server, downloader or discovery fallback. Calls retain upstream client/type identity;
all affected driver callers borrow that client directly and consume its explicit
async `drop` operation at closing. No replacement product API is introduced here.

A separately prepared `ic-testkit-server` 0.106.0 executable owns the server process
for the qualification command. That released launcher is an explicit development
bootstrap, not a new implementation or a migrated Tooling product dependency.
The driver graph has no `ic-testkit` package. Provisioning is outside instance use;
its exported `make m0-setup` delegates to the released CLI instead of an ignored
library setup test. The qualification runner only checks prepared assets offline.
Ordinary driver invocations now require an externally supplied endpoint; this is
an isolated migration recipe, not production cutover of DB's commands.

## Honest failure and lifetime boundaries

PocketIC 16.1.0's asynchronous client has no Rust Drop implementation that deletes
its instance. Its async `drop` sends DELETE but only logs transport failure and
does not classify HTTP status. Returning from that operation establishes a deletion
attempt, not confirmed remote cleanup. The qualification independently observes
`/instances` and server reachability afterward; those observations prove this run's
outcome, not a stronger upstream API contract. The read-only inventory request is
a test oracle, not a new simulator transport implementation.

Construction still inherits upstream panic behavior. The consumer timeout can
cancel creation after remote effects; no instance ID or automatic recovery is
promised in that case. Cancellation of teardown likewise cannot prove deletion.
The external server owner outlives the whole command and stops its own process
on completion/failure. The client receives no server shutdown authority.

## Decision and remaining work

For the admitted borrowed-server DB workflow, use the upstream async client and
explicit preparation boundary; do not add a facade merely to preserve old helper
names. This removes the legacy library lifecycle wrapper and its dependency graph.
The actual consumer test remains in its exported current source, with its raw-memory
oracle unchanged. No legacy implementation or test tree enters product crates.

This is the first Stage 4 ownership result, not completion of Stage 3 or Stage 4.
A new managed-server capability still needs process lifetime, cancellation, bounded
startup, instance/server dependency and cleanup evidence. Jobs single-canister
recovery now uses the same direct upstream client boundary
(see [qualification](../qualification/scheduling/README.md)). The consumer owns
controller identity, snapshot retention and its finite install-rate-limit retry.
Unauthorized capture/restore and exact raw stable-memory restoration pass. This
does not establish multi-canister snapshot atomicity: snapshot sets still need
actual current requirements, explicit callers and partial-effect evidence.
Neither borrowing nor successful deletion promises restoration, rollback of time,
cleanup after every failure, native macOS qualification or live IC behavior.

Platform’s [current raw_rand fixture](../qualification/platform/README.md) also
uses the direct client and caller-owned runtime. The Agent fixture now does the
same, with four ingress identity cases, certificate refusal and actual Principal
package identity admission. The qualification preserves its
producer-byte and runtime rejection/cleanup/cancellation checks. This removes
managed-server custody only from those fixtures; the other Platform drivers and
Query workload remain unqualified. No shared lifecycle type follows from these
successful borrowed-server workflows.

## Scheduling rewrite: Testkit is a demonstrated owner

The active consumer is `../ic-scheduling-new`. It explicitly selects Testkit's
PocketIC re-export and held-reply fixtures. These serve shared type identity,
bounded fixture construction and deterministic pending/release control for Once,
Jobs, watchdog and held-upgrade tests. Removing that dependency is not a goal.
The [replacement contract](testkit-held-replies.md) is grounded in these
[current requirements](../qualification/scheduling/README.md). Its focused baseline
passes pressure and held-upgrade scenarios using released Testkit. Runtime, topology, funding, simulated time and
consumer assertions remain caller-owned; shared mechanisms belong in Testkit.

The earlier direct-client and snapshot experiments used `../ic-scheduling` and
remain [historical evidence](../qualification/scheduling/historical.md). Their
native-snapshot failure and local upload workaround do not establish a snapshot
requirement for the rewrite. No legacy snapshot or held-reply subsystem is admitted
for wholesale import. The new Testkit catalog, generator and exclusive controller
have independent acceptance evidence in the replacement contract; Scheduling's
replacement-supplier qualification remains separate.
