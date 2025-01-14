use super::Stream;

pub struct TcpStream(std::net::TcpStream);

impl TcpStream {
    #[must_use]
    pub fn new(stream: std::net::TcpStream) -> Self {
        Self(stream)
    }
}

impl From<std::net::TcpStream> for TcpStream {
    fn from(value: std::net::TcpStream) -> Self {
        Self::new(value)
    }
}

impl std::io::Read for TcpStream {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

impl std::io::Write for TcpStream {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

impl Stream for TcpStream {
    type Addr = std::net::SocketAddrV4;

    fn connect(addr: Self::Addr) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(Self(std::net::TcpStream::connect(addr)?))
    }

    fn peer_addr(&self) -> Self::Addr {
        let std::net::SocketAddr::V4(addr) = self.0.peer_addr().unwrap() else {
            unreachable!()
        };
        addr
    }
}
