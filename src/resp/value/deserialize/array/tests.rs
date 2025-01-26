use super::*;

#[test]
fn deserialize_array_test() {
    let bytes = b"+Hello\r\n";
    let info = deserialize_array(bytes, 1).unwrap();
    let arr = info.value;
    assert_eq!(info.bytes_read, bytes.len());
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0], Value::bulk_string("Hello"));
}

#[test]
fn deserialize_array_with_invalid_value_errors_test() {
    let bytes = b"noIdent\r\n";
    let result = deserialize_array(bytes, 1);
    assert!(result.is_err());
}

#[test]
fn deserialize_empty_array_comsumes_no_bytes() {
    let bytes = b"+anythingCauseArrWon'tSearch\r\n";
    let bytes_consumed = deserialize_array(bytes, 0).unwrap().bytes_read;
    assert_eq!(bytes_consumed, 0);
}

#[test]
fn deserialize_empty_array_has_no_values() {
    let bytes = b"+anythingCauseArrWon'tSearch\r\n";
    let arr = deserialize_array(bytes, 0).unwrap().value;
    assert_eq!(arr, Vec::<Value>::new());
}

#[test]
fn deserialize_array_consumes_bytes_of_single_item_test() {
    let bytes = b"+MyStr\r\n";
    let (arr, bytes_consumed) = deserialize_array(bytes, 1).unwrap().into();
    assert_eq!(bytes_consumed, bytes.len());
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0], Value::simple_string("MyStr"));
}

#[test]
fn deserialize_array_with_one_item_has_one_item() {
    let bytes = b"+MyStr\r\n";
    let (arr, _bytes_consumed) = deserialize_array(bytes, 1).unwrap().into();
    assert_eq!(arr.len(), 1);
}
#[test]
fn deserialize_array_with_one_item_has_matching_item() {
    let bytes = b"+MyStr\r\n";
    let (arr, _bytes_consumed) = deserialize_array(bytes, 1).unwrap().into();
    assert_eq!(arr[0], Value::SimpleString("MyStr".into()));
}

#[test]
fn deserialize_array_with_bulk_string() {
    let bytes = b"$5\r\nhello\r\n";
    let (arr, consumed) = deserialize_array(bytes, 1).unwrap().into();
    assert_eq!(arr[0], Value::BulkString("hello".into()));
    assert_eq!(consumed, bytes.len());
}
#[test]
fn deserialize_array_with_multiple_items() {
    let bytes = b"+StrOne\r\n+SimpleTwo\r\n";
    let (arr, bytes_consumed) = deserialize_array(bytes, 2).unwrap().into();
    assert_eq!(
        arr,
        [
            Value::SimpleString("StrOne".into()),
            Value::SimpleString("SimpleTwo".into())
        ]
    );
    assert_eq!(bytes_consumed, bytes.len());
}
