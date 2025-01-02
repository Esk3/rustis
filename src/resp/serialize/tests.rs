use super::*;

#[test]
fn serialize_value_test() {
    let value = Value::SimpleString("hello world".to_string());
    let bytes: Vec<u8> = serialize_value(&value);
}

#[test]
fn serialize_simple_string_test() {
    let bytes = serialize_value(&Value::SimpleString("hey".into()));
    assert_eq!(bytes, b"+hey\r\n");

    let bytes = serialize_value(&Value::SimpleString("hello".into()));
    assert_eq!(bytes, b"+hello\r\n");
}

#[test]
fn deserialize_bytes_test() {
    let bytes = b"+helloWorld\r\n";
    let (value, bytes_consumed): (Value, usize) = deserialize_value(bytes).unwrap();
}

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
fn deserialize_bulk_string_test() {
    let bytes = b"hello\r\n";
    let length = 5;
    let (byte_string, consumed) = deserialize_bulk_string(bytes, length).unwrap();
}

#[test]
fn deserialize_bulk_string_test_value_matches() {
    let bytes = b"hello\r\n";
    let length = 5;
    let (byte_string, consumed) = deserialize_bulk_string(bytes, length).unwrap();
    assert_eq!(byte_string, b"hello");
    assert_eq!(consumed, bytes.len());
}

#[test]
fn deserialize_bulk_string_value_test() {
    let bytes = b"$5\r\nhello\r\n";
    let length = 5;
    let (value, v_consumed) = deserialize_value(bytes).unwrap();
    let Value::BulkString(value) = value else {
        panic!();
    };
    let (byte_string, s_consumed) = deserialize_bulk_string(&bytes[4..], length).unwrap();
    assert_eq!(value.as_bytes(), byte_string);
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
        let (byte_string, b_length) = deserialize_bulk_string(&bytes[4..], length).unwrap();
        assert_eq!(s.as_bytes(), byte_string);
        assert_eq!(v_length, b_length);
    }
}

#[test]
fn deserialize_array_test() {
    let bytes = b"+Hello\r\n";
    let arr = deserialize_array(bytes, 1).unwrap();
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
    let (_arr, bytes_consumed) = deserialize_array(bytes, 0).unwrap();
    assert_eq!(bytes_consumed, 0);
}

#[test]
fn deserialize_empty_array_has_no_values() {
    let bytes = b"+anythingCauseArrWon'tSearch\r\n";
    let (arr, _bytes_consumed) = deserialize_array(bytes, 0).unwrap();
    assert_eq!(arr, Vec::<Value>::new());
}

#[test]
fn deserialize_array_consumes_bytes_of_single_item_test() {
    let bytes = b"+MyStr\r\n";
    let (arr, bytes_consumed) = deserialize_array(bytes, 1).unwrap();
    assert_eq!(bytes_consumed, bytes.len());
}

#[test]
fn deserialize_array_with_one_item_has_one_item() {
    let bytes = b"+MyStr\r\n";
    let (arr, _bytes_consumed) = deserialize_array(bytes, 1).unwrap();
    assert_eq!(arr.len(), 1);
}

#[test]
fn deserialize_array_with_one_item_has_matching_item() {
    let bytes = b"+MyStr\r\n";
    let (arr, _bytes_consumed) = deserialize_array(bytes, 1).unwrap();
    assert_eq!(arr[0], Value::SimpleString("MyStr".into()));
}

#[test]
fn get_identifier_from_byte() {
    let identifer = Identifier::from_byte(b'+');
}

fn get_all_idents_variants() -> [Identifier; 15] {
    [
        Identifier::SimpleString,
        Identifier::SimpleError,
        Identifier::Integer,
        Identifier::BulkString,
        Identifier::Array,
        Identifier::Null,
        Identifier::Boolean,
        Identifier::Double,
        Identifier::BigNumber,
        Identifier::BulkError,
        Identifier::VerbatimString,
        Identifier::Map,
        Identifier::Attribute,
        Identifier::Set,
        Identifier::Pushe,
    ]
}

fn get_all_ident_bytes() -> [u8; 15] {
    [
        b'+', b'-', b':', b'$', b'*', b'_', b'#', b',', b'(', b'!', b'=', b'%', b'`', b'~', b'>',
    ]
}

#[test]
fn valid_identifer_returns_ok_test() {
    let idents = get_all_ident_bytes();
    let expected = get_all_idents_variants();

    let results = idents
        .iter()
        .map(|ident| Identifier::from_byte(*ident).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results, expected);
}

