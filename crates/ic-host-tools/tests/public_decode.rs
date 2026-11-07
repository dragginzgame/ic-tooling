//! Embedding consumers configure and classify decoding without Platform types.
use ic_host_tools::candid::decode::{DecodeError, DecodeLimits, decode_one};

// External Candid nat64(42); this consumer does not select a Platform codec API.
const REPLY: &[u8] = b"DIDL\x00\x01\x78\x2a\x00\x00\x00\x00\x00\x00\x00";
const LIMITS: DecodeLimits = DecodeLimits {
    reply_bytes: REPLY.len(),
    decoding_quota: 4096,
    skipping_quota: 4096,
    type_entries: 64,
    header_bytes: 1024,
};

#[test]
fn consumer_owns_limits_and_matches_admission_and_decode_failures() {
    let mut remaining = REPLY.len();
    assert_eq!(
        decode_one::<u64>(REPLY, LIMITS, &mut remaining).unwrap(),
        42
    );
    assert_eq!(remaining, 0);
    assert!(matches!(
        decode_one::<u64>(REPLY, LIMITS, &mut remaining),
        Err(DecodeError::AggregateBytes)
    ));

    remaining = REPLY.len();
    assert!(matches!(
        decode_one::<u64>(
            REPLY,
            DecodeLimits {
                reply_bytes: REPLY.len() - 1,
                ..LIMITS
            },
            &mut remaining
        ),
        Err(DecodeError::ReplyBytes)
    ));
    assert_eq!(remaining, REPLY.len());

    let error = decode_one::<u64>(b"bad candid", LIMITS, &mut remaining).unwrap_err();
    assert!(matches!(error, DecodeError::Candid(_)));
    assert!(std::error::Error::source(&error).is_some());
    assert_eq!(remaining, REPLY.len() - b"bad candid".len());
}
