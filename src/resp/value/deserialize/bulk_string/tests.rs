use super::*;

#[test]
fn deserialize_bulk_string_test() {
    let bytes = b"hello\r\n";
    let length = 5;
    let info = deserialize_bulk_string(bytes, length).unwrap();
}
