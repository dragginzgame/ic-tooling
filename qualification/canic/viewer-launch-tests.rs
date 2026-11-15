use super::*;
use std::{
    fs,
    time::{Duration, Instant},
};

#[test]
fn opener_handoff_keeps_the_child_alive_and_passes_one_literal_url() {
    let evidence = std::path::PathBuf::from(std::env::var_os("CANIC_VIEWER_EVIDENCE").unwrap());
    let url = "http://127.0.0.1:1234/?literal=$(false);&value=two words";
    let mut command = Command::new("/bin/sh");
    command
        .args([
            "-c",
            r#"
        evidence=$0
        test "$#" = 1 || exit 91
        count=0
        while test ! -f "$evidence/continue"; do
            count=$((count + 1))
            if test "$count" -ge 200; then
                printf expired > "$evidence/expired"
                exit 92
            fi
            sleep 0.01
        done
        printf '%s' "$1" > "$evidence/observed-url"
    "#,
        ])
        .arg(&evidence);
    launch(url, command).unwrap();
    // A synchronous wait would let the fixture expire before this gate opens.
    assert!(!evidence.join("expired").exists());
    fs::write(evidence.join("continue"), b"continue after handoff").unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if fs::read(evidence.join("observed-url")).ok().as_deref() == Some(url.as_bytes()) {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "handed-off opener did not finish"
        );
        std::thread::sleep(Duration::from_millis(5));
    }
    assert!(!evidence.join("expired").exists());
}

#[test]
fn missing_opener_preserves_the_spawn_error() {
    let evidence = std::path::PathBuf::from(std::env::var_os("CANIC_VIEWER_EVIDENCE").unwrap());
    let missing = evidence.join("missing-opener");
    assert!(!missing.exists());
    let error = launch("http://127.0.0.1:1234/", Command::new(missing)).unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::NotFound);
}
