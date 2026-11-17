# Changelog

Independent groups own their release notes. There is no shared workspace version.
See [release receipts](docs/release-ownership.md) for publication status;
pending entries and historical checkpoints are not registry releases.

| Group | Packages |
| --- | --- |
| [foundation](releases/foundation/CHANGELOG.md) | `ic-host-artifacts`, `ic-host-fs` |
| [build](releases/build/CHANGELOG.md) | `ic-build` |
| [cargo-metadata](releases/cargo-metadata/CHANGELOG.md) | `ic-cargo-metadata` |
| [http](releases/http/CHANGELOG.md) | `ic-host-http` |
| [process](releases/process/CHANGELOG.md) | `ic-host-process` |
| [sign](releases/sign/CHANGELOG.md) | `ic-host-sign` |
| [text](releases/text/CHANGELOG.md) | `ic-host-text` |
| [testkit](releases/testkit/CHANGELOG.md) | `ic-testkit` |
| [tools](releases/tools/CHANGELOG.md) | `ic-cargo-qualification` |

[Earlier unpublished checkpoints](releases/history.md) preserve pre-group history.
Source identities, validation results and current limitations belong in the
[current handoff](docs/status/current.md) and linked qualification records.

### ▶️ Try it — Sign 0.1.0

Install the released helper (network access), then inspect its protocol entrypoint
without accessing any identity or signing:

```sh
cargo install ic-host-sign --version 0.1.0 --features provider --locked
ic-sign --help
```

Expected: help for the single-request offline helper. Actual requests use the
[signer library and explicit native-keyring configuration](docs/sign-only-integration.md).
The provider requires an already-unlocked/authorized OS credential and never
launches an unlock prompt. Actual provider qualification passes on Linux and
native Intel/ARM macOS.

### ▶️ Try it — Cargo metadata 0.1.0

From a scratch directory, install the released command once (network access),
then render a candidate without replacing the input:

```sh
cargo install ic-cargo-metadata --version 0.1.0 --locked
printf '[package]\nname="demo"\nversion="0.1.0"\n' > input.toml
printf '{"version":"package","dependencies":[]}\n' > selection.json
ic-cargo-metadata --max-input-bytes 8388608 \
  group-manifest input.toml 0.1.0 0.1.1 selection.json > candidate.toml
cat candidate.toml
```

The candidate has version `0.1.1`; `input.toml` stays unchanged. No credentials,
Cargo release transaction or publication is involved in rendering.
