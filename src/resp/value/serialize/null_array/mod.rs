#[must_use]
pub const fn serialize_null_array() -> [u8; 5] {
    *b"*-1\r\n"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialzie_null_array_test() {
        let bytes = serialize_null_array();
        assert_eq!(bytes, *b"*-1\r\n");
    }
}
