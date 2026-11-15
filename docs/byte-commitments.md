# Complete producer commitments

The byte owner adds `hash_with(Length, producer)` to the unpublished 0.80.1
development line. The producer receives a temporary `dyn Write` and reports
success or its original typed error. Only successful producer completion and
length admission return an `Identity`. No incremental digest snapshot escapes.

This operation owns bounded SHA-256 accumulation, not framing, serialization,
storage or publication. It retains only fixed-size hash state and a byte count;
it does not collect payload bytes or forward them to a sink. `flush` therefore
does no external I/O. `Exactly` checks complete length; `AtMost` checks the ceiling.
An over-limit write admits none of that write and permanently refuses later writes
and flushes. If the producer catches that error and returns success, the operation
still returns a typed limit refusal. If the producer reports an error, its original
value takes precedence; no complete identity is available either way. Producer
panics unwind normally. The operation cannot detect semantic failures that a
producer never reports, nor certify external durability or authenticity.

## Requirement and simpler alternatives

Current Backup constructs restore-request commitments and sorted assembled-payload
commitments using multiple writes to a legacy `HashingWriter<sink>`, then requests
an incremental identity. A bounded buffer plus the existing stream reader would
retain the complete encoded commitment unnecessarily. Requiring callers to own
SHA-256 state would duplicate byte admission and completion handling. This narrow
producer operation removes their incremental writer/snapshot obligation without
adding a general hashing writer or a format-aware tree framework.

Backup keeps domain tags, member ordering, little-endian lengths, digest text,
filesystem observations and persisted/public representations. Its separate
multi-step capture hash state is not replaced by this synchronous closure API.
No old digest wrapper is admitted merely to preserve Rust signatures.

## Reuse and acceptance

The old hashing writer was inspected only after selecting complete operation
ownership. Its arbitrary sink forwarding and observable prefix snapshots were
not reused. New accumulation uses the already-selected `sha2` implementation and
existing `Length`/`Identity` types. There are no new dependencies, native APIs,
Backup formats or consumer code in the product crate.

Independent tests use fixed empty/`abc` SHA-256 vectors, segmented writes, flush,
exact and maximum lengths, short completion, a swallowed over-limit write and
subsequent refusals, plus the original producer I/O cause after accepted bytes.
Consumer evidence belongs in [Backup qualification](../qualification/backup/README.md),
not in the product's portable tests. Neither this API nor a matching digest proves
producer side effects were rolled back. Publication remains a separate operation.
