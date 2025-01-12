use super::util::FindLinefeed;

#[cfg(test)]
mod tests;

pub fn deserialize_simple_string(bytes: &[u8]) -> anyhow::Result<(String, usize)> {
    let linefeed = bytes.find_linefeed().unwrap().unwrap();
    Ok((
        String::from_utf8(bytes[..linefeed].to_vec()).unwrap(),
        linefeed + 2,
    ))
}
