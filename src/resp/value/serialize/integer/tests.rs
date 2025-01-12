use super::*;

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
