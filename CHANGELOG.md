# Changelog

Build is prepared as 0.86.1 for the compatible streaming-selector addition.
Foundation retains 0.79.0. No replacement crate version has been published.

## Unreleased tooling

- Add `make test-scheduling-consumer` to qualify the streaming-selection migration
  against Scheduling's real host tests in isolated scratch, without sibling edits.

- Expose Toko's shared `deps-list` and `deps-update` commands and add `make help`
  for development tools, dependency inspection and the standalone LOC reporter.

- Adopt Toko Tooling 0.44.0 through the exact Cargo pin and authenticated registry
  snapshot. Selected executable tooling is unchanged; no local evidence-verifier
  API migration is needed.

- [Build changes](releases/build/CHANGELOG.md)
- Foundation is retained from the source revision in
  [source provenance](docs/design/source-provenance.md), without behavior changes.
