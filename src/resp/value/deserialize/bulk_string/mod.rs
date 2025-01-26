use crate::resp::value::deserialize::util::FindLinefeed;

#[cfg(test)]
mod tests;

pub fn deserialize_bulk_string(
    bytes: &[u8],
    length: usize,
) -> anyhow::Result<super::info::DeserializeInfo<Vec<u8>>> {
    assert!(&bytes[length..].is_at_linefeed().unwrap(),);
    let s = bytes[..length].to_vec();
    Ok(super::info::DeserializeInfo::new(s, length + 2))
}
