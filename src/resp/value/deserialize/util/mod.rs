use crate::resp::value::identifier::Identifier;
use anyhow::{anyhow, bail};

#[cfg(test)]
mod tests;

pub fn deserialize_header(mut bytes: &[u8]) -> anyhow::Result<(isize, usize)> {
    let mut length = 0;
    if bytes
        .first()
        .is_some_and(|byte| Identifier::from_byte(*byte).is_ok())
    {
        bytes = &bytes[1..];
        length += 1;
    }
    let linefeed = bytes
        .find_linefeed()?
        .ok_or(anyhow!("linefeed not found"))?;
    length += linefeed + 2;
    let digits = &bytes[..linefeed];
    let digits = String::from_utf8(digits.to_vec())?;
    let number = digits.parse()?;
    Ok((number, length))
}

pub fn is_linefeed(cr: u8, lf: u8) -> anyhow::Result<bool> {
    if cr == b'\n' {
        bail!("found newline before cr");
    }
    if cr == b'\r' {
        if lf != b'\n' {
            bail!("expected newline found: byte [{cr}] char: [{}]", cr as char);
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

pub trait FindLinefeed {
    fn find_linefeed(&self) -> anyhow::Result<Option<usize>>;
    fn is_at_linefeed(&self) -> anyhow::Result<bool>;
}

impl FindLinefeed for [u8] {
    fn find_linefeed(&self) -> anyhow::Result<Option<usize>> {
        for (i, win) in self.windows(2).enumerate() {
            let (cr, lf) = (win[0], win[1]);
            let is_linefeed = is_linefeed(cr, lf)?;
            if is_linefeed {
                return Ok(Some(i));
            }
        }
        if [b'\r', b'\n'].map(Some).contains(&self.last().copied()) {
            bail!("found single linefeed or cr");
        }

        Ok(None)
    }

    fn is_at_linefeed(&self) -> anyhow::Result<bool> {
        let (Some(cr), Some(lf)) = (self.first(), self.get(1)) else {
            return Ok(false);
        };
        is_linefeed(*cr, *lf)
    }
}

pub trait GetHeader {
    fn get_header(&self) -> anyhow::Result<(isize, usize)>;
}

impl GetHeader for [u8] {
    fn get_header(&self) -> anyhow::Result<(isize, usize)> {
        deserialize_header(self)
    }
}
