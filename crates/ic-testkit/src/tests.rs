use super::*;

#[test]
fn fake_principal_is_deterministic_and_unique() {
    let p1: Principal = fake_principal(7);
    let p2 = fake_principal(7);
    let q = fake_principal(8);

    assert_eq!(p1, p2, "fake_principal should be deterministic");
    assert_ne!(p1, q, "fake_principal should differ for different seeds");

    let bytes = p1.as_slice();
    assert_eq!(bytes.len(), 29, "Principal must be 29 bytes");
}
