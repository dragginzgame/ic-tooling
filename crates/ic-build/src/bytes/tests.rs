use std::{
    io::{self, Read},
    sync::atomic::AtomicBool,
};

use super::capture;
use crate::Failure;

#[test]
fn exact_known_digest_and_ceiling() {
    let (bytes, digest) = capture(&b"abc"[..], 3, &AtomicBool::new(false)).unwrap();
    assert_eq!(bytes, b"abc");
    assert_eq!(
        digest,
        [
            0xba, 0x78, 0x16, 0xbf, 0x8f, 0x01, 0xcf, 0xea, 0x41, 0x41, 0x40, 0xde, 0x5d, 0xae,
            0x22, 0x23, 0xb0, 0x03, 0x61, 0xa3, 0x96, 0x17, 0x7a, 0x9c, 0xb4, 0x10, 0xff, 0x61,
            0xf2, 0x00, 0x15, 0xad
        ]
    );
    assert!(matches!(
        capture(&b"abcd"[..], 3, &AtomicBool::new(false)),
        Err(Failure::ArtifactLimit)
    ));
}

///
/// Refusal
/// Returns the original I/O error during capture.
///

struct Refusal;

impl Read for Refusal {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::from_raw_os_error(13))
    }
}

#[test]
fn cancellation_and_original_io_cause_survive() {
    let Err(Failure::Io(error)) = capture(Refusal, 10, &AtomicBool::new(false)) else {
        panic!("lost original I/O cause");
    };
    assert_eq!(error.raw_os_error(), Some(13));
    assert!(matches!(
        capture(Refusal, 10, &AtomicBool::new(true)),
        Err(Failure::Cancelled)
    ));
}

///
/// Interrupted
/// Interrupts the first read to exercise the retry path.
///

struct Interrupted(bool);

impl Read for Interrupted {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if !self.0 {
            self.0 = true;

            return Err(io::ErrorKind::Interrupted.into());
        }

        b"".as_slice().read(buffer)
    }
}

#[test]
fn interrupted_read_is_retried_without_losing_bytes() {
    assert!(
        capture(Interrupted(false), 1, &AtomicBool::new(false))
            .unwrap()
            .0
            .is_empty()
    );
}
