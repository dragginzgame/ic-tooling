use super::*;

impl PreparedServer {
    pub(in crate::server) fn fixture(path: &Path) -> Self {
        let mut file = File::open(path.join(BINARY)).unwrap();
        let identity =
            ic_host_artifacts::copy(&mut file, io::sink(), Length::AtMost(BINARY_LIMIT as u64))
                .unwrap();
        // Test-local selection only; no public API admits arbitrary executables.
        let digest = Box::leak(hex(identity).into_boxed_str());

        Self::check_for(
            path,
            Asset {
                url: "test",
                archive_sha256: "test",
                binary_sha256: digest,
            },
        )
        .unwrap()
    }
}
