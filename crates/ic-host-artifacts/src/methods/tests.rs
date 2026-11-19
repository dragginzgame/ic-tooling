use super::{Error, Kind, inventory};
use crate::Length;
use std::collections::BTreeMap;

#[test]
fn exact_names_and_kinds_are_comparable_to_independent_declarations() {
    let wasm = wat::parse_str(
        r#"(module
        (import "host" "f" (func $import))
        (export "canister_update  名 " (func $import))
        (func (export "canister_query Read"))
        (func (export "canister_composite_query read"))
        (func (export "canister_init"))
        (func (export "private"))
        (memory (export "canister_query memory") 1))"#,
    )
    .unwrap();
    let length = Length::Exactly(wasm.len() as u64);
    let (actual, identity) = inventory(&wasm[..], length).unwrap();
    let expected = BTreeMap::from([
        (" 名 ".to_owned(), Kind::Update),
        ("Read".to_owned(), Kind::Query),
        ("read".to_owned(), Kind::CompositeQuery),
    ]);
    assert_eq!(actual, expected);
    assert_eq!(identity, crate::read(&wasm[..], length).unwrap().1);
    let mut declarations = expected;
    declarations.remove("Read");
    assert_ne!(actual, declarations);
    declarations.insert("extra".into(), Kind::Query);
    declarations.insert("Read".into(), Kind::Query);
    assert_ne!(actual, declarations);
    declarations.remove("extra");
    declarations.insert("read".into(), Kind::Query);
    assert_ne!(actual, declarations);
}

#[test]
fn refuses_ambiguous_names_and_invalid_entry_point_signatures() {
    for (module, ambiguous) in [
        (
            r#"(module (func (export "canister_query x") (export "canister_update x")))"#,
            true,
        ),
        (
            r#"(module (func (export "canister_update x") (param i32)))"#,
            false,
        ),
        (
            r#"(module (func (export "canister_query x") (result i32) i32.const 1))"#,
            false,
        ),
    ] {
        let wasm = wat::parse_str(module).unwrap();
        let error = inventory(&wasm[..], Length::AtMost(1024)).unwrap_err();

        match (ambiguous, error) {
            (true, Error::Ambiguous(name)) | (false, Error::Signature(name)) => {
                assert_eq!(name, "x")
            }
            (_, other) => panic!("unexpected refusal: {other}"),
        }
    }
}

#[test]
fn validates_the_complete_artifact_before_returning_methods() {
    let wasm = wat::parse_str(r#"(module (func (export "canister_update x")))"#).unwrap();
    let limit = wasm.len() as u64 - 1;
    assert!(matches!(inventory(&wasm[..], Length::AtMost(limit)),
        Err(Error::Admission(crate::Error::Limit { limit: actual })) if actual == limit));
    assert!(matches!(
        inventory(&wasm[..wasm.len() - 1], Length::AtMost(1024)),
        Err(Error::Wasm(_))
    ));
    let invalid = wat::parse_str("(module (func (result i32)))").unwrap();
    assert!(matches!(
        inventory(&invalid[..], Length::AtMost(1024)),
        Err(Error::Wasm(_))
    ));

    for input in [&b"\x1f\x8b"[..], &b"\0asm\x0d\0\x01\0"[..]] {
        assert!(matches!(
            inventory(input, Length::AtMost(1024)),
            Err(Error::Format)
        ));
    }
}
