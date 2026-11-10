//! Consumer qualification of upstream `PocketIC` through Testkit's server custody.
use ic_testkit::pic::{
    CanisterSnapshotTarget, PocketIcStartupConfig, SnapshotRestoreFunding, capture_snapshots,
    restore_snapshots,
};
use pocket_ic::PocketIcBuilder;
use std::time::Duration;

#[tokio::test]
#[ignore = "requires an explicitly admitted PocketIC 16.1 server"]
async fn upstream_client_installs_and_restores_stable_memory() {
    let config = PocketIcStartupConfig::from_env(Duration::from_secs(30)).unwrap();
    let owned = config
        .build_instance(
            PocketIcBuilder::new()
                .with_max_request_time_ms(Some(30_000))
                .with_application_subnet(),
        )
        .await
        .unwrap();
    let pic = owned.instance();
    let id = ic_testkit::pic::create_and_install(
        pic,
        ic_testkit::pic::InstallSpec {
            wasm: b"\0asm\x01\0\0\0".to_vec(),
            init_bytes: vec![],
            cycles: 5_000_000_000_000,
            install_sender: None,
        },
    )
    .await;
    let status: pocket_ic::CanisterStatusResult = pic.canister_status(id, None).await.unwrap();
    assert!(status.module_hash.is_some());
    let subnet = pic.get_subnet(id).await.unwrap();
    assert!(pic.get_subnet_metrics(subnet).await.is_some());
    pic.stop_canister(id, None).await.unwrap();
    pic.set_stable_memory(
        id,
        b"seed".to_vec(),
        pocket_ic::common::rest::BlobCompression::NoCompression,
    )
    .await;
    let snapshots = capture_snapshots(
        pic,
        [CanisterSnapshotTarget {
            canister_id: id,
            sender: None,
        }],
    )
    .await
    .unwrap();
    assert_eq!(snapshots.canister_ids().collect::<Vec<_>>(), [id]);
    for _ in 0..2 {
        pic.set_stable_memory(
            id,
            b"next".to_vec(),
            pocket_ic::common::rest::BlobCompression::NoCompression,
        )
        .await;
        restore_snapshots(pic, &snapshots, SnapshotRestoreFunding::Preserve)
            .await
            .unwrap();
        assert!(pic.get_stable_memory(id).await.starts_with(b"seed"));
    }
    owned.close().await;
}
