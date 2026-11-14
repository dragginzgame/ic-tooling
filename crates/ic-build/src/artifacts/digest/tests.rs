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
