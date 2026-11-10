//! Test-only worker custody; each scenario owns its protocol and response policy.
use ic_host_process::testing::{
    http::{HttpCapture, HttpContentLength, HttpFixtureLimits, capture_http_request},
    net::accept_tcp_until,
};
use std::{
    io,
    net::{Shutdown, TcpListener, TcpStream},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub struct Peer {
    url: url::Url,
    stop: Arc<AtomicBool>,
    active: Arc<Mutex<Option<TcpStream>>>,
    worker: Option<JoinHandle<()>>,
}

/// Borrow the already-admitted request without repeating framing reads.
pub struct Request<'a> {
    pub path: &'a str,
    pub body: &'a [u8],
}

impl<'a> Request<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let end = bytes
            .windows(4)
            .position(|part| part == b"\r\n\r\n")
            .unwrap()
            + 4;
        let line = std::str::from_utf8(&bytes[..end])
            .unwrap()
            .lines()
            .next()
            .unwrap();
        Self {
            path: line.split_whitespace().nth(1).unwrap(),
            body: &bytes[end..],
        }
    }
}

impl Peer {
    pub(crate) fn spawn(
        request_count: Option<usize>,
        quiet: Duration,
        limits: HttpFixtureLimits,
        mut request: impl FnMut(&mut TcpStream, HttpCapture, Instant, &AtomicBool) + Send + 'static,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap())
            .parse()
            .unwrap();
        let deadline = Instant::now().checked_add(limits.timeout).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let active = Arc::new(Mutex::new(None));
        let worker_stop = Arc::clone(&stop);
        let worker_active = Arc::clone(&active);
        let worker = thread::spawn(move || {
            let mut completed = 0;
            while !worker_stop.load(Ordering::Acquire)
                && request_count.is_none_or(|count| completed < count)
            {
                assert!(Instant::now() < deadline, "HTTP peer timed out");
                let accept_deadline = deadline.min(Instant::now() + Duration::from_millis(20));
                let (mut stream, _) = match accept_tcp_until(&listener, accept_deadline) {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == io::ErrorKind::TimedOut => continue,
                    Err(error) => panic!("HTTP peer accept failed: {error}"),
                };
                *worker_active.lock().unwrap() = Some(stream.try_clone().unwrap());
                let bytes = capture_http_request(
                    &mut stream,
                    deadline.saturating_duration_since(Instant::now()),
                    limits.max_header_bytes,
                    limits.max_body_bytes,
                    HttpContentLength::Optional,
                );
                if worker_stop.load(Ordering::Acquire) {
                    break;
                }
                stream
                    .set_write_timeout(Some(deadline.saturating_duration_since(Instant::now())))
                    .unwrap();
                request(&mut stream, bytes, deadline, &worker_stop);
                *worker_active.lock().unwrap() = None;
                completed += 1;
            }
            {
                let quiet_until = deadline.min(Instant::now() + quiet);
                while !worker_stop.load(Ordering::Acquire) && Instant::now() < quiet_until {
                    let until = quiet_until.min(Instant::now() + Duration::from_millis(20));
                    match accept_tcp_until(&listener, until) {
                        Ok(_) => panic!("unexpected extra request or replay"),
                        Err(error) if error.kind() == io::ErrorKind::TimedOut => {}
                        Err(error) => panic!("HTTP peer quiet observation failed: {error}"),
                    }
                }
            }
        });
        Self {
            url,
            stop,
            active,
            worker: Some(worker),
        }
    }

    pub(crate) const fn url(&self) -> &url::Url {
        &self.url
    }

    pub(crate) fn finish(mut self) {
        self.worker
            .take()
            .unwrap()
            .join()
            .expect("HTTP peer worker failed");
    }
}

impl Drop for Peer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);
        if let Some(stream) = self.active.lock().unwrap().as_ref() {
            let _ = stream.shutdown(Shutdown::Both);
        }
        if let Some(worker) = self.worker.take() {
            let result = worker.join();
            if !thread::panicking() {
                result.expect("HTTP peer worker failed");
            }
        }
    }
}
