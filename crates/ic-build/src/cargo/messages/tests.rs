use super::*;

#[test]
fn message_partitions_crlf_and_eof_tails_keep_complete_records() {
    let bytes = b"{\"reason\":\"build-finished\",\"success\":true}\r\n{\"reason\":\"build-finished\",\"success\":false}";
    for width in 1..=bytes.len() {
        let mut decoder = MessageDecoder::new(bytes.len(), false);
        let mut successes = Vec::new();
        let mut receive = |message| {
            let schema::Message::BuildFinished(message) = message else {
                panic!("build-finished record");
            };
            successes.push(message.success);
            Ok(())
        };
        for chunk in bytes.chunks(width) {
            decoder.feed(chunk, &mut receive).unwrap();
        }
        decoder.finish(&mut receive).unwrap();
        assert_eq!(successes, [true, false]);
    }
}

#[test]
fn limits_malformed_json_text_policy_and_original_sink_errors() {
    let mut limited = MessageDecoder::new(3, false);
    assert!(limited.feed(b"1234", |_| Ok(())).is_err());
    for allow_text in [false, true] {
        let mut malformed = MessageDecoder::new(64, allow_text);
        assert!(malformed.feed(b"{invalid}\n", |_| Ok(())).is_err());
    }
    let mut strict = MessageDecoder::new(64, false);
    assert!(strict.feed(b"plain output\n", |_| Ok(())).is_err());
    let mut text = MessageDecoder::new(64, true);
    text.feed(b"plain output\n", |message| {
        assert!(matches!(message, schema::Message::TextLine(_)));
        Ok(())
    })
    .unwrap();
    let bytes = b"{\"reason\":\"build-finished\",\"success\":true}\n";
    let mut decoder = MessageDecoder::new(bytes.len(), false);
    let error = decoder
        .feed(bytes, |_| Err(io::Error::from_raw_os_error(28)))
        .unwrap_err();
    assert_eq!(error.raw_os_error(), Some(28));

    // The same exact bound applies to newline records and accepted EOF tails.
    for bytes in [bytes.as_slice(), &bytes[..bytes.len() - 1]] {
        let mut exact = MessageDecoder::new(bytes.len(), false);
        let mut delivered = 0;
        exact
            .feed(bytes, |_| {
                delivered += 1;
                Ok(())
            })
            .unwrap();
        exact
            .finish(|_| {
                delivered += 1;
                Ok(())
            })
            .unwrap();
        assert_eq!(delivered, 1);
        let mut short = MessageDecoder::new(bytes.len() - 1, false);
        let error = short
            .feed(bytes, |_| panic!("oversized record must not emit"))
            .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert_eq!(
            error.to_string(),
            "Cargo JSON message exceeds caller allowance"
        );
    }
    let mut tail = MessageDecoder::new(bytes.len(), false);
    tail.feed(&bytes[..bytes.len() - 1], |_| panic!("not yet complete"))
        .unwrap();
    assert_eq!(
        tail.finish(|_| Err(io::Error::from_raw_os_error(28)))
            .unwrap_err()
            .raw_os_error(),
        Some(28)
    );
    let mut malformed_tail = MessageDecoder::new(64, true);
    malformed_tail.feed(b"{invalid}", |_| Ok(())).unwrap();
    assert_eq!(
        malformed_tail.finish(|_| Ok(())).unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
}
