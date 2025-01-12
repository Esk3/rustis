use super::*;
use crate::resp::value::{deserialize::util::GetHeader, identifier::GetIdentifier};

#[test]
fn serialize_empty_array_test() {
    let bytes = serialize_array(&[]);
    let items = bytes.get_header().unwrap().0;
    assert_eq!(items, 0);
    let ident = bytes.get_identifier().unwrap();
    assert_eq!(ident, Identifier::Array);
}

#[test]
fn serialize_array_with_one_items_has_len_on_one() {
    let bytes = serialize_array(&[Value::SimpleString(String::new())]);
    let items = bytes.get_header().unwrap().0;
    assert_eq!(items, 1);
    let ident = bytes.get_identifier().unwrap();
    assert_eq!(ident, Identifier::Array);
}
