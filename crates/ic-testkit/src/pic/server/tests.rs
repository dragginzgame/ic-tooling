#[test]
fn protocol_admission_is_independent_of_download_selection() {
    for version in ["16.0.0", "16.0.1", "16.1.0", "16.2.3"] {
        assert!(super::supports_pocket_ic_server(
            format!("pocket-ic-server {version}\n").as_bytes()
        ));
    }
    for output in [
        "pocket-ic-server 15.0.0",
        "pocket-ic-server 17.0.0",
        "pocket-ic-server 16.0",
        "pocket-ic-server 16.1.0.0",
        "pocket-ic-server 16.01.0",
        "pocket-ic-server 16.1.00",
        "pocket-ic-server 16.1.0-extra",
        "pocket-ic-server 16.18446744073709551616.0",
        "other 16.1.0",
    ] {
        assert!(!super::supports_pocket_ic_server(output.as_bytes()));
    }
    assert!(!super::supports_pocket_ic_server(b"\xff"));
}
