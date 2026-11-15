use super::*;
use crate::artifact::encode_gzip;
use std::{cell::Cell, io::BufReader, rc::Rc};

fn gzip(bytes: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    encode_gzip(bytes, &mut output, 1, u64::MAX).unwrap();
    output
}

fn finish(bytes: &[u8], compressed: u64, decoded: u64) -> Result<(), GzipReadError<()>> {
    with_gzip_reader(bytes, compressed, decoded, |_| Ok(()))
}

#[test]
fn buffered_file_consumer_can_stop_early_but_success_requires_full_completion() {
    use std::{fs::OpenOptions, io::Write, time::SystemTime};

    let payload = vec![0x5a; 131_073];
    let compressed = gzip(&payload);
    let path = std::env::temp_dir().join(format!(
        "ic-host-gzip-reader-{}-{}.gz",
        std::process::id(),
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    // Preserve this file if an assertion fails; no artifact is published.
    OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&path)
        .unwrap()
        .write_all(&compressed)
        .unwrap();
    let file = std::fs::File::open(&path).unwrap();
    let first = with_gzip_reader(
        BufReader::with_capacity(3, file),
        compressed.len() as u64,
        payload.len() as u64,
        |reader| {
            let mut first = [0];
            reader.read_exact(&mut first)?;
            Ok::<_, io::Error>(first[0])
        },
    )
    .unwrap();
    assert_eq!(first, 0x5a);
    std::fs::remove_file(path).unwrap();
}

#[test]
fn exact_and_zero_limits_do_not_hide_input_or_expanded_overflow() {
    for payload in [vec![], vec![7; 131_073]] {
        let bytes = gzip(&payload);
        let compressed = bytes.len() as u64;
        let decoded = payload.len() as u64;
        finish(&bytes, compressed, decoded).unwrap();
        assert!(matches!(
            finish(&bytes, 0, decoded),
            Err(GzipReadError::InputLimit { limit: 0 })
        ));
        assert!(matches!(
            finish(&bytes, compressed - 1, decoded),
            Err(GzipReadError::InputLimit { .. })
        ));
        if decoded > 0 {
            assert!(matches!(
                finish(&bytes, compressed, decoded - 1),
                Err(GzipReadError::DecodedLimit { .. })
            ));
            let error = with_gzip_reader(&bytes[..], compressed, 0, |reader| {
                // A parser cannot turn an ignored decoder failure into success.
                let _ = reader.read(&mut [0]);
                Ok::<_, ()>(())
            })
            .unwrap_err();
            assert!(matches!(error, GzipReadError::DecodedLimit { limit: 0 }));
        }
    }
    assert!(matches!(finish(&[], 0, 0), Err(GzipReadError::Decode(_))));
}

#[test]
fn parser_eof_cannot_hide_corruption_truncation_trailing_bytes_or_another_member() {
    let bytes = gzip(b"payload after the parser's logical end");
    for end in 0..bytes.len() {
        assert!(finish(&bytes[..end], bytes.len() as u64, 4096).is_err());
    }
    for offset in [bytes.len() - 8, bytes.len() - 4] {
        let mut corrupt = bytes.clone();
        corrupt[offset] ^= 1;
        assert!(matches!(
            finish(&corrupt, corrupt.len() as u64, 4096),
            Err(GzipReadError::Decode(_))
        ));
    }
    for tail in [vec![0], gzip(b""), gzip(b"another member")] {
        let mut joined = bytes.clone();
        joined.extend(tail);
        assert!(matches!(
            finish(&joined, joined.len() as u64, 4096),
            Err(GzipReadError::TrailingData)
        ));
        // An exact compressed cap is not an artificial EOF hiding the tail.
        assert!(matches!(
            finish(&joined, bytes.len() as u64, 4096),
            Err(GzipReadError::InputLimit { .. })
        ));
    }
}

struct Source<'a> {
    bytes: &'a [u8],
    consumed: Rc<Cell<usize>>,
    interrupt: bool,
    fail_at: Option<usize>,
}

impl Read for Source<'_> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if output.is_empty() {
            return Ok(0);
        }
        let available = self.fill_buf()?;
        let count = available.len().min(output.len());
        output[..count].copy_from_slice(&available[..count]);
        self.consume(count);
        Ok(count)
    }
}

impl BufRead for Source<'_> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if std::mem::take(&mut self.interrupt) {
            return Err(io::ErrorKind::Interrupted.into());
        }
        if self.fail_at.is_some_and(|at| self.consumed.get() >= at) {
            return Err(io::Error::from_raw_os_error(13));
        }
        Ok(&self.bytes[..self.bytes.len().min(3)])
    }
    fn consume(&mut self, count: usize) {
        self.bytes = &self.bytes[count..];
        self.consumed.set(self.consumed.get() + count);
    }
}

#[test]
fn short_interrupted_source_reads_complete_and_native_failures_keep_their_identity() {
    let bytes = gzip(b"gzip traversal with small buffered reads");
    let source = |fail_at| Source {
        bytes: &bytes,
        consumed: Rc::new(Cell::new(0)),
        interrupt: true,
        fail_at,
    };
    with_gzip_reader(source(None), bytes.len() as u64, 4096, |_| Ok::<_, ()>(())).unwrap();
    for at in [0, 12, bytes.len()] {
        let error = with_gzip_reader(source(Some(at)), bytes.len() as u64, 4096, |_| {
            Ok::<_, ()>(())
        })
        .unwrap_err();
        let GzipReadError::Input(cause) = error else {
            panic!("expected source IO: {error:?}");
        };
        assert_eq!(cause.raw_os_error(), Some(13));
    }
}

#[test]
fn consumer_error_remains_typed_without_draining_and_stream_errors_cannot_be_swallowed() {
    #[derive(Debug, PartialEq)]
    struct ConsumerFailure(u8);
    let bytes = gzip(b"payload deliberately left unread");
    let consumed = Rc::new(Cell::new(0));
    let at_callback = Cell::new(0);
    let error = with_gzip_reader(
        Source {
            bytes: &bytes,
            consumed: consumed.clone(),
            interrupt: false,
            fail_at: None,
        },
        bytes.len() as u64,
        4096,
        |_| {
            at_callback.set(consumed.get());
            Err::<(), _>(ConsumerFailure(7))
        },
    )
    .unwrap_err();
    assert!(matches!(error, GzipReadError::Consumer(ConsumerFailure(7))));
    assert_eq!(consumed.get(), at_callback.get());

    let mut corrupt = bytes;
    let last = corrupt.len() - 1;
    corrupt[last] ^= 1;
    let error = with_gzip_reader(&corrupt[..], corrupt.len() as u64, 4096, |reader| {
        let mut output = Vec::new();
        let _ = reader.read_to_end(&mut output);
        Ok::<_, ()>(())
    })
    .unwrap_err();
    assert!(matches!(error, GzipReadError::Decode(_)));
}

#[test]
fn highly_compressible_multichunk_payload_is_drained_with_a_bounded_pull_interface() {
    let payload = vec![0; 3 * 1024 * 1024];
    let bytes = gzip(&payload);
    assert!(bytes.len() < payload.len() / 10);
    let counted = with_gzip_reader(
        BufReader::with_capacity(7, bytes.as_slice()),
        bytes.len() as u64,
        payload.len() as u64,
        |reader| io::copy(reader, &mut io::sink()),
    )
    .unwrap();
    assert_eq!(counted, payload.len() as u64);
    assert!(matches!(
        finish(&bytes, bytes.len() as u64, 1024),
        Err(GzipReadError::DecodedLimit { limit: 1024 })
    ));
}
