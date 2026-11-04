//! Private filesystem fixtures.

pub fn fixture() -> crate::testing::FixtureDirectory {
    crate::testing::FixtureDirectory::new(&std::env::temp_dir(), "ic-host-fs-test")
        .expect("create test fixture")
}
