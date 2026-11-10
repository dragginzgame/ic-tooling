//! Explicit `PocketIC` setup, offline admission and command-scoped ownership.

#[cfg(all(unix, not(target_family = "wasm")))]
mod native;

#[cfg(all(unix, not(target_family = "wasm")))]
fn main() -> std::process::ExitCode {
    native::main()
}

#[cfg(not(all(unix, not(target_family = "wasm"))))]
fn main() {
    eprintln!("ic-testkit-server requires a supported Unix host");
    std::process::exit(2);
}
