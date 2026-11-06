# Current handoff

## Shared Tooling 0.30.9 and Testkit 0.89.3 delivered — 2026-11-06

Shared GitHub revision `18ba8d1fdbc76a5c8334d1ad1c71f9bfcd4580a1` is adopted
through the supported exporter; all 175 managed payloads match that independently
selected commit. Source integration is `a36baa84`. Testkit 0.89.3 is published
from clean release commit `5d9cdbf8f0bfcfc970adf338eec5a571ee12be69`, with
`main` and annotated tag `testkit-v0.89.3` pushed to `dragginzgame/ic-tooling`.
Private Dev/probe follow Testkit and remain unpublished. Other groups are unchanged.

| Group | Published version |
| --- | --- |
| Foundation | 0.74.4 |
| Build | 0.75.0 |
| Text | 0.74.2 |
| Host Tools | 0.83.0 |
| Native | 0.80.0 |
| Testkit | 0.89.3 |

Tooling selects Shared's `release-candidate-setup`: fresh groups prepare host
parsers before version lookup; recovery checks the saved selection offline.
The standard preflight retains editor/server setup and full product qualification.
The old bootstrap prerequisite and local batch-resume recipe are removed. Shared
owns status, resume, guarded reconciliation and explicit retirement. See the
[publishing guide](../publishing.md) and [Shared contract](../frozen-releases.md).

Normal increments capture committed source, qualify, commit/tag/push and publish.
Do not follow a successful default release with another `make publish`. Select
one affected supplier-first roster and recover using its batch ID. Git-only
delivery requires explicitly clearing `RELEASE_FROZEN_PUBLISH_TARGET`.

## Current release evidence and limits

Actual batch `batch-UqiVuh8F` completed from
`target/release-frozen/batch-UqiVuh8F/`; durable Git, intent and plans remain under
`.git/release-state/frozen/batch-UqiVuh8F/`. The saved intent records the selected
setup target and exact delivered publication commit. All 34 complete Linux release
stages passed on Rust 1.99.1, including fresh archives, native/Wasm graphs and
installed PocketIC contracts. In that captured checkout, gate logs are under
`target/release-validation/testkit-gate.6BYBIS/logs/validation.MdGIrz/`, and
publication receipts are under `target/publish/publish.Op8Tkp/`.

Registry readback matches the uploaded archive and exact clean Git provenance.
Its checksum is `df8ce0ea709c2b8b747274540b230a42e5bc39348e71046383189b2005d1695e`.
All three registry-only public-contract cases passed; captured consumer evidence
is `target/published-integration.Nx50BZ/`. The native host matrix is
[running asynchronously at the release commit](https://github.com/dragginzgame/ic-tooling/actions/runs/42256140504).
No new native macOS pass is claimed from Linux evidence.

Focused Make routing, actual offline parser admission, snapshot/source comparison,
Perl syntax, ShellCheck and documentation links passed. Logs are under
`target/shared-tooling-0.30.9-*.log`. The actual hook fixture passed uninterrupted
at `target/tmp/frozen-release.KSvZO0/`: fresh/recovered setup, two publication dry
runs, deferred lock custody, archive identities, uncertain/completed resume and
stale-remote refusal. Product stage bodies were substituted there; the production
release independently completed its full gate and real upload above.

## Deferred work and retained recovery

Shared's `make release-batch-reconcile BATCH=batch-UqiVuh8F` imported the release
refs and fast-forwarded the clean normal checkout. Its transcript is retained at
`.git/release-state/deployment-testkit-0.89.3/reconcile.log`. Deferred lock edits
were preserved first on local branch
`preservation/testkit-0.89.3-lock-61aa17c8bc54` and reapplied deliberately afterward.
The normal checkout contains the delivered release; unrelated lock selections
remain uncommitted and were never included in qualification or publication.

That deployment directory's `deferred-latest` identifies the exact saved lock.
The restoration comparison changes only the three owned Testkit versions from
0.89.2 to 0.89.3. It also retains checksum/provenance, remote-ref and CI receipts.
Later live lock updates remain deferred work; never restore older backups over
them implicitly. The .89.2 deployment record still retains its earlier snapshots,
restored activation-stash identity and failed ordinary-tree release log.

No batches, build artifacts, source pins or recovery records were retired.
Earlier failed hook fixtures retain their original diagnostics; the recovered
.8 qualification remains at `target/tmp/frozen-release.MLbx9d/qualification.json`.
Reconciliation requires a clean normal checkout and only fast-forwards. Retirement
is separately authorized deletion of one completed batch's checkout, build/tool
outputs and recovery records; preserve needed evidence before selecting it.
Legacy plans/receipts keep their original source/adapter identities.

## Maintained contracts and evidence owners

Testkit remains async-only with caller-owned runtimes and explicit teardown.
`CachedPocketIcBaselinePool::shutdown(&self)` closes admission, waits for leases
and returns all attempted deletion failures without replay. Unattempted baselines
survive cancellation; in-flight deletions require caller reconciliation. Focused
Linux pool/installed proof remains at `target/evidence/pool-shutdown.lGjQv6/`.
See [client migration](../testkit/client-migration.md) for consumer obligations.

The II attachment record and fixture-owned cleanup contract belong to the
[consumer guide](../../testing/browser/internet-identity/README.md); earlier browser
acceptance is under `target/evidence/ii-testkit-0.89.1-001/`. Tools/Build validation
narrowing still requires the matched native-host comparison in the
[qualification contract](../design/release-validation.md); uncertain/shared
changes retain the full gate. GitHub issues remain the sole follow-up tracker.
Historical release/recovery narratives remain in the
[earlier committed handoff](https://github.com/dragginzgame/ic-tooling/blob/7f88408f/docs/status/current.md).
