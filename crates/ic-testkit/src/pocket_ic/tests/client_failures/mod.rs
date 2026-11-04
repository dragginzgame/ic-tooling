use crate::pocket_ic::{ClientError, PocketIc, nonblocking};
use std::{
    io::{Read, Write},
    net::TcpListener,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

struct Step {
    path: &'static str,
    status: u16,
    body: &'static str,
    extra_length: usize,
    stall: bool,
}
impl Step {
    fn reply(path: &'static str, status: u16, body: &'static str) -> Self {
        Self {
            path,
            status,
            body,
            extra_length: 0,
            stall: false,
        }
    }
}
struct Server {
    url: reqwest::Url,
    requests: Arc<Mutex<Vec<String>>>,
    thread: thread::JoinHandle<()>,
}
impl Server {
    fn new(steps: Vec<Step>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/", listener.local_addr().unwrap())
            .parse()
            .unwrap();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let recorded = requests.clone();
        let thread = thread::spawn(move || {
            let deadline = Instant::now() + Duration::from_secs(5);
            for step in steps {
                let mut socket = loop {
                    match listener.accept() {
                        Ok((socket, _)) => break socket,
                        Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                            assert!(Instant::now() < deadline, "missing request {}", step.path);
                            thread::sleep(Duration::from_millis(1));
                        }
                        Err(error) => panic!("{error}"),
                    }
                };
                // Accepted sockets can inherit the listener's nonblocking mode
                // on macOS; the fixture reads each request with bounded blocking I/O.
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut byte = [0];
                while !bytes.ends_with(b"\r\n\r\n") {
                    socket.read_exact(&mut byte).unwrap();
                    bytes.push(byte[0]);
                }
                let header = String::from_utf8(bytes).unwrap();
                let length = header
                    .lines()
                    .find_map(|line| {
                        let (name, value) = line.split_once(':')?;
                        name.eq_ignore_ascii_case("content-length")
                            .then(|| value.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                let mut body = vec![0; length];
                socket.read_exact(&mut body).unwrap();
                assert_eq!(
                    header
                        .lines()
                        .next()
                        .unwrap()
                        .split_whitespace()
                        .nth(1)
                        .unwrap(),
                    step.path
                );
                recorded.lock().unwrap().push(format!(
                    "{}\n{}",
                    header.lines().next().unwrap(),
                    String::from_utf8_lossy(&body)
                ));
                if step.stall {
                    thread::sleep(Duration::from_millis(150));
                    continue;
                }
                if step.status == 0 {
                    continue;
                }
                let response = format!(
                    "HTTP/1.1 {} Test\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    step.status,
                    step.body.len() + step.extra_length,
                    step.body
                );
                socket.write_all(response.as_bytes()).unwrap();
            }
            let quiet_until = Instant::now() + Duration::from_millis(100);
            while Instant::now() < quiet_until {
                match listener.accept() {
                    Ok(_) => panic!("unexpected extra request or replay"),
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1));
                    }
                    Err(error) => panic!("{error}"),
                }
            }
        });
        Self {
            url,
            requests,
            thread,
        }
    }
    fn finish(self) -> Vec<String> {
        self.thread.join().unwrap();
        Arc::try_unwrap(self.requests)
            .unwrap()
            .into_inner()
            .unwrap()
    }
}

#[test]
fn missing_server_is_refused_before_any_external_effect() {
    assert!(matches!(
        crate::pocket_ic::PocketIcBuilder::new().build(),
        Err(ClientError::MissingServerUrl)
    ));
    let runtime = tokio::runtime::Runtime::new().unwrap();
    assert!(matches!(
        runtime.block_on(crate::pocket_ic::PocketIcBuilder::new().build_async()),
        Err(ClientError::MissingServerUrl)
    ));
}

#[test]
fn instance_listing_uses_only_the_supplied_endpoint() {
    for asynchronous in [false, true] {
        let server = Server::new(vec![Step::reply(
            "/instances",
            200,
            r#"["Available","Busy"]"#,
        )]);
        let instances = if asynchronous {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(nonblocking::PocketIc::list_instances(server.url.clone()))
        } else {
            PocketIc::list_instances(server.url.clone())
        }
        .unwrap();
        assert_eq!(instances, ["Available", "Busy"]);
        assert_eq!(server.finish().len(), 1);
    }
}

