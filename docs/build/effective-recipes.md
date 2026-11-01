# Effective Wasm recipes

Build's opt-in `cargo::rustc_recipe::RustcRecipe` carries caller-qualified target
cfg requirements through `WasmBuildSpec::with_rustc_recipe`. Build owns composition
and evidence mechanics. The consumer selects its dependency generation, profile,
flags, tools, budgets and qualification. Platform owns its ECDSA recommendation
and vectors; generic Build acquires no Platform or cryptographic dependency.

## Cargo precedence and the compiler boundary

Cargo resolves flags in this priority order: `CARGO_ENCODED_RUSTFLAGS`,
`RUSTFLAGS`, matching target rustflags, then build rustflags. These sources are
exclusive; an explicitly empty encoded value still takes precedence. See the
[Cargo reference](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags).
Appending to an environment variable alone would shadow lower-priority config.

The explicitly selected `ic-build-rustc-wrapper` receives Cargo's resulting argv.
It preserves those arguments and adds missing required cfgs only for the selected
Wasm target, including dependencies. Native build scripts/dependencies and Cargo
capability probes keep their arguments. Existing config or environment settings
therefore cannot silently drop a requirement. Conflicting effective cfg values
fail. A caller can explicitly select `CfgConflict::Replace` with a nonempty
qualification reference; only that cfg key changes, with the override recorded.
There is no ambient environment mutation or automatic flag search.

Existing compiler wrappers, including wrappers in observed Cargo config, are
refused rather than silently replaced or chained. Non-Unicode compiler arguments
are refused when recording evidence. Unrelated recipes without this opt-in retain
their existing behavior. `RustcRecipe` does not infer that a crypto dependency is
present or decide which backend is appropriate.

Opted-in recipes also refuse a configured shared intermediate `build-dir` and
config `[env]` entries that can override compiler, target-directory or reserved
recipe custody. The same bounded config observation performs these checks;
there is no second Cargo config parser. Ordinary flag/profile environment
entries remain Cargo-owned and their resulting compiler arguments are recorded.

## Select and retain one recipe

Prepare/admit the wrapper from the same Build source/version under the consumer's
tool policy. The new executable is available through Build's existing `cli`
feature; this source change does not install it or change any consumer tool pin.
Until the next Foundation publication, use a source-qualified local build for
experiments, not a pin to an unpublished version.

```rust,ignore
use ic_build::cargo::rustc_recipe::{RustcRecipe, RustcRecipeLimits};

// The consumer has admitted Platform 0.16's ECDSA graph and its qualification.
let recipe = RustcRecipe::new(
    &admitted_wrapper_path,
    "wasm32-unknown-unknown",
    [("cpubits".into(), "32".into())].into(),
    "consumer qualification record for Platform 0.16 / k256 0.14",
    RustcRecipeLimits {
        record_bytes: 256 * 1024,
        records: 256,
        total_bytes: 16 * 1024 * 1024,
    },
)?;
let spec = spec.with_isolated_builds().with_rustc_recipe(recipe);
// Later with_environment calls still preserve this selected requirement.
let evidence = WasmBuildEvidence::acquire(spec, selected_spawner)?;
let records = evidence.record().rustc_recipe_records(256, 16 * 1024 * 1024)?;
```

Qualified compiler wrappers currently require isolated Cargo builds. Changing
private wrapper policy does not necessarily make a shared Cargo target recompile
its objects. Refuse that ambiguity instead of claiming effective flags for a
fresh Cargo result. This still permits exact Build cache reuse: policy, complete
child environment, target/profile, config/source observations, Cargo/rustc
identities and the wrapper's watched bytes participate in the existing key.
Keep the wrapper and source under caller custody throughout acquisition.

The retained `.ic-build-rustc-recipes` directory belongs to the exact build entry.
Each invocation records actual compiler path, bounded `rustc -vV`, ordered argv,
and selected requirement/override qualification. The Cargo facts retain profile,
selection and Cargo identity. Missing compiler settings in argv mean that
compiler's default; no guessed LTO/codegen defaults are synthesized. This includes
per-package differences and the effective `-C` flags after profile overrides.
Successful builds and warm hits admit the original bounded record set. A warm
hit does not claim new compiler execution. Keep the build/evidence owner alive
while exporting or measuring the bytes. Arguments may contain private paths or
values; consumers own access control, redaction and evidence publication.

## Keep post-link optimization separate

