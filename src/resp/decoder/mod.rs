use std::io::{BufRead, Read};

pub struct Decoder<R> {
    r: std::io::BufReader<R>,
}

impl<'a> Decoder<std::io::Cursor<&'a [u8]>> {
    pub fn decode_from_buf(buf: &'a [u8]) -> Result<Token, ()> {
        Self::new(std::io::Cursor::new(buf)).decode_token()
    }
}

impl<R> Decoder<R>
where
    R: std::io::Read,
{
    pub fn new(r: R) -> Self {
        Self {
            r: std::io::BufReader::new(r),
        }
    }

    pub fn decode_token(&mut self) -> Result<Token, ()> {
        let header = self.decode_header().unwrap();
        Ok(match header {
            Identifier::SimpleString => Token::String(self.decode_simple_string().unwrap()),
            Identifier::BulkString(size) => match self.decode_bulk_string(size).unwrap() {
                Some(buf) => Token::Buf(buf),
                None => Token::NullBuf,
            },
            Identifier::Array(size) => {
                self.decode_array();
                todo!()
            }
        })
    }

    fn decode_identifier(&mut self) {
        let mut buf = [0; 1];
        self.r.read_exact(&mut buf);
        match buf {
            _ => todo!(),
        }
    }

    fn decode_header(&mut self) -> Result<Identifier, ()> {
        //self.decode_identifier();
        let mut buf = [0; 1];
        self.r.read_exact(&mut buf).unwrap();
        Ok(match buf[0] {
            b'+' => Identifier::SimpleString,
            b'$' => Identifier::BulkString(self.read_digits().unwrap()),
            b'*' => Identifier::Array(self.read_digits().unwrap()),
            _ => return Err(()),
        })
    }
    fn read_digits(&mut self) -> std::io::Result<i64> {
        let mut buf = String::new();
        self.r.read_line(&mut buf)?;
        Ok(buf.parse().unwrap())
    }

    fn decode_simple_string(&mut self) -> std::io::Result<String> {
        let mut buf = String::new();
        self.r.read_line(&mut buf)?;
        buf.truncate(buf.len() - 2);
        Ok(buf)
    }

    fn decode_bulk_string(&mut self, size: i64) -> std::io::Result<Option<Vec<u8>>> {
        if size == -1 {
            return Ok(None);
        }
        assert!(size >= 0);
        let mut buf = vec![0; size as usize];
        self.r.read_exact(&mut buf)?;
        Ok(Some(buf))
    }

    fn decode_array(&mut self) {}

    fn line_feed(&mut self) -> std::io::Result<()> {
        let mut buf = [0; 2];
        self.r.read_exact(&mut buf)?;
        assert_eq!(buf, *b"\r\n", "{} {}", buf[0] as char, buf[1] as char);
        Ok(())
    }
}

pub enum Identifier {
    SimpleString,
    BulkString(i64),
    Array(i64),
}

#[derive(Debug)]
pub enum Token {
    String(String),
    Buf(Vec<u8>),
    Array(Box<Token>),
    NullBuf,
    NullArray,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_string() {
        let x = Decoder::decode_from_buf(b"+hello world\r\nmorestuff").unwrap();
        insta::assert_debug_snapshot!(x);
    }
}
