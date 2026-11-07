//! Installed clients leave logging environment and process-wide setup to callers.
use super::start_server;
use ic_host_process::tool::{OutputLimit, OutputLimits, capture_command};
use ic_logging::{EntryInput, Filter, Level, Limits, Log, PageRequest};
use ic_testkit::pocket_ic::PocketIc;
use std::{
    num::NonZeroUsize,
    path::Path,
    process::Command,
    sync::{Arc, Mutex},
    time::Duration,
};
use tracing::{Event, Subscriber};
use tracing_subscriber::{Layer, layer::Context, prelude::*};

// This application selects the sink, limits, labels and clock. Imported tracing
// compatibility does not install a subscriber or logging engine in Testkit.
struct ApplicationLog(Arc<Mutex<Log>>);
impl<S: Subscriber> Layer<S> for ApplicationLog {
    fn on_event(&self, event: &Event<'_>, _: Context<'_, S>) {
        if event
            .metadata()
            .target()
            .starts_with("ic_testkit::pocket_ic")
        {
            self.0
                .lock()
                .unwrap()
                .append(EntryInput {
                    created_at_secs: 1,
                    level: Level::Debug,
                    source: "pocket-ic",
                    topic: None,
                    message: event.metadata().name(),
                })
                .unwrap();
        }
    }
}

#[test]
#[ignore = "requires an explicitly admitted PocketIC 16.1 server"]
fn clients_preserve_application_logging_and_ignore_legacy_environment() {
    let (_server, url) = start_server();
    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        Path::new(env!("CARGO_TARGET_TMPDIR")),
        "client-logging",
    )
    .unwrap();
    let blocked = fixture.path().join("regular-file");
    std::fs::write(&blocked, b"caller-owned bytes").unwrap();
    let absent = fixture.path().join("must-not-be-created");
    for (mode, path) in [("early", &blocked), ("late", &absent)] {
        // Environment and global subscriber tests require separate processes;
        // no unsafe environment changes race the Tokio/test-runner threads.
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--ignored",
                "--exact",
                "logging::logging_child",
                "--nocapture",
            ])
            .env("IC_TESTKIT_LOGGING_URL", &url)
            .env("IC_TESTKIT_LOGGING_MODE", mode)
            .env("POCKET_IC_LOG_DIR", path)
            .env("POCKET_IC_LOG_DIR_LEVELS", "invalid[filter");
        capture_command(
            &mut command,
            OutputLimits {
                stdout: OutputLimit::Terminate(64 * 1024),
                stderr: OutputLimit::Terminate(64 * 1024),
                timeout: Some(Duration::from_secs(60)),
            },
        )
        .unwrap()
        .require_success()
        .unwrap();
    }
    assert_eq!(std::fs::read(blocked).unwrap(), b"caller-owned bytes");
    assert!(!absent.exists());
}

#[tokio::test]
#[ignore = "isolated child of the installed logging owner test"]
async fn logging_child() {
    // The aggregate installed gate also selects ignored tests. Only the parent
    // supplies this process-isolation hook; its two real child modes own proof.
    let Some(url) = std::env::var_os("IC_TESTKIT_LOGGING_URL") else {
        return;
    };
    let url = url
        .into_string()
        .expect("parent supplies a UTF-8 server URL")
        .parse()
        .unwrap();
    let early = std::env::var("IC_TESTKIT_LOGGING_MODE").unwrap() == "early";
    let log = Arc::new(Mutex::new(Log::new(Limits {
        max_entries: NonZeroUsize::new(64).unwrap(),
        max_message_bytes: 256,
        max_label_bytes: 32,
        max_text_bytes: 16 * 1024,
    })));
    let install = || {
        tracing::subscriber::set_global_default(
            tracing_subscriber::registry().with(ApplicationLog(log.clone())),
        )
        .unwrap();
    };
    if early {
        install();
    }
    let first = PocketIc::new(url, std::time::Duration::from_secs(60))
        .await
        .unwrap();
    let borrowed = PocketIc::new_from_existing_instance(
        first.get_server_url(),
        first.instance_id,
        std::time::Duration::from_secs(5),
    )
    .expect("valid request budget");
    let second = PocketIc::new(first.get_server_url(), std::time::Duration::from_secs(60))
        .await
        .unwrap();
    borrowed.drop().await.unwrap();
    first.topology().await.unwrap(); // borrowed close did not delete its owner.
    second.drop().await.unwrap();
    first.drop().await.unwrap();
    if !early {
        install();
    } // constructors must not occupy global logging.
    tracing::info!(target: "ic_testkit::pocket_ic::application", "caller-selected logging");
    let rows = log
        .lock()
        .unwrap()
        .page(
            Filter::default(),
            PageRequest {
                before: None,
                max_results: NonZeroUsize::new(64).unwrap(),
                max_scanned: NonZeroUsize::new(64).unwrap(),
            },
        )
        .entries
        .len();
    assert!(rows >= if early { 3 } else { 1 });
}
