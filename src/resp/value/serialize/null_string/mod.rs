#[cfg(test)]
mod tests;

#[must_use]
pub const fn serialize_null_string() -> [u8; 5] {
    *b"$-1\r\n"
}
