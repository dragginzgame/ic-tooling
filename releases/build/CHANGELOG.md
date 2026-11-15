# Build changelog

## [0.86.1] - Unreleased

### ⚡ Performance

- Add `CdylibWasmSelector` to select exact Cargo Wasm outputs incrementally with
  caller limits for distinct candidate count and total encoded path bytes.
  The Wasm build example and shared-target capture fixture no longer retain all
  Cargo messages. No elapsed-time or process-memory improvement is claimed yet.

### 🔧 Maintenance

- Share selection logic with the existing slice-based entrypoint, retaining
  exact manifest/recipe matching, duplicate-path handling and completion refusals.
- Preserve observation errors and prevent successful selection after any refusal.
