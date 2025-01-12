use super::*;

#[test]
fn extend_linefeed_test() {
    let mut v: Vec<u8> = Vec::new();
    v.extend_linefeed();
    assert_eq!(v, b"\r\n");
}

#[test]
fn extend_identifier_test() {
    let mut v = Vec::new();
    v.extend_identifier(&Identifier::SimpleString);
    assert_eq!(v[0], Identifier::SimpleString.as_byte());
}

#[test]
fn extend_header_test() {
    let mut v = Vec::new();
    let identifier = &Identifier::BulkString;
    v.extend_header(identifier, 10);
    let mut expected = Vec::new();
    expected.extend_identifier(identifier);
    expected.extend(b"10");
    expected.extend_linefeed();
    assert_eq!(v, expected);
}

#[test]
#[ignore = "todo"]
fn extend_header_length_test() {
    todo!()
}
