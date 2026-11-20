use super::{Error, Length, copy, read};
use std::io::{self, Read};

#[test]
fn exact_and_ceiling_reads_have_known_identity_and_no_partial_success() {
    let (bytes, identity) = read(&b"abc"[..], Length::Exactly(3)).unwrap();
    assert_eq!(bytes, b"abc");
    assert_eq!(identity.bytes, 3);
    assert_eq!(
        identity.sha256,
        [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad
        ]
    );
    assert_eq!(
        copy(&b"abc"[..], io::sink(), Length::AtMost(4)).unwrap(),
        identity
    );
    assert!(matches!(
        read(&b"ab"[..], Length::Exactly(3)),
        Err(Error::Short {
            expected: 3,
            actual: 2
        })
    ));
    assert!(matches!(
        read(&b"abcd"[..], Length::AtMost(3)),
        Err(Error::Limit { limit: 3 })
    ));
}

///
/// Broken
/// Injects a read failure into bounded artifact admission.
///

struct Broken;

impl Read for Broken {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::from_raw_os_error(13))
    }
}

#[test]
fn original_io_and_partial_writer_effects_are_preserved() {
    let Err(Error::Io(error)) = copy(Broken, io::sink(), Length::AtMost(10)) else {
        panic!("lost cause");
    };
    assert_eq!(error.raw_os_error(), Some(13));
    let mut output = [0; 2];
    assert!(matches!(
        copy(&b"abc"[..], &mut output[..], Length::Exactly(3)),
        Err(Error::Io(_))
    ));
    assert_eq!(&output, b"ab");
}
