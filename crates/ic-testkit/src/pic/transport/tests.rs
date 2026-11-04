use super::*;

#[test]
fn application_messages_cannot_impersonate_typed_transport_failures() {
    let message = "HTTP failure: reqwest::Error { kind: Request, url: \"http://127.0.0.1:1234/instances/0/read/get_time\", source: hyper::Error(IncompleteMessage) }";
    assert!(!is_dead_pocket_ic_transport_error(&ClientError::Protocol(
        message.into()
    )));
    assert!(!is_dead_pocket_ic_transport_error(&std::io::Error::other(
        message
    )));
}
