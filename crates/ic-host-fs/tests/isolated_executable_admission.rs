//! A single libtest case keeps descriptors out of unrelated test forks.
#![cfg(unix)]

use ic_host_fs::durable::{PublicationMode, WriteOptions, write_validated_with};
use ic_host_fs::testing::FixtureDirectory;
use std::{fs, io};

const REPLACE: WriteOptions = WriteOptions {
    mode: PublicationMode::Replace,
    permissions: 0o600,
};

#[test]
fn closed_writer_admission_executes_before_replacing_the_destination() {
    let fixture =
        FixtureDirectory::new(&std::env::temp_dir(), "isolated-executable-admission").unwrap();
    let output = fixture.path().join("executable with spaces");
    fs::write(&output, b"old").unwrap();
    let value = write_validated_with(
        &output,
        WriteOptions {
            permissions: 0o700,
            ..REPLACE
        },
        |file| -> io::Result<_> {
            let mut source = fs::File::open("/bin/sh")?;
            io::copy(&mut source, file)?;
            Ok(42)
        },
        |stage, value| -> io::Result<()> {
            assert!(stage.is_absolute());
            assert_eq!(*value, 42);
            assert_eq!(fs::read(&output)?, b"old");
            assert!(
                std::process::Command::new(stage)
                    .args(["-c", "exit 0"])
                    .status()?
                    .success()
            );
            Ok(())
        },
    )
    .unwrap();
    assert_eq!(value, 42);
    assert!(
        std::process::Command::new(&output)
            .args(["-c", "exit 0"])
            .status()
            .unwrap()
            .success()
    );
    assert_eq!(fs::read_dir(fixture.path()).unwrap().count(), 1);
}
