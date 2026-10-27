# Private Wasm cache hash evaluation

On 2026-10-27, [#252 — Evaluate Platform BLAKE3 for private Wasm cache verification](https://github.com/dragginzgame/ic-tooling/issues/252)
was evaluated against Tooling `12d8acb7d61ae9aea83bdd855113ce513273428d`
(0.68.10). Retain SHA-256 and the existing cache format. The measured ordinary
warm-acquisition opportunity is small, and no concrete consumer throughput
requirement justifies the additional dependency, private digest type and format
migration. Prepared readers do have a larger relative opportunity; this decision
does not claim otherwise.

## Method and scope

An isolated source copy instrumented the existing `digest_file` calls whose
domain is `wasm-artifact-v1`. The timer includes opening, metadata, allocation,
streaming, framing and SHA-256 finalization. It leaves every digest, stamp,
freshness check and independent destination verification unchanged. Both exact
output verification and destination verification ran on every measured hit:
two output-hash calls per acquisition.

The harness measured complete ordinary warm acquisitions and one-entry prepared
reader batches. Each input had three warm-up pairs and 25 recorded pairs, with
alternating ordinary/prepared order. It used real Cargo metadata and tool probes
in a minimal fixture workspace. Only the cold artifact producer was substituted:
it copied frozen real Timers/Jobs outputs, plus a synthetic 4 KiB size control.
The cold producer and input-snapshot preparation are outside the measurements.
This measures cache acquisition over these retained bytes, not their compilation
or product qualification. Every acquisition was checked to be a successful hit;
the initial retained bytes were independently compared with the payload.

Linux x86-64, AMD Ryzen Threadripper 7970X, Rust/Cargo 1.99.1, release profile,
warm filesystem cache, without CPU affinity. Other host work was not excluded.
Cargo.lock SHA-256:
`eff2c17d5cfc0c9b8c13d969089466bba002e67b7f4f30e16f1fe7401f9927d4`.
The isolated instrumentation and harness passed affected-owner Clippy before
the measurement. These are Linux measurements, with no native macOS or MSRV
performance claim. The earlier issue's `/tmp/shared-hash-audit.JCGZZB/` directory
was unavailable; its hash-only figures were not reused as current evidence.

## Results

Times are medians in milliseconds. The final column is the median paired
fraction spent in **all output verification**, an optimistic bound if that
entire work became free. It is not measured or predicted BLAKE3 speed-up.

| Payload bytes | Mode | Whole acquisition | Output verification | Optimistic removable fraction |
| ---: | --- | ---: | ---: | ---: |
| 4,096 control | Ordinary | 107.337 | 0.016 | 0.015% |
| 4,096 control | Prepared | 1.588 | 0.016 | 1.03% |
| 263,261 Timers size | Ordinary | 105.532 | 0.313 | 0.30% |
| 263,261 Timers size | Prepared | 1.907 | 0.303 | 16.30% |
| 597,607 Timers runtime | Ordinary | 104.520 | 0.679 | 0.65% |
| 597,607 Timers runtime | Prepared | 2.263 | 0.657 | 29.38% |
| 942,978 Jobs | Ordinary | 106.956 | 1.041 | 0.98% |
| 942,978 Jobs | Prepared | 2.568 | 1.031 | 40.07% |

Even free verification saves only about 0.3–1.0 ms for these real artifacts.
A replacement algorithm still needs bounded reads, framing, changed-size
refusal and independent destination checks, so it cannot recover the entire
measured interval. The prepared percentages are substantial, but do not by
themselves establish a useful consumer-level gain. Larger artifacts, long
prepared-reader cohorts, different filesystems and other CPUs could change the
decision. No integrated BLAKE3 comparison or cold-build speed-up is claimed.

## Retained evidence and contract

The source archive, isolated instrumented source, harness, copied payloads,
compiler/CPU identities, Clippy logs, raw `samples.csv` and `summary.json` remain
under `/tmp/ic-tooling-open-issues.fOTL8a/evaluation/` (the baseline archive is
in its parent). The focused probe was:

```sh
cargo test --release -p ic-build --lib issue252_whole_cache_probe \
  --locked --offline -- --exact \
  artifacts::wasm_cache::tests::issue252_whole_cache_probe \
  --nocapture --test-threads=1
```

The probe and instrumentation are evaluation artifacts, not maintained production
code or a new benchmark service. No Platform/BLAKE3 edge was added to Build.
Public SHA-256 `InputDigest`, build/input keys, receipts, external raw-artifact
digests and private stamp labels/formats remain unchanged. A future justified
adoption must retain the issue's distinct private-type/format and host acceptance
requirements; putting BLAKE3 bytes in a SHA-256 identity would be invalid.
