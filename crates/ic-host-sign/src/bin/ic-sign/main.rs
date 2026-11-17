//! Single-request offline signer. Private key material never leaves this process.

use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;
use ic_host_sign::protocol::{self, Operation, Response, WireRequest};
use ic_platform::native::{Secp256k1Identity, UpdateArgs};
use ic_platform_types::Principal;
mod identity;

#[derive(Parser)]
#[command(
    version,
    about = "One bounded offline IC signing request using an already-unlocked ICP keyring identity; never prompts"
)]
struct Args {}

fn main() -> ExitCode {
    // Native dependencies can panic; their panic payload is not a credential-safe
    // diagnostic interface. Parent-side execution evidence remains authoritative.
    std::panic::set_hook(Box::new(|_| eprintln!("signer internal failure")));
    let _ = Args::parse();
    match sign() {
        Ok(bytes) => {
            let mut output = io::stdout().lock();
            match output.write_all(&bytes).and_then(|()| output.flush()) {
                Ok(()) => ExitCode::SUCCESS,
                Err(_) => ExitCode::from(3),
            }
        }
        Err(()) => {
            eprintln!("signer request refused");
            ExitCode::from(2)
        }
    }
}

fn sign() -> Result<Vec<u8>, ()> {
    let input = protocol::read_frame(io::stdin().lock()).map_err(|_| ())?;
    let wire: WireRequest = serde_json::from_slice(&input).map_err(|_| ())?;
    if wire.protocol != protocol::PROTOCOL
        || protocol::commitment(&wire.configuration, wire.tool_sha256).map_err(|_| ())?
            != wire.commitment
        || protocol::tool_hash(&std::env::current_exe().map_err(|_| ())?).map_err(|_| ())?
            != wire.tool_sha256
    {
        return Err(());
    }
    wire.configuration.admit().map_err(|_| ())?;
    wire.request.admit(&wire.configuration).map_err(|_| ())?;
    #[cfg(target_os = "linux")]
    if std::env::var("DBUS_SESSION_BUS_ADDRESS").ok() != wire.configuration.local_bus {
        return Err(());
    }
    // This existing OS API allocates its secret response before our length check.
    // The helper has no scratch files or secret output; no heap-quota claim is made.
    let pem = identity::read(&wire.configuration.identity)?;
    if pem.len() > 16384 {
        return Err(());
    }
    let identity = Secp256k1Identity::from_pem(std::str::from_utf8(&pem).map_err(|_| ())?, 16384)
        .map_err(|_| ())?;
    drop(pem);
    if identity.principal().to_text() != wire.configuration.sender {
        return Err(());
    }
    wire.request.admit(&wire.configuration).map_err(|_| ())?;
    let effective = Principal::from_text(&wire.request.effective).map_err(|_| ())?;
    let envelope = match &wire.request.operation {
        Operation::Update {
            recipient,
            method,
            arg,
            nonce,
        } => identity
            .prepare_update(
                UpdateArgs {
                    recipient: Principal::from_text(recipient).map_err(|_| ())?,
                    effective,
                    method,
                    arg,
                    nonce: nonce.as_deref(),
                    expiry: wire.request.expires_ns,
                },
                wire.request.envelope_bytes,
            )
            .map_err(|_| ())?
            .envelope_bytes()
            .to_vec(),
        Operation::Status { request_id } => identity
            .prepare_request_status(
                *request_id,
                effective,
                wire.request.expires_ns,
                wire.request.envelope_bytes,
            )
            .map_err(|_| ())?
            .envelope_bytes()
            .to_vec(),
    };
    if protocol::now_ns().map_err(|_| ())? >= wire.request.deadline_ns {
        return Err(());
    }
    protocol::encode_response(&Response {
        protocol: protocol::PROTOCOL.to_owned(),
        commitment: wire.commitment,
        envelope,
    })
    .map_err(|_| ())
}
