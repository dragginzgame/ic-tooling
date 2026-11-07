//! Installed management and IC packet compatibility through the Testkit client.
#![cfg(not(target_family = "wasm"))]
mod logging;
mod snapshot;

use ic_testkit::pic::{PocketIcManagedServer, PocketIcStartupConfig};
use ic_testkit::pocket_ic::{CanisterSettings, PocketIcBuilder};
use std::time::Duration;

#[tokio::test]
#[ignore = "requires an explicitly admitted PocketIC 16.1 server"]
async fn installed_client_preserves_management_and_snapshot_state() {
    let (server, url) = start_server();
    let pic = PocketIcBuilder::new(std::time::Duration::from_secs(60))
        .with_server_url(url.parse().unwrap())
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

fn start_server() -> (Option<PocketIcManagedServer>, String) {
    let config = PocketIcStartupConfig::from_env(Duration::from_secs(60)).unwrap();
    match config {
        PocketIcStartupConfig::Connect { server_url, .. } => (None, server_url),
        spawn @ PocketIcStartupConfig::Spawn { .. } => {
            let server = spawn.start_managed_server().unwrap();
            let url = server.url().to_owned();
            (Some(server), url)
        }
    }
}

#[tokio::test]
#[ignore = "requires an explicitly admitted PocketIC 16.1 server"]
async fn installed_builder_preserves_features_gateway_and_caller_owned_state() {
    use ic_testkit::pocket_ic::common::rest::{
        IcpFeatures, IcpFeaturesConfig, InstanceHttpGatewayConfig,
    };
    let scratch = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(scratch).unwrap();
    let evidence = ic_host_fs::directory::create_unique_directory(
        scratch,
        "pocket-builder-state",
        0o700,
        None,
    )
    .unwrap();
    let state = evidence.join("instance");
    let (server, url) = start_server();
    let enabled = Some(IcpFeaturesConfig::DefaultConfig);
    let pic = PocketIcBuilder::new(std::time::Duration::from_secs(60))
        .with_server_url(url.parse().unwrap())
        .with_application_subnet()
        .with_state_dir(state.clone())
        .with_icp_features(IcpFeatures {
            registry: enabled.clone(),
            cycles_minting: enabled.clone(),
            icp_token: enabled.clone(),
            cycles_token: enabled.clone(),
            ii: enabled,
        })
        .with_http_gateway(InstanceHttpGatewayConfig {
            ip_addr: Some("127.0.0.1".into()),
            port: Some(0),
            domains: None,
            https_config: None,
            domain_custom_provider_local_file: None,
        })
        .with_auto_progress()
        .build()
        .await
        .unwrap();
    assert!(pic.url().is_some());
    assert!(pic.auto_progress_enabled().await.unwrap());
    let topology = pic.topology().await.unwrap();
    assert!(topology.get_nns().is_some());
    assert!(topology.get_ii().is_some());
    let id = pic.create_canister().await.unwrap();
    pic.canister_status(id, None).await.unwrap();
    pic.drop().await.unwrap();
    assert!(state.join("topology.json").is_file());

    // A new instance resumes the caller's directory without a client state handle.
    let resumed = PocketIcBuilder::new(std::time::Duration::from_secs(60))
        .with_server_url(url.parse().unwrap())
        .with_state_dir(state.clone())
        .build()
        .await
        .unwrap();
    resumed.canister_status(id, None).await.unwrap();
    resumed.drop().await.unwrap();
    assert!(state.join("topology.json").is_file());
    drop(server);
    println!(
        "Persistent instance qualification retained: {}",
        evidence.display()
    );
}
