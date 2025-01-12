use super::*;

#[test]
fn is_linefeed_test() {
    assert!(!is_linefeed(1, 2).unwrap());
    assert!(is_linefeed(b'\r', b'\n').unwrap());
    assert_eq!(
        is_linefeed(b'\r', 0).unwrap_err().to_string(),
        "expected newline found: byte [13] char: [\r]"
    );
    assert_eq!(
        is_linefeed(b'\n', 0).unwrap_err().to_string(),
        "found newline before cr"
    );
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
    assert_eq!(f.unwrap(), is_linefeed(b'a', b'b').unwrap());

    let b = b"\r\nabc";
    let t = b.is_at_linefeed();
    assert_eq!(t.unwrap(), is_linefeed(b'\r', b'\n').unwrap());
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
