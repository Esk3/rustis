use super::*;

#[test]
fn deserialize_simple_string_test() {
    let bytes = b"helloWorld\r\n";
    let (s, bytes_consumed): (String, usize) = deserialize_simple_string(bytes).unwrap();
    assert_eq!(s, "helloWorld");
    assert_eq!(bytes_consumed, bytes.len());

    let bytes = b"MyAmazingValue\r\n";
    let (s, bytes_consumed): (String, usize) = deserialize_simple_string(bytes).unwrap();
    assert_eq!(s, "MyAmazingValue");
    assert_eq!(bytes_consumed, bytes.len());
}

#[test]
fn deserialize_simgple_string_consumes_bytes_test() {
    let bytes = b"Astring\r\n";
    let (_, bytes_consumed) = deserialize_simple_string(bytes).unwrap();
    assert_eq!(bytes_consumed, bytes.len());
}
