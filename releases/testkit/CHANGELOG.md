# Testkit family changelog

This ledger owns Testkit, Testkit Native and the private development/probe packages. The published baseline at the split is 0.76.1,
from the [original joint integration release](../integration/CHANGELOG.md#0761---2026-11-01).
Joint history through 0.76.1 remains there once, with its original scope.

## [0.79.0] - Unreleased

### ⚠️ Breaking

- Require Rust 1.99.1 and pin contributor/release compilation to that version.
  Native qualification now owns the former compiler-lane feature and Wasm
  obligations; the stable MSRV status reuses its results. Older compiler support
  is retired; package release groups remain independent.
  [#313 — Adopt Rust 1.99.1 and consolidate compiler qualification](https://github.com/dragginzgame/ic-tooling/issues/313).

- Benchmark marker formatting and canister emission now have one checked path:
  `format_marker` and `performance::measure` return grammar errors. Replace
  `try_format_marker`/`try_measure` with those names and handle their results.
- `pair_benchmark_spans` pairs only within the same prefix, stream, suite and
  label. Replace `pair_benchmark_spans_by_origin` with that canonical function;
  independent producers sharing those fields still need caller separation.
- Parsing, pairing, aggregation and comparison are native-only. Wasm retains
  marker counters, checked formatting, emission and deterministic principals;
  its library graph no longer selects Metrics. Move analysis to the host.

### 🔧 Changed

- Adopt Metrics 0.26 with the already-selected Rust 1.99.1 compiler. Preserve
  native arithmetic-only benchmark averages, existing errors and the Metrics-free
  Wasm graph; no transport features are enabled.

- Select published Types 0.16 directly for the existing external Principal identity.
  Select Runtime 0.17.1 directly for Wasm IC0 and the private performance probe
  entrypoints. Testkit and probe Wasm graphs now exclude the Platform facade;
  raw Testkit remains executor-free. Native Candid/PocketIC adapters, marker
  formats and caller-owned policies are unchanged.

### 🧹 Cleanup

- Reuse bounded process-test isolation for server-asset qualification. Keep
  setup cancellation, authentication, retained failure evidence and child reaping.
- Keep current host guidance separate from the preserved qualification archive,
  and use one installation example without hand-maintained dependency versions.

## [0.78.0] - 2026-11-01

### ⚠️ Breaking

- Align optional Wire transport with IC Wire 0.14. Consumers passing limits to
  `WireCallSpec` or inspecting codec errors must select Wire 0.14 too. This also
  aligns the optional Platform Types bridge with Platform 0.17's owned types;
  frame formats and caller-selected limits remain unchanged.

### 🔧 Changed

- Adopt Metrics 0.25 for benchmark averages with default features disabled.
  Preserve exact scaled averages and Testkit's existing error types without
  enabling Metrics' native Wire metadata.

## [0.77.0] - 2026-11-01

### ⚠️ Breaking

- Adopt IC Platform 0.17.0 and its maintained PocketIC client. Consumers must
  use `ic_platform::pocket_ic` from Platform 0.17 for client and client-owned DTO
  identities passed to Testkit helpers. Method signatures, simulator behavior
  and the PocketIC 16.1.0 server pin remain unchanged.

### 🔧 Changed

- Select Platform's independent Runtime 0.17.1 for Wasm system calls and the
  private performance probe; preserve the executor singleton and native-only
  client boundary.

## [0.76.2] - 2026-11-01

### 🔧 Changed

- Release Host Tools and the Testkit family independently in the existing workspace.
  Select `RELEASE_GROUP=testkit` for this group; package versions and public
  contracts are unchanged. Existing integration journals retain their original
  source, policy and recovery context. ([#299](https://github.com/dragginzgame/ic-tooling/issues/299))
