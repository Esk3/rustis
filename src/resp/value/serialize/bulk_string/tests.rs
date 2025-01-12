use super::*;

#[test]
fn serialize_bulk_string_test() {
    let s = "hello";
    let bytes = serialize_bulk_string(s);
    assert_eq!(bytes, b"$5\r\nhello\r\n");
}

#[test]
fn serialize_bulk_string_test2() {
    let s = "some other string";
    let bytes = serialize_bulk_string(s);
    assert_eq!(bytes, b"$17\r\nsome other string\r\n");
}
