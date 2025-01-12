use super::*;

#[test]
fn serialize_null_string_test() {
    let bytes = serialize_null_string();
    assert_eq!(bytes, *b"$-1\r\n");
}
