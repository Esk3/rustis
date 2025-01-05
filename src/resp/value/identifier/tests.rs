use super::*;

#[test]
fn get_identifier_from_byte() {
    let identifer = Identifier::from_byte(b'+');
}

const fn get_all_idents_variants() -> [Identifier; 15] {
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

const fn get_all_ident_bytes() -> [u8; 15] {
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
fn serialized_value_get_deserialized_to_same_value() {
    let values = super::super::serialize::tests::example_of_all_values();
    let deserialized_values = values
        .iter()
        .map(serialize_value)
        .map(|bytes| deserialize_value(&bytes).unwrap().0)
        .collect::<Vec<_>>();
    assert_eq!(deserialized_values, values);
}
