# Text changelog

Packages: `ic-host-text`.

This group starts with the replacement implementation. Earlier shared development
evidence is in [checkpoint history](../history.md), not a prior release of this group.

## [0.77.5] - 2026-11-19

### ➕ Added

- Provide binary byte-size formatting for Canic build reports, with two decimal
  places and promotion when rounding crosses a unit boundary.

## [0.77.4] - 2026-11-19

### 🔧 Changed

- Adopt Toko Tooling 0.45.21 and its Rust source layout in the packaged crate.

## [0.77.3] - 2026-11-18

### 🔧 Changed

- Refresh the package development tooling selection to Toko Tooling 0.45.8.
  Public APIs, persisted bytes and runtime behavior remain unchanged.

## [0.77.2] - 2026-11-16

First replacement release after the unpublished GitHub checkpoint
[aaebfa4](https://github.com/dragginzgame/ic-tooling-new/commit/aaebfa4db99593c3709a0ee92f71bc691f6ac4b6).
The replacement API requires explicit migration from 0.76.0. Prior qualification
evidence remains tied to its recorded inputs.

### ⚠️ Breaking

- Admit terminal escaping and bordered/plain tables through the new Text API.
  Consumers migrate to these operations explicitly; other legacy Text workflows
  remain unqualified. No progress, clipping or terminal policy is introduced.

### ➕ Added

- Escape terminal controls and bidirectional text before scalar Unicode width
  measurement, preserving printable text and the required ASCII table layout.

- Add plain tables for current Canic with left/right alignment, fixed two-space
  gaps and a header rule, without a general formatting-options framework.

### 🧪 Testing

- Prepare the 0.77.2 registry candidate with complete package metadata and add
  extracted-archive presentation, lint and documentation checks to the native matrix.

- Refresh current Canic caller migrations without replacing new presentation
  behavior. Include Cargo diagnostic escaping, subnet comparison and artifact
  listing; focused display tests and all-target Clippy pass in the combined
  Build/Text/Process export. No new Text API is introduced.

- Combine current Canic Build 0.87.3 and Text 0.77.2 in one app-workflow graph;
  qualify app partial effects and thirteen fleet/placements display cases. Refresh
  new display callers using the existing Text API and retain external selections.

- Qualify current Query presentation/output with unchanged evidence and JSON.
- Qualify DB's installed console schema/query/cursor and rejection behavior with
  the same two operations; independently observe cleanup of its instances.
- Qualify current Canic retained-status, terminal-view and declaration output,
  preserving raw evidence, unknown states and separation from effect authority.
- Refresh current Query qualification for its placements presenter, preserving
  partial JSON and rejection escaping while retaining its selected protocol owner.
