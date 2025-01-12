pub const fn serialize_null_array() -> [u8; 5] {
    *b"*-1\r\n"
}
