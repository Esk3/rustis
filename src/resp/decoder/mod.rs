pub fn decode_value<R>(mut r: &mut R) -> Result<Value, ()>
where
    for<'a> &'a mut R: std::io::Read,
{
    let header = decode_header(&mut r).unwrap();
    Ok(match header {
        Identifier::SimpleString => Value::String(decode_simple_string(&mut r).unwrap()),
        Identifier::BulkString(size) => match decode_bulk_string(size, &mut r).unwrap() {
            Some(buf) => Value::Buf(buf),
            None => Value::NullBuf,
        },
        Identifier::Array(size) => decode_array(size, r)?,
    })
}

fn decode_identifier<R>(mut r: R)
where
    R: std::io::Read,
{
    let mut buf = [0; 1];
    r.read_exact(&mut buf);
    match buf {
        _ => todo!(),
    }
}

fn decode_header<R>(mut r: R) -> Result<Identifier, ()>
where
    R: std::io::Read,
{
    let mut buf = [0; 1];
    r.read_exact(&mut buf).unwrap();
    dbg!(buf[0] as char);
    Ok(match buf[0] {
        b'+' => Identifier::SimpleString,
        b'$' => Identifier::BulkString(read_digits(r).unwrap()),
        b'*' => Identifier::Array(read_digits(r).unwrap()),
        _ => return Err(()),
    })
}
fn read_digits<R>(r: R) -> std::io::Result<i64>
where
    R: std::io::Read,
{
    let buf = read_to_linefeed(r)?;
    let s = String::from_utf8(buf).unwrap();
    Ok(s.parse().unwrap())
}

fn decode_simple_string<R>(r: R) -> std::io::Result<String>
where
    R: std::io::Read,
{
    let buf = read_to_linefeed(r)?;
    let s = String::from_utf8(buf).unwrap();
    Ok(s)
}

fn decode_bulk_string<R>(size: i64, mut r: R) -> std::io::Result<Option<Vec<u8>>>
where
    R: std::io::Read,
{
    if size == -1 {
        return Ok(None);
    }

    assert!(size >= 0);

    let mut buf = vec![0; size as usize];
    r.read_exact(&mut buf)?;
    line_feed(r);
    Ok(Some(buf))
}

fn decode_array<R>(size: i64, r: &mut R) -> Result<Value, ()>
where
    for<'a> &'a mut R: std::io::Read,
{
    if size == -1 {
        return Ok(Value::NullArray);
    }

    assert!(size >= 0);

    let mut buf = Vec::with_capacity(size as usize);
    for _ in 0..size {
        let value = decode_value(r).unwrap();
        buf.push(value);
    }
    Ok(Value::Array(buf))
}

fn line_feed<R>(mut r: R) -> std::io::Result<()>
where
    R: std::io::Read,
{
    let mut buf = [0; 2];
    r.read_exact(&mut buf)?;
    assert_eq!(buf, *b"\r\n", "{} {}", buf[0] as char, buf[1] as char);
    Ok(())
}

fn read_to_linefeed<R>(mut r: R) -> std::io::Result<Vec<u8>>
where
    R: std::io::Read,
{
    let mut buf = Vec::new();
    let mut byte = [0; 1];
    loop {
        r.read_exact(&mut byte)?;
        if byte[0] == b'\r' {
            break;
        }
        buf.push(byte[0]);
    }
    r.read_exact(&mut byte).unwrap();
    assert_eq!(byte[0], b'\n');
    Ok(buf)
}

pub enum Identifier {
    SimpleString,
    BulkString(i64),
    Array(i64),
}

#[derive(Debug)]
pub enum Value {
    String(String),
    Buf(Vec<u8>),
    Array(Vec<Value>),
    NullBuf,
    NullArray,
}

impl Value {
    pub fn into_string(self) -> Result<String, Self> {
        match self {
            Value::String(s) => Ok(s),
            Value::Buf(buf) => match String::from_utf8(buf) {
                Ok(s) => Ok(s),
                Err(e) => Err(Self::Buf(e.into_bytes())),
            },
            _ => Err(self),
        }
    }
    pub fn into_string_lossy(self) -> Result<String, Self> {
        match self {
            Value::String(s) => Ok(s),
            Value::Buf(buf) => Ok(String::from_utf8_lossy(&buf).to_string()),
            _ => Err(self),
        }
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Value::String(s) => s == other,
            Value::Buf(buf) => other.chars().enumerate().all(|(i, c)| buf[i] == c as u8),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_string() {
        let mut r = std::io::Cursor::new(b"+hello world\r\nendstuff".to_vec());
        let x = decode_value(&mut r).unwrap();
        insta::assert_debug_snapshot!(x);
    }

    #[test]
    fn bulk_string() {
        let mut r = std::io::Cursor::new(b"$5\r\nhello\r\n".to_vec());
        let x = decode_value(&mut r).unwrap();
        insta::assert_debug_snapshot!(x);
        insta::assert_debug_snapshot!(x.into_string().unwrap());
        let pos = r.position();
        let len = r.into_inner().len();
        assert_eq!(pos as usize, len);
    }

    #[test]
    fn array() {
        let mut r = std::io::Cursor::new(
            b"*3\r\n+TheSimple start\r\n*2\r\n+Abc\r\n$2\r\nhi\r\n$4\r\nlast\r\n".to_vec(),
        );
        let x = decode_value(&mut r).unwrap();
        insta::assert_debug_snapshot!(x);
    }

    #[test]
    fn string_cmp() {
        assert_eq!(Value::String("hello".to_string()), "hello");
        assert_eq!(Value::Buf(b"hello".to_vec()), "hello");

        assert_ne!(Value::String("hello".to_string()), "bye");
        assert_ne!(Value::Buf(b"hello".to_vec()), "bye");

        assert_ne!(Value::Array(Vec::new()), "anything");
        assert_ne!(Value::NullBuf, "anything");
        assert_ne!(Value::NullArray, "anything");
    }
}