#[test]
fn to_byte_identifier_test() {
    let idents = get_all_idents_variants();
    let idents = idents
        .iter()
        .map(super::Identifier::as_byte)
        .collect::<Vec<u8>>();
    let expected = get_all_ident_bytes();
    assert_eq!(idents, expected);
}

#[test]
fn get_identifier_length_test() {
    let ident = Identifier::SimpleString;
    let length: usize = ident.get_byte_length();
}

#[test]
fn length_of_all_identifiers_is_one() {
    get_all_idents_variants()
        .iter()
        .map(Identifier::get_byte_length)
        .for_each(|i| assert_eq!(i, 1));
}

#[test]
fn invalid_identifier_returns_err_test() {
    let idents = b"abcxyz123";
    let result = idents
        .iter()
        .map(|ident| Identifier::from_byte(*ident))
        .all(|res| res.is_err());
    assert!(result);
}

#[test]
fn get_identifier_from_slice_test() {
    let b = b"+";
    let identifier = b.get_identifier().unwrap();
}

#[test]
fn get_identifier_from_test_is_same_as_get_identifier() {
    let mut s = get_all_ident_bytes().to_vec();
    s.extend(1..10);
    s.into_iter()
        .for_each(|i| match ([i].get_identifier(), Identifier::from_byte(i)) {
            (Err(_), Err(_)) => (),
            (Ok(a), Ok(b)) if a == b => (),
            (a, b) => panic!("{a:?}, {b:?}"),
        });
}

#[test]
fn is_linefeed_test() {
    assert_eq!(is_linefeed(1, 2), Ok(false));
    assert_eq!(is_linefeed(b'\r', b'\n'), Ok(true));
    assert_eq!(is_linefeed(b'\r', 0), Err(()));
    assert_eq!(is_linefeed(b'\n', 0), Err(()));
}

#[test]
fn find_linefeed_returns_start_of_linefeed() {
    let b = b"abc\r\n";
    let pos = b.find_linefeed().unwrap().unwrap();
    assert_eq!(pos, 3);
}

#[test]
fn find_linefeed_return_none_if_not_found() {
    let b = b"abc";
    let none_pos = b.find_linefeed().unwrap();
    assert_eq!(none_pos, None);
}

#[test]
fn find_linefeed_returns_err_if_single_cr() {
    let b = b"abc\rd";
    let err = b.find_linefeed();
    assert!(err.is_err());

    let b = b"abc\r";
    let err = b.find_linefeed();
    assert!(err.is_err());
}

#[test]
fn find_linefeed_returns_err_if_single_lf() {
    let b = b"abc\nd";
    let err = b.find_linefeed();
    assert!(err.is_err());

    let b = b"abc\n";
    let err = b.find_linefeed();
    assert!(err.is_err());
}

#[test]
fn is_at_linefeed_test() {
    let b = b"abc\nd";
    let f = b.is_at_linefeed();
    assert_eq!(f, is_linefeed(b'a', b'b'));

    let b = b"\r\nabc";
    let t = b.is_at_linefeed();
    assert_eq!(t, is_linefeed(b'\r', b'\n'));
}

#[test]
fn get_header_test() {
    let bytes = b"$10\r\nabc\r\n";
    let (header_value, bytes_consumed) = deserialize_header(bytes).unwrap();
}

#[test]
fn get_header_length_test() {
    let bytes = [
        (b"$10\r\n".to_vec(), 10, 5),
        (b"$3\r\nabc".to_vec(), 3, 4),
        (b"2\r\nbc".to_vec(), 2, 3),
    ];
    for (bytes, expected, expected_length) in bytes {
        assert_eq!(
            deserialize_header(&bytes).unwrap(),
            (expected, expected_length)
        );
    }
}

#[test]
fn get_header_on_slice() {
    let bytes = b"$10\r\n";
    let (length, bytes_consumed) = bytes.get_header().unwrap();
}

#[test]
fn get_header_on_slice_follows_deserialize_header() {
    let bytes = [
        b"$10\r\n".to_vec(),
        b"$3\r\nabc".to_vec(),
        b"2\r\nbc".to_vec(),
    ];
    for bytes in bytes {
        assert_eq!(
            deserialize_header(&bytes).unwrap(),
            bytes.get_header().unwrap()
        );
    }
}
