# Current handoff

## Source and ownership

This checkout starts at `ae80ee173250848087b8da3322b68a1ed6e2feba`: version
0.68.7 plus the merged agent-flow clarification in PR #253. Shared Tooling
adoption is prepared locally in the [0.68.8 draft](../changelog/0.68.md#0688---unreleased).
Own package versions remain 0.68.7; the index, Git refs and Cargo.lock are unchanged.
Platform 0.6.18 and Wire/derive 0.7.16 remain selected.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
Shared Tooling is now 0.26.20 at `a7f85d45d4212d4c39225f58597580560a333550`,
recorded in [the snapshot](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and native Linux/macOS 15 Intel and Apple Silicon remain required.

## Current local changes

The canonical exporter refreshed the existing entrypoint selection and added
its required engineering-details and RPC-budget companions. The compact core
now owns task-specific reading, batched focused checks, evidence reuse and
optional accounting; the temporary local flow exception is retired. Existing
executable-tool pins and workflow selections remain unchanged.

Normal release commands start fresh from current committed metadata. Recovery
of a specific saved attempt uses `make release-resume VERSION=X.Y.Z`. A fresh
attempt replaces that candidate's stale plan, staging roster and PR receipt;
other historical attempts and build/validation evidence remain retained.
Current dirty source, locks, candidate tags and destination conflicts still refuse.
No release command or state cleanup was executed during this adoption.

Retention helpers diagnose unsupported initialization, EXIT-trap and assertion
layouts without leaving executable partial probes. The consumer release fixture
now distinguishes a current unnumbered candidate from an archived Draft label:
it retains the former refusal and verifies the latter's exact preservation.

## Qualification and evidence

Adoption evidence is under `/tmp/ic-tooling-shared-02620.1xOqGH/`, including the
starting tree, isolated committed producer, impact preview and exporter results.
The unchanged tool catalogs need no installation. Changed shell sources pass
existing ShellCheck 0.11.0. Consumer retention qualification passes, including
Make routing and release/publication fixtures with substituted external effects,
failure retention and relative-path execution. Snapshot and documentation-link
checks pass. An outdated archived-Draft refusal was corrected; its original
failed fixture remains at `/tmp/host-fixture-retention.8AIbwe/`.
Native macOS execution of this adoption remains outstanding.
No full workspace/release gate, commit, push, release or publication was run.

The earlier cleanup and Platform/Wire adoption evidence remains under
`/tmp/ic-tooling-cleanup-0687-20261026/` and `/tmp/ic-tooling-adopt-20261026/`.
Those observations retain their own source identities and host limits; they are
not new native-host qualification of this tooling update. The previous detailed
handoff remains [in Git history](https://github.com/dragginzgame/ic-tooling/blob/ae80ee173250848087b8da3322b68a1ed6e2feba/docs/status/current.md).

## Dependency blocker and retained history

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
remains separate, with the supplying correction at
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50). The selected
graph still contains crypto-bigint 0.5.5. Actual ECDSA signing prevents an
unused-code exemption; no local crypto fork, override or exemption was added.

Earlier .6 evidence remains under `/tmp/ic-tooling-improvements-20261026/`,
`/tmp/ic-tooling-0686-followup-20261026/` and `/tmp/ic-tooling-251-20261026/`.
Earlier cleanup evidence remains under `/tmp/ic-tooling-complexity-5-20261025/`,
`/tmp/ic-tooling-complexity-4-20261025/`, `/tmp/ic-tooling-complexity-3-20261025/`,
`/tmp/ic-tooling-complexity-2-20261025/` and `/tmp/ic-tooling-complexity-20261025/`.
Dependency-adoption evidence remains at `/tmp/ic-tooling-platform-wire-20261025/`
and `/tmp/ic-tooling-adopt-20261024/` for its own source identities.

Keep `/tmp/ic-tooling-249-250/`, `/tmp/ic-tooling-wire-07/` and
`/tmp/ic-tooling-wire-071/` as earlier graph/codec evidence. Original audits remain
under `/tmp/ic-tooling-audit-fixes/`, `/tmp/ic-tooling-audit-2/`,
`/tmp/ic-tooling-audit-2-fixes/` and `/tmp/ic-tooling-shared-02610/`.
Preserve the browser reproduction at `/tmp/ic-tooling-audit-browser-RwZzM9/`.

The nested-workspace target fix is delivered. Preserve its original failed
`target/package-qualification-3180841-0/`, `/tmp/ic-tooling-239/` and
`/tmp/ic-tooling-package-phase/`. Earlier detailed qualification is retained in
[the released handoff](https://github.com/dragginzgame/ic-tooling/blob/248df96b27b44366095d9d8e195cca8a1023bde1/docs/status/current.md).

Migration and local-repository retirement archives remain under
`/tmp/ic-tooling-migration/`, including `retirement/`; original refs were
preserved in the former Host repository. Stale release-lock recovery evidence
remains at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`. Retained recovery
plans, failed attempts, original artifacts and backups are not disposable.
