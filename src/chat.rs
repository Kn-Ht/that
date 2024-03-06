use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;

pub struct Chat {
    pub self_addr: Option<SocketAddr>,

    pub conn: Option<TcpStream>,
}

impl Chat {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> io::Result<Self> {
        let addr = &addr.into();
        Ok(Self {
            conn: None,
            self_addr: None,
        })
    }
    /// Connect to a remote address
    pub fn connect<A: Into<SocketAddr>>(&mut self, with: A) -> io::Result<()> {
        self.conn = Some(TcpStream::connect_timeout(
            &with.into(),
            Duration::from_secs(120),
        )?);


        Ok(())
    }
}