#[test]
fn sync_and_async_preserve_failure_sources_and_never_resubmit_lost_requests() {
    for asynchronous in [false, true] {
        for failure in ["reset", "body", "json", "server", "timeout"] {
            let mut step = Step::reply("/instances/7/update/tick", 200, "null");
            match failure {
                "reset" => step.status = 0,
                "body" => step.extra_length = 20,
                "json" => step.body = "{",
                "server" => {
                    step.status = 503;
                    step.body = r#"{"message":"busy provider"}"#;
                }
                "timeout" => step.stall = true,
                _ => unreachable!(),
            }
            let server = Server::new(vec![step]);
            let error = if asynchronous {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                let client = nonblocking::PocketIc::new_from_existing_instance(
                    server.url.clone(),
                    7,
                    Some(50),
                );
                runtime.block_on(client.tick()).unwrap_err()
            } else {
                let client = PocketIc::new_from_existing_instance(server.url.clone(), 7, Some(50));
                client.tick().unwrap_err()
            };
            match (&error, failure) {
                (ClientError::Transport { .. }, "reset")
                | (ClientError::Body { .. }, "body")
                | (ClientError::Json { .. }, "json")
                | (ClientError::Timeout { .. }, "timeout") => {}
                (
                    ClientError::Server {
                        status, message, ..
                    },
                    "server",
                ) => {
                    assert_eq!(status.as_u16(), 503);
                    assert_eq!(message, "busy provider");
                }
                _ => panic!("wrong failure {failure}: {error:?}"),
            }
            let context = error.request_context().unwrap();
            assert!(context.may_have_executed);
            assert!(context.operation.is_none());
            if matches!(failure, "reset" | "body" | "json") {
                assert!(std::error::Error::source(&error).is_some());
            }
            assert_eq!(server.finish().len(), 1);
        }
    }
}

#[tokio::test]
async fn accepted_operation_is_polled_without_replaying_the_update() {
    let server = Server::new(vec![
        Step::reply(
            "/instances/7/update/tick",
            409,
            r#"{"state_label":"other","op_id":"busy"}"#,
        ),
        Step::reply(
            "/instances/7/update/tick",
            202,
            r#"{"state_label":"state","op_id":"ours"}"#,
        ),
        Step::reply("/read_graph/state/ours", 404, "pending"),
        Step::reply("/read_graph/state/ours", 200, "null"),
        Step::reply("/prune_graph/state/ours", 200, "null"),
    ]);
    let client =
        nonblocking::PocketIc::new_from_existing_instance(server.url.clone(), 7, Some(1000));
    client.tick().await.unwrap();
    let requests = tokio::task::spawn_blocking(move || server.finish())
        .await
        .unwrap();
    assert_eq!(requests.len(), 5);
    assert_eq!(requests[0], requests[1]);
    assert!(requests[2].starts_with("GET "));
    assert!(requests[3].starts_with("GET "));
    assert!(requests[4].starts_with("DELETE "));
}

#[tokio::test]
async fn lost_poll_retains_our_operation_identity() {
    let server = Server::new(vec![
        Step::reply(
            "/instances/7/update/tick",
            202,
            r#"{"state_label":"state","op_id":"ours"}"#,
        ),
        Step::reply("/read_graph/state/ours", 0, ""),
    ]);
    let client =
        nonblocking::PocketIc::new_from_existing_instance(server.url.clone(), 7, Some(1000));
    let error = client.tick().await.unwrap_err();
    let operation = error.request_context().unwrap().operation.as_ref().unwrap();
    assert_eq!(operation.state_label, "state");
    assert_eq!(operation.op_id, "ours");
    assert_eq!(server.finish().len(), 2);
}

