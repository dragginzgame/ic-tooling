# Current handoff — 2026-11-04

## Local Shared refresh and complexity reduction

This uncommitted batch adopts Shared Tooling 0.29.4 at
`628ccc75d813bfd9bae777bfbaef55838d25b1b2` through the canonical exporter.
All 163 managed files match the selected source; tool catalogs are unchanged.
The CI-selection fixture now prepares the classifier's declared companion closure,
including `StableVersion.pm`, rather than copying its entrypoint alone.

Thirty-seven inline Rust test modules now use ordinary `tests.rs` or `tests/`
discovery. Private fixtures/helpers moved without widening production visibility or
changing Cargo target roots. Three small Unix FS fault-injection entrypoints retain
their restricted boundary, with the reason documented beside them.

The subsequent authorized implementation simplifies 71 blocking PocketIC
forwarders and removes their now-unnecessary shared runtime allocation. The client
requires an explicit server URL: sync/async `new` and `list_instances` take an
endpoint, builders without one return `ClientError::MissingServerUrl`, and
`with_server_binary`, `StartServerParams` and the implicit downloader/launcher
are removed. `pic` remains the sole provisioning/managed-startup owner. Its instance
builders supply the URL. Direct client flate2 and semver selections are removed;
imported temporary instance-state custody remains. This API cut has migration
instructions and pending Testkit 0.85.0 notes, with no manifest version bump.

A separately checksummed `tooling-maintenance.patch` records the client changes.
The source guard reverses it before the unchanged transfer/upstream reconstruction;
original donor hashes, license notices, protocol/crypto identities and fallible
no-replay/snapshot contracts remain. Mutation checks cover the additional patch.

Build now derives routine error formatting while preserving streamed list rendering,
diagnostic text and source chains. Both caches share failed-acquisition finalization:
published/disarmed entries survive later errors; incomplete entries are cleaned
once, retaining original and cleanup causes on failure. Generic commit cleanup no
longer silently retries on Drop. Fingerprints, retained formats, locking, publication
and materialization policies remain distinct and unchanged.

The same LOC command reports 37,743 non-test-file Rust lines and 46,894 test-file
lines, with 1,205 test functions and zero inline test functions. The test-layout-only
stage was 38,144 / 46,792 / 1,201; this implementation removes 401 non-test-file
lines and adds 102 test lines (299 fewer total). The original 44,247 non-test-file
count included inline tests, so its larger decrease is primarily classification.

Focused Linux evidence in `target/complexity-reduction-20261104/` records:
207 artifact/cache unit tests, 21 client tests, 28 startup tests with six existing
ignores, and one installed management/snapshot test against the explicitly admitted
16.1 server. Affected Build/Testkit/Dev Clippy, native/Wasm graphs, deployed Wasm
compilation, source reconstruction/mutations, error diagnostics and documentation
checks passed. Initial sandbox client failures denied local listener creation;
the identical native tests passed with socket access enabled. Full workspace/release
qualification and native macOS qualification were not run for this local batch.
The earlier layout evidence (372 tests, six existing ignores), Shared consumer
fixtures and all 99 CI-selection cases remain in `target/loc-cleanup-20261104/`.

All incoming unrelated `Cargo.lock` content is preserved; the only subsequent
lock edits remove ic-testkit's two retired direct dependency edges. Work remains
local on main, with pending group notes and no commits, pushes or publication.

The read-only retention guard initially refused an old nested checkout and build
tree under `.git/release-state/deployment-tools-0.77.0-testkit-0.77.0/release/`.
Review established clean/inactive state and no unique refs: every ref is already
in current main ancestry. With explicit user approval, that redundant checkout
and its temporary comparison archive were removed. Its 760 KB of completed plans
and receipts remain byte-for-byte under `historical-release-records/` beside the
original delivery evidence, with ref/status inventories and the review record.
Retained-state admission now passes without relaxing the shared limits.

The user has authorized deployment of this batch. Selected releases are
Foundation 0.74.3, Build 0.74.3, Text 0.74.2 and Testkit 0.85.0, in supplier order.
Tools and Native have no new release. The ordinary checkout is the release owner;
unrelated lock selections remain separately preserved rather than delivered.

The following delivered baseline and host evidence precede this local batch.

