use super::*;
use crate::{
    durable::{
        supported::{FileCommitStep, WriterCustody, commit_path_with_options},
        tests::REPLACE_OPTIONS,
    },
    tests::fixture,
};
use std::fs;

#[test]
fn failed_publication_cleans_staging_before_dropping_the_produced_value() {
    use std::{cell::Cell, mem::DropGuard};

    let fixture = fixture();
    let output = fixture.path().join("output");
    for step in [
        FileCommitStep::TemporaryFileSync,
        FileCommitStep::Publication,
    ] {
        fs::write(&output, b"old").unwrap();
        let present_on_drop = Cell::new(None);
        let result = commit_path_with_options(
            &output,
            REPLACE_OPTIONS,
            |stage, file| {
                use std::io::Write as _;
                file.write_all(b"new")?;
                Ok::<_, io::Error>(DropGuard::new(stage.to_path_buf(), |stage| {
                    present_on_drop.set(Some(stage.exists()));
                }))
            },
            (WriterCustody::Writable, |_, _| Ok(())),
            |current, _| {
                if current == step {
                    Err(io::Error::from_raw_os_error(5))
                } else {
                    Ok(())
                }
            },
        );
        assert!(matches!(result,
            Err(NamedWriteError::BeforePublication { source, cleanup_error: None })
            if source.raw_os_error() == Some(5)));
        assert_eq!(present_on_drop.get(), Some(false));
        assert_eq!(fs::read(&output).unwrap(), b"old");
    }
}

#[test]
fn replacement_after_sync_is_rejected_before_rename() {
    use std::cell::RefCell;
    let fixture = fixture();
    let output = fixture.path().join("output");
    fs::write(&output, b"old").unwrap();
    let staged_path = RefCell::new(None);
    let result = commit_path_with_options(
        &output,
        REPLACE_OPTIONS,
        |stage, file| {
            use std::io::Write as _;
            staged_path.replace(Some(stage.to_path_buf()));
            file.write_all(b"produced")
        },
        (WriterCustody::Writable, |_, ()| Ok(())),
        |step, _| {
            if step == FileCommitStep::Publication {
                let borrowed = staged_path.borrow();
                let stage = borrowed.as_ref().unwrap();
                fs::remove_file(stage)?;
                fs::write(stage, b"foreign")?;
            }
            Ok(())
        },
    );
    assert!(matches!(
        result,
        Err(NamedWriteError::BeforePublication {
            cleanup_error: Some(_),
            ..
        })
    ));
    assert_eq!(fs::read(&output).unwrap(), b"old");
    assert_eq!(
        fs::read(staged_path.into_inner().unwrap()).unwrap(),
        b"foreign"
    );
}

#[test]
fn display_keeps_phase_primary_cause_and_secondary_cleanup() {
    use std::error::Error as _;
    for error in [
        NamedWriteError::Producer {
            source: io::Error::other("producer failure"),
            cleanup_error: Some(io::Error::other("cleanup failure")),
        },
        NamedWriteError::BeforePublication {
            source: io::Error::other("filesystem failure"),
            cleanup_error: Some(io::Error::other("cleanup failure")),
        },
    ] {
        let text = error.to_string();
        assert!(text.contains(&error.source().unwrap().to_string()));
        assert!(text.contains("staging cleanup failed: cleanup failure"));
        match error {
            NamedWriteError::Producer { .. } => {
                assert!(text.starts_with("staged producer failed:"));
            }
            NamedWriteError::BeforePublication { .. } => {
                assert!(text.starts_with("publication failed before rename:"));
            }
            NamedWriteError::AfterPublication { .. } => unreachable!(),
        }
    }
}

#[test]
fn source_identity_preserves_direct_and_generic_boxed_causes() {
    use std::error::Error as _;
    for error in [
        NamedWriteError::Producer {
            source: io::Error::other("producer"),
            cleanup_error: Some(io::Error::other("cleanup")),
        },
        NamedWriteError::BeforePublication {
            source: io::Error::other("before"),
            cleanup_error: None,
        },
        NamedWriteError::AfterPublication {
            source: io::Error::other("after"),
        },
    ] {
        let (NamedWriteError::Producer { source, .. }
        | NamedWriteError::BeforePublication { source, .. }
        | NamedWriteError::AfterPublication { source }) = &error;
        assert!(std::ptr::eq(
            error.source().unwrap().downcast_ref::<io::Error>().unwrap(),
            source,
        ));
    }
    let error = NamedWriteError::Producer {
        source: Box::new(io::Error::other("boxed producer")),
        cleanup_error: None,
    };
    let NamedWriteError::Producer { source, .. } = &error else {
        unreachable!()
    };
    assert!(std::ptr::eq(
        error
            .source()
            .unwrap()
            .downcast_ref::<Box<io::Error>>()
            .unwrap(),
        source,
    ));
}
