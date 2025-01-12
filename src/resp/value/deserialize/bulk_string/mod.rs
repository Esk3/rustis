use crate::resp::value::deserialize::util::FindLinefeed;

#[cfg(test)]
mod tests;

pub fn deserialize_bulk_string(bytes: &[u8], length: usize) -> anyhow::Result<(Vec<u8>, usize)> {
    assert!(&bytes[length..].is_at_linefeed().unwrap());
    let s = bytes[..length].to_vec();
    Ok((s, length + 2))
}
