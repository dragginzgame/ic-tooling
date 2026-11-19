# CI setup diagnostics

The `repository` and `secrets` jobs in `build-release-candidate.yml` wrap compiler,
common-tool and secret-scanner setup with `scripts/ci/setup-diagnostics.pl`.
This removes dependence on an
ephemeral runner's console or a raw log path when setup fails before tests start.
Toko Tooling 0.45.21 supplies the selected `SetupDiagnostics.pm` module for command
capture, exit/signal status, redaction and bounded excerpts. It includes the
original redaction corrections from
[Toko Tooling #393](https://github.com/dragginzgame/toko-tooling/issues/393),
the `ACCESS_KEY` environment selector from
[Toko Tooling #396](https://github.com/dragginzgame/toko-tooling/issues/396),
and the hard admitted-file read ceiling from
[Toko Tooling #397](https://github.com/dragginzgame/toko-tooling/issues/397).
Bare Bearer credentials and JSON payload fields are now also omitted by the
[Toko Tooling #398](https://github.com/dragginzgame/toko-tooling/issues/398) fix.
The authenticated snapshot selects the module explicitly. The local wrapper
imports it relative to its own script location and owns only this consumer's
phase/root admission, attempt selection, artifact metadata and upload-path
publication. Its duplicate sanitizer and command-capture implementation are
removed. Other repositories' concurrent fixes stay with their owners.

The repository job selects a temporary tool store at `$RUNNER_TEMP/ic-tooling-setup/tools`.
The same selection is exported through `GITHUB_ENV` for subsequent cache, admission
and formatting steps. Setup output is drained without writing a raw capture;
only sanitized, complete lines enter a bounded tail. Known sensitive environment
values, credential fields, environment assignments, bearer credentials, URL
credentials/parameters, private keys and long opaque strings are omitted.
Oversized lines are dropped whole, and terminal controls are removed. Printed
lines are prefixed so tool output cannot become a GitHub workflow command.
No environment dump, config, source payload, binary or cache is uploaded.

The secrets job uses `$RUNNER_TEMP/ic-tooling-security-setup/tools/scanner` for the
pinned scanner installation. The same collector wraps a 90-second command deadline
with ten seconds before forced kill, inside the two-minute step budget. Only
successful installation exports the deterministic executable path. A setup failure
uploads the collector's sanitized directory for seven days and reports its artifact
URL or an explicit upload failure. Scanner findings are separate from setup errors.

On failure, retain less than 32 KiB of command output plus at most ten sanitized
attempt logs of the same bound (at most 99 retained data lines plus an omission
notice per excerpt, with displayed lines limited to 500 characters).
Attempt selection visits at most 256
entries/four directory levels in the job's tool store, including hidden attempts;
it selects only `install.log`, `download.log` and `stderr.log`. Build/home/payload
trees are not evidence, symlinks are not followed, and each selected file has a
1 MiB read ceiling. Larger files are explicitly omitted. Filenames in the artifact
are fixed; source path hashes identify attempts without exposing arbitrary paths.
The artifact remains below 400 KiB. Missing output, selection caps and collector
failures are reported explicitly. Child exit and signal status remain authoritative.

Only a completed collection supplies an upload path. The failure-only artifact
step retains that fresh sanitized directory for seven days. Missing files fail
upload; the final handler reports upload failure separately or places the returned
artifact URL in the job summary. No `continue-on-error` masks setup failure.

## Focused acceptance

Run `perl scripts/ci/test-setup-diagnostics.t` and
`actionlint .github/workflows/build-release-candidate.yml`. Offline fixtures prove
early failure with a hidden attempt log, bounded output, tested secret redaction,
excluded symlink/config inputs, truncation, missing diagnostics, collection failure,
successful setup, signal status and the workflow's upload-result reporting branch.
The scanner case runs the actual workflow command and pinned installer with a
failing local curl substitute: exit 22, the underlying diagnostic, redaction and
the absence of a published executable are verified. Both jobs' upload-result
handlers are exercised. The current fixture includes unlabelled synthetic
access-key environment values, bare Bearer credentials and JSON payload fields.
Shared-helper adoption adds invocation outside the repository working directory,
command-launch refusal, the tighter per-excerpt bounds and the admitted-file read
ceiling with an unfinished final line. The helper is byte-identical to the
qualified 0.45.11 implementation. The fresh 69-check execution is retained under
`target/toko-rules-refresh-2026-11-19/setup-diagnostics.log`; migration evidence
remains under `target/toko-adoption-0.45.11/` and earlier scanner-setup evidence
under `target/security-setup-review/diagnostics.log`.
The repository job runs this focused fixture before formatting checks, so scanner
setup failure and upload-reporting regressions are part of ordinary CI.
These tests do not claim a hosted artifact upload or retrieval; that evidence
requires an actual hosted setup failure. No live failure was deliberately induced.

The original local wrapper/fixture adapted the first-party DB implementation
after selecting this consumer-owned boundary. Its captured script SHA-256
was `e6942466473d6e7f01f70d6e833a469aaa921ce26f155c34f656b86da01ed753`;
the fixture was `c2addefc6391ce62128d9f00e05491c539b17913c475576b9bfce6973dd2b58f`.
Tooling retains its phase/root and file-open admission, bounded traversal and
artifact policy, and qualifies its own workflow. Toko owns the shared capture and
excerpt implementation; the canonical exporter owns its installed bytes. No
sibling implementation is modified.

## Job deadlines — 2026-11-17

Every job and step in the workflow has an explicit timeout. Budgets use completed
hosted measurements from runs
[43968510609](https://github.com/dragginzgame/ic-tooling/actions/runs/43968510609),
[43974748932](https://github.com/dragginzgame/ic-tooling/actions/runs/43974748932) and
[44062342797](https://github.com/dragginzgame/ic-tooling/actions/runs/44062342797).
The first run passed all hosts; later failed jobs are excluded from successful
duration maxima. API job/step timestamps are retained locally under
`target/ci-timeout-review/`. This small sample informs generous failure bounds,
not a performance claim or a statistical latency guarantee.

| Job | Slowest observed successful job | Job budget | Sum of explicit step budgets |
| --- | ---: | ---: | ---: |
| Repository | 1m30s | 20m | 18m |
| Secrets | 8s | 13m | 11m |
| Libraries, all three hosts | 10m41s | 45m | 42m |
| Native, all three hosts | 2m25s before Sign checks | 30m | 26m |
| Sign provider, all three hosts | All three pass in run 44178285784 | 30m | 24m |

Library setup took up to 6m50s on Intel macOS; its step gets 15m. Library archive
qualification took up to 4m23s; its step gets 12m. CLI lifetime checks get 4m
against an observed 1m32s maximum. Native archive qualification gets 8m against
1m49s. These budgets allow uncached preparation and host variation independently
of the local archive cache. Revisit them when the selected workload changes.

Sign adds focused native-host lint/admission/cancellation checks and three-minute
dependency preparation. In [run 44178285784](https://github.com/dragginzgame/ic-tooling/actions/runs/44178285784),
Linux and ARM macOS pass; Intel Clippy takes 2m26s, then test compilation exceeds
the provisional four-minute combined step. The step now gets eight minutes in a
thirty-minute job, preserving the evidence reserve. The old bound is failed
evidence, not a successful duration measurement. The corrected Intel job passes
in [run 44180582387](https://github.com/dragginzgame/ic-tooling/actions/runs/44180582387),
as do the Linux and ARM macOS jobs. These checks do not access native keychain credentials.

The separate Sign provider matrix uses disposable ICP identities and the existing
sanitized setup collector for compiler, native tools and pinned ICP preparation.
All three real-provider jobs pass in that run, including Intel's cold compilation
inside the eight-minute provider step. Prepared runtime cases take about eleven
seconds on local Linux, three seconds on ARM macOS and twenty-four seconds on
Intel macOS; these are different hosts, not matched performance comparisons. Retention is
seven days and selects only logs, socket/prompt headers and source/tool identity;
temporary credential stores, manifests, IPC bodies and whole target trees are
excluded. This job does not wait on consumer adoption or other hosted jobs.

Step deadlines leave the existing failure/always evidence steps reachable before
the job deadline. Each upload gets 2m; setup reporting gets 1m. The sum of all
explicit step limits is below the job limit, leaving additional time for runner
and post-action overhead. The existing artifact selections and failure conditions
remain. Hard job cancellation, runner loss, or an upload failure can still prevent
retention; `always()` is not a guarantee after a job has exhausted its lifetime.
See GitHub's [step and job timeout semantics](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax#jobsjob_idtimeout-minutes).

On the Ubuntu repository job, GNU `timeout` runs *inside* the existing sanitized
collector: Rust setup gets 150s and tools 270s, each with 10s before forced kill.
The outer step limits are 3m and 5m, leaving time to collect diagnostics and publish
the evidence path. This avoids killing the collector first on an ordinary setup
hang. Timeout status 124 (or forced-kill failure) remains authoritative; there is
no retry or `continue-on-error`. The focused fixture exercises a real timed-out
child, hidden attempt retention and redaction through that same command layering.
Hosted deadline/upload behavior has not been deliberately induced.

## Reviewed CI failures — 2026-11-17

Run [44062342797](https://github.com/dragginzgame/ic-tooling/actions/runs/44062342797)
failed two independent gates. Gitleaks 8.30.1 matched synthetic private-key markers
in `scripts/ci/test-setup-diagnostics.t`, not a credential. `.gitleaksignore` admits
only that original commit/path/rule/line fingerprint. The fixture now assembles
its markers while retaining identical redaction inputs. Full-history and current
fixture scans pass; a separate synthetic finding remains detectable. No scanner
rule or source path is excluded. All 44 diagnostic checks still pass.

The Linux library archive metadata contained both the extracted `ic-host-text`
workspace member and Testkit's registry dependency of the same name. Admission
incorrectly required a globally unique package name. It now selects by workspace
membership and name, then still requires the exact extracted manifest path,
non-registry source, publication policy and description. Dependencies continue to
resolve from the registry. The captured failure metadata passes; registry-only,
missing-member and wrong-path substitutions fail.

The repaired four-group Linux gate (`http text testkit cargo-metadata`) passes in
`target/library-release.LXtTNiqB`, including CLI and installed simulator checks.
That local gate used the preserved working lock (Protocol 0.3.5 and Clap/Nix
updates); those concurrent dependency edits are excluded from this tooling-only
delivery. The admission regression also uses the original hosted metadata.
Focused logs and redacted scanner reports remain under `target/ci-timeout-review/`.
Hosted CI after the fix runs asynchronously; local evidence is not a hosted pass.
