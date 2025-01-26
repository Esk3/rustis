use super::*;

#[test]
fn deserialize_bytes_test() {
    let bytes = b"+helloWorld\r\n";
    let (value, bytes_consumed): (Value, usize) = deserialize_value(bytes).unwrap();
}

#[test]
fn deserialize_simple_string_value() {
    let bytes = b"+mySimpleString\r\n";

    let (value, consumed) = deserialize_value(bytes).unwrap();
    let Value::SimpleString(value) = value else {
        panic!()
    };
    let (s, s_consumed) = deserialize_simple_string(&bytes[1..]).unwrap();
    assert_eq!(value, s);
    assert_eq!(consumed, bytes.len());
}

#[test]
fn deserialize_bulk_string_test_value_matches() {
    let bytes = b"hello\r\n";
    let length = 5;
    let info = deserialize_bulk_string(bytes, length).unwrap();
    assert_eq!(info.value, b"hello");
    assert_eq!(info.bytes_read, bytes.len());
}

#[test]
fn deserialize_bulk_string_value_test() {
    let bytes = b"$5\r\nhello\r\n";
    let length = 5;
    let (value, v_consumed) = deserialize_value(bytes).unwrap();
    let Value::BulkString(value) = value else {
        panic!();
    };
    let info = deserialize_bulk_string(&bytes[4..], length).unwrap();
    assert_eq!(value.as_bytes(), info.value);
    assert_eq!(v_consumed, bytes.len());
}

#[test]
fn deserialize_null_bulk_string_value_test() {
    let (null_str, bytes_consumed) = deserialize_value(b"$-1\r\n").unwrap();
    assert_eq!(null_str, Value::NullString);
    assert_eq!(bytes_consumed, 5);
}

#[test]
fn deserialize_bulk_string_with_more_negative_number_fails() {
    let s = |i| {
        assert!(i < -1, "invalid value to test i: {i}");
        format!("${i}\r\n").as_bytes().to_vec()
    };
    let values = (-11..-2).map(s);
    for value in values {
        assert!(deserialize_value(&value).is_err());
    }
}

#[test]
fn deserialize_bulk_string_value_matches_bulk_string_test() {
    let inputs = [
        b"$1\r\na\r\n".to_vec(),
        b"$2\r\nbc\r\n".to_vec(),
        b"$3\r\nxyz\r\n".to_vec(),
    ]
    .into_iter()
    .enumerate()
    .map(|(i, v)| (v, i + 1))
    .collect::<Vec<_>>();
    for (bytes, length) in inputs {
        let (value, v_length) = deserialize_value(&bytes).unwrap();
        let Value::BulkString(s) = value else {
            panic!()
        };
        let info = deserialize_bulk_string(&bytes[4..], length).unwrap();
        assert_eq!(s.as_bytes(), info.value);
        assert_eq!(v_length, bytes.len());
    }
}

#[test]
fn deserialize_array_value_test() {
    let bytes = b"*1\r\n+hello\r\n";
    let (value, bytes_consumed) = deserialize_value(bytes).unwrap();
    assert_eq!(bytes_consumed, bytes.len());
    let Value::Array(arr) = value else { panic!() };
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0], Value::SimpleString("hello".into()));
}

#[test]
fn deserialize_array_matches_deserialize_array() {
    let bytes = b"*2\r\n+hello\r\n$5\r\nworld\r\n";

    let (arr, _bytes_consumed) = deserialize_array(&bytes[4..], 2).unwrap().into();

    let (value, bytes_consumed) = deserialize_value(bytes).unwrap();
    let Value::Array(arr_value) = value else {
        panic!()
    };
    assert_eq!(arr_value, arr);
    assert_eq!(bytes_consumed, bytes.len());
}
#[test]
fn deserialize_nested_array_test() {
    let bytes = b"*1\r\n+simpleStr\r\n";
    let (arr, consumed) = deserialize_array(bytes, 1).unwrap().into();
    assert_eq!(
        arr[0],
        Value::Array(vec![Value::SimpleString("simpleStr".into())])
    );
    assert_eq!(consumed, bytes.len());
}

#[test]
fn deserialize_null_bulk_array_value_test() {
    let (null_str, bytes_consumed) = deserialize_value(b"*-1\r\n").unwrap();
    assert_eq!(null_str, Value::NullArray);
    assert_eq!(bytes_consumed, 5);
}

#[test]
fn deserialize_bulk_array_with_more_negative_number_fails() {
    let s = |i| {
        assert!(i < -1, "invalid value to test i: {i}");
        format!("*{i}\r\n").as_bytes().to_vec()
    };
    let values = (-11..-2).map(s);
    for value in values {
        assert!(deserialize_value(&value).is_err());
    }
}
