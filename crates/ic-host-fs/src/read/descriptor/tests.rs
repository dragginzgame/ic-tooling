use super::*;
use crate::test_support::fixture;
use ic_host_artifacts::artifact::Sha256Digest;
use std::{
    fs::{self, File},
    io::{Seek, SeekFrom},
};

#[test]
fn borrowed_descriptor_keeps_inode_and_advances_shared_offset() {
    let fixture = fixture();
    let path = fixture.path().join("selected");
    fs::write(&path, b"prefix-content").unwrap();
    let mut file = File::open(&path).unwrap();
    file.seek(SeekFrom::Start(7)).unwrap();
    fs::rename(&path, fixture.path().join("moved")).unwrap();
    fs::write(&path, b"replacement").unwrap();

    let identity = hash_descriptor(&file, 7).unwrap();
    assert_eq!(identity.sha256, Sha256Digest::compute(b"content"));
    assert_eq!(file.stream_position().unwrap(), 14);
    assert_eq!(
        hash_descriptor(&file, 0).unwrap().sha256,
        Sha256Digest::compute(b"")
    );
    file.rewind().unwrap();
    assert!(matches!(
        hash_descriptor(&file, 0),
        Err(ArtifactError::LimitExceeded { limit: 0 })
    ));
    assert_eq!(file.stream_position().unwrap(), 1);
    file.rewind().unwrap();
    assert!(matches!(
        hash_descriptor(&file, 6),
        Err(ArtifactError::LimitExceeded { limit: 6 })
    ));
    assert_eq!(file.stream_position().unwrap(), 7);
    file.rewind().unwrap();
    assert_eq!(
        hash_descriptor(&file, 14).unwrap().sha256,
        Sha256Digest::compute(b"prefix-content")
    );
}

#[test]
fn borrowed_descriptor_preserves_native_read_error_and_handle() {
    let fixture = fixture();
    let file = File::create(fixture.path().join("write-only")).unwrap();
    let Err(ArtifactError::Io(error)) = hash_descriptor(&file, 16) else {
        panic!("write-only descriptor must refuse reads");
    };
    assert_eq!(
        error.raw_os_error(),
        Some(rustix::io::Errno::BADF.raw_os_error())
    );
    assert!(file.metadata().unwrap().is_file());
}