#[test]
fn rejection_remains_distinct_from_transport_failure() {
    let server = Server::new(vec![Step::reply(
        "/instances/7/read/query",
        200,
        r#"{"Err":{"reject_code":"CanisterReject","reject_message":"caller policy","error_code":"CanisterRejectedMessage","certified":false}}"#,
    )]);
    let client = PocketIc::new_from_existing_instance(server.url.clone(), 7, Some(1000));
    let id = candid::Principal::anonymous();
    let ClientError::Canister(rejection) = client.query_call(id, id, "reject", vec![]).unwrap_err()
    else {
        panic!("expected canister rejection")
    };
    assert_eq!(rejection.reject_message, "caller policy");
    assert_eq!(server.finish().len(), 1);
}

#[test]
fn direct_routes_and_failed_deletion_are_fallible_without_drop_replay() {
    let server = Server::new(vec![
        Step::reply(
            "/instances",
            200,
            r#"{"Created":{"instance_id":7,"topology":{"subnet_configs":{},"default_effective_canister_id":{"canister_id":""}},"http_gateway_info":null}}"#,
        ),
        Step::reply("/blobstore/", 200, "00ff"),
        Step::reply("/blobstore/", 200, "not hex"),
        Step::reply("/instances/7/", 0, ""),
    ]);
    let mut client = crate::pocket_ic::PocketIcBuilder::new()
        .with_server_url(server.url.clone())
        .build()
        .unwrap();
    let blob = client
        .upload_blob(
            vec![0, 255],
            crate::pocket_ic::common::rest::BlobCompression::NoCompression,
        )
        .unwrap();
    assert_eq!(blob.0, [0, 255]);
    assert!(matches!(
        client.upload_blob(
            vec![],
            crate::pocket_ic::common::rest::BlobCompression::NoCompression
        ),
        Err(ClientError::Protocol(_))
    ));
    assert!(matches!(
        client.drop_and_take_state(),
        Err(ClientError::Transport { .. })
    ));
    drop(client);
    let requests = server.finish();
    assert_eq!(requests.len(), 4);
    assert!(requests[3].starts_with("DELETE "));
}

#[tokio::test]
async fn creation_decode_and_poll_deadline_keep_request_evidence() {
    let server = Server::new(vec![Step::reply("/instances", 200, "{")]);
    let error = crate::pocket_ic::PocketIcBuilder::new()
        .with_server_url(server.url.clone())
        .build_async()
        .await
        .err()
        .unwrap();
    assert!(matches!(error, ClientError::Json { .. }));
    assert!(error.request_context().unwrap().may_have_executed);
    server.finish();
    let mut delayed = Step::reply("/read_graph/state/ours", 200, "null");
    delayed.stall = true;
    let server = Server::new(vec![
        Step::reply(
            "/instances/7/update/tick",
            202,
            r#"{"state_label":"state","op_id":"ours"}"#,
        ),
        delayed,
    ]);
    let client = nonblocking::PocketIc::new_from_existing_instance(server.url.clone(), 7, Some(50));
    let error = client.tick().await.unwrap_err();
    assert!(matches!(error, ClientError::Timeout { .. }));
    assert_eq!(
        error
            .request_context()
            .unwrap()
            .operation
            .as_ref()
            .unwrap()
            .op_id,
        "ours"
    );
    assert_eq!(server.finish().len(), 2);
}

#[tokio::test]
async fn ingress_id_survives_loss_after_successful_submission() {
    let server = Server::new(vec![
        Step::reply(
            "/instances/7/update/submit_ingress_message",
            200,
            r#"{"Ok":{"effective_principal":{"CanisterId":"BA=="},"message_id":"AQI="}}"#,
        ),
        Step::reply("/instances/7/update/await_ingress_message", 0, ""),
    ]);
    let client =
        nonblocking::PocketIc::new_from_existing_instance(server.url.clone(), 7, Some(1000));
    let id = candid::Principal::anonymous();
    let error = client
        .update_call(id, id, "once", vec![7])
        .await
        .unwrap_err();
    let ingress = error.request_context().unwrap().ingress.as_ref().unwrap();
    assert_eq!(ingress.message_id, [1, 2]);
    assert_eq!(
        ingress.effective_principal,
        crate::pocket_ic::common::rest::RawEffectivePrincipal::CanisterId(vec![4])
    );
    let requests = server.finish();
    assert_eq!(requests.len(), 2);
    assert!(requests[1].contains(r#""message_id":"AQI=""#));
}

mod snapshot;
