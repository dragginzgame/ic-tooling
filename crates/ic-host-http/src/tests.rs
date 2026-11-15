use super::{Error, body};
use std::{
    error::Error as _,
    io::{Read, Write},
    net::TcpListener,
    thread,
    time::Duration,
};

fn read(reply: &'static [u8], limit: usize) -> Result<Vec<u8>, Error> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = listener.local_addr().unwrap();
    let server = thread::spawn(move || {
        let (mut socket, _) = listener.accept().unwrap();
        socket
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut request = Vec::new();
        let mut buffer = [0; 1024];
        while !request.windows(4).any(|bytes| bytes == b"\r\n\r\n") {
            let count = socket.read(&mut buffer).unwrap();
            assert_ne!(count, 0);
            request.extend_from_slice(&buffer[..count]);
        }
        socket.write_all(reply).unwrap();
    });
    let result = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let response = reqwest::Client::builder()
                .no_proxy()
                .timeout(Duration::from_secs(3))
                .build()
                .unwrap()
                .get(format!("http://{address}/secret-test"))
                .send()
                .await
                .unwrap();
            body(response, limit).await
        });
    server.join().unwrap();
    result
}

#[test]
fn complete_bodies_preserve_bytes_and_do_not_classify_status() {
    assert_eq!(
        read(b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n", 0).unwrap(),
        b""
    );
    assert_eq!(
        read(
            b"HTTP/1.1 503 Unavailable\r\nContent-Length: 4\r\n\r\na\0bc",
            4
        )
        .unwrap(),
        b"a\0bc"
    );
    assert_eq!(
        read(
            b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\nab\r\n2\r\ncd\r\n0\r\n\r\n",
            4
        )
        .unwrap(),
        b"abcd"
    );
    assert_eq!(
        read(b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\nabcd", 4).unwrap(),
        b"abcd"
    );
}

#[test]
fn excess_and_truncation_never_return_a_prefix() {
    // The absent body cannot produce a transport error before declared-size admission.
    assert!(matches!(
        read(b"HTTP/1.1 200 OK\r\nContent-Length: 999\r\n\r\n", 4),
        Err(Error::TooLarge {
            limit: 4,
            declared: Some(999)
        })
    ));
    assert!(matches!(read(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\nab\r\n3\r\ncde\r\n0\r\n\r\n", 4), Err(Error::TooLarge { limit: 4, declared: None })));
    assert!(matches!(
        read(b"HTTP/1.1 200 OK\r\nConnection: close\r\n\r\nx", 0),
        Err(Error::TooLarge {
            limit: 0,
            declared: None
        })
    ));
    for reply in [
        b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\n\r\nab".as_slice(),
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n2\r\nab\r\n".as_slice(),
    ] {
        let error = read(reply, 4).unwrap_err();
        assert!(matches!(error, Error::Transport(_)));
        assert!(error.source().is_some());
        assert!(!error.to_string().contains("secret-test"));
    }
}