## Delivered Platform 0.21 adoption

The root catalog selects published Platform 0.21.0 through
[PR #331](https://github.com/dragginzgame/ic-tooling/pull/331), merged at
`a772e6848500c3697c6d2228a7501917c1c35471`. Tools 0.81.0 and Testkit 0.84.1 are
pushed, tagged and published separately. Tools consumers exchanging Platform-owned
certificate/hash-tree or Candid decoder-limit values must align Platform 0.21.
Testkit retains its public client and external DTO/error identities. Types 0.18,
Runtime 0.19, Wire 0.16 and server 16.1.0 remain selected.

Focused Linux checks passed: native/Wasm feature graphs, affected Tools/Testkit/Dev
Clippy, public certificate/Principal/Wire exchanges, direct-client/facade management
DTO identity, installed management/snapshot behavior and six snapshot-helper cases.
Tools and Testkit each passed their complete 35-obligation source-bound Linux
release gates, publication dry runs/uploads and independent registry archive,
checksum, exact VCS and license/provenance verification. A registry-only consumer
compiled the published Tools/Testkit APIs with one Platform 0.21 identity,
including certificate/Candid limits and client/facade management DTO exchange.

Native macOS evidence is separate: runtime, optimizer and browser qualification
passed on all three hosts at adoption source `a772e684` before version preparation.
Full final-release native qualification is dispatched asynchronously at
`c484b50c4011979cd5ee5c4f6299de8cfd5e0257`: [CI 41700137827](https://github.com/dragginzgame/ic-tooling/actions/runs/41700137827),
[optimizer 41700142442](https://github.com/dragginzgame/ic-tooling/actions/runs/41700142442),
[runtime 41700148093](https://github.com/dragginzgame/ic-tooling/actions/runs/41700148093), and
[browser 41700153602](https://github.com/dragginzgame/ic-tooling/actions/runs/41700153602).
Full final-source native CI was pending at handoff; earlier successful host
qualification is retained under its own source identity.

## Delivered cleanup and client ownership

The accepted five-part cleanup is integrated through
[PR #326](https://github.com/dragginzgame/ic-tooling/pull/326). Testkit now owns the
maintained PocketIC 16.1 client in `ic_testkit::pocket_ic`; `pic` retains lifecycle,
sender, funding and rollback policy. Snapshot helpers delegate to that client's
bounded transfer engine. Build's prepared-input batch owner is internal, fixture
retention checks exercise the real initialization/EXIT boundaries, and the current
extraction contract is separate from its historical migration account.

Release qualification found and repaired three integration defects through normal
merges: the exact management DTO pin needed recorded identity evidence
([#327](https://github.com/dragginzgame/ic-tooling/pull/327)); the Make fixture's
independent golden roster needed the explicit tooling entries
([#328](https://github.com/dragginzgame/ic-tooling/pull/328)); and two startup
doctests needed external `ic_testkit::pocket_ic` imports
([#329](https://github.com/dragginzgame/ic-tooling/pull/329)).

| Group | Published version | Release source |
|---|---|---|
| Foundation | 0.74.2 | `aa62709bff680ac7c8a050c88cff769c818eebf2` |
| Build | 0.74.2 | `ba0c79118e9bcb8a0af64143a01dbd71cf2cd61a` |
| Text | 0.74.1 | `df9d9cdffe8d56fb82c3132a4860d2567d6f01a7` |
| Tools | 0.81.0 | `8ac12cf92fe04c6f4aec0ce4aad80fcd13df07f0` |
| Testkit | 0.84.1 | `c484b50c4011979cd5ee5c4f6299de8cfd5e0257` |

Native remains 0.80.0. Private Dev/probe packages follow Testkit 0.84.1 and were not
uploaded. Build and Testkit were released separately through the normal adapters,
with atomic main/group-tag pushes, followed by separate publication commands.
The [group ledgers](../../CHANGELOG.md) retain the finalized release notes.

Consumers select Testkit 0.84 directly and replace former Platform/client imports
using the [migration guide](../testkit/client-migration.md). Remove the independent
client dependency and Platform's `pocket-ic` feature. Client-owned DTO/error
identities move with Testkit; external IC identities and server 16.1.0 remain.
Native dependencies stay absent from the Wasm measurement graph. Platform's
retirement work is separate sibling work and was not edited or released here.

## Shared ownership rule and provenance

[Shared PR #351](https://github.com/dragginzgame/shared-tooling/pull/351) merged the
native Testkit ownership exception. Tooling adopted committed source
`bd193bd407c01f3fb95c8302e45cbdde31609f4b` through the exporter; all 162 managed
files match. The snapshot identifies pending Shared 0.29.0 and retains its UTF-8
fix. No Shared release or tag was included in this delivery.

The transferred client donor is Platform
`41688ced81066b90aee756def7468a8388ee67e9`, including its macOS mock-socket fix.
Source, original Apache notices, reconstruction patches and hashes are preserved
in Testkit and its published archive. Testkit's license is `MIT AND Apache-2.0`;
Tooling-owned package code retains its canonical MIT notice. The former isolated
contribution `69f1be9af8080374393d7c65d65916175a01ec36` and its bundles remain at
the locations recorded by Platform's handoff. Implementation no longer exists
only beneath Platform's `target/`.

## Client-transfer validation and host evidence

Build and Testkit each passed their complete 35-obligation source-bound Linux
release gates. Publication dry runs, uploads, independent registry/archive
checksum and exact VCS readbacks passed. Testkit's archived imported provenance
and license files match the release source byte-for-byte. A registry-only consumer
compiled the published Build snapshot API and Testkit sync/async client, typed
error, snapshot transfer and lifecycle-extension imports.

Focused Linux development evidence additionally covers 15 typed-failure/transfer
tests; installed management identity, controller snapshots, rollback, funding,
timer and hook restoration against server 16.1; native/Wasm graph profiles;
source reconstruction and mutation guards; Build batch resolution and prepared
Wasm integration cases; retention, routing, formatting and documentation links.
Recovery/routing fixtures substitute product qualification effects; those fixture
passes do not themselves prove host execution.

Native macOS evidence is separate: PocketIC runtime qualification passed on
macOS 15 Intel and Apple Silicon, alongside Linux, at implementation source
`e0533bbadd365f9ca6adb69323725a05f9def9cc` in
[runtime 41673847171](https://github.com/dragginzgame/ic-tooling/actions/runs/41673847171).
Optimizer and browser qualification also passed at that source. Its routine full
native CI jobs were superseded by the Build release push, so they are cancelled,
not passes. Explicit final-source qualification was dispatched asynchronously:
[CI 41681210184](https://github.com/dragginzgame/ic-tooling/actions/runs/41681210184),
[optimizer 41681215793](https://github.com/dragginzgame/ic-tooling/actions/runs/41681215793),
[runtime 41681221801](https://github.com/dragginzgame/ic-tooling/actions/runs/41681221801), and
[browser 41681227057](https://github.com/dragginzgame/ic-tooling/actions/runs/41681227057).
All four final-source workflows passed, including full native CI/publication
qualification on Linux x86-64, macOS 15 Intel and macOS 15 Apple Silicon, plus the
compiler status job. [#324](https://github.com/dragginzgame/ic-tooling/issues/324)
is closed with that acceptance evidence. These results qualify the delivered
Testkit 0.84.0 source, separately from the new Platform 0.21 adoption above.

## Checkout and evidence custody

The ordinary checkout is on merged main with Tools 0.81.0 and Testkit 0.84.1.
Its working lock retains unrelated package versions/checksums. Platform 0.21 and
the four local release-package versions are aligned; Cargo normalized eleven
stale Windows dependency links. The original lock is preserved separately from
the normalized working selection. Unrelated selections remain uncommitted and
were excluded from all delivered source/release commits.

Release checkout, logs, archives, failed attempts and publication admissions remain
under `/tmp/ic-tooling-platform-deploy.BGZn8d/release/`. Durable receipts, complete
gate proof, registry readbacks, publication evidence and incoming lock backups are
in `.git/release-state/deployment-platform021-20261103/`, with the client-transfer
delivery in `.git/release-state/deployment-testkit-owner-20261103/`. Development
evidence remains in `target/platform021-adoption/` and `target/cleanup-pass/`.
Earlier failed client-transfer attempts stopped before version
preparation or publication; their original logs and artifacts are retained.
No artifact cleanup was performed.
