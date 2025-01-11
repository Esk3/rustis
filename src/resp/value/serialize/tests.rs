use crate::resp::value::{deserialize::GetHeader, identifier::GetIdentifier};

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
fn serialize_value_test() {
    let value = Value::SimpleString("hello world".to_string());
    let bytes: Vec<u8> = serialize_value(&value);
}

#[test]
fn serialzie_simple_error_test() {
    let value = "myError";
    assert_eq!(
        serialize_value(&Value::SimpleError(value.into()),),
        serialize_simple_error(value)
    )
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

#[test]
fn serialize_bulk_byte_string_test() {
    let s = b"some cool bytes";
    let result = serialize_bulk_byte_string(s);
    assert_eq!(result, b"$15\r\nsome cool bytes\r\n");

    let s = [b"abc".to_vec(), b"with \r\n escape".to_vec()];
    let expected = [
        b"$3\r\nabc\r\n".to_vec(),
        b"$14\r\nwith \r\n escape\r\n".to_vec(),
    ];
    for (s, expected) in s.into_iter().zip(expected) {
        let result = serialize_bulk_byte_string(&s);
        assert_eq!(
            result,
            expected,
            "s: {:?}, bytes: {s:?}",
            String::from_utf8(s.clone())
        );
    }
}

#[test]
fn serialize_positive_int_test() {
    let n = [
        (1, b":1\r\n".to_vec()),
        (2, b":2\r\n".to_vec()),
        (3, b":3\r\n".to_vec()),
        (23, b":23\r\n".to_vec()),
        (42, b":42\r\n".to_vec()),
    ];
    for (n, expected) in n {
        let result = serialize_int(n);
        assert_eq!(
            result.clone(),
            expected.clone(),
            "left: [{}]. right: [{}]",
            String::from_utf8(result).unwrap(),
            String::from_utf8(expected).unwrap()
        );
    }
}

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

#[test]
fn serialize_null_string_test() {
    let bytes = serialize_null_string();
    assert_eq!(bytes, *b"$-1\r\n");
}

#[test]
fn serialzie_null_array_test() {
    let bytes = serialize_null_array();
    assert_eq!(bytes, *b"*-1\r\n");
}

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
