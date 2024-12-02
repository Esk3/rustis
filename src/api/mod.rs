use crate::{
    connection::{request, ConnectionInputOutput},
    resp::decoder::{self, Value},
};

pub struct Api<R, W> {
    r: R,
    w: W,
}

impl<'a> Api<std::io::BufReader<&'a std::net::TcpStream>, &'a std::net::TcpStream> {
    #[must_use]
    pub fn from_tcp_stream(s: &'a std::net::TcpStream) -> Self {
        let r = std::io::BufReader::new(s);
        Self { r, w: s }
    }
}

impl<R, W> Api<R, W>
where
    R: std::io::Read,
    W: std::io::Write,
{
    #[must_use]
    pub fn new(r: R, w: W) -> Self {
        Self { r, w }
    }
}

impl<R, W> Api<R, W> {
    fn parse_token(token: Value) -> Result<request::Request, ()> {
        let Value::Array(mut array) = token else {
            panic!()
        };
        if array[0].eq_ignore_ascii_case("ping") {
            Ok(request::Request::Ping)
        } else if array[0].eq_ignore_ascii_case("echo") {
            let echo = array.swap_remove(1).into_string().unwrap();
            Ok(request::Request::Echo(echo))
        } else if array[0].eq_ignore_ascii_case("get") {
            todo!()
        } else if array[0].eq_ignore_ascii_case("set") {
            todo!()
        } else {
            Err(())
        }
    }
}

impl<R, W> ConnectionInputOutput for Api<R, W>
where
    R: std::io::Read,
    W: std::io::Write,
{
    fn get_request(&mut self) -> Result<crate::connection::request::Request, ()> {
        let (token, bytes_read) = decoder::decode_value(&mut self.r).unwrap();
        Self::parse_token(token)
    }

    fn send_response(&mut self, response: crate::connection::response::Response) -> Result<(), ()> {
        match response {
            crate::connection::response::Response::SendOk => self.w.write_all(b"+Ok\r\n").unwrap(),
            crate::connection::response::Response::SendBytes(_) => todo!(),
            crate::connection::response::Response::SendNull => {
                self.w.write_all(b"*-1\r\n").unwrap();
            }
            crate::connection::response::Response::None => (),
            crate::connection::response::Response::SendPong => {
                self.w.write_all(b"+PONG\r\n").unwrap();
            }
            crate::connection::response::Response::SendBulkString(_) => todo!(),
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    #[test]
    fn ping() {
        let mut r = std::io::Cursor::new(Vec::new());
        r.write_all(b"*1\r\n$4\r\nPING\r\n").unwrap();
        r.set_position(0);
        let mut w = std::io::Cursor::new(Vec::new());
        let mut api = Api::new(&mut r, &mut w);
        let request = api.get_request().unwrap();
        assert_eq!(request, request::Request::Ping);
    }
}
