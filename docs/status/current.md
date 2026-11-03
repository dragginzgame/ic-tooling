# Current handoff — 2026-11-03

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
| Tools | 0.80.0 | `109244b35678e9d89ecec6963ab535c8d65b2f25` |
| Testkit | 0.84.0 | `5f91f4cdbaa72e04cc0d7ba92e4e1c3953b0ae8e` |

Native remains 0.80.0. Private Dev/probe packages follow Testkit 0.84.0 and were not
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

## Validation and host evidence

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
Full final-source native CI was still pending at handoff; no new full macOS
qualification is claimed. [#324](https://github.com/dragginzgame/ic-tooling/issues/324) retains
its supported-host acceptance obligation; Platform's companion #139 remains
outside this delivery.

## Checkout and evidence custody

The ordinary checkout is on merged main with Build 0.74.2 and Testkit 0.84.0.
Its unrelated Cargo.lock registry selections are preserved; only local package
versions were aligned to the released groups. They remain uncommitted and were
excluded from all delivered source/release commits.

Release checkout, logs, archives, failed attempts and publication admissions remain
under `/tmp/ic-tooling-platform-deploy.BGZn8d/release/`. Durable receipts, complete
gate proof, registry readbacks, publication evidence and incoming lock backups are
in `.git/release-state/deployment-testkit-owner-20261103/`. Development evidence
remains in `target/cleanup-pass/`. Earlier failed attempts stopped before version
preparation or publication; their original logs and artifacts are retained.
No artifact cleanup was performed.
