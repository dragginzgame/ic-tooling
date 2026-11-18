# Complete HTTP body admission

Current local Backup uses one shared HTTP capability: consume a Reqwest response
into complete owned bytes under an explicit ceiling. Backup owns its exact request,
status/encoding admission, TLS, timeout, no-retry policy and certificate decoding.

The replacement `ic-host-http` 0.80.0 owns only `body(Response, usize) -> Vec<u8>`
as an async fallible operation. It does not re-export the client or offer TLS
features. Consumers select Reqwest directly and retain its upstream type identity.
This is a native adapter: its selected Reqwest `Response::chunk` operation is
absent from Reqwest's Wasm implementation. The target condition follows that API
boundary, not an assumption that all shared HTTP logic needs a host guard.
Reqwest's optional browser stream API is a different feature/platform contract;
there is no current browser or IC HTTP caller admitted by this workload.
The incompatible version increment is from the observed 0.79.0 supplier.
Version 0.80.0 is published; [release receipts](release-ownership.md#independent-library-publication)
record registry provenance and native package acceptance separately from consumer migration.

An available content length above the ceiling refuses before polling the body.
Every received chunk is checked against remaining capacity before allocation or
copying; the result is returned only on complete EOF. Allocation is fallible and
requested capacity never exceeds the ceiling. The allocator can round allocations
up, and Reqwest's own buffers are outside this bound. Limits apply to bytes yielded
by Reqwest, including any consumer-enabled decoding. No untrusted size hint reserves
memory. Empty bodies may succeed at zero; excess bytes cannot return a prefix.

Failures distinguish declared/observed excess, original transport causes and
allocation refusal. Display is redacted, but the original transport source may
contain URLs. Cancellation drops this response and buffer; no task or retry is
created. Neither failure nor cancellation proves absence of server-side effects.

This adapter admits HTTP chunks directly and needs no content digest. It does not
introduce a generic async stream, incremental hashing or buffer API in the byte
owner just to bridge synchronous readers and HTTP. Its native optional graph stays
outside Build and the portable byte owner. No old source/module tree is copied;
the small loop is implemented against this contract. The prior bounded reader is
behavioral evidence, especially early length refusal and capped fallible growth.

Acceptance uses local HTTP fixtures for exact/empty/chunked/oversized/truncated
bodies and retained transport causes. Current Backup tests independently exercise
one dispatch, status/encoding refusal, deadline/cancellation, TLS refusal, certified
responses and durable replay. These do not establish live-network or macOS release
qualification and make no performance claim.

Latest Linux evidence: two new adapter tests and 13 selected current Backup HTTP
cases pass. Product and consumer warning-denied Clippy, product Rustdoc, formatting
and Toko admission pass. Cargo graph checks exclude HTTP/Tokio from Build and
portable bytes. See [Backup provenance](../qualification/backup/README.md) for the
exact consumer snapshot and remaining qualification boundaries.
