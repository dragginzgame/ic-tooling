# IC Testkit development programs

Private workspace package for coordinated archive qualification, the explicit
Internet Identity fixture and installed Wire acceptance. It is not published.
The optional fixture benchmark driver, worker and report framework are retired.

Prepare the selected PocketIC binary and Rust Wasm target through the normal
[developer setup](../../docs/testkit/overview.md#toolchains-and-checks). Then run
`cargo test --manifest-path qualification/Cargo.toml -p ic-testkit-dev --bin check_packages --locked --offline` for the
archive adapter, or the installed `canister_wire` test with an admitted server.

`make testkit-package-check` delegates exact coordinated archive qualification to
IC Build. It retains attempt evidence and a separate verification target beneath
`target/`, checks the extracted dependency graph, and runs archive tests/doctests
offline. Qualification installs no tools or sources. `make publish-check` uses
normal Cargo registry admission without uploading packages.
