# Timers observation migration

The pinned Timers driver needs to detect input/configuration drift, reject hidden
compiler selections, and bind a receipt to the actual owned-build request and
before/after tool observations. It does not need a generic configuration database,
Cargo precedence resolver or duplicate selection/facts model from Build.

Selected owner: a private module in the migrated consumer driver. It reads bounded
regular configuration files through the new byte/filesystem owners, starting with
Cargo's ancestor and selected-home config locations (extensionless wins), and
follows explicit includes relative to each selected path. Missing optional includes
are allowed; unreadable, malformed, cyclic or over-budget inputs refuse. Compiler
keys in build/env refuse even if empty or shadowed. Bounds remain 1 MiB/file,
16 MiB/observation, 128 files and depth 32. No configuration text or TOML parser
message enters an error or receipt; original filesystem errors remain sources.

Source drift includes lookup names and raw-byte digests. Recipe identity is a new
`owned-build-v1` digest of the actual request selections, relevant environment and
ordered raw configuration digests, rather than the legacy selection's Debug text
and partial semantic TOML projection. This is an explicit receipt identity migration:
report schema/fields and application bytes stay unchanged, old reports still decode,
but different recipe generations refuse comparison. Raw config changes (including
credentials, comments or output-only settings) conservatively invalidate comparison;
values are never serialized into reports. This removes a second semantic model of
Cargo settings, rather than claiming credential-only changes affect compilation.

The private module is newly implemented from these requirements. No old observation
implementation or test tree is transplanted. Existing report and source-drift tests
remain in their pinned export; new focused tests cover include/override/bound/refusal
behavior and recipe stability. No public Build API is added. Registry Host and
Testkit remain separate migration work.
