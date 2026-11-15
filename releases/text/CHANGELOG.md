# Text changelog

Packages: `ic-host-text`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.77.1] - Unreleased

Continues the unpublished development line after GitHub checkpoint
[0352b4e](https://github.com/dragginzgame/ic-tooling-new/commit/0352b4e6fb1996e329e9ebf837ea38e40348e1bf).
This version advance changes no public behavior; prior qualification evidence
remains tied to its recorded inputs.

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

- Qualify current Query presentation/output with unchanged evidence and JSON.
- Qualify DB's installed console schema/query/cursor and rejection behavior with
  the same two operations; independently observe cleanup of its instances.
- Qualify current Canic retained-status, terminal-view and declaration output,
  preserving raw evidence, unknown states and separation from effect authority.
- Refresh current Query qualification for its placements presenter, preserving
  partial JSON and rejection escaping while retaining its selected protocol owner.
