use super::{DecodeError, DecodeLimits, decode_one};
use ic_platform::candid_codec::{self as codec, decoder};

fn encode_one<T: codec::Encode>(value: T) -> Vec<u8> {
    codec::encode_one(&value, 65536).unwrap()
}

fn limits(bytes: usize) -> DecodeLimits {
    DecodeLimits {
        reply_bytes: bytes,
        work: 4096,
        allocation: 4096,
        type_entries: 64,
        header_bytes: 1024,
    }
}

#[test]
fn exact_budget_and_repeated_replies_are_charged() {
    let bytes = encode_one(42_u64);
    let mut remaining = bytes.len() * 2;
    for _ in 0..2 {
        assert_eq!(
            decode_one::<u64>(&bytes, limits(bytes.len()), &mut remaining).unwrap(),
            42
        );
    }
    assert_eq!(remaining, 0);
    assert!(matches!(
        decode_one::<u64>(&bytes, limits(bytes.len()), &mut remaining),
        Err(DecodeError::AggregateBytes)
    ));
    assert_eq!(remaining, 0);
    remaining = bytes.len() - 1;
    assert!(matches!(
        decode_one::<u64>(&bytes, limits(bytes.len()), &mut remaining),
        Err(DecodeError::AggregateBytes)
    ));
    assert_eq!(remaining, bytes.len() - 1);
}

#[test]
fn byte_admission_precedes_decode_and_never_underflows() {
    let bytes = b"not candid";
    let mut remaining = bytes.len();
    assert!(matches!(
        decode_one::<u64>(bytes, limits(bytes.len() - 1), &mut remaining),
        Err(DecodeError::ReplyBytes)
    ));
    assert_eq!(remaining, bytes.len());
    assert!(matches!(
        decode_one::<u64>(bytes, limits(bytes.len()), &mut remaining),
        Err(DecodeError::Candid(_))
    ));
    assert_eq!(remaining, 0);
}

#[test]
fn caller_decoder_budgets_are_enforced() {
    let bytes = encode_one(vec![42_u64; 128]);
    for constrained in [
        DecodeLimits {
            work: 0,
            ..limits(bytes.len())
        },
        DecodeLimits {
            allocation: 0,
            ..limits(bytes.len())
        },
        DecodeLimits {
            type_entries: 0,
            ..limits(bytes.len())
        },
        DecodeLimits {
            header_bytes: 0,
            ..limits(bytes.len())
        },
    ] {
        let mut remaining = bytes.len();
        assert!(matches!(
            decode_one::<Vec<u64>>(&bytes, constrained, &mut remaining),
            Err(DecodeError::Candid(_))
        ));
        assert_eq!(remaining, 0);
    }
}

#[test]
fn skipped_fields_consume_the_selected_work_budget() {
    let bytes = encode_one(vec![42_u64; 128]);
    let mut remaining = bytes.len();
    let constrained = DecodeLimits {
        work: 0,
        ..limits(bytes.len())
    };
    assert!(matches!(
        decode_one::<codec::Reserved>(&bytes, constrained, &mut remaining),
        Err(DecodeError::Candid(_))
    ));
    assert_eq!(remaining, 0);
}

#[test]
fn extra_arguments_follow_candid_compatibility_and_skipping_limits() {
    let bytes = codec::arguments::encode_args(&(42_u64, "future argument"), 1024).unwrap();
    let mut remaining = bytes.len();
    assert_eq!(
        decode_one::<u64>(&bytes, limits(bytes.len()), &mut remaining).unwrap(),
        42
    );
    assert_eq!(remaining, 0);
    remaining = bytes.len();
    let constrained = DecodeLimits {
        work: 0,
        ..limits(bytes.len())
    };
    assert!(matches!(
        decode_one::<u64>(&bytes, constrained, &mut remaining),
        Err(DecodeError::Candid(_))
    ));
    assert_eq!(remaining, 0);
}

#[test]
fn incompatible_values_and_trailing_bytes_preserve_decoder_errors() {
    let incompatible = encode_one("wrong type");
    let mut trailing = encode_one(42_u64);
    trailing.push(0xff);
    for bytes in [incompatible, trailing] {
        let mut remaining = bytes.len();
        let error = decode_one::<u64>(&bytes, limits(bytes.len()), &mut remaining).unwrap_err();
        assert!(matches!(error, DecodeError::Candid(_)));
        let selected = decoder::Limits {
            bytes: bytes.len(),
            work: 4096,
            allocation: 4096,
            types: 64,
            header: 1024,
        };
        let expected = decoder::decode_one::<u64>(&bytes, selected).unwrap_err();
        let DecodeError::Candid(actual) = error else {
            panic!("expected the preserved Candid cause");
        };
        assert_eq!(actual, expected);
        assert_eq!(remaining, 0);
    }
}
