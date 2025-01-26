use info::DeserializeInfo;

use super::*;

pub struct Deserializer<'a> {
    pub bytes: &'a [u8],
    pub offset: usize,
}

impl<'a> Deserializer<'a> {
    pub fn new(bytes: &'a [u8], offset: usize) -> Self {
        Self { bytes, offset }
    }
    pub fn advance<F, T>(&mut self, f: F) -> anyhow::Result<DeserializeInfo<T>>
    where
        F: Fn(&[u8]) -> anyhow::Result<DeserializeInfo<T>>,
    {
        let info = f(self.bytes)?;
        self.offset += info.bytes_read;
        Ok(info)
    }
    pub fn deserialize<F, T, R>(self, f: F) -> anyhow::Result<DeserializeInfo<T>>
    where
        F: Fn(&[u8]) -> anyhow::Result<R>,
        R: Into<DeserializeInfo<T>>,
    {
        let mut info: DeserializeInfo<T> = f(&self.bytes[self.offset..])?.into();
        info.bytes_read += self.offset;
        Ok(info)
    }

    pub fn deserialize_header<F, T, R>(self, null: T, f: F) -> anyhow::Result<DeserializeInfo<T>>
    where
        F: Fn(&[u8], usize) -> anyhow::Result<R>,
        R: Into<DeserializeInfo<T>>,
    {
        dbg!(&self.bytes[self.offset..]);
        let (payload_size, header_size) = self.bytes[self.offset..].get_header().unwrap();
        dbg!(payload_size);
        if payload_size == -1 {
            return Ok(DeserializeInfo::new(null, self.offset + header_size));
        }
        if payload_size < 0 {
            bail!("negative payload size: {payload_size}");
        }
        assert!(payload_size > 0, "negative payload size: {payload_size}");
        let payload_size = payload_size.try_into().unwrap();
        let info = f(&self.bytes[self.offset + header_size..], payload_size)?.into();
        Ok(DeserializeInfo::new(
            info.value,
            info.bytes_read + self.offset + header_size,
        ))
    }
}
