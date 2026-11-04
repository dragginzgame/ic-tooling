# Current handoff

## Delivered Wire 0.17 adoption — 2026-11-04

Testkit 0.86.0 is published on crates.io. Release commit
`71de6abfc21d8a26d33b3ac485499890654513db` and annotated tag
`testkit-v0.86.0` are pushed to `dragginzgame/ic-tooling:main` from the ordinary
`/home/adam/projects/ic-tooling` checkout. Adoption/qualification changes are at
`0a70714040588e2367d62ef4af1859f191e6a0de`. Private Dev/probe members follow 0.86.0
without publication. Wire is 0.17.0; its unused metadata derive leaves the graph.
Public Wire limits/errors require consumers to align that minor identity.
Core frame bytes, budgets, PocketIC behavior and management DTOs are unchanged.

Testkit is the only public Tooling group using Wire. Foundation 0.74.3, Build
0.74.3, Text 0.74.2, Host Tools 0.82.0 and Native 0.80.0 remain published.
Build's additive archive API and pipeline notes remain pending 0.74.4; its code
is Git-delivered, without another Build package publication. Platform remains
0.22, canonical Types 0.18 and Runtime 0.19. The management DTO pin remains 0.11.0.

## Validation and registry evidence

The complete 35-stage Linux release gate passed from stable committed inputs,
followed by prepared/committed-source checks, atomic branch/tag push and separate
publication dry run/upload. Wire graphs, Wasm, installed PocketIC management/
snapshot and CLI contracts, exact archives, ordinary workspace tests, documentation,
formatting, lint and release fixtures all passed. Both publication commands passed.

Independent readback confirms the uploaded archive checksum, exact Git source,
package path, MIT notice and every imported PocketIC provenance byte. SHA-256:
`7f140a5aef804f8bc9ca838f42992fa6e63c6a76c9bf09ce97b1d1ee670d1288`.
A registry-only consumer passed both public-contract tests with Testkit 0.86.0
and Wire 0.17.0, retaining canonical principal/error identities without path patches.
Evidence is under `target/deploy-wire017-20261104/` and
`target/published-integration.Uo0uMi/`; compact receipts are under
`.git/release-state/deployment-wire017-20261104/`.

Native macOS qualification is separate. New Linux/macOS 15 Intel/Apple Silicon CI
is dispatched asynchronously after this handoff push, with actual source/run URL
recorded in that deployment directory's `native-ci.json`. Do not relabel the prior
[green three-host run](https://github.com/dragginzgame/ic-tooling/actions/runs/41840534687)
at `33bc4d2578a95eac3e640eca11b3ac471930dd6f` as evidence for this new batch.

## Preserved user lock selections

Two initial attempts stopped before version preparation because Cargo.lock was
refreshed to the user's pending patch selections. Those bytes match the incoming
user state outside Wire's package records. Every state and failed log is retained;
failed validation was not reused. The maintainer then paused dependency updates;
a fresh complete gate passed with stable inputs. The writer was not identified.

The unrelated selections are restored in the ordinary checkout. Only Wire's
required update and the three Testkit group versions are aligned. Byte-exact
comparison of all unrelated package records and locked/offline metadata passed.
Cargo.lock is intentionally dirty and excluded from delivered commits.
`incoming-Cargo.lock`, `qualification-updated-Cargo.lock`, the canonical release
lock and restoration receipt remain in the Git-side deployment directory.
Task-owned diagnostic monitors were stopped; no IDE settings or system limits
were changed. No artifact cleanup was performed.

## Release-speed implementation and upstream ownership

Build retains its default fresh-target archive API and adds caller-owned checks.
The private driver keeps registry compiler outputs in an owned cache, rebuilds
all extracted workspace packages and runs fresh source/graph/all-target/doc checks.
Same-path/fixed-mtime payload rejection and cache ownership guards remain covered.
Evidence uploads exclude compiler outputs. Native hosted cache effectiveness is
not established by local measurements.

Common adapter corruption/setup/staging probes run once, while each group keeps
preparation, renderer/graph rollback, cross-group refusal and committed-payload
checks. All six adapter owners passed. Retention/evidence fixture reuse binds exact
inputs/tools/host/environment, new source/group admission and original proof/log
custody; ordinary CI executes the fixtures. Earlier focused timings remain under
`target/release-speed/`, without a whole-release performance claim.

Shared [#354](https://github.com/dragginzgame/shared-tooling/issues/354) owns the
parser optimization. The matching helper is now committed in Shared at
`840be0bd14e3321ac5387b6190c68ea464e430b7`; its producer acceptance and consumer
adoption are separate. Tooling still adopts Shared 0.29.4, revision
`628ccc75d813bfd9bae777bfbaef55838d25b1b2`, with all 164 managed files verified.
No Shared delivery or snapshot refresh is included in this Wire deployment.

Prior release/lock evidence remains under
`.git/release-state/deployment-platform022-20261104/`,
`.git/release-state/deployment-cleanup-20261104/` and their matching target trees.
The explicitly retired nested checkout's 760 KB of plans/receipts remain under
`.git/release-state/deployment-tools-0.77.0-testkit-0.77.0/historical-release-records/`.
