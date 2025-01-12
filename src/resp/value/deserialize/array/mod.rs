use crate::resp::Value;

#[cfg(test)]
mod tests;

pub fn deserialize_array(mut bytes: &[u8], items: usize) -> anyhow::Result<(Vec<Value>, usize)> {
    let mut result = Vec::with_capacity(items);
    let mut length = 0;
    for _ in 0..items {
        let (value, bytes_consumed) = super::deserialize_value(bytes)?;
        result.push(value);
        length += bytes_consumed;
        bytes = &bytes[bytes_consumed..];
    }
    Ok((result, length))
}
