use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{JoinHandle, Thread};
use std::time::Duration;
use std::{io, thread};

pub enum Connection {
    Listener(TcpListener),
    Client(TcpStream)
}

pub struct TcpThread<T, R> {
    pub handle: JoinHandle<R>,
    receiver: Receiver<io::Result<T>>,
}

impl<T, R> TcpThread<T, R> {
    pub fn new(handle: JoinHandle<R>) -> (Self, Sender<io::Result<T>>) {
        let (sender, receiver) = mpsc::channel();
        (Self {
            handle,
            receiver,
        }, sender)
    }
}

pub struct Chat {
    pub self_addr: Option<SocketAddr>,
    pub tcp_handle: Option<TcpThread<Connection, ()>>,

    pub conn: Option<Connection>,
}

impl Chat {
    pub fn new<A: Into<SocketAddr>>(addr: A) -> Self {
        let addr = &addr.into();

        Self {
            conn: None,
            tcp_handle: None,
            self_addr: None,
        }
    }

    /// Connect to a remote address
    pub fn connect<A: Into<SocketAddr> + Send + Sync + 'static>(
        &mut self,
        with: A,
    ) -> TcpThread<TcpStream, ()> {
        let (sender, receiver) = mpsc::channel();

        let handle = thread::spawn(move || {
            let stream = TcpStream::connect_timeout(&with.into(), Duration::from_secs(120));
            match stream {
                Ok(stream) => {
                    sender.send(Ok(stream)).unwrap();
                }
                Err(e) => {
                    sender.send(Err(e)).unwrap();
                }
            }
        });

        TcpThread {
            handle,
            receiver
        }
    }

    pub fn listen(&mut self) -> io::Result<()> {
        let listener = TcpListener::bind("0.0.0.0:8997")?;
        self.conn = Some(Connection::Listener(listener));

        Ok(())
    }
}
