use super::{ErrorCode, RejectCode, RejectResponse};
use strum::IntoEnumIterator as _;

#[test]
fn base64_json_preserves_padding_binary_bytes_and_rejections() {
    #[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
    struct Payload {
        #[serde(with = "super::common::rest::base64")]
        bytes: Vec<u8>,
    }
    for (raw, encoded) in [
        (b"".as_slice(), ""),
        (b"f".as_slice(), "Zg=="),
        (b"fo".as_slice(), "Zm8="),
        (b"foo".as_slice(), "Zm9v"),
        ([255, 0, 1, 254].as_slice(), "/wAB/g=="),
    ] {
        let payload = Payload {
            bytes: raw.to_vec(),
        };
        let json = format!(r#"{{"bytes":"{encoded}"}}"#);
        assert_eq!(serde_json::to_string(&payload).unwrap(), json);
        assert_eq!(serde_json::from_str::<Payload>(&json).unwrap(), payload);
    }
    for invalid in ["Zg", "Zg=", "Zh==", "Zm9v=", "_w==", "Zg==\n"] {
        let json = serde_json::json!({"bytes": invalid});
        assert!(
            serde_json::from_value::<Payload>(json).is_err(),
            "{invalid:?}"
        );
    }
}

#[test]
fn rejection_catalog_preserves_protocol_values_and_json() {
    for code in RejectCode::iter() {
        assert_eq!(RejectCode::try_from(code as u64).unwrap(), code);
    }
    for code in ErrorCode::iter() {
        assert_eq!(ErrorCode::try_from(code as u64).unwrap(), code);
    }
    for invalid in [0, 7, u64::MAX] {
        assert!(RejectCode::try_from(invalid).is_err());
    }
    let bytes = br#"{"reject_code":"CanisterReject","reject_message":"caller policy","error_code":"CanisterRejectedMessage","certified":false}"#;
    let rejected: RejectResponse = serde_json::from_slice(bytes).unwrap();
    assert_eq!(rejected.reject_code, RejectCode::CanisterReject);
    assert_eq!(rejected.error_code, ErrorCode::CanisterRejectedMessage);
    assert!(!rejected.certified);
    assert_eq!(serde_json::to_vec(&rejected).unwrap(), bytes);
}

#[cfg(test)]
mod windows_to_wsl_tests {
    use crate::pocket_ic::windows_to_wsl;

    #[test]
    fn converts_absolute_windows_paths() {
        assert_eq!(windows_to_wsl(r"C:\Users\x\y").unwrap(), "/mnt/c/Users/x/y");
        assert_eq!(
            windows_to_wsl(r"d:\pocket-ic\pocket-ic.exe").unwrap(),
            "/mnt/d/pocket-ic/pocket-ic.exe"
        );
        assert_eq!(windows_to_wsl("C:/Users/x/y").unwrap(), "/mnt/c/Users/x/y");
        assert_eq!(windows_to_wsl(r"C:\").unwrap(), "/mnt/c/");
        assert_eq!(windows_to_wsl("C:").unwrap(), "/mnt/c");
        assert_eq!(
            windows_to_wsl(r"\\?\C:\Users\x\y").unwrap(),
            "/mnt/c/Users/x/y"
        );
    }

    #[test]
    fn rejects_paths_without_drive_prefix() {
        assert!(windows_to_wsl("").is_err());
        assert!(windows_to_wsl(r"Users\x\y").is_err());
        assert!(windows_to_wsl(r"\Users\x\y").is_err());
        assert!(windows_to_wsl("/mnt/c/Users/x/y").is_err());
        assert!(windows_to_wsl(r"\\server\share\x").is_err());
        assert!(windows_to_wsl(r"1:\x").is_err());
        assert!(windows_to_wsl(r"C:Users\x").is_err());
    }
}

mod client_failures;
