//! Installed management and IC packet compatibility through the Testkit client.
#![cfg(not(target_family = "wasm"))]
mod snapshot;

use ic_testkit::pic::PocketIcStartupConfig;
use ic_testkit::pocket_ic::{CanisterSettings, PocketIcBuilder};
use std::time::Duration;

#[tokio::test]
#[ignore = "requires an explicitly admitted PocketIC 16.1 server"]
async fn installed_client_preserves_management_and_snapshot_state() {
    let config = PocketIcStartupConfig::from_env(Duration::from_secs(60)).unwrap();
    let (server, url) = match config {
        PocketIcStartupConfig::Connect { server_url, .. } => (None, server_url),
        spawn @ PocketIcStartupConfig::Spawn { .. } => {
            let server = spawn.start_managed_server().unwrap();
            let url = server.url().to_owned();
            (Some(server), url)
        }
    };
    let pic = PocketIcBuilder::new()
        .with_server_url(url.parse().unwrap())
        .with_max_request_time_ms(Some(60_000))
        .with_application_subnet()
        .build()
        .await
        .unwrap();
    let id = pic.create_canister().await.unwrap();
    let subnet = pic.get_subnet(id).await.unwrap().unwrap();
    let metrics = pic.get_subnet_metrics(subnet).await.unwrap();
    assert!(
        metrics.is_some(),
        "application subnet exposes certified metrics"
    );
    let before = pic.canister_status(id, None).await.unwrap();
    let minimum = before.settings.minimum_incoming_canister_call_cycles;
    let visibility = before.settings.status_visibility;
    pic.update_canister_settings(
        id,
        None,
        CanisterSettings {
            freezing_threshold: Some(172_800_u64.into()),
            minimum_incoming_canister_call_cycles: Some(minimum.clone()),
            status_visibility: Some(visibility.clone()),
            ..CanisterSettings::default()
        },
    )
    .await
    .unwrap();
    let after = pic.canister_status(id, None).await.unwrap();
    assert_eq!(after.settings.freezing_threshold, 172_800_u64);
    assert_eq!(
        after.settings.minimum_incoming_canister_call_cycles,
        minimum
    );
    assert_eq!(after.settings.status_visibility, visibility);
    // A complete empty Wasm module is enough to qualify snapshot management.
    pic.install_canister(id, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await
        .unwrap();
    pic.stop_canister(id, None).await.unwrap();
    let snapshot = pic.take_canister_snapshot(id, None, None).await.unwrap();
    let listed = pic.list_canister_snapshots(id, None).await.unwrap();
    assert!(listed.iter().any(|item| item.id == snapshot.id));
    pic.load_canister_snapshot(id, None, snapshot.id.clone())
        .await
        .unwrap();
    pic.delete_canister_snapshot(id, None, snapshot.id)
        .await
        .unwrap();
    assert_eq!(
        pic.list_canister_snapshots(id, None).await.unwrap(),
        [] as [ic_management_canister_types::Snapshot; 0]
    );
    pic.start_canister(id, None).await.unwrap();
    pic.canister_status(id, None).await.unwrap();
    snapshot::qualify(&pic).await;
    pic.drop().await.unwrap();
    drop(server);
    println!(
        "Direct PocketIC owner: management 0.11, certified metrics and snapshot lifecycle passed"
    );
}
