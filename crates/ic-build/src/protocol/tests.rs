use super::Decoder;
use crate::Failure;

const ARTIFACT: &[u8] = br#"{"reason":"compiler-artifact","package_id":"pkg","target":{"name":"lib","kind":["cdylib"],"crate_types":["cdylib"]},"filenames":["/private/lib.wasm"],"fresh":false}
"#;
const COMPLETE: &[u8] = b"{\"reason\":\"build-finished\",\"success\":true}\n";

fn decoder() -> Decoder<'static> {
    Decoder::new("pkg", "lib", 1024, 2)
}

#[test]
fn chunks_select_exact_candidate_only_after_completion() {
    let mut decoder = decoder();
    for byte in ARTIFACT {
        decoder.feed(&[*byte]);
    }
    decoder.feed(COMPLETE);
    assert_eq!(
        decoder.finish().unwrap().to_str(),
        Some("/private/lib.wasm")
    );
}

#[test]
fn missing_duplicate_and_truncated_completion_refuse() {
    for suffix in [b"".as_slice(), &COMPLETE[..COMPLETE.len() - 1]] {
        let mut decoder = decoder();
        decoder.feed(ARTIFACT);
        decoder.feed(suffix);
        assert!(matches!(decoder.finish(), Err(Failure::Protocol(_))));
    }
    let mut decoder = decoder();
    decoder.feed(ARTIFACT);
    decoder.feed(COMPLETE);
    decoder.feed(COMPLETE);
    assert!(matches!(decoder.finish(), Err(Failure::Protocol(_))));
}

#[test]
fn ambiguous_wrong_identity_and_malformed_records_refuse() {
    let mut ambiguous = decoder();
    ambiguous.feed(ARTIFACT);
    ambiguous.feed(ARTIFACT);
    assert!(matches!(ambiguous.finish(), Err(Failure::Protocol(_))));
    let mut wrong = Decoder::new("different-package", "lib", 1024, 2);
    wrong.feed(ARTIFACT);
    wrong.feed(COMPLETE);
    assert!(matches!(wrong.finish(), Err(Failure::Protocol(_))));
    let mut malformed = decoder();
    malformed.feed(b"not-json\n");
    assert!(matches!(malformed.finish(), Err(Failure::Json(_))));
    let mut shape = decoder();
    shape.feed(b"{\"reason\":\"compiler-artifact\"}\n");
    assert!(matches!(shape.finish(), Err(Failure::Protocol(_))));
}

#[test]
fn record_candidate_and_fresh_bounds_refuse() {
    let mut bounded = Decoder::new("pkg", "lib", 3, 1);
    bounded.feed(ARTIFACT);
    assert!(matches!(bounded.finish(), Err(Failure::RecordLimit)));
    let mut bounded = Decoder::new("pkg", "lib", 1024, 1);
    bounded.feed(ARTIFACT);
    bounded.feed(ARTIFACT);
    assert!(matches!(bounded.finish(), Err(Failure::CandidateLimit)));
    let fresh = String::from_utf8(ARTIFACT.to_vec())
        .unwrap()
        .replace("false", "true");
    let mut stale = decoder();
    stale.feed(fresh.as_bytes());
    assert!(matches!(stale.finish(), Err(Failure::Protocol(_))));
}
