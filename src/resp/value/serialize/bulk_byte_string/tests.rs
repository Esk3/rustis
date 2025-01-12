use super::*;

#[test]
fn serialize_bulk_byte_string_test() {
    let s = b"some cool bytes";
    let result = serialize_bulk_byte_string(s);
    assert_eq!(result, b"$15\r\nsome cool bytes\r\n");

    let s = [b"abc".to_vec(), b"with \r\n escape".to_vec()];
    let expected = [
        b"$3\r\nabc\r\n".to_vec(),
        b"$14\r\nwith \r\n escape\r\n".to_vec(),
    ];
    for (s, expected) in s.into_iter().zip(expected) {
        let result = serialize_bulk_byte_string(&s);
        assert_eq!(
            result,
            expected,
            "s: {:?}, bytes: {s:?}",
            String::from_utf8(s.clone())
        );
    }
}
