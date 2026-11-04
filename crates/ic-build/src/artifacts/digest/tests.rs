use super::{
    LabeledPathDigestCache, copy_file_atomic, digest_bytes, digest_file,
    digest_labeled_paths_composable,
};
use crate::testing::fs::unique_temp_directory;
use std::{fs, path::PathBuf};

#[cfg(unix)]
use super::{InputHasher, digest_labeled_paths};
#[cfg(unix)]
use std::{ffi::OsStr, os::unix::ffi::OsStrExt as _};
#[cfg(windows)]
use std::{ffi::OsString, os::windows::ffi::OsStringExt as _};

#[test]
#[cfg(unix)]
fn exclusion_resolution_errors_stop_both_fingerprint_paths() {
    use std::os::unix::fs::symlink;
    let root = unique_temp_directory("invalid-digest-exclusions");
    let input = root.join("input");
    fs::write(&input, b"source").unwrap();
    let cycle = root.join("cycle");
    symlink("cycle", &cycle).unwrap();
    let paths = [(std::path::Path::new("input"), input.as_path())];
    assert!(digest_labeled_paths("exclusions-v1", paths, std::slice::from_ref(&cycle)).is_err());
    let mut cache = LabeledPathDigestCache::default();
    let expected =
        digest_labeled_paths_composable("exclusions-v1", paths, &[], &mut cache).unwrap();
    assert!(digest_labeled_paths_composable("exclusions-v1", paths, &[cycle], &mut cache).is_err());
    // A not-yet-created cache root remains an accepted exclusion, including
    // on a reused source-lease digest. A failed resolution cannot reuse it.
    assert_eq!(
        digest_labeled_paths_composable(
            "exclusions-v1",
            paths,
            &[root.join("missing")],
            &mut cache,
        )
        .unwrap(),
        expected
    );
    assert_eq!(
        digest_labeled_paths("exclusions-v1", paths, &[]).unwrap(),
        digest_labeled_paths("exclusions-v1", paths, &[root.join("missing")]).unwrap()
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn digest_text_preserves_lowercase_hex_and_leading_zeroes() {
    let digest = super::InputDigest(std::array::from_fn(|index| {
        u8::try_from(index).expect("digest byte index must fit")
    }));
    let expected = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    assert_eq!(digest.to_hex(), expected);
    assert_eq!(digest.to_string(), expected);
    assert_eq!(super::InputDigest([0xff; 32]).to_string(), "ff".repeat(32));
}

#[test]
#[cfg(unix)]
fn labeled_path_digests_preserve_native_names_and_sorted_order() {
    let names: &[&[u8]] = &[
        b"\xce\xbb",
        #[cfg(target_os = "linux")]
        b"\xff",
    ];
    for &name in names {
        let root = unique_temp_directory("native-path-digest");
        let tree = root.join("tree");
        fs::create_dir_all(tree.join("nested")).unwrap();
        fs::write(tree.join(OsStr::from_bytes(name)), b"native").unwrap();
        fs::write(tree.join("nested/z"), b"last").unwrap();
        fs::write(tree.join("a"), b"first").unwrap();
        fs::write(root.join("top"), b"top").unwrap();
        let mut paths = [
            (PathBuf::from("tree"), tree),
            (PathBuf::from("aaa"), root.join("top")),
        ];

        let tree_fields = |hasher: &mut InputHasher| {
            hasher.field("directory", b"tree");
            hasher.field("file-path", b"tree/a");
            hasher.field("file-content", b"first");
            hasher.field("directory", b"tree/nested");
            hasher.field("file-path", b"tree/nested/z");
            hasher.field("file-content", b"last");
            hasher.field("file-path", &[b"tree/".as_slice(), name].concat());
            hasher.field("file-content", b"native");
        };
        let mut expected = InputHasher::new("native-path-test-v1");
        expected.field("file-path", b"aaa");
        expected.field("file-content", b"top");
        tree_fields(&mut expected);
        let expected = expected.finish();

        let mut top = InputHasher::new("native-path-test-v1/root-v1");
        top.field("file-path", b"aaa");
        top.field("file-content", b"top");
        let mut tree = InputHasher::new("native-path-test-v1/root-v1");
        tree_fields(&mut tree);
        let mut composable = InputHasher::new("native-path-test-v1/composable-v1");
        composable.field("input-label", b"aaa");
        composable.field("input-digest", top.finish().as_bytes());
        composable.field("input-label", b"tree");
        composable.field("input-digest", tree.finish().as_bytes());
        let composable = composable.finish();

        for _ in 0..2 {
            assert_eq!(
                digest_labeled_paths(
                    "native-path-test-v1",
                    paths.iter().map(|(label, path)| (label, path)),
                    &[],
                )
                .unwrap(),
                expected,
            );
            assert_eq!(
                digest_labeled_paths_composable(
                    "native-path-test-v1",
                    paths
                        .iter()
                        .map(|(label, path)| (label.as_path(), path.as_path())),
                    &[],
                    &mut LabeledPathDigestCache::default(),
                )
                .unwrap(),
                composable,
            );
            paths.reverse();
        }
        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
#[cfg(unix)]
fn native_bytes_preserve_non_utf8_without_a_filesystem_roundtrip() {
    assert_eq!(
        super::os_bytes(OsStr::from_bytes(b"name\xff")).as_ref(),
        b"name\xff"
    );
}

#[test]
#[cfg(windows)]
fn native_names_preserve_utf16_little_endian_encoding() {
    let value = OsString::from_wide(&[0x0061, 0xd800, 0x0100]);
    assert_eq!(super::os_bytes(&value).as_ref(), &[0x61, 0, 0, 0xd8, 0, 1]);
}

#[test]
fn streamed_fields_preserve_bytes_across_different_file_sizes() {
    let root = unique_temp_directory("streamed-field-sizes");
    let source = root.join("source");
    let contents = (0..192 * 1024 + 37)
        .map(|index| u8::try_from(index % 251).unwrap())
        .collect::<Vec<_>>();
    let mut streamed = super::InputHasher::new("streamed-fields-v1");
    let mut expected = super::InputHasher::new("streamed-fields-v1");
    for length in [1, 64 * 1024 - 1, contents.len(), 0, 7, 1024, 64 * 1024 + 1] {
        let bytes = &contents[..length];
        fs::write(&source, bytes).unwrap();
        assert_eq!(streamed.file_field("part", &source).unwrap(), length as u64);
        expected.field("part", bytes);
    }
    assert_eq!(streamed.finish(), expected.finish());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn streaming_digest_and_atomic_copy_preserve_exact_bytes() {
    let root = unique_temp_directory("streaming-digest");
    let source = root.join("source");
    let destination = root.join("destination");
    let mut contents = vec![0_u8; 192 * 1024 + 37];
    for (index, byte) in contents.iter_mut().enumerate() {
        *byte = u8::try_from(index % 251).expect("test byte must fit");
    }
    for length in [
        0,
        1,
        1024,
        16 * 1024,
        64 * 1024 - 1,
        64 * 1024,
        64 * 1024 + 1,
        contents.len(),
    ] {
        let data = &contents[..length];
        fs::write(&source, data).expect("write source");
        let streamed = digest_file("streaming-test-v1", &source).expect("digest file");
        assert_eq!(
            streamed.bytes,
            u64::try_from(length).expect("fixture length must fit in u64")
        );
        assert_eq!(streamed.digest, digest_bytes("streaming-test-v1", data));
    }

    ic_host_fs::durable::write_bytes(
        &destination,
        ic_host_fs::durable::WriteOptions {
            mode: ic_host_fs::durable::PublicationMode::Replace,
            permissions: 0o666,
        },
        b"old",
    )
    .expect("write original destination");
    assert_eq!(
        copy_file_atomic(&source, &destination).expect("copy source atomically"),
        u64::try_from(contents.len()).expect("fixture length must fit in u64")
    );
    assert_eq!(
        fs::read(&destination).expect("read copied destination"),
        contents
    );

    let missing = root.join("missing");
    let error = copy_file_atomic(&missing, &destination).expect_err("missing source must fail");
    let message = error.to_string();
    assert!(message.contains(&missing.display().to_string()));
    assert!(message.contains(&destination.display().to_string()));
    fs::remove_dir_all(root).expect("remove streaming-digest test directory");
}

#[test]
#[cfg(unix)]
fn atomic_copy_retains_publication_phase_and_original_output_on_failure() {
    use ic_host_fs::durable::NamedWriteError;

    let root = unique_temp_directory("atomic-copy-publication-errors");
    let destination = root.join("destination");
    fs::write(&destination, b"retained").unwrap();
    // Opening a directory succeeds on Unix, but copying its bytes fails
    // inside the producer after staging has started.
    let error = copy_file_atomic(&root, &destination).unwrap_err();
    let context = error
        .get_ref()
        .unwrap()
        .downcast_ref::<super::AtomicCopyErrorContext>()
        .unwrap();
    let publication = context
        .source
        .get_ref()
        .unwrap()
        .downcast_ref::<NamedWriteError<std::io::Error>>()
        .unwrap();
    assert!(matches!(
        publication,
        NamedWriteError::Producer {
            cleanup_error: None,
            ..
        }
    ));
    assert_eq!(fs::read(&destination).unwrap(), b"retained");
    assert_eq!(fs::read_dir(&root).unwrap().count(), 1);
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[cfg(unix)]
fn atomic_copy_rejects_invalid_destinations_without_publication() {
    use ic_host_fs::durable::NamedWriteError;

    let root = unique_temp_directory("atomic-copy-directory-suffix");
    let source = root.join("source");
    let destination = root.join("destination");
    fs::write(&source, b"replacement").unwrap();
    fs::write(&destination, b"retained").unwrap();
    for (name, suffix) in [
        ("destination", "/"),
        ("destination", "/."),
        ("missing/child", "/"),
        ("missing/child", "/."),
        ("missing/invalid\0name", ""),
        ("invalid\0parent/child", ""),
    ] {
        let mut raw = root.join(name).into_os_string();
        raw.push(suffix);
        let target = PathBuf::from(raw);
        let error = copy_file_atomic(&source, &target).unwrap_err();
        let context = error
            .get_ref()
            .unwrap()
            .downcast_ref::<super::AtomicCopyErrorContext>()
            .unwrap();
        assert_eq!(context.source_path, source);
        assert_eq!(context.destination_path, target);
        let publication = context
            .source
            .get_ref()
            .unwrap()
            .downcast_ref::<NamedWriteError<std::io::Error>>()
            .unwrap();
        assert!(matches!(
            publication,
            NamedWriteError::BeforePublication { source, cleanup_error: None }
                if source.kind() == std::io::ErrorKind::InvalidInput
        ));
        assert_eq!(fs::read(&destination).unwrap(), b"retained");
        assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[cfg(unix)]
fn atomic_publication_supports_long_destination_names() {
    let root = unique_temp_directory("atomic-long-destination");
    let destination = root.join("a".repeat(255));
    // Establish that the destination itself is valid on this filesystem.
    fs::write(&destination, b"original output").unwrap();
    let source = root.join("source");
    fs::write(&source, b"copied output").unwrap();
    assert_eq!(copy_file_atomic(&source, &destination).unwrap(), 13);
    assert_eq!(fs::read(&destination).unwrap(), b"copied output");
    assert_eq!(fs::read_dir(&root).unwrap().count(), 2);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn composable_digest_reuses_roots_across_irrelevant_exclusion_changes() {
    let root = unique_temp_directory("composable-digest-cache");
    let input = root.join("input");
    fs::create_dir_all(&input).expect("create composable input");
    fs::create_dir_all(root.join("generated-a")).expect("create first generated root");
    fs::create_dir_all(root.join("generated-b")).expect("create second generated root");
    fs::write(input.join("source"), b"source").expect("write composable input");
    let paths = [(PathBuf::from("shared"), input)];
    let mut cache = LabeledPathDigestCache::default();

    let first = digest_labeled_paths_composable(
        "composable-test-v1",
        paths
            .iter()
            .map(|(label, path)| (label.as_path(), path.as_path())),
        &[root.join("generated-a")],
        &mut cache,
    )
    .expect("hash first composable input");
    let second = digest_labeled_paths_composable(
        "composable-test-v1",
        paths
            .iter()
            .map(|(label, path)| (label.as_path(), path.as_path())),
        &[root.join("generated-b")],
        &mut cache,
    )
    .expect("reuse composable input root");

    assert_eq!(first, second);
    assert_eq!(cache.entries.len(), 1);
    fs::remove_dir_all(root).expect("remove composable digest fixture");
}

#[test]
fn composable_digest_rehashes_changed_descendant_exclusions_and_rejects_ancestors() {
    let root = unique_temp_directory("composable-relevant-exclusions");
    let input = root.join("input");
    let generated = input.join("generated");
    fs::create_dir_all(&generated).unwrap();
    fs::write(input.join("source"), b"source").unwrap();
    fs::write(generated.join("artifact"), b"generated").unwrap();
    let paths = [(PathBuf::from("input"), input.clone())];
    let digest = |exclusions: &[PathBuf], cache: &mut LabeledPathDigestCache| {
        digest_labeled_paths_composable(
            "exclusions-test-v1",
            paths
                .iter()
                .map(|(label, path)| (label.as_path(), path.as_path())),
            exclusions,
            cache,
        )
    };
    let mut cache = LabeledPathDigestCache::default();
    let excluded = digest(std::slice::from_ref(&generated), &mut cache).unwrap();
    let included = digest(&[], &mut cache).unwrap();
    assert_ne!(included, excluded);
    assert_eq!(
        included,
        digest(&[], &mut LabeledPathDigestCache::default()).unwrap(),
    );
    for ancestor in [&input, &root] {
        assert_eq!(
            digest(std::slice::from_ref(ancestor), &mut cache)
                .unwrap_err()
                .kind(),
            std::io::ErrorKind::InvalidInput,
        );
    }
    assert_eq!(
        digest(std::slice::from_ref(&generated), &mut cache).unwrap(),
        excluded,
    );
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[cfg(unix)]
fn composable_digest_tracks_exclusions_beyond_an_external_symlink() {
    let root = unique_temp_directory("composable-external-exclusions");
    let input = root.join("input");
    let external = root.join("external");
    fs::create_dir_all(&input).unwrap();
    fs::create_dir_all(external.join("first")).unwrap();
    fs::create_dir_all(external.join("second")).unwrap();
    fs::write(input.join("source"), b"source").unwrap();
    fs::write(external.join("first/file"), b"first").unwrap();
    fs::write(external.join("second/file"), b"second").unwrap();
    std::os::unix::fs::symlink(&external, input.join("linked")).unwrap();
    let paths = [(PathBuf::from("input"), input)];
    let digest = |exclusion: &PathBuf, cache: &mut LabeledPathDigestCache| {
        digest_labeled_paths_composable(
            "external-exclusions-test-v1",
            paths
                .iter()
                .map(|(label, path)| (label.as_path(), path.as_path())),
            std::slice::from_ref(exclusion),
            cache,
        )
    };
    let mut cache = LabeledPathDigestCache::default();
    let first = digest(&external.join("first"), &mut cache).unwrap();
    let second = digest(&external.join("second"), &mut cache).unwrap();
    assert_ne!(first, second);
    assert_eq!(
        second,
        digest(
            &external.join("second"),
            &mut LabeledPathDigestCache::default(),
        )
        .unwrap(),
    );
    assert_eq!(digest(&external.join("first"), &mut cache).unwrap(), first);
    fs::remove_dir_all(root).unwrap();
}