Compiler optimization and post-link transformation are separate recipes. Reuse
`ArtifactCacheSpec`, `AdmittedTool` and the retained Wasm input. Key the admitted
optimizer's bytes/version, exact ordered argument template, complete execution
environment and working directory, pipeline source, compiler recipe directory
and Cargo input snapshot. Bind template input/output placeholders to the retained
input and transaction output, and execute the same ordered flags. Keep both
records alive while consuming results. There is no optimizer dependency or
application optimization profile in Build.

The [qualification driver](../../scripts/fixtures/wasm-recipe/main.rs) demonstrates
this using `wasm-opt` 132 with `-Oz`, debug stripping and explicit admitted Wasm
feature switches. Those are fixture choices, not deployment defaults. The
existing post-link cache tests cover ordered-argument and tool-byte invalidation.

## Downstream adoption

Auth currently selects `--cfg=cpubits="32"` in its target Cargo config. Preserve
that selection and all other flags. Its Wasm build owner can opt into this recipe
to guard environment overrides and retain effective evidence. Identical required
cfgs are not duplicated. Its native graph receives no new cfg.

Canic's actual owners include `canic_host::canister_build` and the
`canic-testing-internal` root/fleet/artifact recipes. Admit each deployed ECDSA
normal/build graph and its Platform/K256 generation before adding this policy to
those Wasm specifications. Keep the caller's profile and environment; select
isolated compiler state for these recipes while retaining exact artifact reuse.
Do not apply the backend globally to unrelated target graphs. If a build bypasses
`WasmBuildSpec`, first converge its command through the existing Build boundary;
adding another environment-only flags helper would recreate the gap.

Other consumers follow the same steps: inventory actual ECDSA deployment graphs,
admit the matching Platform recommendation, supply the qualified wrapper/policy,
then measure their final Wasm and installed behavior. This task does not change
Canic/Auth active checkouts, tool pins or deployment profiles. Their persistent
adoption can follow a published Foundation version using isolated consumer PRs.

## Reproduce qualification

The real Cargo fixture covers no flags, build/target config, ordinary/encoded
precedence (including empty encoded flags), conflicting selections and explicit
overrides, native isolation, dependency coverage, profile settings and cache reuse:

```sh
cargo test -p ic-build --all-features --test rustc_recipe --locked --offline
cargo +1.91.0 test -p ic-build --all-features --test rustc_recipe --locked --offline
```

The manual installed qualification consumes explicitly prepared tools and a
caller-selected Platform checkout **as a vector source only**. The probe and
native runner select published Platform 0.16.0; the probe admits K256 0.14.0 and
crypto-bigint 0.7.10. No sibling source is changed and no tool is installed:

```sh
bash scripts/ci/qualify-wasm-backend.sh \
  /checkout/ic-platform /new/evidence \
  /admitted/ic-build-rustc-wrapper /admitted/wasm-opt /admitted/pocket-ic
```

The fixture compares explicit 64- and 32-bit cfgs under the same Rust 1.91,
release `opt-level=z`, fat LTO, one codegen unit and ordered optimizer arguments.
It reuses Platform's maintained Wasm probe and native vector body, checks
valid/wrong/invalid/high-S/short/long signature cases in stock PocketIC, and retains
three IC instruction samples per case/backend. `recipes.json` reports total
uncompressed module bytes and code-section payload bytes before/after post-link
optimization. It makes no compressed-footprint or whole-application cost claim.
`compiler-*.json`, source/tool hashes, locks, final Wasm and result checksums retain
the effective recipe. Optional `WASM_RECIPE_HOST_TARGET_DIR` selects a caller-owned
native compilation cache; the evidence directory must still be new.

Local Linux, MSRV compilation, installed PocketIC and native macOS evidence remain
separate. Platform's earlier historical savings are not substituted for this run.

### Linux qualification on 2026-11-01

The complete source-qualified run is retained at
`/tmp/ic-tooling-300-qualification-final/`. It passed the installed eight-case
Platform vector body with three matching-result samples per case/backend in
stock PocketIC 16.1.0. Rust 1.91, Binaryen 132 and the exact ordered arguments
above produced these observations:

| Explicit cfg | Compiler module / code bytes | Final module / code bytes | Valid verification instructions |
| --- | ---: | ---: | ---: |
| `cpubits="64"` | 116,885 / 112,054 | 92,028 / 87,587 | 8,077,975 |
| `cpubits="32"` | 62,338 / 57,235 | 44,064 / 39,348 | 4,305,568 |

Module sizes are uncompressed; code bytes are code-section payload bytes.
These are probe observations, not Canic/Auth deployment measurements. Native
macOS qualification of these changes remains separate and has not run locally.
