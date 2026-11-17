# CI scaffold setup diagnostics

The `repository` job in `build-release-candidate.yml` wraps its compiler and common
tool setup with `scripts/ci/setup-diagnostics.pl`. This removes dependence on an
ephemeral runner's console or a raw log path when setup fails before tests start.
Only this job is changed; its compiler/tool commands and failure statuses remain
the same. Other repositories' concurrent fixes stay with their owners.

The job selects a temporary tool store at `$RUNNER_TEMP/ic-tooling-setup/tools`.
The same selection is exported through `GITHUB_ENV` for subsequent cache, admission
and formatting steps. Setup output is drained without writing a raw capture;
only sanitized, complete lines enter a bounded tail. Known sensitive environment
values, credential fields, environment assignments, bearer credentials, URL
credentials/parameters, private keys and long opaque strings are omitted.
Oversized lines are dropped whole, and terminal controls are removed. Printed
lines are prefixed so tool output cannot become a GitHub workflow command.
No environment dump, config, source payload, binary or cache is uploaded.

On failure, retain at most 64 KiB of command output plus at most ten sanitized
attempt logs of the same bound (300 retained lines plus omission markers per log).
Attempt selection visits at most 256
entries/four directory levels in the job's tool store, including hidden attempts;
it selects only `install.log`, `download.log` and `stderr.log`. Build/home/payload
trees are not evidence, symlinks are not followed, and each selected file has a
1 MiB read ceiling. Larger files are explicitly omitted. Filenames in the artifact
are fixed; source path hashes identify attempts without exposing arbitrary paths.
The artifact remains below 750 KiB. Missing output, selection caps and collector
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
These tests do not claim a hosted artifact upload or retrieval; that evidence
requires an actual hosted setup failure. No live failure was deliberately induced.

The local wrapper/fixture adapt the first-party DB implementation inspected during
this task, after selecting this consumer-owned boundary. Its captured script SHA-256
was `e6942466473d6e7f01f70d6e833a469aaa921ce26f155c34f656b86da01ed753`;
the fixture was `c2addefc6391ce62128d9f00e05491c539b17913c475576b9bfce6973dd2b58f`.
Tooling narrows phases/roots, adds file-open admission, bounds traversal and reads,
and qualifies its own workflow. No installed Toko files or sibling work is modified.
