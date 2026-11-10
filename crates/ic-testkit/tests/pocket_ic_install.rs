//! Installation uses the upstream client's panic-based failure contract.
mod support;
use ic_testkit::pic::{InstallSpec, StandaloneCanisterFixture, create_and_install};

fn invalid_module() -> InstallSpec {
    InstallSpec {
        wasm: vec![0xde, 0xad],
        init_bytes: vec![],
        cycles: 0,
        install_sender: None,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn borrowed_install_panic_leaves_the_callers_instance_usable() {
    let owner = support::pocket_ic().await;
    let pic = owner.instance();
    let panic =
        support::catch_async_panic(async || create_and_install(pic, invalid_module()).await);
    assert!(
        panic.is_err(),
        "upstream installation must propagate its panic"
    );
    assert!(!pic.topology().await.subnet_configs.is_empty());
    owner.close().await;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_install_panic_releases_owned_server_custody() {
    let owner = support::pocket_ic().await;
    let url = owner.instance().get_server_url();
    let managed = owner.server().is_some();
    let panic = support::catch_async_panic(async move || {
        StandaloneCanisterFixture::install(owner, invalid_module()).await
    });
    assert!(panic.is_err());
    if managed {
        assert!(
            std::net::TcpStream::connect((
                url.host_str().unwrap(),
                url.port_or_known_default().unwrap()
            ))
            .is_err()
        );
    }
}
