# Current handoff — 2026-10-31

## Source and authority

HEAD/main remains `1a78892fe04edd43e07cb3d1709b36088a1a7e49`.
Foundation 0.73.1 and integration 0.74.0 were released and published through the
standard commands in the preceding deployment. Their tags are
`foundation-v0.73.1` at `3b095515ed2b2020d700b5bf0e90c0c4459f5454` and
`integration-v0.74.0` at HEAD. Seven Foundation and three integration packages
are public; the two private integration members were never uploaded.

The supplier-adoption task requested published Metrics, Platform and Wire.
Admission had a dirty Cargo.lock already selecting Metrics 0.23.2,
Platform/macros/guard 0.15.2 and Types 0.15.1. Those records are preserved.
The scoped update adds Wire/Derive 0.13.0; every other lock record matches
admission. Changes remain local and unstaged. No own package versions, refs,
tags, release plans or registry publications changed in this task.

The user has now authorized deployment of the complete pending integration batch
as 0.75.0. The standard group release and publication commands own its preparation,
qualification and delivery; unaffected Foundation stays at 0.73.1. The source
preparation recorded here precedes those release effects. Deployment admission
and subsequent results are retained at `/tmp/ic-tooling-deploy075.qqw2d8/`.

Shared Tooling remains 0.27.19 at
`54fc49e12f439dbe18d232f622a6d78e61773353`. There is one workspace/catalog/lock,
explicit package versions and two [release groups](../design/release-groups.md).
Foundation remains 0.73.1; integration remains 0.74.0. Edition 2024, MSRV 1.91,
public crate names, API ownership and supported features are preserved.

## Supplier selection and compatibility

The catalog now requires Metrics 0.23.2, Platform 0.15.2 and Wire 0.13.0.
Canonical Types resolves once at 0.15.1. Registry source provenance identifies:

- Metrics: `06020376352355deb068f5d8978813ef5405cedd`.
- Platform/macros/guard: `3587cd2633b5c94fd7ebf9dea3eefdb0433ea59d`.
- Types: `668e44f70455b3118859f81ec1b7aa23de7db63a`.
- Wire/Derive: `65d1b91600d64a1e4255774f6bfc8ad9752a63cb`.

The [integration ledger](../../releases/integration/CHANGELOG.md) proposes 0.75.0:
Testkit exposes Wire limits/errors, so direct typed callers must select the 0.13
identity. Frame bytes, budgets, transport errors and retry policy stay unchanged.
Metrics and Platform patches preserve their existing runtime contracts.

Wire's optional identity schemas now select the same Types generation as the
Platform facade. The isolated consumer explicitly enables `platform-types` and
verifies Principal/Account metadata alongside core frames. Production profiles
keep their original features: arithmetic-only Metrics, no automatic schema or
Endpoint activation, and the existing Candid-free Testkit Wasm boundary.

## Focused qualification

Evidence is retained at `/tmp/ic-tooling-supplier-refresh.nSPNBd/`.
Actual full locked/offline root metadata, inherited declarations, Testkit and
Tools feature graphs pass. Strict all-target/all-feature Clippy passes for the
five affected packages: Tools, Testkit, Native, Dev and the private probe.

Rust 1.91 passes 23 benchmark tests, nine Wire unit tests, three registry tests,
four Native memory tests and two live PocketIC call tests. Testkit/Native Wasm
profiles and the runtime probe compile on 1.91. Live tests use the existing
PocketIC 16.1.0 installation with localhost permission; no server tool changed.

`/tmp/integration-cohort.EoP3tI/` retains a disposable 0.75.0 integration candidate
against published Foundation 0.73.1. Both source and normalized-archive consumers
pass two real tests each on Rust 1.91, including canonical facade/registry types,
Wire limits, Principal/Account schemas and frame round trips, and Native memory.
The fixture changes neither actual versions nor the registry.

No full workspace/release gate or native macOS execution was run for this local
supplier refresh. Earlier release evidence does not qualify the changed graph.
GitHub [#276](https://github.com/dragginzgame/ic-tooling/issues/276) owns release-group
native acceptance; GitHub remains the sole follow-up tracker.

## Retained evidence and recovery

The preceding deployment's complete Linux CI/MSRV gates, publication dry runs,
uploads, remote tag observations and independent registry confirmations remain
at `/tmp/ic-tooling-deploy019.6xOjTH/`. Its `delivery-result.json` records the
delivered versions and source identities. PRs #278–281 were normal merges.
Prior handoff text is retained in this refresh's `status.before.md` and Git history.

Preserve prior supplier, group and cohort evidence under
`/tmp/ic-tooling-platform015.v8GsIj/`, `/tmp/ic-tooling-shared018.eFTK4d/`,
`/tmp/ic-tooling-276-activation.Gjb80U/` and `/tmp/ic-tooling-268-274-276.m4ejqy/`.
Preserve failed package attempts, including `target/package-qualification-2233002-0/`
and `target/package-qualification-1987543-0/`, the deployment's fixture failures,
all successful qualification targets, migration/retirement archives and original
archived refs. No artifact cleanup or sibling checkout mutation was performed.

Legacy ungrouped plans, locks, receipts, notes and partial-publication evidence
retain their original identities. Never reinterpret them under the new groups;
recover against their recorded source/adapter in isolation with explicit authority.
