use super::*;

#[test]
fn completed_segments_match_fixed_sha256_vectors() {
    let empty = hash_with(Length::Exactly(0), |_| Ok::<_, io::Error>(())).unwrap();
    assert_eq!(empty.bytes, 0);
    assert_eq!(
        empty.sha256,
        [
            227, 176, 196, 66, 152, 252, 28, 20, 154, 251, 244, 200, 153, 111, 185, 36, 39, 174,
            65, 228, 100, 155, 147, 76, 164, 149, 153, 27, 120, 82, 184, 85
        ]
    );
    let abc = hash_with(Length::Exactly(3), |writer| {
        writer.write_all(b"a")?;
        writer.flush()?;
        writer.write_all(b"bc")
    })
    .unwrap();
    assert_eq!(abc.bytes, 3);
    assert_eq!(
        abc.sha256,
        [
            186, 120, 22, 191, 143, 1, 207, 234, 65, 65, 64, 222, 93, 174, 34, 35, 176, 3, 97, 163,
            150, 23, 122, 156, 180, 16, 255, 97, 242, 0, 21, 173
        ]
    );
}

#[test]
fn swallowed_overflow_cannot_return_a_prefix_commitment() {
    let result = hash_with(Length::AtMost(3), |writer| {
        assert_eq!(writer.write(b"ab")?, 2);
        assert!(writer.write(b"cd").is_err());
        assert!(writer.write(b"c").is_err());
        assert!(writer.write(b"").is_err());
        assert!(writer.flush().is_err());
        Ok::<_, io::Error>(())
    });
    assert!(matches!(
        result,
        Err(ProduceError::Admission(Error::Limit { limit: 3 }))
    ));
    assert!(matches!(
        hash_with(Length::Exactly(3), |w| w.write_all(b"ab")),
        Err(ProduceError::Admission(Error::Short {
            expected: 3,
            actual: 2
        }))
    ));
    assert!(hash_with(Length::AtMost(3), |w| w.write_all(b"abc")).is_ok());
}

#[test]
fn original_producer_failure_wins_without_certifying_accepted_bytes() {
    let result = hash_with(Length::AtMost(1), |writer| {
        writer.write_all(b"a")?;
        assert!(writer.write(b"b").is_err());
        Err(io::Error::from_raw_os_error(13))
    });
    let Err(ProduceError::Producer(error)) = result else {
        panic!("lost original producer refusal")
    };
    assert_eq!(error.raw_os_error(), Some(13));
}
