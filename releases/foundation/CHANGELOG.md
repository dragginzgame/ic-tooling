# Foundation changelog

Packages: `ic-host-artifacts`, `ic-host-fs`.

This group starts with the replacement implementation. All entries are pending;
no replacement package has been published. Earlier shared development evidence is
in [checkpoint history](../history.md), not a prior release of this group.

## [0.80.1] - Unreleased

### ⚠️ Breaking

- The replacement byte and filesystem APIs require explicit consumer migration
  from the production packages. Filesystem publication distinguishes refusal
  before effects from visible-but-unconfirmed outcomes; retained recovery files
  have an explicit lifecycle. Existing stored bytes remain unchanged.

### 🔧 Changed

- Share bounded complete-stream admission and held-directory custody with Build
  and Backup. Add held-file identity-checked removal for producer cleanup while
  keeping parent synchronization explicit.
- Continue the unpublished 0.80 line after the Stage 1–2 checkpoint; update
  Backup's dependency selection with the suppliers.

### 🧪 Testing

- Qualify current Backup metadata, payload and transfer-binding behavior, retaining
  its budget, commitment comparison, recovery evidence and uncertain-write refusals.
- Qualify Canic retained-record admission and signed recovery. Current local-source
  evidence supersedes earlier pinned runs without extending build/publication scope.
- Qualify Jobs capture through retained staging and producer-identity cleanup,
  preserving optimizer flags, status, logs and failed-capture evidence.
