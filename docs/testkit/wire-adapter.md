# Wire adapter ownership and migration

Testkit 0.104 removes its `wire` feature and `pic` typed-call exports.
`ic-testkit-wire` 0.1 owns those calls with Wire 0.22. The Rust API moves to
`ic_testkit_wire::{WireCallSpec, WireCallContext, WireCallError, query_wire,
update_wire}` without compatibility re-exports. Lifecycle, server admission,
custody and raw calls remain in `ic-testkit` and its selected upstream
`ic_testkit::pocket_ic` client. The adapter uses that client's exact types.

Remove `wire` from the Testkit dependency's feature list and add an independently
selected `ic-testkit-wire` dependency. Move typed-call imports and qualified
paths to `ic_testkit_wire`. Keep direct `ic-wire` requirements aligned with the
adapter when passing limits or matching typed errors. Endpoint contracts remain
consumer-owned; pass their method identity and all four limits without projection
through a codec-independent substitute. No stored data or frame format changes.

Wire remains the only framing/codec authority. The adapter encodes before
transport, executes exactly one query/update, then admits the reply with its
explicit limits. Original Wire errors, upstream rejection values, caller,
canister and method identity survive. Transport panics propagate. A refused
reply can follow an executed update; callers still own reconciliation and replay.
PocketIC allocates reply bytes before codec admission, so decode limits are not
a network pre-allocation or peak-heap guarantee.

## Independent publication boundary

The `testkit-wire` group versions only `ic-testkit-wire`. Testkit's normal/build
graph excludes Wire with every feature selected. A future Wire compatibility
change updates the adapter's Wire selection and typed-call tests; it does not
require a Testkit release when the selected client's transport API is unchanged.
Each consumer chooses its matching adapter/Wire pair. Do not impose one Wire
version on the whole lockfile or exchange `Limits`/errors across generations.
New incompatible adapter lines use normal Cargo version identities, not aliases,
limit conversions, old-codec bridges or another runtime/client.

For the first publication, release Testkit 0.104 before adapter 0.1 so its selected
minimum and qualification lock can be projected through the existing group
editor. The adapter starts at an unpublished 0.0.0 source version. Afterwards a
Wire-only compatibility change selects `RELEASE_GROUP=testkit-wire`; leave
Testkit's version alone. The adapter currently selects the complete release gate.

Focused development qualification:

```sh
bash scripts/ci/check-testkit-graphs.sh
cargo test -p ic-testkit-wire --locked --offline --lib
# Admit the prepared server, then run the installed adapter boundary.
server="$(target/debug/ic-testkit-server check)"
POCKET_IC_BIN="$server" cargo test -p ic-testkit-wire --locked --offline --test pocket_ic_calls
POCKET_IC_BIN="$server" cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --locked --offline --test canister_wire
```

Unit tests retain pre-transport rejection, all directional limits, exact framing,
typed causes, panic identity and one-shot execution. Installed tests retain
caller identity, update effects after refused decoding, raw malformed frames and
live rejections. Graph checks separately admit lifecycle, adapter and Wasm roots.
They do not stand in for native macOS qualification or consumer acceptance.

## Auth and Blob adoption

[#357](https://github.com/dragginzgame/ic-tooling/issues/357) authorizes local
implementation, not sibling edits or releases. The inspected consumers need:

- Auth: replace the `wire` feature in
  `qualification/ic-auth-qualification/Cargo.toml`; move the typed imports,
  `spec!` expansion and qualified session calls in `tests/signatures/` to the
  adapter. Canonical Auth method constants and limits remain unchanged. Qualify
  signatures/session behavior and typed rejection matching on its matching Wire.
- Blob: replace the `wire` feature in
  `qualification/ic-blob-storage-pocketic-tests/Cargo.toml`; migrate
  `tests/support/{service_calls,fixture_calls}` and typed failure matching in
  `tests/storage.rs`. Preserve endpoint catalogs, reply-byte tightening and
  original error downcasts. Qualify installed calls and malformed/oversized replies.

Both migrations were independently completed locally by their owners and
inspected read-only on 2026-11-14. Auth's `target/upstream-1114/` records native
and Wasm Clippy/graph checks, five installed upgrade/certification/ingress cases
and three persistent lifecycle cases. Blob's `target/testkit-refresh-20261114/`
records qualification Clippy and seven installed ingress, typed-error,
lost-acknowledgement and repeated-upgrade cases. They use published Testkit
0.104.0 and adapter 0.1.0 with Wire 0.22.1 directly. These are owners' local
Linux checks, not claims of sibling delivery or native macOS qualification.

## Multiple-generation composition proof

`make testkit-wire-generations-check` is an explicit architecture probe, separate
from normal deployment gates. It needs the prepared server and cached Cargo
inputs for Wire 0.21.5 and 0.22.1. It never downloads or changes product locks;
missing inputs refuse and retain the attempt for explicit setup.

The probe copies the single current adapter implementation and its existing
installed-call test into two disposable package selections. Only temporary
manifest versions and Wire requirements differ. Both consumers coexist in one
Cargo graph, each exchanging its own exact Wire `Limits` and errors through its
matching adapter, with exactly one current Testkit and upstream PocketIC owner.
Source comparisons refuse drift from the authoritative implementation/test.
Both generations exercise live query/update decoding, malformed replies,
original typed causes and upstream rejections: four installed tests passed on
Linux under Rust 1.100. Evidence is retained under
`target/tmp/wire-adapter-generations.WUBgPW/`.

The old-generation package is an unpublished fixture, not a supported legacy
product line. This proves that changing codec selection at the adapter owner
does not require changing lifecycle source/version; it does not claim a second
released adapter. The existing unit/canister tests separately retain all budgets,
request frames, caller identity and no-retry update effects. Combined with the
Auth/Blob evidence, this completes the local acceptance of
[#357](https://github.com/dragginzgame/ic-tooling/issues/357).
