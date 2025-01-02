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
fn deserialize_simple_string_value() {
    let bytes = b"+mySimpleString\r\n";

    let (value, consumed) = deserialize_value(bytes).unwrap();
    let (s, s_consumed) = deserialize_simple_string(&bytes[1..]).unwrap();
    assert_eq!(value.into_string().unwrap(), s);
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
fn deserialize_bulk_string_value() {
    let bytes = b"$5\r\nhello\r\n";
    let length = 5;
    let (value, v_consumed) = deserialize_value(bytes).unwrap();
    let (byte_string, s_consumed) = deserialize_bulk_string(&bytes[5..], length).unwrap();
    assert_eq!(value.into_byte_string().unwrap(), bytes);
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
