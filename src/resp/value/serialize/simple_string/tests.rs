use super::*;

#[test]
fn serialize_simple_string_test() {
    let s = "hey";
    let bytes = serialize_simple_string(s);
    assert_eq!(bytes, b"+hey\r\n");
}

#[test]
fn serialize_simple_string_test2() {
    let s = "hello";
    let bytes = serialize_simple_string(s);
    assert_eq!(
        bytes.clone(),
        b"+hello\r\n",
        r"expected: +hello\r\n, got: {}",
        String::from_utf8(bytes).unwrap()
    );
}
