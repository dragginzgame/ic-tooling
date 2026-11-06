# Tooling Candid source boundary

`make candid-boundary-check` runs Shared's unmodified lexical checker and source
masker from the committed 0.30.6 snapshot, revision
`45c0295e23d3d23225e31f47c018c30ff0c75581`. The canonical exporter added the
checker and its companions; no local scanner or lexer is maintained. See the
[shared contract](../candid-boundary.md) and
[reviewed policy](../../ci/candid-boundary.json).

## Selected source inventory

One complete policy is applied to `crates`, `apps`, `scripts` and `docs`:

| Root | Covered Rust sources |
| --- | --- |
| `crates` | Every Foundation, Build, Text, Tools, Testkit and Native library; private Dev/probe packages; native tests, examples and test support, including the Internet Identity fixture driver. |
| `apps` | Foundation's Wasm inspector binary and its unit/integration tests. |
| `scripts` | The independent final-Wasm recipe driver copied into an isolated Cargo workspace during qualification. |
| `docs` | Retained Rust audit probes, including the historical Host reuse probe. |

The root Cargo manifest contains only workspace metadata; all twelve package
manifests belong to these roots. There is no separate maintained Rust workspace
under `testing` or `ci`. Browser fixtures there contain JS, JSON and WAT; they do
not introduce another Rust root. Generated compiler outputs, registry sources,
installed tools and Node dependencies are not first-party maintained Rust.
The recipe driver remains checked at its real source before it is copied.
New maintained Rust roots or generated external DTO sources must join this
explicit selection before their build/CI path is adopted.

The checker sees every source cfg branch without compiling. `dependency-pins-check`
requires this source check before metadata admission, so ordinary CI, publication
qualification and all release groups retain it through their existing common
stage. `RELEASE_GROUP` never truncates the roots or policy. The full inventory is
checked once per validation invocation, outside the group-specific lint/graph/test
loops; requesting both Make targets does not run it twice. A partial-root call
with the complete policy is invalid under Shared's allowance contract.

## Three exact external declarations

- **Tools bounded decoder:** its generic Candid bound adapts external replies via
  Platform's decoder while retaining caller-owned resource limits. It supplies
  no Candid application DTO.
- **Tools ledger block:** the existing external ledger query record contains
  Platform `Nat` and legacy `Value`. Platform's `BlockWithId` contains
  `ICRC3Value`, a distinct shape; replacing this existing public adapter is not a
  type-identical substitution. The allowance covers only that named declaration,
  not ledger evidence reports, limits, tips or another struct in the same file.
- **Native PocketIC management snapshots:** the existing helper bounds exact
  upstream management arguments for `candid::encode_one` and decodes their replies.
  Preserve the explicit native Testkit exception, management DTO identity,
  bounded snapshot transport and no-replay ingress custody in [AGENTS.md](../../AGENTS.md).

Foundation, Build, Text, Native, private packages and the inspected native
tests/examples need no declaration allowances. There are no codec-comparison or
owned-application-type allowances. Prefer Platform's existing external DTOs;
changing a declaration requires review of its named protocol and decoder.

## Limits and consumer rejection control

Literal qualified paths, manual implementations, generic bounds and conditional
derives are covered, including same-file explicit aliases from Platform re-exports.
This is not Rust type resolution or macro expansion. Review cross-module aliases,
wildcard re-exports, procedural macros, blanket implementations and generated
source strings separately. The current owned RPC macros use Wire; retained
compiler-plumbing test strings and provenance patches are not Candid DTO owners.
No separate generated first-party Candid implementation was found in this audit.

`bash scripts/ci/test-product-candid-boundary.sh` exercises the actual Make
admission and product policy in a disposable fixture. It proves that one
allowed ledger declaration cannot admit an owned report through Platform, and
that an aliased, cfg-disabled Candid derive in the independent recipe fixture
also stops validation. The unrelated metadata command is substituted; no Cargo
or native server runs. The control is in the existing tooling-command roster;
Shared's generic scanner regression matrix remains with Shared.

This adoption enables **Candid-only** source enforcement. It does not claim
CBOR or browser SDK enforcement, protocol byte compatibility or cryptographic
assurance. Shared's optional CBOR mode and dependency checks retain their own
contract; this task adds no bespoke CBOR scanner.
