# Current handoff

## Shared Tooling 0.30.9 and Testkit 0.89.3 deployment — in progress

The requested refresh selects GitHub commit
`18ba8d1fdbc76a5c8334d1ad1c71f9bfcd4580a1` through the supported exporter.
All 175 managed payloads match that independently selected revision. The incoming
Cargo.lock is unrelated work and remains outside this adoption; no package
versions, release tags or registry uploads are changed by the refresh.
The subsequent deployment request selects Testkit 0.89.3 as the first production
run with .9's centralized setup and guarded reconciliation. Other groups retain
their versions. The complete new release gate and publication are pending.

Tooling now selects Shared's `release-candidate-setup` interface. Fresh batches
prepare host parsers before version lookup; resumed hooks check the saved parser
selection offline. The standard release preflight retains editor/server setup and
full product qualification. The old bootstrap prerequisite and local batch-resume
recipe are removed. Shared owns status, resume, guarded fast-forward reconciliation
and explicit retirement. No retained batch has been reconciled or retired here.
See the [publishing guide](../publishing.md) and [Shared contract](../frozen-releases.md).

Focused Make routing passes for fresh/captured releases, all four batch commands,
setup prepare/check/failure, missing batch selection and publication reconciliation,
with tool/runner effects substituted. Actual offline parser admission and read-only
status of the completed batch below are checked separately. The status command
leaves its saved intent unchanged. Snapshot/source comparison, Perl syntax,
ShellCheck and documentation links qualify this adoption; the earlier complete
release gate below does not establish a new full gate or native macOS pass for .9.
Logs are under `target/shared-tooling-0.30.9-*.log`.
The actual hook fixture now passes uninterrupted at
`target/tmp/frozen-release.KSvZO0/`: fresh and recovered setup, two publication
dry runs, deferred lock preservation, archive identities, uncertain/completed
resume and stale-remote refusal. Product stage bodies were substituted there;
the authorized production Testkit 0.89.3 gate and upload remain separate.

## Published baseline and frozen delivery

| Group | Published version |
| --- | --- |
| Foundation | 0.74.4 |
| Build | 0.75.0 |
| Text | 0.74.2 |
| Host Tools | 0.83.0 |
| Native | 0.80.0 |
| Testkit | 0.89.2 |

Testkit's clean release commit is `33c44602ec27c996ab67eb92458bf5022e630760`,
with pushed tag `testkit-v0.89.2`. Private Dev/probe follow its version and remain
unpublished. Source integration was `9eba165d`; frozen activation was `2ff0b115`.
The normal checkout includes the release and subsequent handoff commit `7f88408f`.

Normal increment commands capture committed source, qualify, commit/tag/push and
publish. Do not follow successful default delivery with another `make publish`.
Select affected supplier-first groups once; recover by batch ID. Git-only delivery
requires explicitly clearing `RELEASE_FROZEN_PUBLISH_TARGET`.

Completed production batch `batch-c3bPF1yB` executed under
`target/release-frozen/batch-c3bPF1yB/`; durable Git, intent and plans remain under
`.git/release-state/frozen/batch-c3bPF1yB/`. Its complete 34-stage Linux gate passed
on Rust 1.99.1. Gate logs are in the captured checkout's
`target/release-validation/testkit-gate.hjklTi/logs/validation.CYPGTe/`;
publication receipts are in `target/publish/publish.53uLfL/` there.

Registry readback matches the uploaded archive and exact clean Git provenance.
The checksum is `65358cb9f4473db0f059e252d57d0d946b9d33c29c79b01fba34597024ab8e02`.
All three registry-only public-contract cases passed; captured consumer evidence
is `target/published-integration.LLQca4/`. Native host qualification was dispatched
[asynchronously at that release](https://github.com/dragginzgame/ic-tooling/actions/runs/42233444909).
Do not infer native macOS qualification from the Linux result.

## Deferred work and retained recovery

Unrelated normal-checkout lock updates were preserved while the completed batch
used its fixed committed graph. The latest pre-reconciliation backup is identified
by `.git/release-state/deployment-testkit-0.89.2/deferred-latest`; exact comparison
changed only the three owned Testkit versions to 0.89.2. Later live lock updates
remain deferred work too; never restore an older backup over them implicitly.
That directory also retains readback/CI receipts, earlier lock snapshots, the
restored activation stash identity and the failed ordinary-tree release log.

Shared 0.30.8 fixed generated underscore IDs with 19 producer cases. The recovered
consumer hook proof remains at `target/tmp/frozen-release.MLbx9d/qualification.json`:
actual metadata/Git/publication dry runs, substituted product stages, two archive
identities, deferred lock custody, explicit uncertain-publication reconciliation,
no replay on completed resume and stale-remote refusal. Earlier failed fixtures
retain their original diagnostics. No artifacts or source pins were cleaned.

Shared reconciliation requires a clean normal checkout and only fast-forwards;
preserve deferred work before selecting it and deliberately reapply afterward.
Retirement is separately authorized deletion of one completed batch, including
its build/tool outputs and recovery state; preserve still-required evidence first.
Legacy plans/receipts keep their original adapter/source identities. Never rename
or reinterpret them as current qualification.

## Maintained contracts and evidence owners

Testkit remains async-only with caller-owned runtimes and explicit teardown.
`CachedPocketIcBaselinePool::shutdown(&self)` closes admission, waits for leases
and returns all attempted deletion failures without replay. Unattempted baselines
survive cancellation; in-flight deletions require caller reconciliation. Its
focused Linux pool/installed proof is `target/evidence/pool-shutdown.lGjQv6/`.
See [client migration](../testkit/client-migration.md) for consumer obligations.

The II attachment record and fixture-owned cleanup contract belong to the
[consumer guide](../../testing/browser/internet-identity/README.md). Earlier browser
acceptance evidence remains under `target/evidence/ii-testkit-0.89.1-001/`.
Tools/Build validation narrowing still requires the matched native-host comparison
in the [qualification contract](../design/release-validation.md); uncertain/shared
changes retain the complete gate. GitHub issues remain the sole follow-up tracker.
Historical release, adoption and recovery narratives are preserved in the
[previous committed handoff](https://github.com/dragginzgame/ic-tooling/blob/7f88408f/docs/status/current.md).
