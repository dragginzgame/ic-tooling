# Current handoff

## Source and ownership

This checkout is based on `52457b72b18e4943134a731e402bc692faaef69c`, release
0.68.5. The next local improvements are prepared for the compatible
[0.68.6 draft](../changelog/0.68.md#0686---unreleased). Own package versions remain
0.68.5; Git refs, the index, dependency requirements and Cargo.lock are unchanged.
Platform/macros 0.6.16 and Wire/derive 0.7.12 remain selected.

Host, Build and Testkit retain the [extraction boundaries](../extraction.md).
Consumers own credentials, budgets, targets, retries, installation and recovery.
The reviewed Shared Tooling snapshot is 0.26.14 at
`e14050bcc0c627f9c8a9064051699cac44fd7ab3`, recorded in
[the snapshot manifest](../../.shared-tooling.snapshot). Rust edition 2024,
MSRV 1.91 and Linux/native macOS 15 Intel and Apple Silicon remain required.

## Current local improvements

Native executable admission now requires one successful Cargo build-completion
record. Invalid completion remains subordinate to an actual failed Cargo exit,
retaining the original execution cause. Synthetic Cargo launchers must retain
that terminal record; ordinary Cargo already emits it.
Host library diagnostic selection also retries interrupted reads and refuses
impossible reader counts without a panic.
Native cancellation qualification requires complete readiness bytes, covering
empty/partial markers in both phases
([#251 — Native cancellation readiness on macOS Intel](https://github.com/dragginzgame/ic-tooling/issues/251)).

Checked marker formatting/Wasm emission and pairing by prefix/stream are additive.
Raw emission and suite/label pairing keep their public behavior. The report example
and private Wasm probe select the checked path. Origin fields do not authenticate
producers; consumers still separate independent sources with coinciding fields.

The driver requests cooperative stop on interruption or failed sampling. Workers
install signal handling before owning a server, finish active work, stop queuing
and drop fixtures/server ownership. The driver waits for cleanup and preserves
its primary error and prior report. There is no forced-kill fallback or new
runtime deadline; an active PocketIC operation can delay shutdown.

CI uses one reviewed checkout revision without credential persistence. MSRV
compilation has a separate optional cache; main pushes alone save it. Existing
job gates remain. Successful native/MSRV jobs retain lightweight clean-source,
lockfile, compiler/Cargo, host and profile records for 14 days. Failed observers
or source/compiler mismatches cannot create a completion marker. The final source
check includes untracked files created during observation. Hosted upload
behavior and cache effectiveness still require actual CI execution.

## Qualification and evidence

Evidence is under `/tmp/ic-tooling-improvements-20261026/`, including the starting
tracked-source archive and unchanged lockfile. Native Cargo, marker and private
driver tests pass. Prepared PocketIC 16.1.0 runs the checked Wasm marker probe,
including log parsing, Wire calls and traps. Both fresh and pooled long-running
workloads stop on Ctrl-C, return 130, reap worker/server and preserve a prior
report. The ordinary benchmark retains its 16-sample report/timing contracts.
Follow-up evidence is under `/tmp/ic-tooling-0686-followup-20261026/` (diagnostic
reads/source status) and `/tmp/ic-tooling-251-20261026/` (cancellation readiness).
Both preserve their incoming dirty source. Focused tests pass on development Rust
and 1.91; changed-owner Clippy, formatting and doc links pass.

Changed-owner Clippy, actual Rust 1.91 checks and deployed Testkit Wasm profiles
pass on both compilers. Testkit graph guards and base Build/Testkit profiles pass
for Linux and both macOS target selections. Qualification-record fixtures cover
Linux/Darwin observations and partial/dirty/wrong-floor refusals. Available
Actionlint 1.7.12 and ShellCheck 0.11.0 match the adopted catalog; no setup ran.
Final Make routing, fixture retention (including relative temporary directories),
formatting, workflow lint, shell lint, dependency pins and local doc links pass.
Native macOS 15 Intel and Apple Silicon execution remains outstanding; named-target
graph checks and substituted host observations do not qualify native hosts.
No full CI/release gate, release, publication or new tool download was run.

Earlier cleanup evidence remains at `/tmp/ic-tooling-complexity-5-20261025/`,
`/tmp/ic-tooling-complexity-4-20261025/`, `/tmp/ic-tooling-complexity-3-20261025/`,
`/tmp/ic-tooling-complexity-2-20261025/` and `/tmp/ic-tooling-complexity-20261025/`
for its own source identities. Dependency-adoption evidence remains at
`/tmp/ic-tooling-platform-wire-20261025/` and `/tmp/ic-tooling-adopt-20261024/`.

## Dependency blocker and retained history

[Crypto qualification #240](https://github.com/dragginzgame/ic-tooling/issues/240)
remains separate, with the supplying correction at
[Platform #50](https://github.com/dragginzgame/ic-platform/issues/50). The selected
graph still contains crypto-bigint 0.5.5. Actual ECDSA signing prevents an
unused-code exemption; no local crypto fork, override or exemption was added.
Platform 0.6.16 and Wire adoption do not resolve that dependency risk.

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
remains at `/tmp/ic-tooling-stale-release-lock.qHQFTn/lock`. Retained plans,
failed attempts, original artifacts and backups are not disposable.
