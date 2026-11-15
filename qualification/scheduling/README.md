# Scheduling owned-build qualification

`make test-scheduling-consumer` snapshots the current local `SCHEDULING_SOURCE`
(default `../ic-scheduling`) without modifying it, including local edits and
non-ignored additions, then applies
[the migration](owned-build.patch), and verifies that the Timers driver resolves
its build dependency to local `ic-build` 0.87.1, with no legacy Build anywhere in
the graph. No sibling edits are made. Sixteen Timers host tests (including three new
observation tests) and two exact installed subjects run in that export:
watchdog survival through trap/upgrade, and Wire rejection before timer mutation.
Jobs adds five capture tests and installed management/upgrade/watchdog qualification
of the original Wasm and real Binaryen `-O3`, `-Os`, `-Oz` captures.
This is focused installed evidence, not the complete watchdog/cohort release roster.

See [source selection](../../docs/qualification/local-sources.md). The current
checkout is clean at `0e5e09ed24e88d6d62a6d2dc923afbb53dc8619e`; its patch
still applies, so the prior installed evidence remains relevant to unchanged
inputs. No new installed run was performed for this source-selection change.

Prepare Rust 1.100.0, its Wasm target and the consumer's locked dependencies first.
The selected source checkout also needs its prepared Binaryen tool bundle; the
local export's Toko admission script checks it offline without installing anything.
Select an already prepared `ic-testkit-server` 0.106.0 CLI and its asset directory:

```sh
SCHEDULING_TESTKIT_CLI=/absolute/path/to/ic-testkit-server \
SCHEDULING_SERVER_DIRECTORY=/absolute/path/to/ic-testkit-server-assets \
make test-scheduling-consumer
```

The CLI checks the server bytes offline before use. PocketIC requires local
loopback access. `SCHEDULING_SOURCE` may select another current local checkout;
its snapshot must admit the migration patch. Scratch, original diagnostics, captured Wasm and runtime
receipts remain under the printed `target/scheduling-consumer.*` export; Cargo's
host cache is under `target/scheduling-consumer-build`. No implicit tool download,
registry publication or protected release artifact is involved.

The consumer now calls one owned build instead of creating/locking a shared target,
collecting every Cargo message, separately checking execution/protocol, selecting a
mutable path and capturing it afterward. The complete owned bytes are published
into the invocation's diagnostic directory for the existing installed harness.
Source/configuration drift checks, before/after tool identities, exact installed
artifact hashes, subject admission and receipt validation remain in that driver.
No original test bodies or supplier implementation are copied into new products.

The dependency uses its canonical `ic-build` name. Timers owns its private bounded
configuration admission and receipt evidence; the old generic observation/selection
types and supplier are removed. See the [observation contract](observations.md),
including the explicit recipe-identity migration and conservative comparison rules.
Jobs capture uses held filesystem staging, exact byte admission and explicit
producer cleanup. Registry Host and Testkit remain unmigrated.
There is one Timers compile/capture path, with no fallback
to the old execution API. This does not establish complete Scheduling supplier
cutover, cohort comparability, native macOS acceptance or shared-target performance.

PocketIC 16.1.0 pins `thiserror` to 2.0.18. Product requirements now admit that
compatible version; the product lock still selects 2.0.21. The consumer lock keeps
retained registry versions/checksums, adds the local suppliers plus `const-oid`,
and removes the unused old Build graph. The private parser uses the already locked
TOML library directly. Full Stage 3 still includes Auth/Storage and Canic workflows;
remaining Host/Testkit responsibilities stay explicit. Shared incremental targets
need measured justification first.

Jobs retains complete `.capture.pending` bytes when destination publication fails;
its error identifies staging versus visible-but-unconfirmed destination outcomes.
Successful capture retires only the observed producer identity and keeps the log.
Existing recovery staging refuses further capture until the caller reconciles it.
The CLI and optimizer flags are unchanged. Source/destination parents passed to
external tools remain stable caller-owned paths. The migration declares explicit
product workspace roots in the export to disambiguate Cargo inheritance with
local suppliers from another workspace; sibling manifests are untouched.
