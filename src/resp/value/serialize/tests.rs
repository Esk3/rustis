use {
    array::serialize_array, bulk_byte_string::serialize_bulk_byte_string,
    bulk_string::serialize_bulk_string, integer::serialize_int, null_array::serialize_null_array,
    null_string::serialize_null_string,
};

use super::*;

pub fn example_of_all_values() -> Vec<Value> {
    let no_arr = vec![
        Value::SimpleString("ASimpleString".into()),
        Value::BulkString("MyBulkString".into()),
        Value::BulkByteString(b"my Amzing bulk bytestring".to_vec()),
        Value::NullString,
        Value::NullArray,
    ];
    let mut vec = Vec::new();
    vec.extend(no_arr.clone());
    vec.push(Value::Array(no_arr));
    vec
}

#[test]
fn serialize_bulk_string_value_test() {
    let s = "my cool bulk string";
    let s_bytes = serialize_bulk_string(s);
    let value_bytes = serialize_value(&Value::BulkString(s.into()));
    assert_eq!(value_bytes, s_bytes);
}

#[test]
fn serialize_bulk_byte_string_value_test() {
    let s = [b"some string".to_vec(), b"some other string".to_vec()];
    for s in s {
        let expected = serialize_bulk_byte_string(&s);
        let result = serialize_value(&Value::BulkByteString(s));
        assert_eq!(result, expected);
    }
}

#[test]
fn serialize_array_value_test() {
    let arr = vec![
        Value::SimpleString("abc".into()),
        Value::BulkString("qwerty".into()),
    ];
    let arr_bytes = serialize_array(&arr);
    let value_bytes = serialize_value(&Value::Array(arr));
    assert_eq!(arr_bytes, value_bytes);
}

#[test]
fn serialize_null_string_value_test() {
    let value = Value::NullString;
    assert_eq!(serialize_value(&value), serialize_null_string());
}

#[test]
fn serialize_null_array_value_test() {
    let value = Value::NullArray;
    assert_eq!(serialize_value(&value), serialize_null_array());
}

#[test]
fn serialize_integer_value_test() {
    let value = Value::Integer(42);
    assert_eq!(serialize_value(&value), serialize_int(42));
}

#[test]
fn serialize_value_test() {
    let value = Value::SimpleString("hello world".to_string());
    let bytes: Vec<u8> = serialize_value(&value);
}

#[test]
fn serialize_array_contains_serialized_value_test() {
    for value in example_of_all_values() {
        let value_bytes = serialize_value(&value);
        let arr_bytes = serialize_array(&[value]);
        let result = arr_bytes
            .windows(value_bytes.len())
            .any(|win| win == value_bytes);
        assert!(
            result,
            "expected: {}, got: {}",
            String::from_utf8(value_bytes).unwrap(),
            String::from_utf8(arr_bytes).unwrap()
        );
    }
}
