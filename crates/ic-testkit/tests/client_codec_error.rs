//! Embedding callers classify codec failures without any Platform type names.
#![cfg(not(target_family = "wasm"))]
use ic_testkit::pocket_ic::{CborErrorKind, ClientError};
use std::{error::Error, io};

#[test]
fn consumer_matches_codec_kind_and_preserves_an_opaque_typed_source() {
    fn classify(error: &ClientError) -> Option<CborErrorKind> {
        match error {
            ClientError::Cbor { kind, .. } => Some(*kind),
            _ => None,
        }
    }

    let error = ClientError::Cbor {
        kind: CborErrorKind::Decode,
        source: Box::new(io::Error::from(io::ErrorKind::InvalidData)),
    };
    assert_eq!(classify(&error), Some(CborErrorKind::Decode));
    assert_eq!(
        error
            .source()
            .unwrap()
            .downcast_ref::<io::Error>()
            .unwrap()
            .kind(),
        io::ErrorKind::InvalidData
    );
}
