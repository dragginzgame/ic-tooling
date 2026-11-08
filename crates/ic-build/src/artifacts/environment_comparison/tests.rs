use super::*;
#[test]
fn keyed_values_are_private_bounded_and_key_specific() {
    let a = vec![
        ("TOKEN".into(), "secret-a".into()),
        ("BAD\nKEY".into(), "hidden".into()),
    ];
    let b = vec![("TOKEN".into(), "secret-b".into())];
    let before = EnvironmentComparison::capture(&[1; 32], &a, 2).unwrap();
    let after = EnvironmentComparison::capture(&[1; 32], &b, 2).unwrap();
    assert_eq!(after.changed_keys(&before, 2, 8).unwrap(), ["TOKEN"]);
    assert_eq!(
        after.changed_keys(&before, 2, 0).unwrap(),
        Vec::<String>::new()
    );
    let encoded = serde_json::to_string(&before).unwrap();
    assert!(!encoded.contains("secret-a"));
    assert!(!encoded.contains("BAD"));
    let other = EnvironmentComparison::capture(&[2; 32], &b, 2).unwrap();
    assert_eq!(
        after.changed_keys(&other, 2, 8),
        Err(ComparisonError::DifferentKeys)
    );
    assert!(matches!(
        EnvironmentComparison::capture(&[1; 32], &a, 1),
        Err(ComparisonError::InputLimit)
    ));
    assert_eq!(
        after.changed_keys(&before, 0, 8),
        Err(ComparisonError::InputLimit)
    );
}
#[test]
fn tags_retain_the_existing_hmac_domain_and_length_encoding() {
    let a = tag(&[7; 32], b"environment-value", &[b"AB", b"C"]);
    let b = tag(&[7; 32], b"environment-value", &[b"A", b"BC"]);
    assert_eq!(
        a,
        [
            199, 34, 102, 48, 40, 247, 6, 49, 231, 51, 50, 108, 218, 178, 254, 77, 110, 107, 25,
            250, 106, 190, 57, 227, 64, 100, 84, 95, 53, 16, 60, 192
        ]
    );
    assert_ne!(a, b);
    assert_ne!(a, tag(&[7; 32], b"environment-key-id", &[b"AB", b"C"]));
}
