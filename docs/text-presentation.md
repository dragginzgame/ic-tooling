# Text presentation for current Query

The admitted owner is a pure presentation leaf: `escape` returns terminal-safe
text and `bordered_table` renders caller-selected headers/cells. It performs no
I/O, terminal detection, clipping, progress, parsing, identity normalization or
report selection. Query owns its raw evidence, JSON, report meanings and budgets.

Current Query uses standalone escaping and dynamic left-aligned ASCII tables.
The reference inspected is production `ic-host-text` 0.76.0 in read-only
`../ic-tooling`. Preserve its required display outcomes: escape Unicode controls,
bidirectional marks/overrides/isolates and line separators with Rust scalar escape
spelling; printable Unicode and literal backslashes remain unchanged. This is
not reversible encoding. Escape before measuring; sum `unicode-width` scalar
columns, including separate widths for joined emoji. Do not silently substitute
sequence/grapheme width rules. Missing cells are empty; extra cells add columns;
zero columns is empty; header-only tables retain borders; no final newline.

Implement these operations anew with one escaping owner and one table layout.
Retain only the needed `unicode-width` dependency. A general table framework would
still need our escaping and scalar-width policy and introduces unneeded options.
No legacy source or test tree is imported. Allocation follows ordinary String/Vec
semantics; callers bound inputs and output. No memory/performance gain is claimed.

Published 0.77.2 breaks from the observed 0.76.0 line; see
[release receipts](release-ownership.md#independent-library-publication).
Other Text callers and operations remain unqualified; this does not retire them.

Current DB’s local console is a second caller of the same two operations. Its
exported migration composes with direct PocketIC, retaining the consumer’s installed
console test. No extra Text API or copied console implementation is required.

Current rewritten Canic adds one admitted layout: unbordered tables with a dashed
header rule, two spaces between columns and per-column left/right alignment.
Every inspected caller uses those same gap/rule settings. `plain_table` therefore
has no general formatting-options object; fixed-size row/header/alignment arrays
ensure the column count agrees. Lines omit trailing whitespace, matching current
output. Escaping and scalar width retain the same owner as bordered tables.
Only the Text package implements layout; all Canic-specific code stays in its
read-only sibling snapshot plus the qualification migration patch.
