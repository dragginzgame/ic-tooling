use super::super::{CborErrorKind, ClientError, error::cbor_error};
use ic_platform::{certification::cbor::CborError, ic_protocol::decode_subnet_metrics};
use std::error::Error;

#[test]
fn codec_failure_kinds_keep_the_original_typed_causal_chain() {
    let decode = decode_subnet_metrics(&[0xff], 1).unwrap_err();
    let CborError::Decode(encode_cause) = decode_subnet_metrics(&[0xff], 1).unwrap_err() else {
        panic!("malformed input must retain its decoder cause");
    };
    for (source, kind, nested_cause) in [
        (
            CborError::InputTooLarge,
            CborErrorKind::InputTooLarge,
            false,
        ),
        (
            CborError::OutputTooLarge,
            CborErrorKind::OutputTooLarge,
            false,
        ),
        (decode, CborErrorKind::Decode, true),
        (CborError::Encode(encode_cause), CborErrorKind::Encode, true),
    ] {
        let expected_display = format!("CBOR failed: {source}");
        let error = cbor_error(source);
        assert!(matches!(&error, ClientError::Cbor { kind: actual, .. } if *actual == kind));
        let cause = error.source().unwrap().downcast_ref::<CborError>().unwrap();
        assert_eq!(cause.source().is_some(), nested_cause);
        assert_eq!(error.to_string(), expected_display);
        assert!(error.request_context().is_none());
    }
}
