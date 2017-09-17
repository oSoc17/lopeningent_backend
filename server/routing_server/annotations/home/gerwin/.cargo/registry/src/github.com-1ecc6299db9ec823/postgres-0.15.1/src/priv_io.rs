               :use std::io::{self, BufWriter, Read, Write};
               :use std::fmt;
               :use std::net::TcpStream;
               :use std::time::Duration;
               :use std::result;
               :use bytes::{BufMut, BytesMut};
               :#[cfg(unix)]
               :use std::os::unix::net::UnixStream;
               :#[cfg(unix)]
               :use std::os::unix::io::{AsRawFd, RawFd};
               :#[cfg(windows)]
               :use std::os::windows::io::{AsRawSocket, RawSocket};
               :use postgres_protocol::message::frontend;
               :use postgres_protocol::message::backend;
               :
               :use {Result, TlsMode};
               :use error;
               :use tls::TlsStream;
               :use params::{ConnectParams, Host};
               :
               :const INITIAL_CAPACITY: usize = 8 * 1024;
               :
               :pub struct MessageStream {
               :    stream: BufWriter<Box<TlsStream>>,
               :    in_buf: BytesMut,
               :    out_buf: Vec<u8>,
               :}
               :
               :impl MessageStream {
               :    pub fn new(stream: Box<TlsStream>) -> MessageStream {
               :        MessageStream {
               :            stream: BufWriter::new(stream),
               :            in_buf: BytesMut::with_capacity(INITIAL_CAPACITY),
               :            out_buf: vec![],
               :        }
               :    }
               :
               :    pub fn get_ref(&self) -> &Box<TlsStream> {
               :        self.stream.get_ref()
               :    }
               :
               :    pub fn write_message<F, E>(&mut self, f: F) -> result::Result<(), E>
               :    where
               :        F: FnOnce(&mut Vec<u8>) -> result::Result<(), E>,
               :        E: From<io::Error>,
               :    {
               :        self.out_buf.clear();
               :        f(&mut self.out_buf)?;
               :        self.stream.write_all(&self.out_buf).map_err(From::from)
               :    }
               :
    18 4.8e-04 :    pub fn read_message(&mut self) -> io::Result<backend::Message> { /* postgres::priv_io::MessageStream::read_message::h632468b36b61ba31 total:    244  0.0064 */
               :        loop {
               :            match backend::Message::parse(&mut self.in_buf) {
    16 4.2e-04 :                Ok(Some(message)) => return Ok(message),
     3 7.9e-05 :                Ok(None) => self.read_in()?,
     1 2.6e-05 :                Err(e) => return Err(e),
               :            }
     6 1.6e-04 :        }
     6 1.6e-04 :    }
               :
               :    fn read_in(&mut self) -> io::Result<()> { /* postgres::priv_io::MessageStream::read_in::ha0ae4f8c4d243094 total:      3 7.9e-05 */
               :        self.in_buf.reserve(1);
               :        match self.stream.get_mut().read(
               :            unsafe { self.in_buf.bytes_mut() },
               :        ) {
               :            Ok(0) => Err(io::Error::new(
               :                io::ErrorKind::UnexpectedEof,
               :                "unexpected EOF",
               :            )),
               :            Ok(n) => {
               :                unsafe { self.in_buf.advance_mut(n) };
               :                Ok(())
               :            }
               :            Err(e) => Err(e),
               :        }
     1 2.6e-05 :    }
               :
               :    pub fn read_message_timeout(
               :        &mut self,
               :        timeout: Duration,
               :    ) -> io::Result<Option<backend::Message>> {
               :        if self.in_buf.is_empty() {
               :            self.set_read_timeout(Some(timeout))?;
               :            let r = self.read_in();
               :            self.set_read_timeout(None)?;
               :
               :            match r {
               :                Ok(()) => {}
               :                Err(ref e)
               :                    if e.kind() == io::ErrorKind::WouldBlock ||
               :                           e.kind() == io::ErrorKind::TimedOut => return Ok(None),
               :                Err(e) => return Err(e),
               :            }
               :        }
               :
               :        self.read_message().map(Some)
               :    }
               :
               :    pub fn read_message_nonblocking(&mut self) -> io::Result<Option<backend::Message>> {
               :        if self.in_buf.is_empty() {
               :            self.set_nonblocking(true)?;
               :            let r = self.read_in();
               :            self.set_nonblocking(false)?;
               :
               :            match r {
               :                Ok(()) => {}
               :                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => return Ok(None),
               :                Err(e) => return Err(e),
               :            }
               :        }
               :
               :        self.read_message().map(Some)
               :    }
               :
               :    pub fn flush(&mut self) -> io::Result<()> {
               :        self.stream.flush()
               :    }
               :
               :    fn set_read_timeout(&self, timeout: Option<Duration>) -> io::Result<()> {
               :        match self.stream.get_ref().get_ref().0 {
               :            InternalStream::Tcp(ref s) => s.set_read_timeout(timeout),
               :            #[cfg(unix)]
               :            InternalStream::Unix(ref s) => s.set_read_timeout(timeout),
               :        }
               :    }
               :
               :    fn set_nonblocking(&self, nonblock: bool) -> io::Result<()> {
               :        match self.stream.get_ref().get_ref().0 {
               :            InternalStream::Tcp(ref s) => s.set_nonblocking(nonblock),
               :            #[cfg(unix)]
               :            InternalStream::Unix(ref s) => s.set_nonblocking(nonblock),
               :        }
               :    }
               :}
               :
               :/// A connection to the Postgres server.
               :///
               :/// It implements `Read`, `Write` and `TlsStream`, as well as `AsRawFd` on
               :/// Unix platforms and `AsRawSocket` on Windows platforms.
               :pub struct Stream(InternalStream);
               :
               :impl fmt::Debug for Stream {
               :    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
               :        match self.0 {
               :            InternalStream::Tcp(ref s) => fmt::Debug::fmt(s, fmt),
               :            #[cfg(unix)]
               :            InternalStream::Unix(ref s) => fmt::Debug::fmt(s, fmt),
               :        }
               :    }
               :}
               :
               :impl Read for Stream {
     1 2.6e-05 :    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> { /* _$LT$postgres..priv_io..Stream$u20$as$u20$std..io..Read$GT$::read::h263636e2517634db total:      4 1.1e-04 */
               :        self.0.read(buf)
     2 5.3e-05 :    }
               :}
               :
               :impl Write for Stream {
               :    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
               :        self.0.write(buf)
               :    }
               :
               :    fn flush(&mut self) -> io::Result<()> {
               :        self.0.flush()
               :    }
               :}
               :
               :impl TlsStream for Stream {
               :    fn get_ref(&self) -> &Stream {
               :        self
               :    }
               :
               :    fn get_mut(&mut self) -> &mut Stream {
               :        self
               :    }
               :}
               :
               :#[cfg(unix)]
               :impl AsRawFd for Stream {
               :    fn as_raw_fd(&self) -> RawFd {
               :        match self.0 {
               :            InternalStream::Tcp(ref s) => s.as_raw_fd(),
               :            InternalStream::Unix(ref s) => s.as_raw_fd(),
               :        }
               :    }
               :}
               :
               :#[cfg(windows)]
               :impl AsRawSocket for Stream {
               :    fn as_raw_socket(&self) -> RawSocket {
               :        // Unix sockets aren't supported on windows, so no need to match
               :        match self.0 {
               :            InternalStream::Tcp(ref s) => s.as_raw_socket(),
               :        }
               :    }
               :}
               :
               :enum InternalStream {
               :    Tcp(TcpStream),
               :    #[cfg(unix)]
               :    Unix(UnixStream),
               :}
               :
               :impl Read for InternalStream {
               :    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
               :        match *self {
     1 2.6e-05 :            InternalStream::Tcp(ref mut s) => s.read(buf),
               :            #[cfg(unix)]
               :            InternalStream::Unix(ref mut s) => s.read(buf),
               :        }
               :    }
               :}
               :
               :impl Write for InternalStream {
               :    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
               :        match *self {
               :            InternalStream::Tcp(ref mut s) => s.write(buf),
               :            #[cfg(unix)]
               :            InternalStream::Unix(ref mut s) => s.write(buf),
               :        }
               :    }
               :
               :    fn flush(&mut self) -> io::Result<()> {
               :        match *self {
               :            InternalStream::Tcp(ref mut s) => s.flush(),
               :            #[cfg(unix)]
               :            InternalStream::Unix(ref mut s) => s.flush(),
               :        }
               :    }
               :}
               :
               :fn open_socket(params: &ConnectParams) -> Result<InternalStream> {
               :    let port = params.port();
               :    match *params.host() {
               :        Host::Tcp(ref host) => {
               :            Ok(TcpStream::connect(&(&**host, port)).map(
               :                InternalStream::Tcp,
               :            )?)
               :        }
               :        #[cfg(unix)]
               :        Host::Unix(ref path) => {
               :            let path = path.join(&format!(".s.PGSQL.{}", port));
               :            Ok(UnixStream::connect(&path).map(InternalStream::Unix)?)
               :        }
               :        #[cfg(not(unix))]
               :        Host::Unix(..) => {
               :            Err(
               :                io::Error::new(
               :                    io::ErrorKind::InvalidInput,
               :                    "unix sockets are not supported on this system",
               :                ).into(),
               :            )
               :        }
               :    }
               :}
               :
               :pub fn initialize_stream(params: &ConnectParams, tls: TlsMode) -> Result<Box<TlsStream>> {
               :    let mut socket = Stream(open_socket(params)?);
               :
               :    let (tls_required, handshaker) = match tls {
               :        TlsMode::None => return Ok(Box::new(socket)),
               :        TlsMode::Prefer(handshaker) => (false, handshaker),
               :        TlsMode::Require(handshaker) => (true, handshaker),
               :    };
               :
               :    let mut buf = vec![];
               :    frontend::ssl_request(&mut buf);
               :    socket.write_all(&buf)?;
               :    socket.flush()?;
               :
               :    let mut b = [0; 1];
               :    socket.read_exact(&mut b)?;
               :    if b[0] == b'N' {
               :        if tls_required {
               :            return Err(error::tls("the server does not support TLS".into()));
               :        } else {
               :            return Ok(Box::new(socket));
               :        }
               :    }
               :
               :    let host = match *params.host() {
               :        Host::Tcp(ref host) => host,
               :        // Postgres doesn't support TLS over unix sockets
               :        Host::Unix(_) => return Err(::bad_response().into()),
               :    };
               :
               :    handshaker.tls_handshake(host, socket).map_err(error::tls)
               :}
/* 
 * Total samples for file : "/home/gerwin/.cargo/registry/src/github.com-1ecc6299db9ec823/postgres-0.15.1/src/priv_io.rs"
 * 
 *     55  0.0015
 */


/* 
 * Command line: opannotate --source --output-dir=annotations ./target/release/routing_server 
 * 
 * Interpretation of command line:
 * Output annotated source file with samples
 * Output all files
 * 
 * CPU: Intel Ivy Bridge microarchitecture, speed 3100 MHz (estimated)
 * Counted CPU_CLK_UNHALTED events (Clock cycles when not halted) with a unit mask of 0x00 (No unit mask) count 90000
 */
