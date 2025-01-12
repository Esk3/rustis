#[cfg(test)]
mod tests;

pub const fn serialize_null_string() -> [u8; 5] {
    *b"$-1\r\n"
}
