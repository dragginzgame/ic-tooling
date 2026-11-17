# Archive compilation reuse

The Build and library archive runners retain fresh packages, extracted candidates,
logs, installed CLI roots and completion evidence on every attempt. They reuse
only disposable Cargo compilation output, under
`target/archive-qualification/build` and `target/archive-qualification/library`.
This removes the repeated dependency compilation caused by putting Cargo output
inside each fresh evidence directory. Archive qualification remains script-owned,
outside the Build product API.

Both runners call `scripts/qualification/archive-compilation.sh` after candidate
assembly. It sets `CARGO_TARGET_DIR` and `CARGO_BUILD_BUILD_DIR`, records the selected
directory in `compilation-target.txt`, and refreshes candidate file timestamps
without changing bytes. Cargo owns artifact selection and locking. All existing
lint, graph, test, documentation and selected consumer/runtime gates still run;
cached artifacts never substitute for their fresh completion evidence.

Timestamp refresh is required: a prototype sharing only the target directory
incorrectly reused a passing unit-test executable after an implementation change
in a fresh candidate whose files had older timestamps. The runners therefore
recheck candidate source on each attempt while retaining reusable dependencies.
Do not remove that refresh based on an unchanged-source timing improvement.

Set `ARCHIVE_QUALIFICATION_CACHE=false` for a fresh compilation directory under
the attempt's evidence root. The default is `true`; other values are rejected.
The two shared directories are disposable and are covered by root `cargo clean`.
No new CI cache upload/download or automatic cache eviction is configured.
Prepared Cargo downloads and simulator assets are separate from this cache.
See Cargo's [build cache contract](https://doc.rust-lang.org/cargo/reference/build-cache.html).

## Invalidation acceptance — 2026-11-17

Run the regression with completed shared-cache Build and Testkit evidence:

```sh
bash scripts/qualification/test-archive-cache.sh \
  target/build-release.EXAMPLE target/library-release.EXAMPLE
```

The regression copies each candidate to a fresh directory, changes one existing
implementation initializer, backdates all copied files, then invokes the actual
cache preparation helper. An existing exact unit test must fail with Cargo exit
101, and Cargo's JSON output must report that test target as rebuilt. A fresh
unmodified copy must pass again. Test assertions, qualified archives, original
candidate evidence and checkout source are unchanged.

Both rejection/recovery pairs pass in
`target/evidence/archive-cache/invalidation.qj1K91PM`. The unsafe prototype's
false-fresh result is retained in `invalidation.lT4hwBbF` under the same parent.
ShellCheck and Bash syntax checks cover the helper, runners and regression.

## Local timing — 2026-11-17

Measurements use Linux x86_64 and Rust/Cargo 1.100.0. Each pair runs sequentially
with the same source, lock and prepared tools: a fresh compilation directory via
`ARCHIVE_QUALIFICATION_CACHE=false`, then the populated shared cache. Cargo's
download cache and OS caches are already prepared; these are compiler-output
cold runs, not clean-machine provisioning measurements. Each pair's package
archive hashes match, and the checkout lock remains unchanged.

Build selects `BUILD_RELEASE_CONSUMER=none`, covering the four packaged crates'
native unit/terminal, Clippy and Rustdoc gates without claiming new Canic
qualification. The library selection is `testkit`, including feature isolation,
CLI installation/setup/lifetime and installed simulator checks. Timings include
the entire selected runner, not just compilation. This is one local pair per
runner, not a general deployment-speed or macOS performance claim.

| Selected runner | Cold wall time | Warm wall time | Cold / warm evidence under `target/` |
| --- | ---: | ---: | --- |
| Build native archives | 5.04 s | 2.36 s | `build-release.0Zi9huwZ` / `build-release.Fw4QQQr5` |
| Testkit library archive | 137.14 s | 83.87 s | `library-release.4kAqc1fj` / `library-release.uTwPuGUs` |

Both pairs complete successfully. The library source checkpoints also match.
`target/evidence/archive-cache/{build,library}-fixed-{cold,warm}.{time,log}`
retains `/usr/bin/time -p` results and runner output. Earlier timing files without
`fixed` measured the unsafe prototype and do not qualify the current implementation.
