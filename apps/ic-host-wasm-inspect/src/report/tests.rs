use super::*;
use ic_host_artifacts::wasm::{InspectionError, InspectionResource};
use std::io;

const MODULE: &[u8] = b"\0asm\x01\0\0\0\x01\x04\x01\x60\0\0\x03\x02\x01\0\x07\x05\x01\x01f\0\0\x0a\x04\x01\x02\0\x0b\0\x04\x01x\x07\x08";
const LIMITS: InspectionLimits = InspectionLimits {
    module_bytes: 100,
    sections: 10,
    exports: 1,
    custom_sections: 1,
};

#[test]
fn reports_raw_and_structural_units_separately() {
    let mut output = Vec::new();
    write(MODULE, LIMITS, None, false, &mut output).unwrap();
    let text = String::from_utf8(output).unwrap();
    let rows: Vec<_> = text.lines().collect();
    assert_eq!(rows.len(), 2);
    let fields: Vec<_> = rows[1].split('\t').collect();
    assert_eq!(fields.len(), 12);
    assert_eq!(
        &fields[1..],
        ["37", "4", "3", "0", "1", "0", "0", "0", "0", "1", "1"]
    );
}

#[test]
fn malformed_and_over_budget_inputs_publish_no_report() {
    let mut output = Vec::new();
    assert!(matches!(
        write(
            MODULE,
            LIMITS,
            Some(Sha256Digest::compute(b"different")),
            true,
            &mut output
        ),
        Err(Error::Input(
            ic_host_artifacts::artifact::ArtifactError::DigestMismatch { .. }
        ))
    ));
    assert_eq!(output.as_slice(), b"");
    assert!(matches!(
        write(b"not Wasm", LIMITS, None, false, &mut output),
        Err(Error::Inspection(_))
    ));
    assert_eq!(output.as_slice(), b"");
    for limits in [
        InspectionLimits {
            module_bytes: 36,
            ..LIMITS
        },
        InspectionLimits {
            custom_sections: 0,
            ..LIMITS
        },
        InspectionLimits {
            exports: 0,
            ..LIMITS
        },
    ] {
        assert!(matches!(
            write(MODULE, limits, None, false, &mut output),
            Err(Error::Inspection(InspectionError::LimitExceeded {
                resource: InspectionResource::ModuleBytes
                    | InspectionResource::CustomSections
                    | InspectionResource::Exports,
                ..
            }))
        ));
        assert_eq!(output.as_slice(), b"");
    }
}

#[test]
fn output_failures_are_typed() {
    struct FailedOutput;
    impl Write for FailedOutput {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "closed fixture output",
            ))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    assert!(
        matches!(write(MODULE, LIMITS, None, false, &mut FailedOutput), Err(Error::Output(error)) if error.kind() == io::ErrorKind::BrokenPipe)
    );
}
